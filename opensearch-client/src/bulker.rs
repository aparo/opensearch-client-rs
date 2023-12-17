use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tokio::{
  sync::mpsc,
  task::JoinHandle,
  time::{sleep, Duration},
};
use tracing::{debug, error};

use crate::{
  types::bulk::{BulkAction, CreateAction, DeleteAction, IndexAction, UpdateAction, UpdateActionBody},
  Error, OsClient,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkerStatistic {
  // Number of deleted actions
  pub delete_actions: u64,
  // Number of created actions
  pub create_actions: u64,
  // Number of updated actions
  pub update_actions: u64,
  // Number of index actions
  pub index_actions: u64,
  // Number of records in the queue
  pub queue_size: usize,
  // Number of running Reqwest calls
  pub running_reqwest_calls: usize,
  // Number of total Reqwest calls
  pub total_reqwest_calls: usize,
  // Number of finished Reqwest calls
  pub finished_reqwest_calls: usize,
  // Number of error calls
  pub error_reqwest_calls: usize,
  // Number of action without errors
  pub success_actions: usize,
  // Number of action with errors
  pub error_actions: usize,
  // Number of creation action with errors
  pub error_create_actions: usize,
}

#[derive(Debug, Clone)]
struct Action {
  action: BulkAction,
  document: Option<String>,
}
// BulkerBuilder is a helper struct to build a Bulker instance.
#[derive(Debug, Clone)]
pub struct BulkerBuilder {
  // OpenSearch Client
  os_client: Arc<OsClient>,
  // Max items for bulk
  bulk_size: u32,
  // Max parallel bulks
  max_concurrent_connections: u32,
}

impl BulkerBuilder {
  /// Create a new BulkerBuilder instance.
  pub fn new(os_client: Arc<OsClient>, bulk_size: u32) -> Self {
    BulkerBuilder {
      os_client,
      bulk_size,
      max_concurrent_connections: 10,
    }
  }

  /// Set the bulk size.
  pub fn bulk_size(mut self, bulk_size: u32) -> Self {
    self.bulk_size = bulk_size;
    self
  }

  /// Set the max concurrent connections.
  pub fn max_concurrent_connections(mut self, max_concurrent_connections: u32) -> Self {
    self.max_concurrent_connections = max_concurrent_connections;
    self
  }

  /// Build a Bulker instance.
  pub fn build(self) -> Bulker {
    let (handle, bulker) = Bulker::new(self.os_client, self.bulk_size, self.max_concurrent_connections);
    tokio::spawn(async move {
      handle.await.unwrap();
    });
    bulker
  }
}

/// Bulker is a helper struct to make bulk requests to OpenSearch.
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
  // statistics
  statistics: Arc<Mutex<BulkerStatistic>>,
}

