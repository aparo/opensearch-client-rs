use std::collections::HashMap;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateAction {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub doc: Option<Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub doc_as_upsert: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub script: Option<Script>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkAction {
  #[serde(rename = "_id")]
  pub id: Option<String>,
  #[serde(rename = "_index")]
  pub index: String,
  pub pipeline: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkResponse {
  pub took: u64,
  pub errors: bool,
  pub items: Vec<HashMap<String, BulkItemResponse>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkError {
  #[serde(rename = "_index")]
  pub index: Option<String>,
  #[serde(default)]
  pub index_uuid: Option<String>,
  #[serde(default)]
  pub reason: String,
  #[serde(default)]
  pub shard: Option<String>,
  #[serde(rename = "type")]
  pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkItemResponse {
  #[serde(rename = "_id")]
  pub id: String,
  #[serde(rename = "_index")]
  pub index: String,
  #[serde(rename = "_version")]
  pub version: Option<u32>,
  #[serde(default)]
  pub status: u16,
  #[serde(default)]
  pub found: Option<bool>,
//   #[serde(rename = "_shards")]
//   shards: Option<Shards>,
  #[serde(default)]
  pub error: Option<BulkError>,
  #[serde(default, rename = "_primary_term")]
  pub primary_term: Option<u32>,
  #[serde(default, rename = "_seq_no")]
  pub seq_no: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Script {
  #[serde(default)]
  pub source: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub params: Option<serde_json::Value>,
}

