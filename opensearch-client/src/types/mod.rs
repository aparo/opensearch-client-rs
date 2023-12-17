#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
pub mod bulk;
pub use bulk::{BulkAction, BulkError, BulkItemResponse, BulkResponse, IndexResponse, UpdateAction};

///The unit in which to display byte values.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Bytes {
  #[serde(rename = "b")]
  B,
  #[serde(rename = "k")]
  K,
  #[serde(rename = "kb")]
  Kb,
  #[serde(rename = "m")]
  M,
  #[serde(rename = "mb")]
  Mb,
  #[serde(rename = "g")]
  G,
  #[serde(rename = "gb")]
  Gb,
  #[serde(rename = "t")]
  T,
  #[serde(rename = "tb")]
  Tb,
  #[serde(rename = "p")]
  P,
  #[serde(rename = "pb")]
  Pb,
}

impl From<&Bytes> for Bytes {
  fn from(value: &Bytes) -> Self {
    value.clone()
  }
}

impl ToString for Bytes {
  fn to_string(&self) -> String {
    match *self {
      Self::B => "b".to_string(),
      Self::K => "k".to_string(),
      Self::Kb => "kb".to_string(),
      Self::M => "m".to_string(),
      Self::Mb => "mb".to_string(),
      Self::G => "g".to_string(),
      Self::Gb => "gb".to_string(),
      Self::T => "t".to_string(),
      Self::Tb => "tb".to_string(),
      Self::P => "p".to_string(),
      Self::Pb => "pb".to_string(),
    }
  }
}

