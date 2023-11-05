use std::sync::Arc;

use crate::OsClient;

mod builder;
mod types;
pub struct Ingest<'a> {
  os_client: &'a OsClient,
}

impl<'a> Ingest<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
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
  /// let response = client.get_pipeline()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_pipeline(&self) -> builder::IngestGetPipeline {
    builder::IngestGetPipeline::new(self.os_client)
  }

  ///Allows to simulate a pipeline with example documents.
  ///
  ///Sends a `GET` request to `/_ingest/pipeline/_simulate`
  ///
  ///Arguments:
  /// - `verbose`: Verbose mode. Display data output for each processor in
  ///   executed pipeline.
  ///```ignore
  /// let response = client.ingest_simulate_get()
  ///    .verbose(verbose)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_simulate_get(&self) -> builder::IngestSimulateGet {
    builder::IngestSimulateGet::new(self.os_client)
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
  /// let response = client.ingest_simulate_post()
  ///    .verbose(verbose)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_simulate_post(&self) -> builder::IngestSimulatePost {
    builder::IngestSimulatePost::new(self.os_client)
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
  /// let response = client.ingest_get_pipeline_with_id()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_get_pipeline_with_id(&self) -> builder::IngestGetPipelineWithId {
    builder::IngestGetPipelineWithId::new(self.os_client)
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
  /// let response = client.ingest_put_pipeline()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_put_pipeline(&self) -> builder::IngestPutPipeline {
    builder::IngestPutPipeline::new(self.os_client)
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
  /// let response = client.ingest_delete_pipeline()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_delete_pipeline(&self) -> builder::IngestDeletePipeline {
    builder::IngestDeletePipeline::new(self.os_client)
  }

  ///Allows to simulate a pipeline with example documents.
  ///
  ///Sends a `GET` request to `/_ingest/pipeline/{id}/_simulate`
  ///
  ///Arguments:
  /// - `id`: Pipeline ID.
  /// - `verbose`: Verbose mode. Display data output for each processor in
  ///   executed pipeline.
  ///```ignore
  /// let response = client.ingest_simulate_get_with_id()
  ///    .id(id)
  ///    .verbose(verbose)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_simulate_get_with_id(&self) -> builder::IngestSimulateGetWithId {
    builder::IngestSimulateGetWithId::new(self.os_client)
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
  /// let response = client.ingest_simulate_post_with_id()
  ///    .id(id)
  ///    .verbose(verbose)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_simulate_post_with_id(&self) -> builder::IngestSimulatePostWithId {
    builder::IngestSimulatePostWithId::new(self.os_client)
  }

  ///Returns a list of the built-in patterns.
  ///
  ///Sends a `GET` request to `/_ingest/processor/grok`
  ///
  ///```ignore
  /// let response = client.ingest_processor_grok()
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_processor_grok(&self) -> builder::IngestProcessorGrok {
    builder::IngestProcessorGrok::new(self.os_client)
  }
}
