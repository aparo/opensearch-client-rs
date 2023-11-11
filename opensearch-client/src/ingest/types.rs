#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

















///The ingest definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IngestPutPipelineBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IngestPutPipelineBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<IngestPutPipelineBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IngestPutPipelineBodyParams) -> Self {
    value.0
  }
}

impl From<&IngestPutPipelineBodyParams> for IngestPutPipelineBodyParams {
  fn from(value: &IngestPutPipelineBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for IngestPutPipelineBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}









///The simulate definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IngestSimulateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IngestSimulateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<IngestSimulateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IngestSimulateBodyParams) -> Self {
    value.0
  }
}

impl From<&IngestSimulateBodyParams> for IngestSimulateBodyParams {
  fn from(value: &IngestSimulateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for IngestSimulateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
















