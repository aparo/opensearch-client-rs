use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::Request;

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

impl Request for ModelTrainRequest {
  type Response = ModelTrainResponse;

  fn body(&self) -> Result<Option<String>, crate::Error> {
    let body = serde_json::to_string(&self)?;
    Ok(Some(body))
  }

  fn method(&self) -> reqwest::Method {
    reqwest::Method::POST
  }

  fn path(&self) -> Result<String, crate::Error> {
    Ok("/_plugins/_ml/_train/kmeans".to_string())
  }

  fn query_args(&self) -> Result<Option<std::collections::HashMap<String, String>>, crate::Error> {
    let mut args = HashMap::new();
    args.insert("async".to_string(), self.async_execution.to_string());
    Ok(Some(args))
  }
}

impl ModelTrainRequestBuilder {
  pub fn new() -> Self {
    Self::default()
  }
}

#[derive(Default, Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct ModelTrainRequestParameters {
  pub centroids: u32,
  pub iterations: u32,
  #[serde(rename = "distance_type")]
  pub distance_type: String,
}

#[derive(Default, Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct ModelTrainRequestInputQuery {
  #[serde(rename = "_source")]
  pub source: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub size: Option<u32>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ModelAlgorithm {
  #[serde(rename = "LINEAR_REGRESSION")]
  LinearRegression,
  #[serde(rename = "KMEANS")]
  KMeans,
  #[serde(rename = "AD_LIBSVM")]
  AdLibsvm,
  #[serde(rename = "SAMPLE_ALGO")]
  SampleAlgo,
  #[serde(rename = "LOCAL_SAMPLE_CALCULATOR")]
  LocalSampleCalculator,
  #[serde(rename = "FIT_RCF")]
  FitRcf,
  #[serde(rename = "BATCH_RCF")]
  BatchRcf,
  #[serde(rename = "ANOMALY_LOCALIZATION")]
  AnomalyLocalization,
  #[serde(rename = "RCF_SUMMARIZE")]
  RcfSummarize,
  #[serde(rename = "LOGISTIC_REGRESSION")]
  LogisticRegression,
  #[serde(rename = "TEXT_EMBEDDING")]
  TextEmbedding,
  #[serde(rename = "SPARSE_ENCODING")]
  SparseEncoding,
  #[serde(rename = "SPARSE_TOKENIZE")]
  SparseTokenize,
  #[serde(rename = "METRICS_CORRELATION")]
  MetricsCorrelation,
  #[serde(rename = "REMOTE")]
  Remote,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MLModelFormat {
  Onnx,
  TorchScript,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
  name: String,
  #[serde(default)]
  pub algorithm: Option<ModelAlgorithm>,
  #[serde(default)]
  pub version: Option<String>,
  #[serde(default)]
  pub content: Option<String>,
  #[serde(default)]
  pub user: Option<String>,
  #[serde(default)]
  pub description: Option<String>,
  #[serde(default)]
  pub model_format: Option<MLModelFormat>,
  #[serde(default)]
  pub model_state: Option<String>,
  #[serde(default)]
  pub model_content_size_in_bytes: Option<u64>,
  #[serde(default)]
  pub model_content_hash: Option<String>,
  #[serde(default)]
  pub model_config: Option<String>,
  #[serde(default)]
  pub created_time: Option<u64>,
  #[serde(default)]
  pub last_updated_time: Option<u64>,
  #[serde(default)]
  pub last_registered_time: Option<u64>,
  #[serde(default)]
  pub last_deployed_time: Option<u64>,
  #[serde(default)]
  pub last_undeployed_time: Option<u64>,
  #[serde(default)]
  pub model_id: Option<String>,
  #[serde(default)]
  pub auto_redeploy_retry_times: Option<u32>,
  #[serde(default)]
  pub chunk_number: Option<u32>,
  #[serde(default)]
  pub total_chunks: Option<u32>,
  #[serde(default)]
  pub planning_worker_node_count: Option<u32>,
  #[serde(default)]
  pub current_worker_node_count: Option<u32>,
  #[serde(default)]
  pub planning_worker_nodes: Option<Vec<String>>,
  #[serde(default)]
  pub deploy_to_all_nodes: bool,
}

#[derive(Serialize)]
pub struct GetModelRequest {
  model_id: String,
}

impl GetModelRequest {
  pub fn new(model_id: String) -> Self {
    Self { model_id }
  }
}

impl Request for GetModelRequest {
  type Response = Model;

  fn body(&self) -> Result<Option<String>, crate::Error> {
    Ok(None)
  }

  fn method(&self) -> reqwest::Method {
    reqwest::Method::GET
  }

  fn path(&self) -> Result<String, crate::Error> {
    Ok(format!("/_plugins/_ml/models/{}", self.model_id))
  }

  fn query_args(&self) -> Result<Option<std::collections::HashMap<String, String>>, crate::Error> {
    Ok(None)
  }
}

#[cfg(test)]
mod tests {

  use std::{default, path::PathBuf};

  use serde::de::DeserializeOwned;

  use super::*;
  fn load_entity<T: DeserializeOwned>(name: &str) -> T {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/ml/{name}"));
    let text = std::fs::read_to_string(filename).unwrap();
    serde_json::from_str(&text).unwrap()
  }

  #[test]
  fn test_ml_train_request() {
    let decoded: ModelTrainRequest = load_entity("model_train.request.json");
    let request = ModelTrainRequest {
      parameters: ModelTrainRequestParameters {
        centroids: 3,
        iterations: 10,
        distance_type: String::from("COSINE"),
      },
      input_query: ModelTrainRequestInputQuery {
        source: vec![String::from("petal_length_in_cm"), String::from("petal_width_in_cm")],
        size: Some(10000),
      },
      input_index: vec![String::from("iris_data")],
      ..default::Default::default()
    };
    assert_eq!(request, decoded);
  }

  #[test]
  fn test_ml_train_response() {
    let decoded: ModelTrainResponse = load_entity("model_train.response.json");
    assert_eq!(decoded.model_id, String::from("lblVmX8BO5w8y8RaYYvN"));
  }

  #[test]
  fn test_get_response() {
    let decoded: Model = load_entity("model_get.response.json");
    assert_eq!(
      decoded.name,
      String::from("huggingface/sentence-transformers/all-MiniLM-L12-v2")
    );
    assert_eq!(decoded.algorithm, Some(ModelAlgorithm::TextEmbedding));
  }
}
