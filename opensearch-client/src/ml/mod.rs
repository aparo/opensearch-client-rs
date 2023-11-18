use futures_util::{pin_mut, StreamExt};
use opensearch_dsl::{FieldSort, Query, SortCollection};

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

  // ///Search Models.
  // ///
  // ///Sends a `POST` request to `/_plugins/_ml/models/_search`
  // pub async fn search_models(&self, model_id: &str) ->
  // Result<ResponseValue<DocumentDeleteResponse>, Error> {   self
  //     .os_client
  //     .send(types::DeleteModelRequest::new(model_id.to_string()))
  //     .await
  // }

  // List all models
  pub async fn get_all_models(&self) -> Result<Vec<types::Model>, Error> {
    let query: Query = Query::match_all().into();
    let sort = SortCollection::new().field(FieldSort::descending("created_time"));
    let stream = self
      .os_client
      .search_stream::<types::Model>(".plugins-ml-model", &query, &sort, 10)
      .await?;
    pin_mut!(stream);

    let mut models = Vec::new();

    while let Some(hit) = stream.next().await {
      if let Some(model) = hit.source {
        models.push(model);
      }
    }
    Ok(models)
  }
}
