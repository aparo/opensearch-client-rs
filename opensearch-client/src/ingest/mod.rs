use serde::Serialize;

use crate::OsClient;
mod builder;
mod types;
pub mod pipeline;

pub struct Ingest<'a> {
  os_client: &'a OsClient,
}

impl<'a> Ingest<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Allows to simulate a pipeline with example documents.
  ///
  ///Sends a `POST` request to `/_ingest/pipeline/_simulate`
  ///
  ///Arguments:
  /// - `verbose`: Verbose mode. Display data output for each processor in
  ///   executed pipeline.
  /// - `body`
  ///```ignore
  /// let response = client.ingest().simulate()
  ///    .verbose(verbose)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn simulate(&self) -> builder::Simulate {
    builder::Simulate::new(self.os_client)
  }

  ///Returns a pipeline.
  ///
  ///Sends a `GET` request to `/_ingest/pipeline`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.ingest().get_pipelines()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_pipelines(&self) -> builder::IngestGetPipeline {
    builder::IngestGetPipeline::new(self.os_client)
  }

  ///Returns a pipeline.
  ///
  ///Sends a `GET` request to `/_ingest/pipeline/{id}`
  ///
  ///Arguments:
  /// - `id`: Comma-separated list of pipeline ids. Wildcards supported.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.ingest().get_pipelines()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_pipeline(&self, id: &str) -> builder::IngestGetPipeline {
    builder::IngestGetPipeline::new(self.os_client).id(Some(id.to_owned()))
  }

  ///Creates or updates a pipeline.
  ///
  ///Sends a `PUT` request to `/_ingest/pipeline/{id}`
  ///
  ///Arguments:
  /// - `id`: Pipeline ID.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.ingest().put_pipeline(id, body)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_pipeline<T: Serialize>(&self, id: &str, body: T) -> builder::IngestPutPipeline<T> {
    builder::IngestPutPipeline::new(self.os_client).id(id).body(body)
  }

  ///Deletes a pipeline.
  ///
  ///Sends a `DELETE` request to `/_ingest/pipeline/{id}`
  ///
  ///Arguments:
  /// - `id`: Pipeline ID.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.ingest(id).delete_pipeline()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_pipeline(&self, id: &str) -> builder::IngestDeletePipeline {
    builder::IngestDeletePipeline::new(self.os_client).id(id)
  }

  ///Allows to simulate a pipeline with example documents.
  ///
  ///Sends a `POST` request to `/_ingest/pipeline/{id}/_simulate`
  ///
  ///Arguments:
  /// - `id`: Pipeline ID.
  /// - `verbose`: Verbose mode. Display data output for each processor in
  ///   executed pipeline.
  /// - `body`
  ///```ignore
  /// let response = client.ingest().simulate_with_id()
  ///    .id(id)
  ///    .verbose(verbose)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn simulate_with_id(&self, id: &str) -> builder::IngestSimulateWithId {
    builder::IngestSimulateWithId::new(self.os_client).id(id)
  }

  ///Returns a list of the built-in patterns.
  ///
  ///Sends a `GET` request to `/_ingest/processor/grok`
  ///
  ///```ignore
  /// let response = client.ingest().processor_grok()
  ///    .send()
  ///    .await;
  /// ```
  pub fn processor_grok(&self) -> builder::IngestProcessorGrok {
    builder::IngestProcessorGrok::new(self.os_client)
  }
}
