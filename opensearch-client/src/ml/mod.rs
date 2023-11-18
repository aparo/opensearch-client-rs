use serde::Serialize;

use crate::OsClient;
pub mod types;

pub struct ML<'a> {
  os_client: &'a OsClient,
}

impl<'a> ML<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Training can occur both synchronously and asynchronously..
  ///
  ///Sends a `POST` request to `/_plugins/_ml/_train/kmeans`
  ///
  ///Arguments:
  ///```ignore
  /// let response = client.ml().model_train()
  ///    .verbose(verbose)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn model_train(&self) -> types::ModelTrainRequestBuilder {
    types::ModelTrainRequestBuilder::default()
  }
}
