#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::types::UserDefinedValueMap;
///Define ids, documents, parameters or a list of parameters per document
/// here. You must at least provide a list of document ids. See
/// documentation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MtermvectorsBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for MtermvectorsBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<MtermvectorsBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: MtermvectorsBodyParams) -> Self {
    value.0
  }
}
impl From<&MtermvectorsBodyParams> for MtermvectorsBodyParams {
  fn from(value: &MtermvectorsBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for MtermvectorsBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