impl Bulker {
  /// Spawn an extraction service on a separate thread and return an extraction
  /// instance to interact with it
  pub fn new(os_client: Arc<OsClient>, bulk_size: u32, max_concurrent_connections: u32) -> (JoinHandle<()>, Bulker) {
    let (sender, receiver) = mpsc::channel::<Action>((bulk_size * (max_concurrent_connections + 1)) as usize);
    let statistics = Arc::new(Mutex::new(BulkerStatistic::default()));
    let service = Bulker {
      bulk_size,
      max_concurrent_connections,
      sender,
      os_client: os_client.clone(),
      queue: Arc::new(Mutex::new(Vec::new())),
      statistics: statistics.clone(),
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
        statistics.clone(),
      )
      .await
      .unwrap();
    });

    (handle, service)
  }

  pub fn statistics(&self) -> BulkerStatistic {
    self.statistics.lock().unwrap().clone()
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
  pub async fn index<T: Serialize>(&self, index: &str, body: &T, id: Option<String>) -> Result<(), Error> {
    let action = BulkAction::Index(IndexAction {
      index: index.to_owned(),
      id: id.clone(),
      pipeline: None,
    });
    self
      .sender
      .send(Action {
        action,
        document: Some(serde_json::to_string(&body)?),
      })
      .await
      .map_err(|e| Error::InternalError(format!("{}", e)))?;

    self.statistics.lock().unwrap().index_actions += 1;
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
        action,
        document: Some(serde_json::to_string(&body)?),
      })
      .await
      .map_err(|e| Error::InternalError(format!("{}", e)))?;
    self.statistics.lock().unwrap().create_actions += 1;
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
      .send(Action { action, document: None })
      .await
      .map_err(|e| Error::InternalError(format!("{}", e)))?;
    self.statistics.lock().unwrap().delete_actions += 1;
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
    self.statistics.lock().unwrap().update_actions += 1;
    Ok(())
  }

  // wait that every reqwest is completed
  pub async fn flush(&self) {
    loop {
      self.refresh_queue_size();
      let statistics = self.statistics.lock().unwrap();
      if statistics.finished_reqwest_calls == statistics.total_reqwest_calls && statistics.queue_size == 0 {
        break;
      }
      drop(statistics);
      sleep(Duration::from_secs(1)).await;
    }
  }

  fn refresh_queue_size(&self) {
    // we fresh the queue size
    let mut statistics = self.statistics.lock().unwrap();
    statistics.queue_size = self.queue.lock().unwrap().len();
  }
}