impl std::str::FromStr for Bytes {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "b" => Ok(Self::B),
      "k" => Ok(Self::K),
      "kb" => Ok(Self::Kb),
      "m" => Ok(Self::M),
      "mb" => Ok(Self::Mb),
      "g" => Ok(Self::G),
      "gb" => Ok(Self::Gb),
      "t" => Ok(Self::T),
      "tb" => Ok(Self::Tb),
      "p" => Ok(Self::P),
      "pb" => Ok(Self::Pb),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for Bytes {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for Bytes {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for Bytes {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///Comma-separated list of scroll IDs to clear if none was specified via
/// the scroll_id parameter
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClearScrollBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for ClearScrollBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<ClearScrollBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: ClearScrollBodyParams) -> Self {
    value.0
  }
}

impl From<&ClearScrollBodyParams> for ClearScrollBodyParams {
  fn from(value: &ClearScrollBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for ClearScrollBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Specify the level of detail for returned information.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ClusterHealthLevel {
  #[serde(rename = "cluster")]
  Cluster,
  #[serde(rename = "indices")]
  Indices,
  #[serde(rename = "shards")]
  Shards,
  #[serde(rename = "awareness_attributes")]
  AwarenessAttributes,
}

impl From<&ClusterHealthLevel> for ClusterHealthLevel {
  fn from(value: &ClusterHealthLevel) -> Self {
    value.clone()
  }
}

impl ToString for ClusterHealthLevel {
  fn to_string(&self) -> String {
    match *self {
      Self::Cluster => "cluster".to_string(),
      Self::Indices => "indices".to_string(),
      Self::Shards => "shards".to_string(),
      Self::AwarenessAttributes => "awareness_attributes".to_string(),
    }
  }
}

impl std::str::FromStr for ClusterHealthLevel {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "cluster" => Ok(Self::Cluster),
      "indices" => Ok(Self::Indices),
      "shards" => Ok(Self::Shards),
      "awareness_attributes" => Ok(Self::AwarenessAttributes),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for ClusterHealthLevel {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterHealthLevel {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterHealthLevel {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///What to do when the operation encounters version conflicts?.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Conflicts {
  #[serde(rename = "abort")]
  Abort,
  #[serde(rename = "proceed")]
  Proceed,
}

impl From<&Conflicts> for Conflicts {
  fn from(value: &Conflicts) -> Self {
    value.clone()
  }
}

impl ToString for Conflicts {
  fn to_string(&self) -> String {
    match *self {
      Self::Abort => "abort".to_string(),
      Self::Proceed => "proceed".to_string(),
    }
  }
}

impl std::str::FromStr for Conflicts {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "abort" => Ok(Self::Abort),
      "proceed" => Ok(Self::Proceed),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for Conflicts {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for Conflicts {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for Conflicts {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CountResponse {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub _shards: Option<ShardStatistics>,
  #[serde(default)]
  pub count: u32,
}

///Query to restrict the results specified with the Query DSL (optional)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CountBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for CountBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<CountBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: CountBodyParams) -> Self {
    value.0
  }
}

impl From<&CountBodyParams> for CountBodyParams {
  fn from(value: &CountBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for CountBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The document
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateBodyParams(pub serde_json::Value);
impl std::ops::Deref for CreateBodyParams {
  type Target = serde_json::Value;

  fn deref(&self) -> &serde_json::Value {
    &self.0
  }
}

impl From<CreateBodyParams> for serde_json::Value {
  fn from(value: CreateBodyParams) -> Self {
    value.0
  }
}

impl From<&CreateBodyParams> for CreateBodyParams {
  fn from(value: &CreateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Value> for CreateBodyParams {
  fn from(value: serde_json::Value) -> Self {
    Self(value)
  }
}

/// OpenSearch Id
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OpenSearchId(String);
impl std::ops::Deref for OpenSearchId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<OpenSearchId> for String {
  fn from(value: OpenSearchId) -> Self {
    value.0
  }
}

impl From<&OpenSearchId> for OpenSearchId {
  fn from(value: &OpenSearchId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for OpenSearchId {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err(
        "doesn't match pattern \"^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$\"",
      );
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for OpenSearchId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for OpenSearchId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for OpenSearchId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for OpenSearchId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
/// OpenSearch Name/Value
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OpenSearchNameValue(String);
impl std::ops::Deref for OpenSearchNameValue {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<OpenSearchNameValue> for String {
  fn from(value: OpenSearchNameValue) -> Self {
    value.0
  }
}

impl From<&OpenSearchNameValue> for OpenSearchNameValue {
  fn from(value: &OpenSearchNameValue) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for OpenSearchNameValue {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err(
        "doesn't match pattern \"^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$\"",
      );
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for OpenSearchNameValue {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for OpenSearchNameValue {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for OpenSearchNameValue {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for OpenSearchNameValue {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

/// SIngle Index Name
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexName(String);
impl std::ops::Deref for IndexName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexName> for String {
  fn from(value: IndexName) -> Self {
    value.0
  }
}

impl From<&IndexName> for IndexName {
  fn from(value: &IndexName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexName {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err(
        "doesn't match pattern \"^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$\"",
      );
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for IndexName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexNames(String);
impl std::ops::Deref for IndexNames {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexNames> for String {
  fn from(value: IndexNames) -> Self {
    value.0
  }
}

impl From<&IndexNames> for IndexNames {
  fn from(value: &IndexNames) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexNames {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err(
        "doesn't match pattern \"^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$\"",
      );
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for IndexNames {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexNames {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexNames {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexNames {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePitResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub creation_time: Option<i64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub pit_id: Option<String>,
  #[serde(rename = "_shard", default, skip_serializing_if = "Option::is_none")]
  pub shard: Option<ShardStatistics>,
}

impl From<&CreatePitResponseContent> for CreatePitResponseContent {
  fn from(value: &CreatePitResponseContent) -> Self {
    value.clone()
  }
}

impl CreatePitResponseContent {
  pub fn builder() -> builder::CreatePitResponseContent {
    builder::CreatePitResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataStream {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub generation: Option<i64>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub indices: Vec<DataStreamIndex>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<DataStreamStatus>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub template: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub timestamp_field: Option<DataStreamTimestampField>,
}

impl From<&DataStream> for DataStream {
  fn from(value: &DataStream) -> Self {
    value.clone()
  }
}

impl DataStream {
  pub fn builder() -> builder::DataStream {
    builder::DataStream::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataStreamIndex {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index_name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index_uuid: Option<String>,
}

impl From<&DataStreamIndex> for DataStreamIndex {
  fn from(value: &DataStreamIndex) -> Self {
    value.clone()
  }
}

impl DataStreamIndex {
  pub fn builder() -> builder::DataStreamIndex {
    builder::DataStreamIndex::default()
  }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DataStreamStatus {
  #[serde(rename = "green")]
  Green,
  #[serde(rename = "yellow")]
  Yellow,
  #[serde(rename = "red")]
  Red,
}

impl From<&DataStreamStatus> for DataStreamStatus {
  fn from(value: &DataStreamStatus) -> Self {
    value.clone()
  }
}

impl ToString for DataStreamStatus {
  fn to_string(&self) -> String {
    match *self {
      Self::Green => "green".to_string(),
      Self::Yellow => "yellow".to_string(),
      Self::Red => "red".to_string(),
    }
  }
}

impl std::str::FromStr for DataStreamStatus {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "green" => Ok(Self::Green),
      "yellow" => Ok(Self::Yellow),
      "red" => Ok(Self::Red),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for DataStreamStatus {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DataStreamStatus {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DataStreamStatus {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataStreamTimestampField {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

impl From<&DataStreamTimestampField> for DataStreamTimestampField {
  fn from(value: &DataStreamTimestampField) -> Self {
    value.clone()
  }
}

impl DataStreamTimestampField {
  pub fn builder() -> builder::DataStreamTimestampField {
    builder::DataStreamTimestampField::default()
  }
}

///The default operator for query string query (AND or OR).
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DefaultOperator {
  #[serde(rename = "AND")]
  And,
  #[serde(rename = "OR")]
  Or,
}

impl From<&DefaultOperator> for DefaultOperator {
  fn from(value: &DefaultOperator) -> Self {
    value.clone()
  }
}

impl ToString for DefaultOperator {
  fn to_string(&self) -> String {
    match *self {
      Self::And => "AND".to_string(),
      Self::Or => "OR".to_string(),
    }
  }
}

impl std::str::FromStr for DefaultOperator {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "AND" => Ok(Self::And),
      "OR" => Ok(Self::Or),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for DefaultOperator {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DefaultOperator {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DefaultOperator {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteAllPitsResponseContent {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub pits: Vec<PitsDetailsDeleteAll>,
}

impl From<&DeleteAllPitsResponseContent> for DeleteAllPitsResponseContent {
  fn from(value: &DeleteAllPitsResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteAllPitsResponseContent {
  pub fn builder() -> builder::DeleteAllPitsResponseContent {
    builder::DeleteAllPitsResponseContent::default()
  }
}

///The search definition using the Query DSL
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteByQueryBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for DeleteByQueryBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<DeleteByQueryBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: DeleteByQueryBodyParams) -> Self {
    value.0
  }
}

impl From<&DeleteByQueryBodyParams> for DeleteByQueryBodyParams {
  fn from(value: &DeleteByQueryBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for DeleteByQueryBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteByQueryScroll(String);
impl std::ops::Deref for DeleteByQueryScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteByQueryScroll> for String {
  fn from(value: DeleteByQueryScroll) -> Self {
    value.0
  }
}

impl From<&DeleteByQueryScroll> for DeleteByQueryScroll {
  fn from(value: &DeleteByQueryScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteByQueryScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for DeleteByQueryScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteByQueryScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteByQueryScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteByQueryScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeletePitBodyParams {
  pub pit_id: Vec<String>,
}

impl From<&DeletePitBodyParams> for DeletePitBodyParams {
  fn from(value: &DeletePitBodyParams) -> Self {
    value.clone()
  }
}

impl DeletePitBodyParams {
  pub fn builder() -> builder::DeletePitBodyParams {
    builder::DeletePitBodyParams::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeletePitResponseContent {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub pits: Vec<DeletedPit>,
}

impl From<&DeletePitResponseContent> for DeletePitResponseContent {
  fn from(value: &DeletePitResponseContent) -> Self {
    value.clone()
  }
}

impl DeletePitResponseContent {
  pub fn builder() -> builder::DeletePitResponseContent {
    builder::DeletePitResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeletedPit {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub pit_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub successful: Option<bool>,
}

impl From<&DeletedPit> for DeletedPit {
  fn from(value: &DeletedPit) -> Self {
    value.clone()
  }
}

impl DeletedPit {
  pub fn builder() -> builder::DeletedPit {
    builder::DeletedPit::default()
  }
}

///Whether to expand wildcard expression to concrete indices that are open,
/// closed or both.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ExpandWildcards {
  #[serde(rename = "all")]
  All,
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "closed")]
  Closed,
  #[serde(rename = "hidden")]
  Hidden,
  #[serde(rename = "none")]
  None,
}

impl From<&ExpandWildcards> for ExpandWildcards {
  fn from(value: &ExpandWildcards) -> Self {
    value.clone()
  }
}

impl ToString for ExpandWildcards {
  fn to_string(&self) -> String {
    match *self {
      Self::All => "all".to_string(),
      Self::Open => "open".to_string(),
      Self::Closed => "closed".to_string(),
      Self::Hidden => "hidden".to_string(),
      Self::None => "none".to_string(),
    }
  }
}

impl std::str::FromStr for ExpandWildcards {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "all" => Ok(Self::All),
      "open" => Ok(Self::Open),
      "closed" => Ok(Self::Closed),
      "hidden" => Ok(Self::Hidden),
      "none" => Ok(Self::None),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for ExpandWildcards {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ExpandWildcards {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ExpandWildcards {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///The query definition using the Query DSL
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExplainBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for ExplainBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<ExplainBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: ExplainBodyParams) -> Self {
    value.0
  }
}

impl From<&ExplainBodyParams> for ExplainBodyParams {
  fn from(value: &ExplainBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for ExplainBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///An index filter specified with the Query DSL
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldCapsBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for FieldCapsBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<FieldCapsBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: FieldCapsBodyParams) -> Self {
    value.0
  }
}

impl From<&FieldCapsBodyParams> for FieldCapsBodyParams {
  fn from(value: &FieldCapsBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for FieldCapsBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetAllPitsResponseContent {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub pits: Vec<PitDetail>,
}

impl From<&GetAllPitsResponseContent> for GetAllPitsResponseContent {
  fn from(value: &GetAllPitsResponseContent) -> Self {
    value.clone()
  }
}

impl GetAllPitsResponseContent {
  pub fn builder() -> builder::GetAllPitsResponseContent {
    builder::GetAllPitsResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetResponseContent<T> {
  #[serde(rename = "_fields", default, skip_serializing_if = "Option::is_none")]
  pub fields: Option<UserDefinedValueMap>,
  pub found: bool,
  #[serde(rename = "_id")]
  pub id: String,
  #[serde(rename = "_index")]
  pub index: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub primary_term: Option<i64>,
  #[serde(rename = "_routing", default, skip_serializing_if = "Option::is_none")]
  pub routing: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub seq_no: Option<i64>,
  #[serde(rename = "_source", default, skip_serializing_if = "Option::is_none")]
  pub source: Option<T>,
  #[serde(rename = "_type", default, skip_serializing_if = "Option::is_none")]
  pub type_: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub version: Option<i32>,
}

impl<T> From<&GetResponseContent<T>> for GetResponseContent<T> {
  fn from(value: &GetResponseContent<T>) -> Self {
    value.into()
  }
}

impl<T> GetResponseContent<T> {
  pub fn builder() -> builder::GetResponseContent<T> {
    builder::GetResponseContent::default()
  }
}

///Group tasks by nodes or parent/child relationships.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GroupBy {
  #[serde(rename = "nodes")]
  Nodes,
  #[serde(rename = "parents")]
  Parents,
  #[serde(rename = "none")]
  None,
}

impl From<&GroupBy> for GroupBy {
  fn from(value: &GroupBy) -> Self {
    value.clone()
  }
}

impl ToString for GroupBy {
  fn to_string(&self) -> String {
    match *self {
      Self::Nodes => "nodes".to_string(),
      Self::Parents => "parents".to_string(),
      Self::None => "none".to_string(),
    }
  }
}

impl std::str::FromStr for GroupBy {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "nodes" => Ok(Self::Nodes),
      "parents" => Ok(Self::Parents),
      "none" => Ok(Self::None),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for GroupBy {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for GroupBy {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for GroupBy {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///Health status ('green', 'yellow', or 'red') to filter only indices
/// matching the specified health status.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Health {
  #[serde(rename = "green")]
  Green,
  #[serde(rename = "yellow")]
  Yellow,
  #[serde(rename = "red")]
  Red,
}

impl From<&Health> for Health {
  fn from(value: &Health) -> Self {
    value.clone()
  }
}

impl ToString for Health {
  fn to_string(&self) -> String {
    match *self {
      Self::Green => "green".to_string(),
      Self::Yellow => "yellow".to_string(),
      Self::Red => "red".to_string(),
    }
  }
}

impl std::str::FromStr for Health {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "green" => Ok(Self::Green),
      "yellow" => Ok(Self::Yellow),
      "red" => Ok(Self::Red),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for Health {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for Health {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for Health {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Hit<T> {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub fields: Option<serde_json::Value>,
  #[serde(rename = "_id", default)]
  pub id: String,
  #[serde(rename = "_index")]
  pub index: String,
  #[serde(rename = "_score", default, skip_serializing_if = "Option::is_none")]
  pub score: Option<f64>,
  #[serde(rename = "_source", default, skip_serializing_if = "Option::is_none")]
  pub source: Option<T>,
  #[serde(rename = "_type", default, skip_serializing_if = "Option::is_none")]
  pub type_: Option<String>,
  #[serde(rename = "sort", default, skip_serializing_if = "Option::is_none")]
  pub sort: Option<serde_json::Value>,
}

impl<T> From<&Hit<T>> for Hit<T> {
  fn from(value: &Hit<T>) -> Self {
    value.into()
  }
}

impl<T> Hit<T> {
  pub fn builder() -> builder::Hit<T> {
    builder::Hit::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HitsMetadata<T> {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub hits: Vec<Hit<T>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub max_score: Option<f64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub total: Option<Total>,
}

impl<T> From<&HitsMetadata<T>> for HitsMetadata<T> {
  fn from(value: &HitsMetadata<T>) -> Self {
    value.into()
  }
}

impl<T> HitsMetadata<T> {
  pub fn builder() -> builder::HitsMetadata<T> {
    builder::HitsMetadata::default()
  }
}

impl<T> Default for HitsMetadata<T> {
  fn default() -> Self {
    Self {
      hits: Vec::new(),
      max_score: None,
      total: None,
    }
  }
}

///The document
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndexBodyParams(pub serde_json::Value);
impl std::ops::Deref for IndexBodyParams {
  type Target = serde_json::Value;

  fn deref(&self) -> &serde_json::Value {
    &self.0
  }
}

impl From<IndexBodyParams> for serde_json::Value {
  fn from(value: IndexBodyParams) -> Self {
    value.0
  }
}

impl From<&IndexBodyParams> for IndexBodyParams {
  fn from(value: &IndexBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Value> for IndexBodyParams {
  fn from(value: serde_json::Value) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InfoResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub cluster_name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub cluster_uuid: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub tagline: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub version: Option<InfoVersion>,
}

impl From<&InfoResponseContent> for InfoResponseContent {
  fn from(value: &InfoResponseContent) -> Self {
    value.clone()
  }
}

impl InfoResponseContent {
  pub fn builder() -> builder::InfoResponseContent {
    builder::InfoResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InfoVersion {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub build_date: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub build_hash: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub build_snapshot: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub build_type: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub distribution: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub lucene_version: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub minimum_index_compatibility_version: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub minimum_wire_compatibility_version: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub number: Option<String>,
}

impl From<&InfoVersion> for InfoVersion {
  fn from(value: &InfoVersion) -> Self {
    value.clone()
  }
}

impl InfoVersion {
  pub fn builder() -> builder::InfoVersion {
    builder::InfoVersion::default()
  }
}

///Document identifiers; can be either `docs` (containing full document
/// information) or `ids` (when index is provided in the URL.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MgetBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for MgetBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<MgetBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: MgetBodyParams) -> Self {
    value.0
  }
}

impl From<&MgetBodyParams> for MgetBodyParams {
  fn from(value: &MgetBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for MgetBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The request definitions (metadata-search request definition pairs),
/// separated by newlines
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsearchBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for MsearchBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<MsearchBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: MsearchBodyParams) -> Self {
    value.0
  }
}

impl From<&MsearchBodyParams> for MsearchBodyParams {
  fn from(value: &MsearchBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for MsearchBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The request definitions (metadata-search request definition pairs),
/// separated by newlines
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsearchTemplateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for MsearchTemplateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<MsearchTemplateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: MsearchTemplateBodyParams) -> Self {
    value.0
  }
}

impl From<&MsearchTemplateBodyParams> for MsearchTemplateBodyParams {
  fn from(value: &MsearchTemplateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for MsearchTemplateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Operation timeout.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Timeout(String);
impl std::ops::Deref for Timeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<Timeout> for String {
  fn from(value: Timeout) -> Self {
    value.0
  }
}

impl From<&Timeout> for Timeout {
  fn from(value: &Timeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for Timeout {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for Timeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for Timeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for Timeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for Timeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Explicit operation type. Defaults to `index` for requests with an
/// explicit document ID, and to `create`for requests without an explicit
/// document ID.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OpType {
  #[serde(rename = "index")]
  Index,
  #[serde(rename = "create")]
  Create,
}

impl From<&OpType> for OpType {
  fn from(value: &OpType) -> Self {
    value.clone()
  }
}

impl ToString for OpType {
  fn to_string(&self) -> String {
    match *self {
      Self::Index => "index".to_string(),
      Self::Create => "create".to_string(),
    }
  }
}

impl std::str::FromStr for OpType {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "index" => Ok(Self::Index),
      "create" => Ok(Self::Create),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for OpType {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for OpType {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for OpType {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PitDetail {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub creation_time: Option<i64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub keep_alive: Option<i64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub pit_id: Option<String>,
}

impl From<&PitDetail> for PitDetail {
  fn from(value: &PitDetail) -> Self {
    value.clone()
  }
}

impl PitDetail {
  pub fn builder() -> builder::PitDetail {
    builder::PitDetail::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PitsDetailsDeleteAll {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub pit_id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub successful: Option<bool>,
}

impl From<&PitsDetailsDeleteAll> for PitsDetailsDeleteAll {
  fn from(value: &PitsDetailsDeleteAll) -> Self {
    value.clone()
  }
}

impl PitsDetailsDeleteAll {
  pub fn builder() -> builder::PitsDetailsDeleteAll {
    builder::PitsDetailsDeleteAll::default()
  }
}

///The document
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PutScriptBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for PutScriptBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<PutScriptBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: PutScriptBodyParams) -> Self {
    value.0
  }
}

impl From<&PutScriptBodyParams> for PutScriptBodyParams {
  fn from(value: &PutScriptBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for PutScriptBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The ranking evaluation search definition, including search requests,
/// document ratings and ranking metric definition.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RankEvalBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for RankEvalBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<RankEvalBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: RankEvalBodyParams) -> Self {
    value.0
  }
}

impl From<&RankEvalBodyParams> for RankEvalBodyParams {
  fn from(value: &RankEvalBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for RankEvalBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///If `true` then refresh the affected shards to make this operation
/// visible to search, if `wait_for` then wait for a refresh to make this
/// operation visible to search, if `false` (the default) then do nothing
/// with refreshes.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RefreshEnum {
  #[serde(rename = "true")]
  True,
  #[serde(rename = "false")]
  False,
  #[serde(rename = "wait_for")]
  WaitFor,
}

impl From<&RefreshEnum> for RefreshEnum {
  fn from(value: &RefreshEnum) -> Self {
    value.clone()
  }
}

impl ToString for RefreshEnum {
  fn to_string(&self) -> String {
    match *self {
      Self::True => "true".to_string(),
      Self::False => "false".to_string(),
      Self::WaitFor => "wait_for".to_string(),
    }
  }
}

impl std::str::FromStr for RefreshEnum {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "true" => Ok(Self::True),
      "false" => Ok(Self::False),
      "wait_for" => Ok(Self::WaitFor),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for RefreshEnum {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for RefreshEnum {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for RefreshEnum {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///The search definition using the Query DSL and the prototype for the
/// index request.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReindexBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for ReindexBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<ReindexBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: ReindexBodyParams) -> Self {
    value.0
  }
}

impl From<&ReindexBodyParams> for ReindexBodyParams {
  fn from(value: &ReindexBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for ReindexBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ReindexScroll(String);
impl std::ops::Deref for ReindexScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ReindexScroll> for String {
  fn from(value: ReindexScroll) -> Self {
    value.0
  }
}

impl From<&ReindexScroll> for ReindexScroll {
  fn from(value: &ReindexScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ReindexScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for ReindexScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ReindexScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ReindexScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ReindexScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Time each individual bulk request should wait for shards that are

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Relation {
  #[serde(rename = "eq")]
  Eq,
  #[serde(rename = "gte")]
  Gte,
}

impl From<&Relation> for Relation {
  fn from(value: &Relation) -> Self {
    value.clone()
  }
}

impl ToString for Relation {
  fn to_string(&self) -> String {
    match *self {
      Self::Eq => "eq".to_string(),
      Self::Gte => "gte".to_string(),
    }
  }
}

impl std::str::FromStr for Relation {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "eq" => Ok(Self::Eq),
      "gte" => Ok(Self::Gte),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for Relation {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for Relation {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for Relation {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///The search definition template and its params
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RenderSearchTemplateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for RenderSearchTemplateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<RenderSearchTemplateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: RenderSearchTemplateBodyParams) -> Self {
    value.0
  }
}

impl From<&RenderSearchTemplateBodyParams> for RenderSearchTemplateBodyParams {
  fn from(value: &RenderSearchTemplateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for RenderSearchTemplateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The type to sample.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SampleType {
  #[serde(rename = "cpu")]
  Cpu,
  #[serde(rename = "wait")]
  Wait,
  #[serde(rename = "block")]
  Block,
}

impl From<&SampleType> for SampleType {
  fn from(value: &SampleType) -> Self {
    value.clone()
  }
}

impl ToString for SampleType {
  fn to_string(&self) -> String {
    match *self {
      Self::Cpu => "cpu".to_string(),
      Self::Wait => "wait".to_string(),
      Self::Block => "block".to_string(),
    }
  }
}

impl std::str::FromStr for SampleType {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "cpu" => Ok(Self::Cpu),
      "wait" => Ok(Self::Wait),
      "block" => Ok(Self::Block),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for SampleType {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SampleType {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SampleType {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///The script to execute
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScriptsPainlessExecuteBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for ScriptsPainlessExecuteBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<ScriptsPainlessExecuteBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: ScriptsPainlessExecuteBodyParams) -> Self {
    value.0
  }
}

impl From<&ScriptsPainlessExecuteBodyParams> for ScriptsPainlessExecuteBodyParams {
  fn from(value: &ScriptsPainlessExecuteBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for ScriptsPainlessExecuteBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The scroll ID if not passed by URL or query parameter.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScrollBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for ScrollBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<ScrollBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: ScrollBodyParams) -> Self {
    value.0
  }
}

impl From<&ScrollBodyParams> for ScrollBodyParams {
  fn from(value: &ScrollBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for ScrollBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ScrollGetScroll(String);
impl std::ops::Deref for ScrollGetScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ScrollGetScroll> for String {
  fn from(value: ScrollGetScroll) -> Self {
    value.0
  }
}

impl From<&ScrollGetScroll> for ScrollGetScroll {
  fn from(value: &ScrollGetScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ScrollGetScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for ScrollGetScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ScrollGetScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ScrollGetScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ScrollGetScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ScrollGetWithScrollIdScroll(String);
impl std::ops::Deref for ScrollGetWithScrollIdScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ScrollGetWithScrollIdScroll> for String {
  fn from(value: ScrollGetWithScrollIdScroll) -> Self {
    value.0
  }
}

impl From<&ScrollGetWithScrollIdScroll> for ScrollGetWithScrollIdScroll {
  fn from(value: &ScrollGetWithScrollIdScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ScrollGetWithScrollIdScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for ScrollGetWithScrollIdScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ScrollGetWithScrollIdScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ScrollGetWithScrollIdScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ScrollGetWithScrollIdScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ScrollPostScroll(String);
impl std::ops::Deref for ScrollPostScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ScrollPostScroll> for String {
  fn from(value: ScrollPostScroll) -> Self {
    value.0
  }
}

impl From<&ScrollPostScroll> for ScrollPostScroll {
  fn from(value: &ScrollPostScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ScrollPostScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for ScrollPostScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ScrollPostScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ScrollPostScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ScrollPostScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ScrollPostWithScrollIdScroll(String);
impl std::ops::Deref for ScrollPostWithScrollIdScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ScrollPostWithScrollIdScroll> for String {
  fn from(value: ScrollPostWithScrollIdScroll) -> Self {
    value.0
  }
}

impl From<&ScrollPostWithScrollIdScroll> for ScrollPostWithScrollIdScroll {
  fn from(value: &ScrollPostWithScrollIdScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ScrollPostWithScrollIdScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for ScrollPostWithScrollIdScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ScrollPostWithScrollIdScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ScrollPostWithScrollIdScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ScrollPostWithScrollIdScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

// ///The search definition using the Query DSL
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct SearchBodyParams {
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub docvalue_fields: Option<String>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub explain: Option<bool>,
//   #[serde(default, skip_serializing_if = "Vec::is_empty")]
//   pub fields: Vec<String>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub from: Option<u32>,
//   #[serde(default, skip_serializing_if = "Vec::is_empty")]
//   pub indices_boost: Vec<serde_json::Value>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub min_score: Option<u32>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub query: Option<UserDefinedObjectStructure>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub seq_no_primary_term: Option<bool>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub size: Option<u32>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub source: Option<String>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub stats: Option<String>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub terminate_after: Option<u32>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub timeout: Option<Time>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub version: Option<bool>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub search_after: Option<serde_json::Value>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   pub sort: Option<serde_json::Value>,
// }

// impl Default for SearchBodyParams {
//   fn default() -> Self {
//     Self {
//       docvalue_fields: None,
//       explain: None,
//       fields: Vec::new(),
//       from: None,
//       indices_boost: Vec::new(),
//       min_score: None,
//       query: None,
//       seq_no_primary_term: None,
//       size: None,
//       source: None,
//       stats: None,
//       terminate_after: None,
//       timeout: None,
//       version: None,
//       search_after: None,
//       sort: None,
//     }
//   }
// }

// impl From<&SearchBodyParams> for SearchBodyParams {
//   fn from(value: &SearchBodyParams) -> Self {
//     value.clone()
//   }
// }

// impl SearchBodyParams {
//   pub fn builder() -> builder::SearchBodyParams {
//     builder::SearchBodyParams::default()
//   }
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchGetResponseContent<T> {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hits: Option<HitsMetadata<T>>,
  #[serde(rename = "_scroll_id", default, skip_serializing_if = "Option::is_none")]
  pub scroll_id: Option<String>,
  #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<ShardStatistics>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub timed_out: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub took: Option<i64>,
}

impl<T> From<&SearchGetResponseContent<T>> for SearchGetResponseContent<T> {
  fn from(value: &SearchGetResponseContent<T>) -> Self {
    value.into()
  }
}

impl<T> SearchGetResponseContent<T> {
  pub fn builder() -> builder::SearchGetResponseContent<T> {
    builder::SearchGetResponseContent::default()
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchGetScroll(String);
impl std::ops::Deref for SearchGetScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchGetScroll> for String {
  fn from(value: SearchGetScroll) -> Self {
    value.0
  }
}

impl From<&SearchGetScroll> for SearchGetScroll {
  fn from(value: &SearchGetScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchGetScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for SearchGetScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchGetScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchGetScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchGetScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchGetWithIndexScroll(String);
impl std::ops::Deref for SearchGetWithIndexScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchGetWithIndexScroll> for String {
  fn from(value: SearchGetWithIndexScroll) -> Self {
    value.0
  }
}

impl From<&SearchGetWithIndexScroll> for SearchGetWithIndexScroll {
  fn from(value: &SearchGetWithIndexScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchGetWithIndexScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for SearchGetWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchGetWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchGetWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchGetWithIndexScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregations {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchPostResponseContent<T> {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hits: Option<HitsMetadata<T>>,
  #[serde(rename = "_scroll_id", default, skip_serializing_if = "Option::is_none")]
  pub scroll_id: Option<String>,
  #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<ShardStatistics>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub timed_out: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub took: Option<i64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub aggregations: Option<Aggregations>,
}

impl<T> From<&SearchPostResponseContent<T>> for SearchPostResponseContent<T> {
  fn from(value: &SearchPostResponseContent<T>) -> Self {
    value.into()
  }
}

impl<T> SearchPostResponseContent<T> {
  pub fn builder() -> builder::SearchPostResponseContent<T> {
    builder::SearchPostResponseContent::default()
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchPostScroll(String);
impl std::ops::Deref for SearchPostScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchPostScroll> for String {
  fn from(value: SearchPostScroll) -> Self {
    value.0
  }
}

impl From<&SearchPostScroll> for SearchPostScroll {
  fn from(value: &SearchPostScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchPostScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for SearchPostScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchPostScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchPostScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchPostScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct SearchResult<T> {
  #[serde(default)]
  pub hits: HitsMetadata<T>,
  #[serde(rename = "_scroll_id", default, skip_serializing_if = "Option::is_none")]
  pub scroll_id: Option<String>,
  #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<ShardStatistics>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub timed_out: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub took: Option<i64>,
}

impl<T> From<&SearchResult<T>> for SearchResult<T> {
  fn from(value: &SearchResult<T>) -> Self {
    value.into()
  }
}

impl<T> SearchResult<T> {
  pub fn builder() -> builder::SearchResult<T> {
    builder::SearchResult::default()
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchPostWithIndexScroll(String);
impl std::ops::Deref for SearchPostWithIndexScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchPostWithIndexScroll> for String {
  fn from(value: SearchPostWithIndexScroll) -> Self {
    value.0
  }
}

impl From<&SearchPostWithIndexScroll> for SearchPostWithIndexScroll {
  fn from(value: &SearchPostWithIndexScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchPostWithIndexScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for SearchPostWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchPostWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchPostWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchPostWithIndexScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The search definition template and its params
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchTemplateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for SearchTemplateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<SearchTemplateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: SearchTemplateBodyParams) -> Self {
    value.0
  }
}

impl From<&SearchTemplateBodyParams> for SearchTemplateBodyParams {
  fn from(value: &SearchTemplateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for SearchTemplateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchTemplateGetScroll(String);
impl std::ops::Deref for SearchTemplateGetScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchTemplateGetScroll> for String {
  fn from(value: SearchTemplateGetScroll) -> Self {
    value.0
  }
}

impl From<&SearchTemplateGetScroll> for SearchTemplateGetScroll {
  fn from(value: &SearchTemplateGetScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchTemplateGetScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for SearchTemplateGetScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchTemplateGetScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchTemplateGetScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchTemplateGetScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchTemplateGetWithIndexScroll(String);
impl std::ops::Deref for SearchTemplateGetWithIndexScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchTemplateGetWithIndexScroll> for String {
  fn from(value: SearchTemplateGetWithIndexScroll) -> Self {
    value.0
  }
}

impl From<&SearchTemplateGetWithIndexScroll> for SearchTemplateGetWithIndexScroll {
  fn from(value: &SearchTemplateGetWithIndexScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchTemplateGetWithIndexScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for SearchTemplateGetWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchTemplateGetWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchTemplateGetWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchTemplateGetWithIndexScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchTemplatePostScroll(String);
impl std::ops::Deref for SearchTemplatePostScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchTemplatePostScroll> for String {
  fn from(value: SearchTemplatePostScroll) -> Self {
    value.0
  }
}

impl From<&SearchTemplatePostScroll> for SearchTemplatePostScroll {
  fn from(value: &SearchTemplatePostScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchTemplatePostScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for SearchTemplatePostScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchTemplatePostScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchTemplatePostScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchTemplatePostScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchTemplatePostWithIndexScroll(String);
impl std::ops::Deref for SearchTemplatePostWithIndexScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchTemplatePostWithIndexScroll> for String {
  fn from(value: SearchTemplatePostWithIndexScroll) -> Self {
    value.0
  }
}

impl From<&SearchTemplatePostWithIndexScroll> for SearchTemplatePostWithIndexScroll {
  fn from(value: &SearchTemplatePostWithIndexScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchTemplatePostWithIndexScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for SearchTemplatePostWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchTemplatePostWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchTemplatePostWithIndexScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchTemplatePostWithIndexScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Search operation type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SearchType {
  #[serde(rename = "query_then_fetch")]
  QueryThenFetch,
  #[serde(rename = "dfs_query_then_fetch")]
  DfsQueryThenFetch,
}

impl From<&SearchType> for SearchType {
  fn from(value: &SearchType) -> Self {
    value.clone()
  }
}

impl ToString for SearchType {
  fn to_string(&self) -> String {
    match *self {
      Self::QueryThenFetch => "query_then_fetch".to_string(),
      Self::DfsQueryThenFetch => "dfs_query_then_fetch".to_string(),
    }
  }
}

impl std::str::FromStr for SearchType {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "query_then_fetch" => Ok(Self::QueryThenFetch),
      "dfs_query_then_fetch" => Ok(Self::DfsQueryThenFetch),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for SearchType {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchType {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchType {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///Search operation type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SearchTypeMulti {
  #[serde(rename = "query_then_fetch")]
  QueryThenFetch,
  #[serde(rename = "query_and_fetch")]
  QueryAndFetch,
  #[serde(rename = "dfs_query_then_fetch")]
  DfsQueryThenFetch,
  #[serde(rename = "dfs_query_and_fetch")]
  DfsQueryAndFetch,
}

impl From<&SearchTypeMulti> for SearchTypeMulti {
  fn from(value: &SearchTypeMulti) -> Self {
    value.clone()
  }
}

impl ToString for SearchTypeMulti {
  fn to_string(&self) -> String {
    match *self {
      Self::QueryThenFetch => "query_then_fetch".to_string(),
      Self::QueryAndFetch => "query_and_fetch".to_string(),
      Self::DfsQueryThenFetch => "dfs_query_then_fetch".to_string(),
      Self::DfsQueryAndFetch => "dfs_query_and_fetch".to_string(),
    }
  }
}

impl std::str::FromStr for SearchTypeMulti {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "query_then_fetch" => Ok(Self::QueryThenFetch),
      "query_and_fetch" => Ok(Self::QueryAndFetch),
      "dfs_query_then_fetch" => Ok(Self::DfsQueryThenFetch),
      "dfs_query_and_fetch" => Ok(Self::DfsQueryAndFetch),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for SearchTypeMulti {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchTypeMulti {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchTypeMulti {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ShardStatistics {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub failed: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub skipped: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub successful: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub total: Option<i32>,
}

impl From<&ShardStatistics> for ShardStatistics {
  fn from(value: &ShardStatistics) -> Self {
    value.clone()
  }
}

impl ShardStatistics {
  pub fn builder() -> builder::ShardStatistics {
    builder::ShardStatistics::default()
  }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StatusMember {
  #[serde(rename = "green")]
  Green,
  #[serde(rename = "yellow")]
  Yellow,
  #[serde(rename = "red")]
  Red,
  #[serde(rename = "all")]
  All,
}

impl From<&StatusMember> for StatusMember {
  fn from(value: &StatusMember) -> Self {
    value.clone()
  }
}

impl ToString for StatusMember {
  fn to_string(&self) -> String {
    match *self {
      Self::Green => "green".to_string(),
      Self::Yellow => "yellow".to_string(),
      Self::Red => "red".to_string(),
      Self::All => "all".to_string(),
    }
  }
}

impl std::str::FromStr for StatusMember {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "green" => Ok(Self::Green),
      "yellow" => Ok(Self::Yellow),
      "red" => Ok(Self::Red),
      "all" => Ok(Self::All),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for StatusMember {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for StatusMember {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for StatusMember {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///Specify suggest mode.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SuggestMode {
  #[serde(rename = "missing")]
  Missing,
  #[serde(rename = "popular")]
  Popular,
  #[serde(rename = "always")]
  Always,
}

impl From<&SuggestMode> for SuggestMode {
  fn from(value: &SuggestMode) -> Self {
    value.clone()
  }
}

impl ToString for SuggestMode {
  fn to_string(&self) -> String {
    match *self {
      Self::Missing => "missing".to_string(),
      Self::Popular => "popular".to_string(),
      Self::Always => "always".to_string(),
    }
  }
}

impl std::str::FromStr for SuggestMode {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "missing" => Ok(Self::Missing),
      "popular" => Ok(Self::Popular),
      "always" => Ok(Self::Always),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for SuggestMode {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SuggestMode {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SuggestMode {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///Define parameters and or supply a document to get termvectors for. See
/// documentation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TermvectorsBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for TermvectorsBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<TermvectorsBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: TermvectorsBodyParams) -> Self {
    value.0
  }
}

impl From<&TermvectorsBodyParams> for TermvectorsBodyParams {
  fn from(value: &TermvectorsBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for TermvectorsBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The unit in which to display time values.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Time {
  #[serde(rename = "d")]
  D,
  #[serde(rename = "h")]
  H,
  #[serde(rename = "m")]
  M,
  #[serde(rename = "s")]
  S,
  #[serde(rename = "ms")]
  Ms,
  #[serde(rename = "micros")]
  Micros,
  #[serde(rename = "nanos")]
  Nanos,
}

impl From<&Time> for Time {
  fn from(value: &Time) -> Self {
    value.clone()
  }
}

impl ToString for Time {
  fn to_string(&self) -> String {
    match *self {
      Self::D => "d".to_string(),
      Self::H => "h".to_string(),
      Self::M => "m".to_string(),
      Self::S => "s".to_string(),
      Self::Ms => "ms".to_string(),
      Self::Micros => "micros".to_string(),
      Self::Nanos => "nanos".to_string(),
    }
  }
}

impl std::str::FromStr for Time {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "d" => Ok(Self::D),
      "h" => Ok(Self::H),
      "m" => Ok(Self::M),
      "s" => Ok(Self::S),
      "ms" => Ok(Self::Ms),
      "micros" => Ok(Self::Micros),
      "nanos" => Ok(Self::Nanos),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for Time {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for Time {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for Time {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Total {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub relation: Option<Relation>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub value: Option<i32>,
}

impl From<&Total> for Total {
  fn from(value: &Total) -> Self {
    value.clone()
  }
}

impl Total {
  pub fn builder() -> builder::Total {
    builder::Total::default()
  }
}

///The request definition requires either `script` or partial `doc`
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateBodyParams(pub serde_json::Value);
impl std::ops::Deref for UpdateBodyParams {
  type Target = serde_json::Value;

  fn deref(&self) -> &serde_json::Value {
    &self.0
  }
}

impl From<UpdateBodyParams> for serde_json::Value {
  fn from(value: UpdateBodyParams) -> Self {
    value.0
  }
}

impl From<&UpdateBodyParams> for UpdateBodyParams {
  fn from(value: &UpdateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Value> for UpdateBodyParams {
  fn from(value: serde_json::Value) -> Self {
    Self(value)
  }
}

///The search definition using the Query DSL
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateByQueryBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for UpdateByQueryBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<UpdateByQueryBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: UpdateByQueryBodyParams) -> Self {
    value.0
  }
}

impl From<&UpdateByQueryBodyParams> for UpdateByQueryBodyParams {
  fn from(value: &UpdateByQueryBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for UpdateByQueryBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Specify how long a consistent view of the index should be maintained for
/// scrolled search.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UpdateByQueryScroll(String);
impl std::ops::Deref for UpdateByQueryScroll {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<UpdateByQueryScroll> for String {
  fn from(value: UpdateByQueryScroll) -> Self {
    value.0
  }
}

impl From<&UpdateByQueryScroll> for UpdateByQueryScroll {
  fn from(value: &UpdateByQueryScroll) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for UpdateByQueryScroll {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"^([0-9]+)(?:d|h|m|s|ms|micros|nanos)$\"");
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for UpdateByQueryScroll {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for UpdateByQueryScroll {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for UpdateByQueryScroll {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for UpdateByQueryScroll {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserDefinedObjectStructure {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub bool: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub boosting: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub combined_fields: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub constant_score: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub dis_max: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub distance_feature: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub exists: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub field_masking_span: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub function_score: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub fuzzy: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub geo_bounding_box: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub geo_distance: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub geo_polygon: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub geo_shape: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub has_child: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub has_parent: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ids: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub intervals: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub knn: Option<serde_json::Value>,
  #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
  pub match_: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_all: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_bool_prefix: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_none: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_phrase: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_phrase_prefix: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub more_like_this: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub multi_match: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub nested: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub parent_id: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub percolate: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub pinned: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub prefix: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub query_string: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub range: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub rank_feature: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub regexp: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub script: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub script_score: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub shape: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub simple_query_string: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_containing: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_first: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_multi: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_near: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_not: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_or: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_term: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_within: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub term: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub terms: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub terms_set: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub wildcard: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub wrapper: Option<serde_json::Value>,
}

impl From<&UserDefinedObjectStructure> for UserDefinedObjectStructure {
  fn from(value: &UserDefinedObjectStructure) -> Self {
    value.clone()
  }
}

impl UserDefinedObjectStructure {
  pub fn builder() -> builder::UserDefinedObjectStructure {
    builder::UserDefinedObjectStructure::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserDefinedStructure {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub alias: Option<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub aliases: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub filter: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index_routing: Option<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub indices: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_write_index: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub must_exist: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub routing: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub search_routing: Option<String>,
}

impl From<&UserDefinedStructure> for UserDefinedStructure {
  fn from(value: &UserDefinedStructure) -> Self {
    value.clone()
  }
}

impl UserDefinedStructure {
  pub fn builder() -> builder::UserDefinedStructure {
    builder::UserDefinedStructure::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserDefinedValueMap(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for UserDefinedValueMap {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<UserDefinedValueMap> for serde_json::Map<String, serde_json::Value> {
  fn from(value: UserDefinedValueMap) -> Self {
    value.0
  }
}

impl From<&UserDefinedValueMap> for UserDefinedValueMap {
  fn from(value: &UserDefinedValueMap) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for UserDefinedValueMap {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Specific version type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VersionType {
  #[serde(rename = "internal")]
  Internal,
  #[serde(rename = "external")]
  External,
  #[serde(rename = "external_gte")]
  ExternalGte,
  #[serde(rename = "force")]
  Force,
}

impl From<&VersionType> for VersionType {
  fn from(value: &VersionType) -> Self {
    value.clone()
  }
}

impl ToString for VersionType {
  fn to_string(&self) -> String {
    match *self {
      Self::Internal => "internal".to_string(),
      Self::External => "external".to_string(),
      Self::ExternalGte => "external_gte".to_string(),
      Self::Force => "force".to_string(),
    }
  }
}

impl std::str::FromStr for VersionType {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "internal" => Ok(Self::Internal),
      "external" => Ok(Self::External),
      "external_gte" => Ok(Self::ExternalGte),
      "force" => Ok(Self::Force),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for VersionType {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for VersionType {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for VersionType {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///Wait until all currently queued events with the given priority are
/// processed.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WaitForEvents {
  #[serde(rename = "immediate")]
  Immediate,
  #[serde(rename = "urgent")]
  Urgent,
  #[serde(rename = "high")]
  High,
  #[serde(rename = "normal")]
  Normal,
  #[serde(rename = "low")]
  Low,
  #[serde(rename = "languid")]
  Languid,
}

impl From<&WaitForEvents> for WaitForEvents {
  fn from(value: &WaitForEvents) -> Self {
    value.clone()
  }
}

impl ToString for WaitForEvents {
  fn to_string(&self) -> String {
    match *self {
      Self::Immediate => "immediate".to_string(),
      Self::Urgent => "urgent".to_string(),
      Self::High => "high".to_string(),
      Self::Normal => "normal".to_string(),
      Self::Low => "low".to_string(),
      Self::Languid => "languid".to_string(),
    }
  }
}

impl std::str::FromStr for WaitForEvents {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "immediate" => Ok(Self::Immediate),
      "urgent" => Ok(Self::Urgent),
      "high" => Ok(Self::High),
      "normal" => Ok(Self::Normal),
      "low" => Ok(Self::Low),
      "languid" => Ok(Self::Languid),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for WaitForEvents {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for WaitForEvents {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for WaitForEvents {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///Wait until cluster is in a specific state.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WaitForStatus {
  #[serde(rename = "green")]
  Green,
  #[serde(rename = "yellow")]
  Yellow,
  #[serde(rename = "red")]
  Red,
}

impl From<&WaitForStatus> for WaitForStatus {
  fn from(value: &WaitForStatus) -> Self {
    value.clone()
  }
}

impl ToString for WaitForStatus {
  fn to_string(&self) -> String {
    match *self {
      Self::Green => "green".to_string(),
      Self::Yellow => "yellow".to_string(),
      Self::Red => "red".to_string(),
    }
  }
}

impl std::str::FromStr for WaitForStatus {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "green" => Ok(Self::Green),
      "yellow" => Ok(Self::Yellow),
      "red" => Ok(Self::Red),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for WaitForStatus {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for WaitForStatus {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for WaitForStatus {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

pub mod builder {
  use super::Aggregations;

  #[derive(Clone, Debug)]
  pub struct CreatePitResponseContent {
    creation_time: Result<Option<i64>, String>,
    pit_id: Result<Option<String>, String>,
    shard: Result<Option<super::ShardStatistics>, String>,
  }

  impl Default for CreatePitResponseContent {
    fn default() -> Self {
      Self {
        creation_time: Ok(Default::default()),
        pit_id: Ok(Default::default()),
        shard: Ok(Default::default()),
      }
    }
  }

  impl CreatePitResponseContent {
    pub fn creation_time<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i64>>,
      T::Error: std::fmt::Display, {
      self.creation_time = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for creation_time: {}", e));
      self
    }

    pub fn pit_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.pit_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
      self
    }

    pub fn shard<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::ShardStatistics>>,
      T::Error: std::fmt::Display, {
      self.shard = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for shard: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreatePitResponseContent> for super::CreatePitResponseContent {
    type Error = String;

    fn try_from(value: CreatePitResponseContent) -> Result<Self, String> {
      Ok(Self {
        creation_time: value.creation_time?,
        pit_id: value.pit_id?,
        shard: value.shard?,
      })
    }
  }

  impl From<super::CreatePitResponseContent> for CreatePitResponseContent {
    fn from(value: super::CreatePitResponseContent) -> Self {
      Self {
        creation_time: Ok(value.creation_time),
        pit_id: Ok(value.pit_id),
        shard: Ok(value.shard),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DataStream {
    generation: Result<Option<i64>, String>,
    indices: Result<Vec<super::DataStreamIndex>, String>,
    name: Result<Option<String>, String>,
    status: Result<Option<super::DataStreamStatus>, String>,
    template: Result<Option<String>, String>,
    timestamp_field: Result<Option<super::DataStreamTimestampField>, String>,
  }

  impl Default for DataStream {
    fn default() -> Self {
      Self {
        generation: Ok(Default::default()),
        indices: Ok(Default::default()),
        name: Ok(Default::default()),
        status: Ok(Default::default()),
        template: Ok(Default::default()),
        timestamp_field: Ok(Default::default()),
      }
    }
  }

  impl DataStream {
    pub fn generation<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i64>>,
      T::Error: std::fmt::Display, {
      self.generation = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for generation: {}", e));
      self
    }

    pub fn indices<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::DataStreamIndex>>,
      T::Error: std::fmt::Display, {
      self.indices = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for indices: {}", e));
      self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.name = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for name: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::DataStreamStatus>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }

    pub fn template<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.template = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for template: {}", e));
      self
    }

    pub fn timestamp_field<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::DataStreamTimestampField>>,
      T::Error: std::fmt::Display, {
      self.timestamp_field = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for timestamp_field: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DataStream> for super::DataStream {
    type Error = String;

    fn try_from(value: DataStream) -> Result<Self, String> {
      Ok(Self {
        generation: value.generation?,
        indices: value.indices?,
        name: value.name?,
        status: value.status?,
        template: value.template?,
        timestamp_field: value.timestamp_field?,
      })
    }
  }

  impl From<super::DataStream> for DataStream {
    fn from(value: super::DataStream) -> Self {
      Self {
        generation: Ok(value.generation),
        indices: Ok(value.indices),
        name: Ok(value.name),
        status: Ok(value.status),
        template: Ok(value.template),
        timestamp_field: Ok(value.timestamp_field),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DataStreamIndex {
    index_name: Result<Option<String>, String>,
    index_uuid: Result<Option<String>, String>,
  }

  impl Default for DataStreamIndex {
    fn default() -> Self {
      Self {
        index_name: Ok(Default::default()),
        index_uuid: Ok(Default::default()),
      }
    }
  }

  impl DataStreamIndex {
    pub fn index_name<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.index_name = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index_name: {}", e));
      self
    }

    pub fn index_uuid<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.index_uuid = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index_uuid: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DataStreamIndex> for super::DataStreamIndex {
    type Error = String;

    fn try_from(value: DataStreamIndex) -> Result<Self, String> {
      Ok(Self {
        index_name: value.index_name?,
        index_uuid: value.index_uuid?,
      })
    }
  }

  impl From<super::DataStreamIndex> for DataStreamIndex {
    fn from(value: super::DataStreamIndex) -> Self {
      Self {
        index_name: Ok(value.index_name),
        index_uuid: Ok(value.index_uuid),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DataStreamTimestampField {
    name: Result<Option<String>, String>,
  }

  impl Default for DataStreamTimestampField {
    fn default() -> Self {
      Self {
        name: Ok(Default::default()),
      }
    }
  }

  impl DataStreamTimestampField {
    pub fn name<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.name = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for name: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DataStreamTimestampField> for super::DataStreamTimestampField {
    type Error = String;

    fn try_from(value: DataStreamTimestampField) -> Result<Self, String> {
      Ok(Self { name: value.name? })
    }
  }

  impl From<super::DataStreamTimestampField> for DataStreamTimestampField {
    fn from(value: super::DataStreamTimestampField) -> Self {
      Self { name: Ok(value.name) }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteAllPitsResponseContent {
    pits: Result<Vec<super::PitsDetailsDeleteAll>, String>,
  }

  impl Default for DeleteAllPitsResponseContent {
    fn default() -> Self {
      Self {
        pits: Ok(Default::default()),
      }
    }
  }

  impl DeleteAllPitsResponseContent {
    pub fn pits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::PitsDetailsDeleteAll>>,
      T::Error: std::fmt::Display, {
      self.pits = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pits: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteAllPitsResponseContent> for super::DeleteAllPitsResponseContent {
    type Error = String;

    fn try_from(value: DeleteAllPitsResponseContent) -> Result<Self, String> {
      Ok(Self { pits: value.pits? })
    }
  }

  impl From<super::DeleteAllPitsResponseContent> for DeleteAllPitsResponseContent {
    fn from(value: super::DeleteAllPitsResponseContent) -> Self {
      Self { pits: Ok(value.pits) }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeletePitBodyParams {
    pit_id: Result<Vec<String>, String>,
  }

  impl Default for DeletePitBodyParams {
    fn default() -> Self {
      Self {
        pit_id: Err("no value supplied for pit_id".to_string()),
      }
    }
  }

  impl DeletePitBodyParams {
    pub fn pit_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.pit_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeletePitBodyParams> for super::DeletePitBodyParams {
    type Error = String;

    fn try_from(value: DeletePitBodyParams) -> Result<Self, String> {
      Ok(Self { pit_id: value.pit_id? })
    }
  }

  impl From<super::DeletePitBodyParams> for DeletePitBodyParams {
    fn from(value: super::DeletePitBodyParams) -> Self {
      Self {
        pit_id: Ok(value.pit_id),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeletePitResponseContent {
    pits: Result<Vec<super::DeletedPit>, String>,
  }

  impl Default for DeletePitResponseContent {
    fn default() -> Self {
      Self {
        pits: Ok(Default::default()),
      }
    }
  }

  impl DeletePitResponseContent {
    pub fn pits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::DeletedPit>>,
      T::Error: std::fmt::Display, {
      self.pits = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pits: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeletePitResponseContent> for super::DeletePitResponseContent {
    type Error = String;

    fn try_from(value: DeletePitResponseContent) -> Result<Self, String> {
      Ok(Self { pits: value.pits? })
    }
  }

  impl From<super::DeletePitResponseContent> for DeletePitResponseContent {
    fn from(value: super::DeletePitResponseContent) -> Self {
      Self { pits: Ok(value.pits) }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeletedPit {
    pit_id: Result<Option<String>, String>,
    successful: Result<Option<bool>, String>,
  }

  impl Default for DeletedPit {
    fn default() -> Self {
      Self {
        pit_id: Ok(Default::default()),
        successful: Ok(Default::default()),
      }
    }
  }

  impl DeletedPit {
    pub fn pit_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.pit_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
      self
    }

    pub fn successful<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.successful = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for successful: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeletedPit> for super::DeletedPit {
    type Error = String;

    fn try_from(value: DeletedPit) -> Result<Self, String> {
      Ok(Self {
        pit_id: value.pit_id?,
        successful: value.successful?,
      })
    }
  }

  impl From<super::DeletedPit> for DeletedPit {
    fn from(value: super::DeletedPit) -> Self {
      Self {
        pit_id: Ok(value.pit_id),
        successful: Ok(value.successful),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct GetAllPitsResponseContent {
    pits: Result<Vec<super::PitDetail>, String>,
  }

  impl Default for GetAllPitsResponseContent {
    fn default() -> Self {
      Self {
        pits: Ok(Default::default()),
      }
    }
  }

  impl GetAllPitsResponseContent {
    pub fn pits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::PitDetail>>,
      T::Error: std::fmt::Display, {
      self.pits = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pits: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<GetAllPitsResponseContent> for super::GetAllPitsResponseContent {
    type Error = String;

    fn try_from(value: GetAllPitsResponseContent) -> Result<Self, String> {
      Ok(Self { pits: value.pits? })
    }
  }

  impl From<super::GetAllPitsResponseContent> for GetAllPitsResponseContent {
    fn from(value: super::GetAllPitsResponseContent) -> Self {
      Self { pits: Ok(value.pits) }
    }
  }

  #[derive(Clone, Debug)]
  pub struct GetResponseContent<T> {
    fields: Result<Option<super::UserDefinedValueMap>, String>,
    found: Result<bool, String>,
    id: Result<String, String>,
    index: Result<String, String>,
    primary_term: Result<Option<i64>, String>,
    routing: Result<Option<String>, String>,
    seq_no: Result<Option<i64>, String>,
    source: Result<Option<T>, String>,
    type_: Result<Option<String>, String>,
    version: Result<Option<i32>, String>,
  }

  impl<T> Default for GetResponseContent<T> {
    fn default() -> Self {
      Self {
        fields: Ok(Default::default()),
        found: Err("no value supplied for found".to_string()),
        id: Err("no value supplied for id".to_string()),
        index: Err("no value supplied for index".to_string()),
        primary_term: Ok(Default::default()),
        routing: Ok(Default::default()),
        seq_no: Ok(Default::default()),
        source: Ok(Default::default()),
        type_: Ok(Default::default()),
        version: Ok(Default::default()),
      }
    }
  }

  impl<T3> GetResponseContent<T3> {
    pub fn fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.fields = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for fields: {}", e));
      self
    }

    pub fn found<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<bool>,
      T::Error: std::fmt::Display, {
      self.found = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for found: {}", e));
      self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for id: {}", e));
      self
    }

    pub fn index<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.index = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index: {}", e));
      self
    }

    pub fn primary_term<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i64>>,
      T::Error: std::fmt::Display, {
      self.primary_term = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for primary_term: {}", e));
      self
    }

    pub fn routing<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.routing = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for routing: {}", e));
      self
    }

    pub fn seq_no<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i64>>,
      T::Error: std::fmt::Display, {
      self.seq_no = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for seq_no: {}", e));
      self
    }

    pub fn source(mut self, value: T3) -> Self
    where
      T3: std::convert::TryInto<Option<T3>>,
      T3::Error: std::fmt::Display, {
      self.source = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for source: {}", e));
      self
    }

    pub fn type_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.type_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for type_: {}", e));
      self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.version = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for version: {}", e));
      self
    }
  }

  impl<T2> std::convert::TryFrom<GetResponseContent<T2>> for super::GetResponseContent<T2> {
    type Error = String;

    fn try_from(value: GetResponseContent<T2>) -> Result<Self, String> {
      Ok(Self {
        fields: value.fields?,
        found: value.found?,
        id: value.id?,
        index: value.index?,
        primary_term: value.primary_term?,
        routing: value.routing?,
        seq_no: value.seq_no?,
        source: value.source?,
        type_: value.type_?,
        version: value.version?,
      })
    }
  }

  impl<T2> From<super::GetResponseContent<T2>> for GetResponseContent<T2> {
    fn from(value: super::GetResponseContent<T2>) -> Self {
      Self {
        fields: Ok(value.fields),
        found: Ok(value.found),
        id: Ok(value.id),
        index: Ok(value.index),
        primary_term: Ok(value.primary_term),
        routing: Ok(value.routing),
        seq_no: Ok(value.seq_no),
        source: Ok(value.source),
        type_: Ok(value.type_),
        version: Ok(value.version),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct Hit<T> {
    fields: Result<Option<serde_json::Value>, String>,
    id: Result<String, String>,
    index: Result<String, String>,
    score: Result<Option<f64>, String>,
    source: Result<Option<T>, String>,
    type_: Result<Option<String>, String>,
    sort: Result<Option<serde_json::Value>, String>,
  }

  impl<T> Default for Hit<T> {
    fn default() -> Self {
      Self {
        fields: Ok(Default::default()),
        id: Ok(Default::default()),
        index: Ok(Default::default()),
        score: Ok(Default::default()),
        source: Ok(Default::default()),
        type_: Ok(Default::default()),
        sort: Ok(Default::default()),
      }
    }
  }

  impl<T3> Hit<T3> {
    pub fn fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.fields = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for fields: {}", e));
      self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for id: {}", e));
      self
    }

    pub fn index<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.index = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index: {}", e));
      self
    }

    pub fn score<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<f64>>,
      T::Error: std::fmt::Display, {
      self.score = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for score: {}", e));
      self
    }

    // pub fn source(mut self, value: T3) -> Self
    // where
    //   T3: std::convert::TryInto<Option<serde_json::Value>>,
    //   T3::Error: std::fmt::Display, {
    //   self.source = value
    //     .try_into()
    //     .map_err(|e| format!("error converting supplied value for source: {}",
    // e));   self
    // }

    pub fn type_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.type_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for type_: {}", e));
      self
    }
  }

  impl<T2> std::convert::TryFrom<Hit<T2>> for super::Hit<T2> {
    type Error = String;

    fn try_from(value: Hit<T2>) -> Result<Self, String> {
      Ok(Self {
        fields: value.fields?,
        id: value.id?,
        index: value.index?,
        score: value.score?,
        source: value.source?,
        type_: value.type_?,
        sort: value.sort?,
      })
    }
  }

  impl<T2> From<super::Hit<T2>> for Hit<T2> {
    fn from(value: super::Hit<T2>) -> Self {
      Self {
        fields: Ok(value.fields),
        id: Ok(value.id),
        index: Ok(value.index),
        score: Ok(value.score),
        source: Ok(value.source),
        type_: Ok(value.type_),
        sort: Ok(value.sort),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct HitsMetadata<T> {
    hits: Result<Vec<super::Hit<T>>, String>,
    max_score: Result<Option<f64>, String>,
    total: Result<Option<super::Total>, String>,
  }

  impl<T> Default for HitsMetadata<T> {
    fn default() -> Self {
      Self {
        hits: Ok(Default::default()),
        max_score: Ok(Default::default()),
        total: Ok(Default::default()),
      }
    }
  }

  impl<T2> HitsMetadata<T2> {
    pub fn hits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::Hit<T2>>>,
      T::Error: std::fmt::Display, {
      self.hits = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hits: {}", e));
      self
    }

    pub fn max_score<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<f64>>,
      T::Error: std::fmt::Display, {
      self.max_score = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for max_score: {}", e));
      self
    }

    pub fn total<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::Total>>,
      T::Error: std::fmt::Display, {
      self.total = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for total: {}", e));
      self
    }
  }

  impl<T> std::convert::TryFrom<HitsMetadata<T>> for super::HitsMetadata<T> {
    type Error = String;

    fn try_from(value: HitsMetadata<T>) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        max_score: value.max_score?,
        total: value.total?,
      })
    }
  }

  impl<T> From<super::HitsMetadata<T>> for HitsMetadata<T> {
    fn from(value: super::HitsMetadata<T>) -> Self {
      Self {
        hits: Ok(value.hits),
        max_score: Ok(value.max_score),
        total: Ok(value.total),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct InfoResponseContent {
    cluster_name: Result<Option<String>, String>,
    cluster_uuid: Result<Option<String>, String>,
    name: Result<Option<String>, String>,
    tagline: Result<Option<String>, String>,
    version: Result<Option<super::InfoVersion>, String>,
  }

  impl Default for InfoResponseContent {
    fn default() -> Self {
      Self {
        cluster_name: Ok(Default::default()),
        cluster_uuid: Ok(Default::default()),
        name: Ok(Default::default()),
        tagline: Ok(Default::default()),
        version: Ok(Default::default()),
      }
    }
  }

  impl InfoResponseContent {
    pub fn cluster_name<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.cluster_name = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for cluster_name: {}", e));
      self
    }

    pub fn cluster_uuid<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.cluster_uuid = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for cluster_uuid: {}", e));
      self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.name = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for name: {}", e));
      self
    }

    pub fn tagline<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.tagline = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for tagline: {}", e));
      self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::InfoVersion>>,
      T::Error: std::fmt::Display, {
      self.version = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for version: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<InfoResponseContent> for super::InfoResponseContent {
    type Error = String;

    fn try_from(value: InfoResponseContent) -> Result<Self, String> {
      Ok(Self {
        cluster_name: value.cluster_name?,
        cluster_uuid: value.cluster_uuid?,
        name: value.name?,
        tagline: value.tagline?,
        version: value.version?,
      })
    }
  }

  impl From<super::InfoResponseContent> for InfoResponseContent {
    fn from(value: super::InfoResponseContent) -> Self {
      Self {
        cluster_name: Ok(value.cluster_name),
        cluster_uuid: Ok(value.cluster_uuid),
        name: Ok(value.name),
        tagline: Ok(value.tagline),
        version: Ok(value.version),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct InfoVersion {
    build_date: Result<Option<String>, String>,
    build_hash: Result<Option<String>, String>,
    build_snapshot: Result<Option<bool>, String>,
    build_type: Result<Option<String>, String>,
    distribution: Result<Option<String>, String>,
    lucene_version: Result<Option<String>, String>,
    minimum_index_compatibility_version: Result<Option<String>, String>,
    minimum_wire_compatibility_version: Result<Option<String>, String>,
    number: Result<Option<String>, String>,
  }

  impl Default for InfoVersion {
    fn default() -> Self {
      Self {
        build_date: Ok(Default::default()),
        build_hash: Ok(Default::default()),
        build_snapshot: Ok(Default::default()),
        build_type: Ok(Default::default()),
        distribution: Ok(Default::default()),
        lucene_version: Ok(Default::default()),
        minimum_index_compatibility_version: Ok(Default::default()),
        minimum_wire_compatibility_version: Ok(Default::default()),
        number: Ok(Default::default()),
      }
    }
  }

  impl InfoVersion {
    pub fn build_date<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.build_date = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for build_date: {}", e));
      self
    }

    pub fn build_hash<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.build_hash = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for build_hash: {}", e));
      self
    }

    pub fn build_snapshot<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.build_snapshot = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for build_snapshot: {}", e));
      self
    }

    pub fn build_type<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.build_type = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for build_type: {}", e));
      self
    }

    pub fn distribution<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.distribution = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for distribution: {}", e));
      self
    }

    pub fn lucene_version<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.lucene_version = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for lucene_version: {}", e));
      self
    }

    pub fn minimum_index_compatibility_version<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.minimum_index_compatibility_version = value.try_into().map_err(|e| {
        format!(
          "error converting supplied value for minimum_index_compatibility_version: {}",
          e
        )
      });
      self
    }

    pub fn minimum_wire_compatibility_version<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.minimum_wire_compatibility_version = value.try_into().map_err(|e| {
        format!(
          "error converting supplied value for minimum_wire_compatibility_version: {}",
          e
        )
      });
      self
    }

    pub fn number<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.number = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for number: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<InfoVersion> for super::InfoVersion {
    type Error = String;

    fn try_from(value: InfoVersion) -> Result<Self, String> {
      Ok(Self {
        build_date: value.build_date?,
        build_hash: value.build_hash?,
        build_snapshot: value.build_snapshot?,
        build_type: value.build_type?,
        distribution: value.distribution?,
        lucene_version: value.lucene_version?,
        minimum_index_compatibility_version: value.minimum_index_compatibility_version?,
        minimum_wire_compatibility_version: value.minimum_wire_compatibility_version?,
        number: value.number?,
      })
    }
  }

  impl From<super::InfoVersion> for InfoVersion {
    fn from(value: super::InfoVersion) -> Self {
      Self {
        build_date: Ok(value.build_date),
        build_hash: Ok(value.build_hash),
        build_snapshot: Ok(value.build_snapshot),
        build_type: Ok(value.build_type),
        distribution: Ok(value.distribution),
        lucene_version: Ok(value.lucene_version),
        minimum_index_compatibility_version: Ok(value.minimum_index_compatibility_version),
        minimum_wire_compatibility_version: Ok(value.minimum_wire_compatibility_version),
        number: Ok(value.number),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PitDetail {
    creation_time: Result<Option<i64>, String>,
    keep_alive: Result<Option<i64>, String>,
    pit_id: Result<Option<String>, String>,
  }

  impl Default for PitDetail {
    fn default() -> Self {
      Self {
        creation_time: Ok(Default::default()),
        keep_alive: Ok(Default::default()),
        pit_id: Ok(Default::default()),
      }
    }
  }

  impl PitDetail {
    pub fn creation_time<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i64>>,
      T::Error: std::fmt::Display, {
      self.creation_time = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for creation_time: {}", e));
      self
    }

    pub fn keep_alive<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i64>>,
      T::Error: std::fmt::Display, {
      self.keep_alive = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for keep_alive: {}", e));
      self
    }

    pub fn pit_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.pit_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PitDetail> for super::PitDetail {
    type Error = String;

    fn try_from(value: PitDetail) -> Result<Self, String> {
      Ok(Self {
        creation_time: value.creation_time?,
        keep_alive: value.keep_alive?,
        pit_id: value.pit_id?,
      })
    }
  }

  impl From<super::PitDetail> for PitDetail {
    fn from(value: super::PitDetail) -> Self {
      Self {
        creation_time: Ok(value.creation_time),
        keep_alive: Ok(value.keep_alive),
        pit_id: Ok(value.pit_id),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PitsDetailsDeleteAll {
    pit_id: Result<Option<String>, String>,
    successful: Result<Option<bool>, String>,
  }

  impl Default for PitsDetailsDeleteAll {
    fn default() -> Self {
      Self {
        pit_id: Ok(Default::default()),
        successful: Ok(Default::default()),
      }
    }
  }

  impl PitsDetailsDeleteAll {
    pub fn pit_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.pit_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
      self
    }

    pub fn successful<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.successful = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for successful: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PitsDetailsDeleteAll> for super::PitsDetailsDeleteAll {
    type Error = String;

    fn try_from(value: PitsDetailsDeleteAll) -> Result<Self, String> {
      Ok(Self {
        pit_id: value.pit_id?,
        successful: value.successful?,
      })
    }
  }

  impl From<super::PitsDetailsDeleteAll> for PitsDetailsDeleteAll {
    fn from(value: super::PitsDetailsDeleteAll) -> Self {
      Self {
        pit_id: Ok(value.pit_id),
        successful: Ok(value.successful),
      }
    }
  }

  // #[derive(Clone, Debug)]
  // pub struct SearchBodyParams {
  //   docvalue_fields: Result<Option<String>, String>,
  //   explain: Result<Option<bool>, String>,
  //   fields: Result<Vec<String>, String>,
  //   from: Result<Option<u32>, String>,
  //   indices_boost: Result<Vec<serde_json::Value>, String>,
  //   min_score: Result<Option<u32>, String>,
  //   query: Result<Option<super::UserDefinedObjectStructure>, String>,
  //   seq_no_primary_term: Result<Option<bool>, String>,
  //   size: Result<Option<u32>, String>,
  //   source: Result<Option<String>, String>,
  //   stats: Result<Option<String>, String>,
  //   terminate_after: Result<Option<u32>, String>,
  //   timeout: Result<Option<super::Time>, String>,
  //   version: Result<Option<bool>, String>,
  //   search_after: Result<Option<serde_json::Value>, String>,
  //   sort: Result<Option<serde_json::Value>, String>,
  // }

  // impl Default for SearchBodyParams {
  //   fn default() -> Self {
  //     Self {
  //       docvalue_fields: Ok(Default::default()),
  //       explain: Ok(Default::default()),
  //       fields: Ok(Default::default()),
  //       from: Ok(Default::default()),
  //       indices_boost: Ok(Default::default()),
  //       min_score: Ok(Default::default()),
  //       query: Ok(Default::default()),
  //       seq_no_primary_term: Ok(Default::default()),
  //       size: Ok(Default::default()),
  //       source: Ok(Default::default()),
  //       stats: Ok(Default::default()),
  //       terminate_after: Ok(Default::default()),
  //       timeout: Ok(Default::default()),
  //       version: Ok(Default::default()),
  //       search_after: Ok(Default::default()),
  //       sort: Ok(Default::default()),
  //     }
  //   }
  // }

  // impl SearchBodyParams {
  //   pub fn docvalue_fields<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<String>>,
  //     T::Error: std::fmt::Display, {
  //     self.docvalue_fields = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for
  // docvalue_fields: {}", e));     self
  //   }

  //   pub fn explain<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<bool>>,
  //     T::Error: std::fmt::Display, {
  //     self.explain = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for explain: {}",
  // e));     self
  //   }

  //   pub fn fields<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Vec<String>>,
  //     T::Error: std::fmt::Display, {
  //     self.fields = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for fields: {}",
  // e));     self
  //   }

  //   pub fn from<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<u32>>,
  //     T::Error: std::fmt::Display, {
  //     self.from = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for from: {}",
  // e));     self
  //   }

  //   pub fn indices_boost<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Vec<serde_json::Value>>,
  //     T::Error: std::fmt::Display, {
  //     self.indices_boost = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for
  // indices_boost: {}", e));     self
  //   }

  //   pub fn min_score<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<u32>>,
  //     T::Error: std::fmt::Display, {
  //     self.min_score = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for min_score:
  // {}", e));     self
  //   }

  //   pub fn query<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<super::UserDefinedObjectStructure>>,
  //     T::Error: std::fmt::Display, {
  //     self.query = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for query: {}",
  // e));     self
  //   }

  //   pub fn seq_no_primary_term<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<bool>>,
  //     T::Error: std::fmt::Display, {
  //     self.seq_no_primary_term = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for
  // seq_no_primary_term: {}", e));     self
  //   }

  //   pub fn size<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<u32>>,
  //     T::Error: std::fmt::Display, {
  //     self.size = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for size: {}",
  // e));     self
  //   }

  //   pub fn source<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<String>>,
  //     T::Error: std::fmt::Display, {
  //     self.source = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for source: {}",
  // e));     self
  //   }

  //   pub fn stats<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<String>>,
  //     T::Error: std::fmt::Display, {
  //     self.stats = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for stats: {}",
  // e));     self
  //   }

  //   pub fn terminate_after<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<u32>>,
  //     T::Error: std::fmt::Display, {
  //     self.terminate_after = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for
  // terminate_after: {}", e));     self
  //   }

  //   pub fn timeout<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<super::Time>>,
  //     T::Error: std::fmt::Display, {
  //     self.timeout = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for timeout: {}",
  // e));     self
  //   }

  //   pub fn version<T>(mut self, value: T) -> Self
  //   where
  //     T: std::convert::TryInto<Option<bool>>,
  //     T::Error: std::fmt::Display, {
  //     self.version = value
  //       .try_into()
  //       .map_err(|e| format!("error converting supplied value for version: {}",
  // e));     self
  //   }
  // }

  // impl std::convert::TryFrom<SearchBodyParams> for super::SearchBodyParams {
  //   type Error = String;

  //   fn try_from(value: SearchBodyParams) -> Result<Self, String> {
  //     Ok(Self {
  //       docvalue_fields: value.docvalue_fields?,
  //       explain: value.explain?,
  //       fields: value.fields?,
  //       from: value.from?,
  //       indices_boost: value.indices_boost?,
  //       min_score: value.min_score?,
  //       query: value.query?,
  //       seq_no_primary_term: value.seq_no_primary_term?,
  //       size: value.size?,
  //       source: value.source?,
  //       stats: value.stats?,
  //       terminate_after: value.terminate_after?,
  //       timeout: value.timeout?,
  //       version: value.version?,
  //       search_after: value.search_after?,
  //       sort: value.sort?,
  //     })
  //   }
  // }

  // impl From<super::SearchBodyParams> for SearchBodyParams {
  //   fn from(value: super::SearchBodyParams) -> Self {
  //     Self {
  //       docvalue_fields: Ok(value.docvalue_fields),
  //       explain: Ok(value.explain),
  //       fields: Ok(value.fields),
  //       from: Ok(value.from),
  //       indices_boost: Ok(value.indices_boost),
  //       min_score: Ok(value.min_score),
  //       query: Ok(value.query),
  //       seq_no_primary_term: Ok(value.seq_no_primary_term),
  //       size: Ok(value.size),
  //       source: Ok(value.source),
  //       stats: Ok(value.stats),
  //       terminate_after: Ok(value.terminate_after),
  //       timeout: Ok(value.timeout),
  //       version: Ok(value.version),
  //       search_after: Ok(value.search_after),
  //       sort: Ok(value.sort),
  //     }
  //   }
  // }

  #[derive(Clone, Debug)]
  pub struct SearchGetResponseContent<T> {
    hits: Result<Option<super::HitsMetadata<T>>, String>,
    scroll_id: Result<Option<String>, String>,
    shards: Result<Option<super::ShardStatistics>, String>,
    timed_out: Result<Option<bool>, String>,
    took: Result<Option<i64>, String>,
  }

  impl<T2> Default for SearchGetResponseContent<T2> {
    fn default() -> Self {
      Self {
        hits: Ok(Default::default()),
        scroll_id: Ok(Default::default()),
        shards: Ok(Default::default()),
        timed_out: Ok(Default::default()),
        took: Ok(Default::default()),
      }
    }
  }

  impl<T2> SearchGetResponseContent<T2> {
    pub fn hits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::HitsMetadata<T2>>>,
      T::Error: std::fmt::Display, {
      self.hits = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hits: {}", e));
      self
    }

    pub fn scroll_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.scroll_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for scroll_id: {}", e));
      self
    }

    pub fn shards<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::ShardStatistics>>,
      T::Error: std::fmt::Display, {
      self.shards = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for shards: {}", e));
      self
    }

    pub fn timed_out<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.timed_out = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for timed_out: {}", e));
      self
    }

    pub fn took<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i64>>,
      T::Error: std::fmt::Display, {
      self.took = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for took: {}", e));
      self
    }
  }

  impl<T2> std::convert::TryFrom<SearchGetResponseContent<T2>> for super::SearchGetResponseContent<T2> {
    type Error = String;

    fn try_from(value: SearchGetResponseContent<T2>) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        scroll_id: value.scroll_id?,
        shards: value.shards?,
        timed_out: value.timed_out?,
        took: value.took?,
      })
    }
  }

  impl<T2> From<super::SearchGetResponseContent<T2>> for SearchGetResponseContent<T2> {
    fn from(value: super::SearchGetResponseContent<T2>) -> Self {
      Self {
        hits: Ok(value.hits),
        scroll_id: Ok(value.scroll_id),
        shards: Ok(value.shards),
        timed_out: Ok(value.timed_out),
        took: Ok(value.took),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct SearchPostResponseContent<T> {
    hits: Result<Option<super::HitsMetadata<T>>, String>,
    scroll_id: Result<Option<String>, String>,
    shards: Result<Option<super::ShardStatistics>, String>,
    timed_out: Result<Option<bool>, String>,
    took: Result<Option<i64>, String>,
    aggregations: Result<Option<Aggregations>, String>,
  }

  impl<T2> Default for SearchPostResponseContent<T2> {
    fn default() -> Self {
      Self {
        hits: Ok(Default::default()),
        scroll_id: Ok(Default::default()),
        shards: Ok(Default::default()),
        timed_out: Ok(Default::default()),
        took: Ok(Default::default()),
        aggregations: Ok(Default::default()),
      }
    }
  }

  impl<T2> SearchPostResponseContent<T2> {
    pub fn hits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::HitsMetadata<T2>>>,
      T::Error: std::fmt::Display, {
      self.hits = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hits: {}", e));
      self
    }

    pub fn scroll_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.scroll_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for scroll_id: {}", e));
      self
    }

    pub fn shards<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::ShardStatistics>>,
      T::Error: std::fmt::Display, {
      self.shards = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for shards: {}", e));
      self
    }

    pub fn timed_out<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.timed_out = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for timed_out: {}", e));
      self
    }

    pub fn took<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i64>>,
      T::Error: std::fmt::Display, {
      self.took = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for took: {}", e));
      self
    }
  }

  impl<T2> std::convert::TryFrom<SearchPostResponseContent<T2>> for super::SearchPostResponseContent<T2> {
    type Error = String;

    fn try_from(value: SearchPostResponseContent<T2>) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        scroll_id: value.scroll_id?,
        shards: value.shards?,
        timed_out: value.timed_out?,
        took: value.took?,
        aggregations: value.aggregations?,
      })
    }
  }

  impl<T2> From<super::SearchPostResponseContent<T2>> for SearchPostResponseContent<T2> {
    fn from(value: super::SearchPostResponseContent<T2>) -> Self {
      Self {
        hits: Ok(value.hits),
        scroll_id: Ok(value.scroll_id),
        shards: Ok(value.shards),
        timed_out: Ok(value.timed_out),
        took: Ok(value.took),
        aggregations: Ok(value.aggregations),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct SearchResult<T> {
    hits: Result<super::HitsMetadata<T>, String>,
    scroll_id: Result<Option<String>, String>,
    shards: Result<Option<super::ShardStatistics>, String>,
    timed_out: Result<Option<bool>, String>,
    took: Result<Option<i64>, String>,
  }

  impl<T> Default for SearchResult<T> {
    fn default() -> Self {
      Self {
        hits: Ok(super::HitsMetadata::<T>::default()),
        scroll_id: Ok(Default::default()),
        shards: Ok(Default::default()),
        timed_out: Ok(Default::default()),
        took: Ok(Default::default()),
      }
    }
  }

  impl<T2> SearchResult<T2> {
    pub fn hits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<super::HitsMetadata<T2>>,
      T::Error: std::fmt::Display, {
      self.hits = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hits: {}", e));
      self
    }

    pub fn scroll_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.scroll_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for scroll_id: {}", e));
      self
    }

    pub fn shards<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::ShardStatistics>>,
      T::Error: std::fmt::Display, {
      self.shards = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for shards: {}", e));
      self
    }

    pub fn timed_out<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.timed_out = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for timed_out: {}", e));
      self
    }

    pub fn took<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i64>>,
      T::Error: std::fmt::Display, {
      self.took = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for took: {}", e));
      self
    }
  }

  impl<T> std::convert::TryFrom<SearchResult<T>> for super::SearchResult<T> {
    type Error = String;

    fn try_from(value: SearchResult<T>) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        scroll_id: value.scroll_id?,
        shards: value.shards?,
        timed_out: value.timed_out?,
        took: value.took?,
      })
    }
  }

  impl<T> From<super::SearchResult<T>> for SearchResult<T> {
    fn from(value: super::SearchResult<T>) -> Self {
      Self {
        hits: Ok(value.hits),
        scroll_id: Ok(value.scroll_id),
        shards: Ok(value.shards),
        timed_out: Ok(value.timed_out),
        took: Ok(value.took),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ShardStatistics {
    failed: Result<Option<i32>, String>,
    skipped: Result<Option<i32>, String>,
    successful: Result<Option<i32>, String>,
    total: Result<Option<i32>, String>,
  }

  impl Default for ShardStatistics {
    fn default() -> Self {
      Self {
        failed: Ok(Default::default()),
        skipped: Ok(Default::default()),
        successful: Ok(Default::default()),
        total: Ok(Default::default()),
      }
    }
  }

  impl ShardStatistics {
    pub fn failed<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.failed = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for failed: {}", e));
      self
    }

    pub fn skipped<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.skipped = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for skipped: {}", e));
      self
    }

    pub fn successful<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.successful = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for successful: {}", e));
      self
    }

    pub fn total<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.total = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for total: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ShardStatistics> for super::ShardStatistics {
    type Error = String;

    fn try_from(value: ShardStatistics) -> Result<Self, String> {
      Ok(Self {
        failed: value.failed?,
        skipped: value.skipped?,
        successful: value.successful?,
        total: value.total?,
      })
    }
  }

  impl From<super::ShardStatistics> for ShardStatistics {
    fn from(value: super::ShardStatistics) -> Self {
      Self {
        failed: Ok(value.failed),
        skipped: Ok(value.skipped),
        successful: Ok(value.successful),
        total: Ok(value.total),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct Total {
    relation: Result<Option<super::Relation>, String>,
    value: Result<Option<i32>, String>,
  }

  impl Default for Total {
    fn default() -> Self {
      Self {
        relation: Ok(Default::default()),
        value: Ok(Default::default()),
      }
    }
  }

  impl Total {
    pub fn relation<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::Relation>>,
      T::Error: std::fmt::Display, {
      self.relation = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for relation: {}", e));
      self
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.value = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for value: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<Total> for super::Total {
    type Error = String;

    fn try_from(value: Total) -> Result<Self, String> {
      Ok(Self {
        relation: value.relation?,
        value: value.value?,
      })
    }
  }

  impl From<super::Total> for Total {
    fn from(value: super::Total) -> Self {
      Self {
        relation: Ok(value.relation),
        value: Ok(value.value),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct UserDefinedObjectStructure {
    bool: Result<Option<serde_json::Value>, String>,
    boosting: Result<Option<serde_json::Value>, String>,
    combined_fields: Result<Option<serde_json::Value>, String>,
    constant_score: Result<Option<serde_json::Value>, String>,
    dis_max: Result<Option<serde_json::Value>, String>,
    distance_feature: Result<Option<serde_json::Value>, String>,
    exists: Result<Option<serde_json::Value>, String>,
    field_masking_span: Result<Option<serde_json::Value>, String>,
    function_score: Result<Option<serde_json::Value>, String>,
    fuzzy: Result<Option<super::UserDefinedValueMap>, String>,
    geo_bounding_box: Result<Option<serde_json::Value>, String>,
    geo_distance: Result<Option<serde_json::Value>, String>,
    geo_polygon: Result<Option<serde_json::Value>, String>,
    geo_shape: Result<Option<serde_json::Value>, String>,
    has_child: Result<Option<serde_json::Value>, String>,
    has_parent: Result<Option<serde_json::Value>, String>,
    ids: Result<Option<serde_json::Value>, String>,
    intervals: Result<Option<super::UserDefinedValueMap>, String>,
    knn: Result<Option<serde_json::Value>, String>,
    match_: Result<Option<super::UserDefinedValueMap>, String>,
    match_all: Result<Option<serde_json::Value>, String>,
    match_bool_prefix: Result<Option<super::UserDefinedValueMap>, String>,
    match_none: Result<Option<serde_json::Value>, String>,
    match_phrase: Result<Option<super::UserDefinedValueMap>, String>,
    match_phrase_prefix: Result<Option<super::UserDefinedValueMap>, String>,
    more_like_this: Result<Option<serde_json::Value>, String>,
    multi_match: Result<Option<serde_json::Value>, String>,
    nested: Result<Option<serde_json::Value>, String>,
    parent_id: Result<Option<serde_json::Value>, String>,
    percolate: Result<Option<serde_json::Value>, String>,
    pinned: Result<Option<serde_json::Value>, String>,
    prefix: Result<Option<super::UserDefinedValueMap>, String>,
    query_string: Result<Option<serde_json::Value>, String>,
    range: Result<Option<super::UserDefinedValueMap>, String>,
    rank_feature: Result<Option<serde_json::Value>, String>,
    regexp: Result<Option<super::UserDefinedValueMap>, String>,
    script: Result<Option<serde_json::Value>, String>,
    script_score: Result<Option<serde_json::Value>, String>,
    shape: Result<Option<serde_json::Value>, String>,
    simple_query_string: Result<Option<serde_json::Value>, String>,
    span_containing: Result<Option<serde_json::Value>, String>,
    span_first: Result<Option<serde_json::Value>, String>,
    span_multi: Result<Option<serde_json::Value>, String>,
    span_near: Result<Option<serde_json::Value>, String>,
    span_not: Result<Option<serde_json::Value>, String>,
    span_or: Result<Option<serde_json::Value>, String>,
    span_term: Result<Option<super::UserDefinedValueMap>, String>,
    span_within: Result<Option<serde_json::Value>, String>,
    term: Result<Option<super::UserDefinedValueMap>, String>,
    terms: Result<Option<serde_json::Value>, String>,
    terms_set: Result<Option<super::UserDefinedValueMap>, String>,
    wildcard: Result<Option<super::UserDefinedValueMap>, String>,
    wrapper: Result<Option<serde_json::Value>, String>,
  }

  impl Default for UserDefinedObjectStructure {
    fn default() -> Self {
      Self {
        bool: Ok(Default::default()),
        boosting: Ok(Default::default()),
        combined_fields: Ok(Default::default()),
        constant_score: Ok(Default::default()),
        dis_max: Ok(Default::default()),
        distance_feature: Ok(Default::default()),
        exists: Ok(Default::default()),
        field_masking_span: Ok(Default::default()),
        function_score: Ok(Default::default()),
        fuzzy: Ok(Default::default()),
        geo_bounding_box: Ok(Default::default()),
        geo_distance: Ok(Default::default()),
        geo_polygon: Ok(Default::default()),
        geo_shape: Ok(Default::default()),
        has_child: Ok(Default::default()),
        has_parent: Ok(Default::default()),
        ids: Ok(Default::default()),
        intervals: Ok(Default::default()),
        knn: Ok(Default::default()),
        match_: Ok(Default::default()),
        match_all: Ok(Default::default()),
        match_bool_prefix: Ok(Default::default()),
        match_none: Ok(Default::default()),
        match_phrase: Ok(Default::default()),
        match_phrase_prefix: Ok(Default::default()),
        more_like_this: Ok(Default::default()),
        multi_match: Ok(Default::default()),
        nested: Ok(Default::default()),
        parent_id: Ok(Default::default()),
        percolate: Ok(Default::default()),
        pinned: Ok(Default::default()),
        prefix: Ok(Default::default()),
        query_string: Ok(Default::default()),
        range: Ok(Default::default()),
        rank_feature: Ok(Default::default()),
        regexp: Ok(Default::default()),
        script: Ok(Default::default()),
        script_score: Ok(Default::default()),
        shape: Ok(Default::default()),
        simple_query_string: Ok(Default::default()),
        span_containing: Ok(Default::default()),
        span_first: Ok(Default::default()),
        span_multi: Ok(Default::default()),
        span_near: Ok(Default::default()),
        span_not: Ok(Default::default()),
        span_or: Ok(Default::default()),
        span_term: Ok(Default::default()),
        span_within: Ok(Default::default()),
        term: Ok(Default::default()),
        terms: Ok(Default::default()),
        terms_set: Ok(Default::default()),
        wildcard: Ok(Default::default()),
        wrapper: Ok(Default::default()),
      }
    }
  }

  impl UserDefinedObjectStructure {
    pub fn bool<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.bool = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for bool: {}", e));
      self
    }

    pub fn boosting<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.boosting = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for boosting: {}", e));
      self
    }

    pub fn combined_fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.combined_fields = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for combined_fields: {}", e));
      self
    }

    pub fn constant_score<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.constant_score = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for constant_score: {}", e));
      self
    }

    pub fn dis_max<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.dis_max = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for dis_max: {}", e));
      self
    }

    pub fn distance_feature<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.distance_feature = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for distance_feature: {}", e));
      self
    }

    pub fn exists<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.exists = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for exists: {}", e));
      self
    }

    pub fn field_masking_span<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.field_masking_span = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for field_masking_span: {}", e));
      self
    }

    pub fn function_score<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.function_score = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for function_score: {}", e));
      self
    }

    pub fn fuzzy<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.fuzzy = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for fuzzy: {}", e));
      self
    }

    pub fn geo_bounding_box<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.geo_bounding_box = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for geo_bounding_box: {}", e));
      self
    }

    pub fn geo_distance<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.geo_distance = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for geo_distance: {}", e));
      self
    }

    pub fn geo_polygon<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.geo_polygon = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for geo_polygon: {}", e));
      self
    }

    pub fn geo_shape<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.geo_shape = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for geo_shape: {}", e));
      self
    }

    pub fn has_child<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.has_child = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for has_child: {}", e));
      self
    }

    pub fn has_parent<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.has_parent = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for has_parent: {}", e));
      self
    }

    pub fn ids<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.ids = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for ids: {}", e));
      self
    }

    pub fn intervals<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.intervals = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for intervals: {}", e));
      self
    }

    pub fn knn<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.knn = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for knn: {}", e));
      self
    }

    pub fn match_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.match_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for match_: {}", e));
      self
    }

    pub fn match_all<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.match_all = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for match_all: {}", e));
      self
    }

    pub fn match_bool_prefix<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.match_bool_prefix = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for match_bool_prefix: {}", e));
      self
    }

    pub fn match_none<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.match_none = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for match_none: {}", e));
      self
    }

    pub fn match_phrase<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.match_phrase = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for match_phrase: {}", e));
      self
    }

    pub fn match_phrase_prefix<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.match_phrase_prefix = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for match_phrase_prefix: {}", e));
      self
    }

    pub fn more_like_this<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.more_like_this = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for more_like_this: {}", e));
      self
    }

    pub fn multi_match<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.multi_match = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for multi_match: {}", e));
      self
    }

    pub fn nested<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.nested = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for nested: {}", e));
      self
    }

    pub fn parent_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.parent_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for parent_id: {}", e));
      self
    }

    pub fn percolate<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.percolate = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for percolate: {}", e));
      self
    }

    pub fn pinned<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.pinned = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pinned: {}", e));
      self
    }

    pub fn prefix<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.prefix = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for prefix: {}", e));
      self
    }

    pub fn query_string<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.query_string = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for query_string: {}", e));
      self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.range = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for range: {}", e));
      self
    }

    pub fn rank_feature<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.rank_feature = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for rank_feature: {}", e));
      self
    }

    pub fn regexp<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.regexp = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for regexp: {}", e));
      self
    }

    pub fn script<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.script = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for script: {}", e));
      self
    }

    pub fn script_score<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.script_score = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for script_score: {}", e));
      self
    }

    pub fn shape<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.shape = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for shape: {}", e));
      self
    }

    pub fn simple_query_string<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.simple_query_string = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for simple_query_string: {}", e));
      self
    }

    pub fn span_containing<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.span_containing = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for span_containing: {}", e));
      self
    }

    pub fn span_first<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.span_first = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for span_first: {}", e));
      self
    }

    pub fn span_multi<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.span_multi = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for span_multi: {}", e));
      self
    }

    pub fn span_near<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.span_near = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for span_near: {}", e));
      self
    }

    pub fn span_not<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.span_not = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for span_not: {}", e));
      self
    }

    pub fn span_or<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.span_or = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for span_or: {}", e));
      self
    }

    pub fn span_term<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.span_term = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for span_term: {}", e));
      self
    }

    pub fn span_within<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.span_within = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for span_within: {}", e));
      self
    }

    pub fn term<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.term = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for term: {}", e));
      self
    }

    pub fn terms<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.terms = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for terms: {}", e));
      self
    }

    pub fn terms_set<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.terms_set = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for terms_set: {}", e));
      self
    }

    pub fn wildcard<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.wildcard = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for wildcard: {}", e));
      self
    }

    pub fn wrapper<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.wrapper = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for wrapper: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UserDefinedObjectStructure> for super::UserDefinedObjectStructure {
    type Error = String;

    fn try_from(value: UserDefinedObjectStructure) -> Result<Self, String> {
      Ok(Self {
        bool: value.bool?,
        boosting: value.boosting?,
        combined_fields: value.combined_fields?,
        constant_score: value.constant_score?,
        dis_max: value.dis_max?,
        distance_feature: value.distance_feature?,
        exists: value.exists?,
        field_masking_span: value.field_masking_span?,
        function_score: value.function_score?,
        fuzzy: value.fuzzy?,
        geo_bounding_box: value.geo_bounding_box?,
        geo_distance: value.geo_distance?,
        geo_polygon: value.geo_polygon?,
        geo_shape: value.geo_shape?,
        has_child: value.has_child?,
        has_parent: value.has_parent?,
        ids: value.ids?,
        intervals: value.intervals?,
        knn: value.knn?,
        match_: value.match_?,
        match_all: value.match_all?,
        match_bool_prefix: value.match_bool_prefix?,
        match_none: value.match_none?,
        match_phrase: value.match_phrase?,
        match_phrase_prefix: value.match_phrase_prefix?,
        more_like_this: value.more_like_this?,
        multi_match: value.multi_match?,
        nested: value.nested?,
        parent_id: value.parent_id?,
        percolate: value.percolate?,
        pinned: value.pinned?,
        prefix: value.prefix?,
        query_string: value.query_string?,
        range: value.range?,
        rank_feature: value.rank_feature?,
        regexp: value.regexp?,
        script: value.script?,
        script_score: value.script_score?,
        shape: value.shape?,
        simple_query_string: value.simple_query_string?,
        span_containing: value.span_containing?,
        span_first: value.span_first?,
        span_multi: value.span_multi?,
        span_near: value.span_near?,
        span_not: value.span_not?,
        span_or: value.span_or?,
        span_term: value.span_term?,
        span_within: value.span_within?,
        term: value.term?,
        terms: value.terms?,
        terms_set: value.terms_set?,
        wildcard: value.wildcard?,
        wrapper: value.wrapper?,
      })
    }
  }

  impl From<super::UserDefinedObjectStructure> for UserDefinedObjectStructure {
    fn from(value: super::UserDefinedObjectStructure) -> Self {
      Self {
        bool: Ok(value.bool),
        boosting: Ok(value.boosting),
        combined_fields: Ok(value.combined_fields),
        constant_score: Ok(value.constant_score),
        dis_max: Ok(value.dis_max),
        distance_feature: Ok(value.distance_feature),
        exists: Ok(value.exists),
        field_masking_span: Ok(value.field_masking_span),
        function_score: Ok(value.function_score),
        fuzzy: Ok(value.fuzzy),
        geo_bounding_box: Ok(value.geo_bounding_box),
        geo_distance: Ok(value.geo_distance),
        geo_polygon: Ok(value.geo_polygon),
        geo_shape: Ok(value.geo_shape),
        has_child: Ok(value.has_child),
        has_parent: Ok(value.has_parent),
        ids: Ok(value.ids),
        intervals: Ok(value.intervals),
        knn: Ok(value.knn),
        match_: Ok(value.match_),
        match_all: Ok(value.match_all),
        match_bool_prefix: Ok(value.match_bool_prefix),
        match_none: Ok(value.match_none),
        match_phrase: Ok(value.match_phrase),
        match_phrase_prefix: Ok(value.match_phrase_prefix),
        more_like_this: Ok(value.more_like_this),
        multi_match: Ok(value.multi_match),
        nested: Ok(value.nested),
        parent_id: Ok(value.parent_id),
        percolate: Ok(value.percolate),
        pinned: Ok(value.pinned),
        prefix: Ok(value.prefix),
        query_string: Ok(value.query_string),
        range: Ok(value.range),
        rank_feature: Ok(value.rank_feature),
        regexp: Ok(value.regexp),
        script: Ok(value.script),
        script_score: Ok(value.script_score),
        shape: Ok(value.shape),
        simple_query_string: Ok(value.simple_query_string),
        span_containing: Ok(value.span_containing),
        span_first: Ok(value.span_first),
        span_multi: Ok(value.span_multi),
        span_near: Ok(value.span_near),
        span_not: Ok(value.span_not),
        span_or: Ok(value.span_or),
        span_term: Ok(value.span_term),
        span_within: Ok(value.span_within),
        term: Ok(value.term),
        terms: Ok(value.terms),
        terms_set: Ok(value.terms_set),
        wildcard: Ok(value.wildcard),
        wrapper: Ok(value.wrapper),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct UserDefinedStructure {
    alias: Result<Option<String>, String>,
    aliases: Result<Vec<String>, String>,
    filter: Result<Option<serde_json::Value>, String>,
    index: Result<Option<String>, String>,
    index_routing: Result<Option<String>, String>,
    indices: Result<Vec<String>, String>,
    is_hidden: Result<Option<bool>, String>,
    is_write_index: Result<Option<bool>, String>,
    must_exist: Result<Option<String>, String>,
    routing: Result<Option<String>, String>,
    search_routing: Result<Option<String>, String>,
  }

  impl Default for UserDefinedStructure {
    fn default() -> Self {
      Self {
        alias: Ok(Default::default()),
        aliases: Ok(Default::default()),
        filter: Ok(Default::default()),
        index: Ok(Default::default()),
        index_routing: Ok(Default::default()),
        indices: Ok(Default::default()),
        is_hidden: Ok(Default::default()),
        is_write_index: Ok(Default::default()),
        must_exist: Ok(Default::default()),
        routing: Ok(Default::default()),
        search_routing: Ok(Default::default()),
      }
    }
  }

  impl UserDefinedStructure {
    pub fn alias<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.alias = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for alias: {}", e));
      self
    }

    pub fn aliases<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.aliases = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for aliases: {}", e));
      self
    }

    pub fn filter<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.filter = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for filter: {}", e));
      self
    }

    pub fn index<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.index = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index: {}", e));
      self
    }

    pub fn index_routing<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.index_routing = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index_routing: {}", e));
      self
    }

    pub fn indices<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.indices = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for indices: {}", e));
      self
    }

    pub fn is_hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.is_hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for is_hidden: {}", e));
      self
    }

    pub fn is_write_index<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.is_write_index = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for is_write_index: {}", e));
      self
    }

    pub fn must_exist<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.must_exist = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for must_exist: {}", e));
      self
    }

    pub fn routing<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.routing = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for routing: {}", e));
      self
    }

    pub fn search_routing<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.search_routing = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for search_routing: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UserDefinedStructure> for super::UserDefinedStructure {
    type Error = String;

    fn try_from(value: UserDefinedStructure) -> Result<Self, String> {
      Ok(Self {
        alias: value.alias?,
        aliases: value.aliases?,
        filter: value.filter?,
        index: value.index?,
        index_routing: value.index_routing?,
        indices: value.indices?,
        is_hidden: value.is_hidden?,
        is_write_index: value.is_write_index?,
        must_exist: value.must_exist?,
        routing: value.routing?,
        search_routing: value.search_routing?,
      })
    }
  }

  impl From<super::UserDefinedStructure> for UserDefinedStructure {
    fn from(value: super::UserDefinedStructure) -> Self {
      Self {
        alias: Ok(value.alias),
        aliases: Ok(value.aliases),
        filter: Ok(value.filter),
        index: Ok(value.index),
        index_routing: Ok(value.index_routing),
        indices: Ok(value.indices),
        is_hidden: Ok(value.is_hidden),
        is_write_index: Ok(value.is_write_index),
        must_exist: Ok(value.must_exist),
        routing: Ok(value.routing),
        search_routing: Ok(value.search_routing),
      }
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentDeleteResponse {
  #[serde(rename = "_index")]
  pub index: String,
  #[serde(rename = "_id")]
  pub id: String,
  #[serde(rename = "_version")]
  pub version: u32,
  pub result: String,
  #[serde(rename = "_shards")]
  pub shards: ShardStatistics,
  #[serde(rename = "_seq_no")]
  pub seq_no: u32,
  #[serde(rename = "_primary_term")]
  pub primary_term: u32,
}
