use std::default;

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Default, Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct ModelTrainRequest {
  pub parameters: ModelTrainRequestParameters,
  #[serde(rename = "input_query")]
  pub input_query: ModelTrainRequestInputQuery,
  #[serde(rename = "input_index")]
  pub input_index: Vec<String>,
  #[serde(skip_serializing, default)]
  pub async_execution: bool,
}

impl ModelTrainRequestBuilder {
  pub fn new() -> Self {
    Self::default()
  }
}

#[derive(Default, Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct ModelTrainRequestParameters {
  pub centroids: i64,
  pub iterations: i64,
  #[serde(rename = "distance_type")]
  pub distance_type: String,
}

#[derive(Default, Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct ModelTrainRequestInputQuery {
  #[serde(rename = "_source")]
  pub source: Vec<String>,
  pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelTrainResponse {
  #[serde(rename = "model_id")]
  pub model_id: String,
  pub status: ModelStatus,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum ModelStatus {
  #[default]
  #[serde(rename = "COMPLETED")]
  Completed,
  #[serde(rename = "CREATED")]
  Created,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModelState {
  #[serde(rename = "DEPLOYED")]
  Deployed,
  #[serde(rename = "TRAINED")]
  Trained,
  #[serde(rename = "UNDEPLOYED")]
  Unloaded,
  #[serde(rename = "REGISTERED")]
  Registered,
  #[serde(rename = "REGISTERING")]
  Registering,
  #[serde(rename = "DEPLOYING")]
  Deploying,
  #[serde(rename = "PARTIALLY_DEPLOYED")]
  PartiallyDeployed,
  #[serde(rename = "DEPLOY_FAILED")]
  DeployFailed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSearchModelBase {
  name: String,
  model_id: String,
  model_state: ModelState,
  model_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSearchSelfTrainedModel {
  #[serde(flatten)]
  base: OpenSearchModelBase,
  algorithm: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
  all_config: Option<String>,
  embedding_dimension: u32,
  framework_type: String,
  model_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSearchCustomerModel {
  #[serde(flatten)]
  base: OpenSearchModelBase,
  chunk_number: u32,
  created_time: u64,
  description: String,
  last_loaded_time: Option<u64>,
  last_unloaded_time: Option<u64>,
  last_uploaded_time: u64,
  model_config: ModelConfig,
  model_content: String,
  model_content_hash_value: String,
  model_content_size_in_bytes: String,
  model_format: String,
  total_chunks: u32,
  version: u32,
  planning_worker_nodes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModelSearchSort {
  #[serde(rename = "name-asc")]
  NameAsc,
  #[serde(rename = "name-desc")]
  NameDesc,
  #[serde(rename = "id-asc")]
  IdAsc,
  #[serde(rename = "model_state-asc")]
  ModelStateAsc,
  #[serde(rename = "model_state-desc")]
  ModelStateDesc,
  #[serde(rename = "id-desc")]
  IdDesc,
}