impl Drop for Bulker {
  fn drop(&mut self) {
    tokio::task::block_in_place(|| {
      tokio::runtime::Handle::current().block_on(async {
        // Clone the records from the queue to process synchronously
        let records_to_process: Vec<Action> = self.queue.lock().unwrap().clone();
        if records_to_process.len() > 0 {
          debug!("Bulker: Processing remaining records: {:?}", records_to_process.len());
          make_reqwest_calls(self.os_client.clone(), records_to_process, self.statistics.clone()).await;
        }
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
  statistics: Arc<Mutex<BulkerStatistic>>,
) -> Result<(), Error> {
  let mut reqwest_calls: Vec<tokio::task::JoinHandle<()>> = Vec::new();
  let mut start = std::time::Instant::now();
  loop {
    tokio::select! {
        // Handle incoming JSON records
        Some(json_record) = receiver.recv() => {
            // Put the JSON record in the queue
            queue.lock().unwrap().push(json_record.clone());

            // Check conditions for making async Reqwest calls
            let queue_size = queue.lock().unwrap().len();
            let running_reqwest_calls = reqwest_calls.iter().filter(|task| !task.is_finished()).count();
            {
                let mut statistics = statistics.lock().unwrap();
                statistics.queue_size = queue_size;
                statistics.running_reqwest_calls = running_reqwest_calls;
            }
            let end= std::time::Instant::now();

            if (queue_size >= bulk_size && running_reqwest_calls <= max_concurrent_connections) || (queue_size > 0 && end.duration_since(start).as_secs() > 1 && running_reqwest_calls <= max_concurrent_connections) {
                // Clone the records from the queue to process asynchronously
                let records_to_process: Vec<Action> = queue.lock().unwrap().clone();

                // Clear the queue
                queue.lock().unwrap().clear();
                {
                  let mut statistics = statistics.lock().unwrap();
                  statistics.total_reqwest_calls += 1;
                  statistics.queue_size = 0;
                }

                // Spawn an async task to make the Reqwest calls
                reqwest_calls.push(tokio::spawn(make_reqwest_calls(os_client.clone(), records_to_process, statistics.clone())));
                start= std::time::Instant::now();
            }
        }
        // Handle elapsed time for Reqwest calls
        _ = sleep(Duration::from_secs(1)) => {
            reqwest_calls.retain(|task| !task.is_finished());
        }
    }
    // Check if all the records have been processed or timeout to send data
    {
      let end = std::time::Instant::now();
      let queue_size = queue.lock().unwrap().len();
      let running_reqwest_calls = reqwest_calls.iter().filter(|task| !task.is_finished()).count();
      if (queue_size >= bulk_size && running_reqwest_calls <= max_concurrent_connections)
        || (queue_size > 0
          && end.duration_since(start).as_secs() > 1
          && running_reqwest_calls <= max_concurrent_connections)
      {
        // Clone the records from the queue to process asynchronously
        let records_to_process: Vec<Action> = queue.lock().unwrap().clone();

        // Clear the queue
        queue.lock().unwrap().clear();
        {
          let mut statistics = statistics.lock().unwrap();
          statistics.total_reqwest_calls += 1;
          statistics.queue_size = 0;
        }

        // Spawn an async task to make the Reqwest calls
        reqwest_calls.push(tokio::spawn(make_reqwest_calls(
          os_client.clone(),
          records_to_process,
          statistics.clone(),
        )));
        start = std::time::Instant::now();
      }
    }
  }
}

async fn make_reqwest_calls(os_client: Arc<OsClient>, records: Vec<Action>, statistics: Arc<Mutex<BulkerStatistic>>) {
  let mut bulker = String::new();
  let total = &records.len();

  for record in records {
    let j = serde_json::to_string(&record.action).unwrap();
    bulker.push_str(j.as_str());
    bulker.push('\n');
    if let Some(document) = record.document {
      bulker.push_str(document.as_str());
      bulker.push('\n');
    }
  }

  match os_client.bulk().body(bulker).send().await {
    Ok(response) => {
      let mut statistics = statistics.lock().unwrap();
      statistics.finished_reqwest_calls += 1;
      if response.status().is_success() {
        let bulk_response = response.into_inner();
        statistics.success_actions += bulk_response.count_ok();
        statistics.error_actions += bulk_response.count_errors();
        statistics.error_create_actions += bulk_response.count_create_errors();
        debug!("Request successful for record: {:?}", &bulk_response.items.len());
      } else {
        statistics.error_reqwest_calls += 1;
        statistics.error_actions += total;
        error!("Request failed for record: {:?}", response.into_inner().items.len());
      }
    }
    Err(e) => {
      let mut statistics = statistics.lock().unwrap();
      statistics.total_reqwest_calls += 1;
      statistics.finished_reqwest_calls += 1;
      statistics.error_reqwest_calls += 1;
      statistics.error_actions += total;
      let message = format!("Error making Reqwest call: {:?}", e);
      error!(message);
    }
  }
}

#[cfg(test)]
mod tests {
  use serde_json::json;
  use testcontainers::clients;
  use opensearch_testcontainer::*;
  use tracing_test::traced_test;

  use crate::{url::Url, OsClientBuilder};

  #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
  #[traced_test]
  async fn bulker_ingester() -> Result<(), Box<dyn std::error::Error>> {
    let docker = clients::Cli::default();
    let os_image = OpenSearch::default();
    let node = docker.run(os_image.clone());
    let host_port = node.get_host_port_ipv4(9200);

    let client = OsClientBuilder::new()
      .accept_invalid_certificates(true)
      .base_url(Url::parse(&format!("https://127.0.0.1:{host_port}")).unwrap())
      .basic_auth(os_image.username(), os_image.password())
      .build();

    let bulker = client.bulker().bulk_size(1000).max_concurrent_connections(10).build();
    for i in 0..10000 {
      bulker
        .index("test", &json!({"id":i}), Some(i.to_string()))
        .await
        .unwrap();
    }
    bulker.flush().await;
    let statitics = bulker.statistics();
    drop(bulker);

    assert_eq!(statitics.index_actions, 10000);
    assert_eq!(statitics.create_actions, 0);
    assert_eq!(statitics.delete_actions, 0);
    assert_eq!(statitics.update_actions, 0);
    client.indices().refresh_post().send().await.unwrap();

    let count = client.count().index("test").send().await.unwrap().into_inner();
    assert_eq!(count.count, 10000);
    Ok(())
  }
}
