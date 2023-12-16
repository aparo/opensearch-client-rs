use std::sync::{Arc, Mutex};

use serde::Serialize;
use tokio::{
  sync::mpsc,
  task::JoinHandle,
  time::{sleep, Duration},
};

use crate::{
  types::bulk::{BulkAction, CreateAction, DeleteAction, IndexAction, UpdateAction, UpdateActionBody},
  Error, OsClient,
};

#[derive(Debug, Clone)]
struct Action {
  action: BulkAction,
  document: Option<String>,
}

/// Runner for translation
#[derive(Debug, Clone)]
pub struct Bulker {
  sender: mpsc::Sender<Action>,
  // Create a shared queue using Arc and Mutex
  queue: Arc<Mutex<Vec<Action>>>,
  // OPenSearch Client
  os_client: Arc<OsClient>,
  // Max items for bulk
  bulk_size: u32,
  // Max parallel bulks
  max_concurrent_connections: u32,
}

impl Bulker {
  /// Spawn an extraction service on a separate thread and return an extraction
  /// instance to interact with it
  pub fn new(os_client: Arc<OsClient>, bulk_size: u32, max_concurrent_connections: u32) -> (JoinHandle<()>, Bulker) {
    let (sender, receiver) = mpsc::channel::<Action>((bulk_size * (max_concurrent_connections + 1)) as usize);
    let service = Bulker {
      bulk_size,
      max_concurrent_connections,
      sender,
      os_client: os_client.clone(),
      queue: Arc::new(Mutex::new(Vec::new())),
    };
    let queue = service.queue.clone();
    let o_client = os_client.clone();
    // Spawn a background task to process the queue and make async Reqwest calls
    let handle = tokio::spawn(async move {
      process_queue(
        queue.clone(),
        receiver,
        o_client,
        bulk_size as usize,
        max_concurrent_connections as usize,
      )
      .await;
    });

    (handle, service)
  }

  /// Sends a bulk index request to OpenSearch with the specified index, id and
  /// document body.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index.
  /// * `id` - An optional string slice that holds the id of the document.
  /// * `body` - A reference to a serializable document body.
  ///
  /// # Returns
  ///
  /// Returns () on success, or an `Error` on failure.
  pub async fn index<T: Serialize>(&self, index: &String, body: &T, id: Option<String>) -> Result<(), Error> {
    let action = BulkAction::Index(IndexAction {
      index: index.to_owned(),
      id: id.clone(),
      pipeline: None,
    });
    self
      .sender
      .send(Action {
        action: action,
        document: Some(serde_json::to_string(&body)?),
      })
      .await
      .map_err(|e| Error::InternalError(format!("{}", e)))?;

    Ok(())
  }

  /// Sends a bulk create request to the OpenSearch cluster with the specified
  /// index, id and body.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index.
  /// * `id` - A string slice that holds the id of the document.
  /// * `body` - A generic type `T` that holds the body of the document to be
  ///   created.
  ///
  /// # Returns
  ///
  /// Returns () on success, or an `Error` on failure.
  pub async fn create<T: Serialize>(&self, index: &str, id: &str, body: &T) -> Result<(), Error> {
    let action = BulkAction::Create(CreateAction {
      index: index.to_owned(),
      id: id.to_owned(),
      ..Default::default()
    });
    self
      .sender
      .send(Action {
        action: action,
        document: Some(serde_json::to_string(&body)?),
      })
      .await
      .map_err(|e| Error::InternalError(format!("{}", e)))?;
    Ok(())
  }

  /// Sends a bulk delete request to the OpenSearch cluster with the specified
  /// index and id.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index.
  /// * `id` - A string slice that holds the id of the document.
  ///
  /// # Returns
  ///
  /// Returns () on success, or an `Error` on failure.
  pub async fn delete<T: Serialize>(&self, index: &str, id: &str) -> Result<(), Error> {
    let action = BulkAction::Delete(DeleteAction {
      index: index.to_owned(),
      id: id.to_owned(),
      ..Default::default()
    });
    self
      .sender
      .send(Action {
        action: action,
        document: None,
      })
      .await
      .map_err(|e| Error::InternalError(format!("{}", e)))?;
    Ok(())
  }

  /// Asynchronously updates a document in bulk.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index.
  /// * `id` - A string slice that holds the ID of the document to update.
  /// * `body` - An `UpdateAction` struct that holds the update action to
  ///   perform.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing a `serde_json::Value` on success, or an
  /// `Error` on failure.
  pub async fn update(&self, index: &str, id: &str, body: &UpdateActionBody) -> Result<(), Error> {
    let action = BulkAction::Update(UpdateAction {
      index: index.to_owned(),
      id: id.to_owned(),
      ..Default::default()
    });
    self
      .sender
      .send(Action {
        action: action,
        document: Some(serde_json::to_string(&body)?),
      })
      .await
      .map_err(|e| Error::InternalError(format!("{}", e)))?;
    Ok(())
  }
}

impl Drop for Bulker {
  fn drop(&mut self) {
    tokio::task::block_in_place(|| {
      tokio::runtime::Handle::current().block_on(async {
        // Clone the records from the queue to process synchronously
        let records_to_process: Vec<Action> = self.queue.lock().unwrap().clone();
        make_reqwest_calls(self.os_client.clone(), records_to_process).await;
        // Clear the queue
        self.queue.lock().unwrap().clear();
      });
    });
  }
}

async fn process_queue(
  queue: Arc<Mutex<Vec<Action>>>,
  mut receiver: mpsc::Receiver<Action>,
  os_client: Arc<OsClient>,
  bulk_size: usize,
  max_concurrent_connections: usize,
) -> Result<(), Error> {
  let mut reqwest_calls: Vec<tokio::task::JoinHandle<()>> = Vec::new();

  loop {
    tokio::select! {
        // Handle incoming JSON records
        Some(json_record) = receiver.recv() => {
            // Put the JSON record in the queue
            queue.lock().unwrap().push(json_record.clone());

            // Check conditions for making async Reqwest calls
            let queue_size = queue.lock().unwrap().len();
            let elapsed_time = reqwest_calls.iter().filter(|task| !task.is_finished()).count();

            if queue_size >= bulk_size || (queue_size > 0 && elapsed_time <= max_concurrent_connections) {
                // Clone the records from the queue to process asynchronously
                let records_to_process: Vec<Action> = queue.lock().unwrap().clone();

                // Clear the queue
                queue.lock().unwrap().clear();

                // Spawn an async task to make the Reqwest calls
                reqwest_calls.push(tokio::spawn(make_reqwest_calls(os_client.clone(), records_to_process)));
            }
        }
        // Handle elapsed time for Reqwest calls
        _ = sleep(Duration::from_secs(1)) => {
            reqwest_calls.retain(|task| !task.is_finished());
        }
    }
  }
}

async fn make_reqwest_calls(os_client: Arc<OsClient>, records: Vec<Action>) {
  let mut bulker = String::new();

  for record in records {
    let j = serde_json::to_string(&record.action).unwrap();
    bulker.push_str(j.as_str());
    bulker.push('\n');
    let j = serde_json::to_string(&record.document).unwrap();
    bulker.push_str(j.as_str());
    bulker.push('\n');
  }

  match os_client.bulk().body(bulker).send().await {
    Ok(_response) => {
      // if response.status().is_success() {
      //   println!("Request successful for record: {:?}", cloned_record);
      // } else {
      //   println!("Request failed for record: {:?}", cloned_record);
      // }
    }
    Err(e) => {
      eprintln!("Error making Reqwest call: {:?}", e);
    }
  }
}
