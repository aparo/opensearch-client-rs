use serde::Serialize;

use crate::{Error, OsClient, ResponseValue};
pub mod types;
use crate::types::DocumentDeleteResponse;

pub struct ML<'a> {
  os_client: &'a OsClient,
}

impl<'a> ML<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Training can occur both synchronously and asynchronously.
  ///
  ///Sends a `POST` request to `/_plugins/_ml/_train/kmeans`
  pub fn model_train(&self) -> types::ModelTrainRequestBuilder {
    types::ModelTrainRequestBuilder::default()
  }

  ///Get a model.
  ///
  ///Sends a `GET` request to `/_plugins/_ml/models/<model_id>`
  pub async fn get_model(&self, model_id: &str) -> Result<ResponseValue<types::Model>, Error> {
    self
      .os_client
      .send(types::GetModelRequest::new(model_id.to_string()))
      .await
  }

  ///Delete a model.
  ///
  ///Sends a `Delete` request to `/_plugins/_ml/models/<model_id>`
  pub async fn delete_model(&self, model_id: &str) -> Result<ResponseValue<DocumentDeleteResponse>, Error> {
    self
      .os_client
      .send(types::DeleteModelRequest::new(model_id.to_string()))
      .await
  }
}
