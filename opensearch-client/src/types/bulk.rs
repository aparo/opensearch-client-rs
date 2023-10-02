use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use super::ShardStatistics;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateAction {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub doc: Option<Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub doc_as_upsert: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub script: Option<Script>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexResponse {
  #[serde(rename = "_index")]
  pub index: String,
  #[serde(rename = "_id")]
  pub id: String,
  #[serde(rename = "_version")]
  pub version: i64,
  #[serde(rename = "result")]
  pub result: String,
  #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<ShardStatistics>,
  #[serde(rename = "_seq_no", default)]
  pub seq_no: i64,
  #[serde(rename = "_primary_term", default)]
  pub primary_term: i64,
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
pub struct BulkItemResponse {
  #[serde(rename = "_id")]
  pub id: String,
  #[serde(rename = "_index")]
  pub index: String,
  #[serde(rename = "_version", default, skip_serializing_if = "Option::is_none")]
  pub version: Option<i64>,
  #[serde(default)]
  pub status: i16,
  #[serde(default)]
  pub found: Option<bool>,
  #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<ShardStatistics>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub error: Option<BulkError>,
  #[serde(default, rename = "_primary_term", skip_serializing_if = "Option::is_none")]
  pub primary_term: Option<i32>,
  #[serde(default, rename = "_seq_no", skip_serializing_if = "Option::is_none")]
  pub seq_no: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Script {
  #[serde(default)]
  pub source: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub params: Option<serde_json::Value>,
}
