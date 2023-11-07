#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::types::{DataStream, UserDefinedStructure, UserDefinedValueMap};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActionObjectStructure {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub add: Option<UserDefinedStructure>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub remove: Option<UserDefinedStructure>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub remove_index: Option<UserDefinedStructure>,
}

impl From<&ActionObjectStructure> for ActionObjectStructure {
  fn from(value: &ActionObjectStructure) -> Self {
    value.clone()
  }
}

impl ActionObjectStructure {
  pub fn builder() -> builder::ActionObjectStructure {
    builder::ActionObjectStructure::default()
  }
}

///Return stats aggregated at cluster, index or shard level.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IndiciesStatLevel {
  #[serde(rename = "cluster")]
  Cluster,
  #[serde(rename = "indices")]
  Indices,
  #[serde(rename = "shards")]
  Shards,
}

impl From<&IndiciesStatLevel> for IndiciesStatLevel {
  fn from(value: &IndiciesStatLevel) -> Self {
    value.clone()
  }
}

impl ToString for IndiciesStatLevel {
  fn to_string(&self) -> String {
    match *self {
      Self::Cluster => "cluster".to_string(),
      Self::Indices => "indices".to_string(),
      Self::Shards => "shards".to_string(),
    }
  }
}

impl std::str::FromStr for IndiciesStatLevel {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "cluster" => Ok(Self::Cluster),
      "indices" => Ok(Self::Indices),
      "shards" => Ok(Self::Shards),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for IndiciesStatLevel {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndiciesStatLevel {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndiciesStatLevel {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

///The block to add (one of read, write, read_only or metadata).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesAddBlockBlock(String);
impl std::ops::Deref for IndicesAddBlockBlock {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesAddBlockBlock> for String {
  fn from(value: IndicesAddBlockBlock) -> Self {
    value.0
  }
}
impl From<&IndicesAddBlockBlock> for IndicesAddBlockBlock {
  fn from(value: &IndicesAddBlockBlock) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesAddBlockBlock {
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
impl std::convert::TryFrom<&str> for IndicesAddBlockBlock {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesAddBlockBlock {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesAddBlockBlock {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesAddBlockBlock {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Define analyzer/tokenizer parameters and the text on which the analysis
/// should be performed
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesAnalyzeBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesAnalyzeBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesAnalyzeBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesAnalyzeBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesAnalyzeBodyParams> for IndicesAnalyzeBodyParams {
  fn from(value: &IndicesAnalyzeBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesAnalyzeBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The configuration for the target index (`settings` and `aliases`)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesCloneBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesCloneBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesCloneBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesCloneBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesCloneBodyParams> for IndicesCloneBodyParams {
  fn from(value: &IndicesCloneBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesCloneBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The name of the target index.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePostTarget(String);
impl std::ops::Deref for IndicesClonePostTarget {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesClonePostTarget> for String {
  fn from(value: IndicesClonePostTarget) -> Self {
    value.0
  }
}
impl From<&IndicesClonePostTarget> for IndicesClonePostTarget {
  fn from(value: &IndicesClonePostTarget) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesClonePostTarget {
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
impl std::convert::TryFrom<&str> for IndicesClonePostTarget {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesClonePostTarget {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesClonePostTarget {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesClonePostTarget {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The name of the target index.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePutTarget(String);
impl std::ops::Deref for IndicesClonePutTarget {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesClonePutTarget> for String {
  fn from(value: IndicesClonePutTarget) -> Self {
    value.0
  }
}
impl From<&IndicesClonePutTarget> for IndicesClonePutTarget {
  fn from(value: &IndicesClonePutTarget) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesClonePutTarget {
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
impl std::convert::TryFrom<&str> for IndicesClonePutTarget {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesClonePutTarget {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesClonePutTarget {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesClonePutTarget {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The configuration for the index (`settings` and `mappings`)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesCreateBodyParams {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub aliases: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub mapping: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub settings: Option<UserDefinedValueMap>,
}
impl From<&IndicesCreateBodyParams> for IndicesCreateBodyParams {
  fn from(value: &IndicesCreateBodyParams) -> Self {
    value.clone()
  }
}
impl IndicesCreateBodyParams {
  pub fn builder() -> builder::IndicesCreateBodyParams {
    builder::IndicesCreateBodyParams::default()
  }
}
///The data stream definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesCreateDataStreamBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesCreateDataStreamBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesCreateDataStreamBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesCreateDataStreamBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesCreateDataStreamBodyParams> for IndicesCreateDataStreamBodyParams {
  fn from(value: &IndicesCreateDataStreamBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesCreateDataStreamBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The name of the data stream.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesCreateDataStreamName(String);
impl std::ops::Deref for IndicesCreateDataStreamName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesCreateDataStreamName> for String {
  fn from(value: IndicesCreateDataStreamName) -> Self {
    value.0
  }
}
impl From<&IndicesCreateDataStreamName> for IndicesCreateDataStreamName {
  fn from(value: &IndicesCreateDataStreamName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesCreateDataStreamName {
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
impl std::convert::TryFrom<&str> for IndicesCreateDataStreamName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesCreateDataStreamName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesCreateDataStreamName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesCreateDataStreamName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesCreateDataStreamResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub acknowledged: Option<bool>,
}
impl From<&IndicesCreateDataStreamResponseContent> for IndicesCreateDataStreamResponseContent {
  fn from(value: &IndicesCreateDataStreamResponseContent) -> Self {
    value.clone()
  }
}
impl IndicesCreateDataStreamResponseContent {
  pub fn builder() -> builder::IndicesCreateDataStreamResponseContent {
    builder::IndicesCreateDataStreamResponseContent::default()
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesCreateResponseContent {
  pub acknowledged: bool,
  pub index: String,
  pub shards_acknowledged: bool,
}
impl From<&IndicesCreateResponseContent> for IndicesCreateResponseContent {
  fn from(value: &IndicesCreateResponseContent) -> Self {
    value.clone()
  }
}
impl IndicesCreateResponseContent {
  pub fn builder() -> builder::IndicesCreateResponseContent {
    builder::IndicesCreateResponseContent::default()
  }
}
///Comma-separated list of data streams; use `_all` or empty string to
/// perform the operation on all data streams.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDataStreamsStatsWithNameName(String);
impl std::ops::Deref for IndicesDataStreamsStatsWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesDataStreamsStatsWithNameName> for String {
  fn from(value: IndicesDataStreamsStatsWithNameName) -> Self {
    value.0
  }
}
impl From<&IndicesDataStreamsStatsWithNameName> for IndicesDataStreamsStatsWithNameName {
  fn from(value: &IndicesDataStreamsStatsWithNameName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesDataStreamsStatsWithNameName {
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
impl std::convert::TryFrom<&str> for IndicesDataStreamsStatsWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesDataStreamsStatsWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesDataStreamsStatsWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesDataStreamsStatsWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of aliases to delete (supports wildcards); use
/// `_all` to delete all aliases for the specified indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasName(String);
impl std::ops::Deref for IndicesDeleteAliasName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesDeleteAliasName> for String {
  fn from(value: IndicesDeleteAliasName) -> Self {
    value.0
  }
}
impl From<&IndicesDeleteAliasName> for IndicesDeleteAliasName {
  fn from(value: &IndicesDeleteAliasName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesDeleteAliasName {
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
impl std::convert::TryFrom<&str> for IndicesDeleteAliasName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesDeleteAliasName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesDeleteAliasName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Comma-separated list of aliases to delete (supports wildcards); use
/// `_all` to delete all aliases for the specified indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasPluralName(String);
impl std::ops::Deref for IndicesDeleteAliasPluralName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesDeleteAliasPluralName> for String {
  fn from(value: IndicesDeleteAliasPluralName) -> Self {
    value.0
  }
}
impl From<&IndicesDeleteAliasPluralName> for IndicesDeleteAliasPluralName {
  fn from(value: &IndicesDeleteAliasPluralName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesDeleteAliasPluralName {
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
impl std::convert::TryFrom<&str> for IndicesDeleteAliasPluralName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesDeleteAliasPluralName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesDeleteAliasPluralName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasPluralName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Comma-separated list of data streams; use `_all` or empty string to
/// perform the operation on all data streams.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteDataStreamName(String);
impl std::ops::Deref for IndicesDeleteDataStreamName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesDeleteDataStreamName> for String {
  fn from(value: IndicesDeleteDataStreamName) -> Self {
    value.0
  }
}
impl From<&IndicesDeleteDataStreamName> for IndicesDeleteDataStreamName {
  fn from(value: &IndicesDeleteDataStreamName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesDeleteDataStreamName {
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
impl std::convert::TryFrom<&str> for IndicesDeleteDataStreamName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesDeleteDataStreamName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesDeleteDataStreamName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesDeleteDataStreamName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesDeleteDataStreamResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub acknowledged: Option<bool>,
}
impl From<&IndicesDeleteDataStreamResponseContent> for IndicesDeleteDataStreamResponseContent {
  fn from(value: &IndicesDeleteDataStreamResponseContent) -> Self {
    value.clone()
  }
}
impl IndicesDeleteDataStreamResponseContent {
  pub fn builder() -> builder::IndicesDeleteDataStreamResponseContent {
    builder::IndicesDeleteDataStreamResponseContent::default()
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesDeleteResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub acknowledged: Option<bool>,
}
impl From<&IndicesDeleteResponseContent> for IndicesDeleteResponseContent {
  fn from(value: &IndicesDeleteResponseContent) -> Self {
    value.clone()
  }
}
impl IndicesDeleteResponseContent {
  pub fn builder() -> builder::IndicesDeleteResponseContent {
    builder::IndicesDeleteResponseContent::default()
  }
}
///The name of the template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteTemplateName(String);
impl std::ops::Deref for IndicesDeleteTemplateName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesDeleteTemplateName> for String {
  fn from(value: IndicesDeleteTemplateName) -> Self {
    value.0
  }
}
impl From<&IndicesDeleteTemplateName> for IndicesDeleteTemplateName {
  fn from(value: &IndicesDeleteTemplateName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesDeleteTemplateName {
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
impl std::convert::TryFrom<&str> for IndicesDeleteTemplateName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesDeleteTemplateName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesDeleteTemplateName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesDeleteTemplateName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Operation timeout.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesTimeout(String);
impl std::ops::Deref for IndicesTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesTimeout> for String {
  fn from(value: IndicesTimeout) -> Self {
    value.0
  }
}
impl From<&IndicesTimeout> for IndicesTimeout {
  fn from(value: &IndicesTimeout) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesTimeout {
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
impl std::convert::TryFrom<&str> for IndicesTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Comma-separated list of alias names.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesExistsAliasName(String);
impl std::ops::Deref for IndicesExistsAliasName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesExistsAliasName> for String {
  fn from(value: IndicesExistsAliasName) -> Self {
    value.0
  }
}
impl From<&IndicesExistsAliasName> for IndicesExistsAliasName {
  fn from(value: &IndicesExistsAliasName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesExistsAliasName {
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
impl std::convert::TryFrom<&str> for IndicesExistsAliasName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesExistsAliasName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesExistsAliasName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesExistsAliasName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The name of the template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesExistsIndexTemplateName(String);
impl std::ops::Deref for IndicesExistsIndexTemplateName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesExistsIndexTemplateName> for String {
  fn from(value: IndicesExistsIndexTemplateName) -> Self {
    value.0
  }
}
impl From<&IndicesExistsIndexTemplateName> for IndicesExistsIndexTemplateName {
  fn from(value: &IndicesExistsIndexTemplateName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesExistsIndexTemplateName {
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
impl std::convert::TryFrom<&str> for IndicesExistsIndexTemplateName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesExistsIndexTemplateName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesExistsIndexTemplateName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesExistsIndexTemplateName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Comma-separated names of the index templates.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesExistsTemplateName(String);
impl std::ops::Deref for IndicesExistsTemplateName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesExistsTemplateName> for String {
  fn from(value: IndicesExistsTemplateName) -> Self {
    value.0
  }
}
impl From<&IndicesExistsTemplateName> for IndicesExistsTemplateName {
  fn from(value: &IndicesExistsTemplateName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesExistsTemplateName {
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
impl std::convert::TryFrom<&str> for IndicesExistsTemplateName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesExistsTemplateName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesExistsTemplateName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesExistsTemplateName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of alias names.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetAliasWithNameName(String);
impl std::ops::Deref for IndicesGetAliasWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesGetAliasWithNameName> for String {
  fn from(value: IndicesGetAliasWithNameName) -> Self {
    value.0
  }
}
impl From<&IndicesGetAliasWithNameName> for IndicesGetAliasWithNameName {
  fn from(value: &IndicesGetAliasWithNameName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesGetAliasWithNameName {
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
impl std::convert::TryFrom<&str> for IndicesGetAliasWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesGetAliasWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesGetAliasWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesGetAliasWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesGetDataStreamResponseContent {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub data_streams: Vec<DataStream>,
}
impl From<&IndicesGetDataStreamResponseContent> for IndicesGetDataStreamResponseContent {
  fn from(value: &IndicesGetDataStreamResponseContent) -> Self {
    value.clone()
  }
}
impl IndicesGetDataStreamResponseContent {
  pub fn builder() -> builder::IndicesGetDataStreamResponseContent {
    builder::IndicesGetDataStreamResponseContent::default()
  }
}
///Comma-separated list of data streams; use `_all` or empty string to
/// perform the operation on all data streams.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetDataStreamWithNameName(String);
impl std::ops::Deref for IndicesGetDataStreamWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesGetDataStreamWithNameName> for String {
  fn from(value: IndicesGetDataStreamWithNameName) -> Self {
    value.0
  }
}
impl From<&IndicesGetDataStreamWithNameName> for IndicesGetDataStreamWithNameName {
  fn from(value: &IndicesGetDataStreamWithNameName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesGetDataStreamWithNameName {
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
impl std::convert::TryFrom<&str> for IndicesGetDataStreamWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesGetDataStreamWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesGetDataStreamWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesGetDataStreamWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesGetDataStreamWithNameResponseContent {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub data_streams: Vec<DataStream>,
}
impl From<&IndicesGetDataStreamWithNameResponseContent> for IndicesGetDataStreamWithNameResponseContent {
  fn from(value: &IndicesGetDataStreamWithNameResponseContent) -> Self {
    value.clone()
  }
}
impl IndicesGetDataStreamWithNameResponseContent {
  pub fn builder() -> builder::IndicesGetDataStreamWithNameResponseContent {
    builder::IndicesGetDataStreamWithNameResponseContent::default()
  }
}
///Comma-separated list of fields.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetFieldMappingFields(String);
impl std::ops::Deref for IndicesGetFieldMappingFields {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesGetFieldMappingFields> for String {
  fn from(value: IndicesGetFieldMappingFields) -> Self {
    value.0
  }
}
impl From<&IndicesGetFieldMappingFields> for IndicesGetFieldMappingFields {
  fn from(value: &IndicesGetFieldMappingFields) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesGetFieldMappingFields {
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
impl std::convert::TryFrom<&str> for IndicesGetFieldMappingFields {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesGetFieldMappingFields {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesGetFieldMappingFields {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesGetFieldMappingFields {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Comma-separated list of fields.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetFieldMappingWithIndexFields(String);
impl std::ops::Deref for IndicesGetFieldMappingWithIndexFields {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesGetFieldMappingWithIndexFields> for String {
  fn from(value: IndicesGetFieldMappingWithIndexFields) -> Self {
    value.0
  }
}
impl From<&IndicesGetFieldMappingWithIndexFields> for IndicesGetFieldMappingWithIndexFields {
  fn from(value: &IndicesGetFieldMappingWithIndexFields) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesGetFieldMappingWithIndexFields {
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
impl std::convert::TryFrom<&str> for IndicesGetFieldMappingWithIndexFields {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesGetFieldMappingWithIndexFields {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesGetFieldMappingWithIndexFields {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesGetFieldMappingWithIndexFields {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Comma-separated names of the index templates.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetIndexTemplateWithNameName(String);
impl std::ops::Deref for IndicesGetIndexTemplateWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesGetIndexTemplateWithNameName> for String {
  fn from(value: IndicesGetIndexTemplateWithNameName) -> Self {
    value.0
  }
}
impl From<&IndicesGetIndexTemplateWithNameName> for IndicesGetIndexTemplateWithNameName {
  fn from(value: &IndicesGetIndexTemplateWithNameName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesGetIndexTemplateWithNameName {
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
impl std::convert::TryFrom<&str> for IndicesGetIndexTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesGetIndexTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesGetIndexTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesGetIndexTemplateWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of settings.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsWithIndexNameName(String);
impl std::ops::Deref for IndicesGetSettingsWithIndexNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesGetSettingsWithIndexNameName> for String {
  fn from(value: IndicesGetSettingsWithIndexNameName) -> Self {
    value.0
  }
}
impl From<&IndicesGetSettingsWithIndexNameName> for IndicesGetSettingsWithIndexNameName {
  fn from(value: &IndicesGetSettingsWithIndexNameName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesGetSettingsWithIndexNameName {
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
impl std::convert::TryFrom<&str> for IndicesGetSettingsWithIndexNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesGetSettingsWithIndexNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesGetSettingsWithIndexNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithIndexNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Comma-separated list of settings.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsWithNameName(String);
impl std::ops::Deref for IndicesGetSettingsWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesGetSettingsWithNameName> for String {
  fn from(value: IndicesGetSettingsWithNameName) -> Self {
    value.0
  }
}
impl From<&IndicesGetSettingsWithNameName> for IndicesGetSettingsWithNameName {
  fn from(value: &IndicesGetSettingsWithNameName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesGetSettingsWithNameName {
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
impl std::convert::TryFrom<&str> for IndicesGetSettingsWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesGetSettingsWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesGetSettingsWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Comma-separated names of the index templates.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetTemplateWithNameName(String);
impl std::ops::Deref for IndicesGetTemplateWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesGetTemplateWithNameName> for String {
  fn from(value: IndicesGetTemplateWithNameName) -> Self {
    value.0
  }
}
impl From<&IndicesGetTemplateWithNameName> for IndicesGetTemplateWithNameName {
  fn from(value: &IndicesGetTemplateWithNameName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesGetTemplateWithNameName {
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
impl std::convert::TryFrom<&str> for IndicesGetTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesGetTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesGetTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesGetTemplateWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The settings for the alias, such as `routing` or `filter`
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesPutAliasBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesPutAliasBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesPutAliasBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesPutAliasBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesPutAliasBodyParams> for IndicesPutAliasBodyParams {
  fn from(value: &IndicesPutAliasBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesPutAliasBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The name of the alias to be created or updated.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostName(String);
impl std::ops::Deref for IndicesPutAliasPostName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesPutAliasPostName> for String {
  fn from(value: IndicesPutAliasPostName) -> Self {
    value.0
  }
}
impl From<&IndicesPutAliasPostName> for IndicesPutAliasPostName {
  fn from(value: &IndicesPutAliasPostName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesPutAliasPostName {
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
impl std::convert::TryFrom<&str> for IndicesPutAliasPostName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesPutAliasPostName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesPutAliasPostName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The name of the alias to be created or updated.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostPluralName(String);
impl std::ops::Deref for IndicesPutAliasPostPluralName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesPutAliasPostPluralName> for String {
  fn from(value: IndicesPutAliasPostPluralName) -> Self {
    value.0
  }
}
impl From<&IndicesPutAliasPostPluralName> for IndicesPutAliasPostPluralName {
  fn from(value: &IndicesPutAliasPostPluralName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesPutAliasPostPluralName {
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
impl std::convert::TryFrom<&str> for IndicesPutAliasPostPluralName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesPutAliasPostPluralName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesPutAliasPostPluralName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostPluralName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The name of the alias to be created or updated.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutName(String);
impl std::ops::Deref for IndicesPutAliasPutName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesPutAliasPutName> for String {
  fn from(value: IndicesPutAliasPutName) -> Self {
    value.0
  }
}
impl From<&IndicesPutAliasPutName> for IndicesPutAliasPutName {
  fn from(value: &IndicesPutAliasPutName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesPutAliasPutName {
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
impl std::convert::TryFrom<&str> for IndicesPutAliasPutName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesPutAliasPutName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesPutAliasPutName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The name of the alias to be created or updated.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutPluralName(String);
impl std::ops::Deref for IndicesPutAliasPutPluralName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesPutAliasPutPluralName> for String {
  fn from(value: IndicesPutAliasPutPluralName) -> Self {
    value.0
  }
}
impl From<&IndicesPutAliasPutPluralName> for IndicesPutAliasPutPluralName {
  fn from(value: &IndicesPutAliasPutPluralName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesPutAliasPutPluralName {
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
impl std::convert::TryFrom<&str> for IndicesPutAliasPutPluralName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesPutAliasPutPluralName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesPutAliasPutPluralName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutPluralName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The template definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesPutIndexTemplateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesPutIndexTemplateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesPutIndexTemplateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesPutIndexTemplateBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesPutIndexTemplateBodyParams> for IndicesPutIndexTemplateBodyParams {
  fn from(value: &IndicesPutIndexTemplateBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesPutIndexTemplateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The name of the template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutIndexTemplatePostName(String);
impl std::ops::Deref for IndicesPutIndexTemplatePostName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesPutIndexTemplatePostName> for String {
  fn from(value: IndicesPutIndexTemplatePostName) -> Self {
    value.0
  }
}
impl From<&IndicesPutIndexTemplatePostName> for IndicesPutIndexTemplatePostName {
  fn from(value: &IndicesPutIndexTemplatePostName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesPutIndexTemplatePostName {
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
impl std::convert::TryFrom<&str> for IndicesPutIndexTemplatePostName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesPutIndexTemplatePostName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesPutIndexTemplatePostName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesPutIndexTemplatePostName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The name of the template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutIndexTemplatePutName(String);
impl std::ops::Deref for IndicesPutIndexTemplatePutName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesPutIndexTemplatePutName> for String {
  fn from(value: IndicesPutIndexTemplatePutName) -> Self {
    value.0
  }
}
impl From<&IndicesPutIndexTemplatePutName> for IndicesPutIndexTemplatePutName {
  fn from(value: &IndicesPutIndexTemplatePutName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesPutIndexTemplatePutName {
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
impl std::convert::TryFrom<&str> for IndicesPutIndexTemplatePutName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesPutIndexTemplatePutName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesPutIndexTemplatePutName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesPutIndexTemplatePutName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The mapping definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesPutMappingBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesPutMappingBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesPutMappingBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesPutMappingBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesPutMappingBodyParams> for IndicesPutMappingBodyParams {
  fn from(value: &IndicesPutMappingBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesPutMappingBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesPutMappingPostResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub acknowledged: Option<bool>,
}
impl From<&IndicesPutMappingPostResponseContent> for IndicesPutMappingPostResponseContent {
  fn from(value: &IndicesPutMappingPostResponseContent) -> Self {
    value.clone()
  }
}
impl IndicesPutMappingPostResponseContent {
  pub fn builder() -> builder::IndicesPutMappingPostResponseContent {
    builder::IndicesPutMappingPostResponseContent::default()
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesPutMappingPutResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub acknowledged: Option<bool>,
}
impl From<&IndicesPutMappingPutResponseContent> for IndicesPutMappingPutResponseContent {
  fn from(value: &IndicesPutMappingPutResponseContent) -> Self {
    value.clone()
  }
}
impl IndicesPutMappingPutResponseContent {
  pub fn builder() -> builder::IndicesPutMappingPutResponseContent {
    builder::IndicesPutMappingPutResponseContent::default()
  }
}
///The index settings to be updated
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesPutSettingsBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesPutSettingsBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesPutSettingsBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesPutSettingsBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesPutSettingsBodyParams> for IndicesPutSettingsBodyParams {
  fn from(value: &IndicesPutSettingsBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesPutSettingsBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The template definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesPutTemplateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesPutTemplateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesPutTemplateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesPutTemplateBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesPutTemplateBodyParams> for IndicesPutTemplateBodyParams {
  fn from(value: &IndicesPutTemplateBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesPutTemplateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The name of the template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutTemplatePostName(String);
impl std::ops::Deref for IndicesPutTemplatePostName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesPutTemplatePostName> for String {
  fn from(value: IndicesPutTemplatePostName) -> Self {
    value.0
  }
}
impl From<&IndicesPutTemplatePostName> for IndicesPutTemplatePostName {
  fn from(value: &IndicesPutTemplatePostName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesPutTemplatePostName {
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
impl std::convert::TryFrom<&str> for IndicesPutTemplatePostName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesPutTemplatePostName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesPutTemplatePostName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesPutTemplatePostName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The name of the template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutTemplatePutName(String);
impl std::ops::Deref for IndicesPutTemplatePutName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesPutTemplatePutName> for String {
  fn from(value: IndicesPutTemplatePutName) -> Self {
    value.0
  }
}
impl From<&IndicesPutTemplatePutName> for IndicesPutTemplatePutName {
  fn from(value: &IndicesPutTemplatePutName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesPutTemplatePutName {
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
impl std::convert::TryFrom<&str> for IndicesPutTemplatePutName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesPutTemplatePutName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesPutTemplatePutName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesPutTemplatePutName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The name of the alias to rollover.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRolloverAlias(String);
impl std::ops::Deref for IndicesRolloverAlias {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesRolloverAlias> for String {
  fn from(value: IndicesRolloverAlias) -> Self {
    value.0
  }
}
impl From<&IndicesRolloverAlias> for IndicesRolloverAlias {
  fn from(value: &IndicesRolloverAlias) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesRolloverAlias {
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
impl std::convert::TryFrom<&str> for IndicesRolloverAlias {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesRolloverAlias {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesRolloverAlias {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesRolloverAlias {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The conditions that needs to be met for executing rollover
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesRolloverBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesRolloverBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesRolloverBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesRolloverBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesRolloverBodyParams> for IndicesRolloverBodyParams {
  fn from(value: &IndicesRolloverBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesRolloverBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The configuration for the target index (`settings` and `aliases`)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesShrinkBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesShrinkBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesShrinkBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesShrinkBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesShrinkBodyParams> for IndicesShrinkBodyParams {
  fn from(value: &IndicesShrinkBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesShrinkBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The name of the target index.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPostTarget(String);
impl std::ops::Deref for IndicesShrinkPostTarget {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesShrinkPostTarget> for String {
  fn from(value: IndicesShrinkPostTarget) -> Self {
    value.0
  }
}
impl From<&IndicesShrinkPostTarget> for IndicesShrinkPostTarget {
  fn from(value: &IndicesShrinkPostTarget) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesShrinkPostTarget {
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
impl std::convert::TryFrom<&str> for IndicesShrinkPostTarget {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesShrinkPostTarget {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesShrinkPostTarget {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesShrinkPostTarget {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The name of the target index.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPutTarget(String);
impl std::ops::Deref for IndicesShrinkPutTarget {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesShrinkPutTarget> for String {
  fn from(value: IndicesShrinkPutTarget) -> Self {
    value.0
  }
}
impl From<&IndicesShrinkPutTarget> for IndicesShrinkPutTarget {
  fn from(value: &IndicesShrinkPutTarget) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesShrinkPutTarget {
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
impl std::convert::TryFrom<&str> for IndicesShrinkPutTarget {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesShrinkPutTarget {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesShrinkPutTarget {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesShrinkPutTarget {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///New index template definition, which will be included in the simulation,
/// as if it already exists in the system
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesSimulateIndexTemplateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesSimulateIndexTemplateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesSimulateIndexTemplateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesSimulateIndexTemplateBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesSimulateIndexTemplateBodyParams> for IndicesSimulateIndexTemplateBodyParams {
  fn from(value: &IndicesSimulateIndexTemplateBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesSimulateIndexTemplateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The name of the index (it must be a concrete index name).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSimulateIndexTemplateName(String);
impl std::ops::Deref for IndicesSimulateIndexTemplateName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesSimulateIndexTemplateName> for String {
  fn from(value: IndicesSimulateIndexTemplateName) -> Self {
    value.0
  }
}
impl From<&IndicesSimulateIndexTemplateName> for IndicesSimulateIndexTemplateName {
  fn from(value: &IndicesSimulateIndexTemplateName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesSimulateIndexTemplateName {
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
impl std::convert::TryFrom<&str> for IndicesSimulateIndexTemplateName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesSimulateIndexTemplateName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesSimulateIndexTemplateName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesSimulateIndexTemplateName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///New index template definition to be simulated, if no index template name
/// is specified
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesSimulateTemplateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesSimulateTemplateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesSimulateTemplateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesSimulateTemplateBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesSimulateTemplateBodyParams> for IndicesSimulateTemplateBodyParams {
  fn from(value: &IndicesSimulateTemplateBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesSimulateTemplateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The name of the template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSimulateTemplateWithNameName(String);
impl std::ops::Deref for IndicesSimulateTemplateWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesSimulateTemplateWithNameName> for String {
  fn from(value: IndicesSimulateTemplateWithNameName) -> Self {
    value.0
  }
}
impl From<&IndicesSimulateTemplateWithNameName> for IndicesSimulateTemplateWithNameName {
  fn from(value: &IndicesSimulateTemplateWithNameName) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesSimulateTemplateWithNameName {
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
impl std::convert::TryFrom<&str> for IndicesSimulateTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesSimulateTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesSimulateTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesSimulateTemplateWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The configuration for the target index (`settings` and `aliases`)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesSplitBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesSplitBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesSplitBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesSplitBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesSplitBodyParams> for IndicesSplitBodyParams {
  fn from(value: &IndicesSplitBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesSplitBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The name of the target index.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPostTarget(String);
impl std::ops::Deref for IndicesSplitPostTarget {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesSplitPostTarget> for String {
  fn from(value: IndicesSplitPostTarget) -> Self {
    value.0
  }
}
impl From<&IndicesSplitPostTarget> for IndicesSplitPostTarget {
  fn from(value: &IndicesSplitPostTarget) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesSplitPostTarget {
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
impl std::convert::TryFrom<&str> for IndicesSplitPostTarget {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesSplitPostTarget {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesSplitPostTarget {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesSplitPostTarget {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The name of the target index.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPutTarget(String);
impl std::ops::Deref for IndicesSplitPutTarget {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesSplitPutTarget> for String {
  fn from(value: IndicesSplitPutTarget) -> Self {
    value.0
  }
}
impl From<&IndicesSplitPutTarget> for IndicesSplitPutTarget {
  fn from(value: &IndicesSplitPutTarget) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesSplitPutTarget {
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
impl std::convert::TryFrom<&str> for IndicesSplitPutTarget {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesSplitPutTarget {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesSplitPutTarget {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesSplitPutTarget {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Limit the information returned the specific metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesStatsWithIndexMetricMetric(String);
impl std::ops::Deref for IndicesStatsWithIndexMetricMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesStatsWithIndexMetricMetric> for String {
  fn from(value: IndicesStatsWithIndexMetricMetric) -> Self {
    value.0
  }
}
impl From<&IndicesStatsWithIndexMetricMetric> for IndicesStatsWithIndexMetricMetric {
  fn from(value: &IndicesStatsWithIndexMetricMetric) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesStatsWithIndexMetricMetric {
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
impl std::convert::TryFrom<&str> for IndicesStatsWithIndexMetricMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesStatsWithIndexMetricMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesStatsWithIndexMetricMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesStatsWithIndexMetricMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Limit the information returned the specific metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesStatsWithMetricMetric(String);
impl std::ops::Deref for IndicesStatsWithMetricMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<IndicesStatsWithMetricMetric> for String {
  fn from(value: IndicesStatsWithMetricMetric) -> Self {
    value.0
  }
}
impl From<&IndicesStatsWithMetricMetric> for IndicesStatsWithMetricMetric {
  fn from(value: &IndicesStatsWithMetricMetric) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for IndicesStatsWithMetricMetric {
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
impl std::convert::TryFrom<&str> for IndicesStatsWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for IndicesStatsWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for IndicesStatsWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for IndicesStatsWithMetricMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The definition of `actions` to perform
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesUpdateAliasesBodyParams {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub actions: Option<ActionObjectStructure>,
}
impl From<&IndicesUpdateAliasesBodyParams> for IndicesUpdateAliasesBodyParams {
  fn from(value: &IndicesUpdateAliasesBodyParams) -> Self {
    value.clone()
  }
}
impl IndicesUpdateAliasesBodyParams {
  pub fn builder() -> builder::IndicesUpdateAliasesBodyParams {
    builder::IndicesUpdateAliasesBodyParams::default()
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesUpdateAliasesResponseContent {
  pub acknowledged: bool,
}
impl From<&IndicesUpdateAliasesResponseContent> for IndicesUpdateAliasesResponseContent {
  fn from(value: &IndicesUpdateAliasesResponseContent) -> Self {
    value.clone()
  }
}
impl IndicesUpdateAliasesResponseContent {
  pub fn builder() -> builder::IndicesUpdateAliasesResponseContent {
    builder::IndicesUpdateAliasesResponseContent::default()
  }
}
///The query definition specified with the Query DSL
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesValidateQueryBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesValidateQueryBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesValidateQueryBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesValidateQueryBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesValidateQueryBodyParams> for IndicesValidateQueryBodyParams {
  fn from(value: &IndicesValidateQueryBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesValidateQueryBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

pub mod builder {

  #[derive(Clone, Debug)]
  pub struct IndicesCreateBodyParams {
    aliases: Result<Option<super::UserDefinedValueMap>, String>,
    mapping: Result<Option<super::UserDefinedValueMap>, String>,
    settings: Result<Option<super::UserDefinedValueMap>, String>,
  }

  impl Default for IndicesCreateBodyParams {
    fn default() -> Self {
      Self {
        aliases: Ok(Default::default()),
        mapping: Ok(Default::default()),
        settings: Ok(Default::default()),
      }
    }
  }

  impl IndicesCreateBodyParams {
    pub fn aliases<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.aliases = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for aliases: {}", e));
      self
    }

    pub fn mapping<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.mapping = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for mapping: {}", e));
      self
    }

    pub fn settings<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.settings = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for settings: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesCreateBodyParams> for super::IndicesCreateBodyParams {
    type Error = String;

    fn try_from(value: IndicesCreateBodyParams) -> Result<Self, String> {
      Ok(Self {
        aliases: value.aliases?,
        mapping: value.mapping?,
        settings: value.settings?,
      })
    }
  }

  impl From<super::IndicesCreateBodyParams> for IndicesCreateBodyParams {
    fn from(value: super::IndicesCreateBodyParams) -> Self {
      Self {
        aliases: Ok(value.aliases),
        mapping: Ok(value.mapping),
        settings: Ok(value.settings),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesCreateDataStreamResponseContent {
    acknowledged: Result<Option<bool>, String>,
  }

  impl Default for IndicesCreateDataStreamResponseContent {
    fn default() -> Self {
      Self {
        acknowledged: Ok(Default::default()),
      }
    }
  }

  impl IndicesCreateDataStreamResponseContent {
    pub fn acknowledged<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.acknowledged = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesCreateDataStreamResponseContent> for super::IndicesCreateDataStreamResponseContent {
    type Error = String;

    fn try_from(value: IndicesCreateDataStreamResponseContent) -> Result<Self, String> {
      Ok(Self {
        acknowledged: value.acknowledged?,
      })
    }
  }

  impl From<super::IndicesCreateDataStreamResponseContent> for IndicesCreateDataStreamResponseContent {
    fn from(value: super::IndicesCreateDataStreamResponseContent) -> Self {
      Self {
        acknowledged: Ok(value.acknowledged),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesCreateResponseContent {
    acknowledged: Result<bool, String>,
    index: Result<String, String>,
    shards_acknowledged: Result<bool, String>,
  }

  impl Default for IndicesCreateResponseContent {
    fn default() -> Self {
      Self {
        acknowledged: Err("no value supplied for acknowledged".to_string()),
        index: Err("no value supplied for index".to_string()),
        shards_acknowledged: Err("no value supplied for shards_acknowledged".to_string()),
      }
    }
  }

  impl IndicesCreateResponseContent {
    pub fn acknowledged<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<bool>,
      T::Error: std::fmt::Display, {
      self.acknowledged = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
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

    pub fn shards_acknowledged<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<bool>,
      T::Error: std::fmt::Display, {
      self.shards_acknowledged = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for shards_acknowledged: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesCreateResponseContent> for super::IndicesCreateResponseContent {
    type Error = String;

    fn try_from(value: IndicesCreateResponseContent) -> Result<Self, String> {
      Ok(Self {
        acknowledged: value.acknowledged?,
        index: value.index?,
        shards_acknowledged: value.shards_acknowledged?,
      })
    }
  }

  impl From<super::IndicesCreateResponseContent> for IndicesCreateResponseContent {
    fn from(value: super::IndicesCreateResponseContent) -> Self {
      Self {
        acknowledged: Ok(value.acknowledged),
        index: Ok(value.index),
        shards_acknowledged: Ok(value.shards_acknowledged),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesDeleteDataStreamResponseContent {
    acknowledged: Result<Option<bool>, String>,
  }

  impl Default for IndicesDeleteDataStreamResponseContent {
    fn default() -> Self {
      Self {
        acknowledged: Ok(Default::default()),
      }
    }
  }

  impl IndicesDeleteDataStreamResponseContent {
    pub fn acknowledged<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.acknowledged = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesDeleteDataStreamResponseContent> for super::IndicesDeleteDataStreamResponseContent {
    type Error = String;

    fn try_from(value: IndicesDeleteDataStreamResponseContent) -> Result<Self, String> {
      Ok(Self {
        acknowledged: value.acknowledged?,
      })
    }
  }

  impl From<super::IndicesDeleteDataStreamResponseContent> for IndicesDeleteDataStreamResponseContent {
    fn from(value: super::IndicesDeleteDataStreamResponseContent) -> Self {
      Self {
        acknowledged: Ok(value.acknowledged),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesDeleteResponseContent {
    acknowledged: Result<Option<bool>, String>,
  }

  impl Default for IndicesDeleteResponseContent {
    fn default() -> Self {
      Self {
        acknowledged: Ok(Default::default()),
      }
    }
  }

  impl IndicesDeleteResponseContent {
    pub fn acknowledged<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.acknowledged = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesDeleteResponseContent> for super::IndicesDeleteResponseContent {
    type Error = String;

    fn try_from(value: IndicesDeleteResponseContent) -> Result<Self, String> {
      Ok(Self {
        acknowledged: value.acknowledged?,
      })
    }
  }

  impl From<super::IndicesDeleteResponseContent> for IndicesDeleteResponseContent {
    fn from(value: super::IndicesDeleteResponseContent) -> Self {
      Self {
        acknowledged: Ok(value.acknowledged),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesGetDataStreamResponseContent {
    data_streams: Result<Vec<super::DataStream>, String>,
  }

  impl Default for IndicesGetDataStreamResponseContent {
    fn default() -> Self {
      Self {
        data_streams: Ok(Default::default()),
      }
    }
  }

  impl IndicesGetDataStreamResponseContent {
    pub fn data_streams<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::DataStream>>,
      T::Error: std::fmt::Display, {
      self.data_streams = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for data_streams: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesGetDataStreamResponseContent> for super::IndicesGetDataStreamResponseContent {
    type Error = String;

    fn try_from(value: IndicesGetDataStreamResponseContent) -> Result<Self, String> {
      Ok(Self {
        data_streams: value.data_streams?,
      })
    }
  }

  impl From<super::IndicesGetDataStreamResponseContent> for IndicesGetDataStreamResponseContent {
    fn from(value: super::IndicesGetDataStreamResponseContent) -> Self {
      Self {
        data_streams: Ok(value.data_streams),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesGetDataStreamWithNameResponseContent {
    data_streams: Result<Vec<super::DataStream>, String>,
  }

  impl Default for IndicesGetDataStreamWithNameResponseContent {
    fn default() -> Self {
      Self {
        data_streams: Ok(Default::default()),
      }
    }
  }

  impl IndicesGetDataStreamWithNameResponseContent {
    pub fn data_streams<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::DataStream>>,
      T::Error: std::fmt::Display, {
      self.data_streams = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for data_streams: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesGetDataStreamWithNameResponseContent>
    for super::IndicesGetDataStreamWithNameResponseContent
  {
    type Error = String;

    fn try_from(value: IndicesGetDataStreamWithNameResponseContent) -> Result<Self, String> {
      Ok(Self {
        data_streams: value.data_streams?,
      })
    }
  }

  impl From<super::IndicesGetDataStreamWithNameResponseContent> for IndicesGetDataStreamWithNameResponseContent {
    fn from(value: super::IndicesGetDataStreamWithNameResponseContent) -> Self {
      Self {
        data_streams: Ok(value.data_streams),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesPutMappingPostResponseContent {
    acknowledged: Result<Option<bool>, String>,
  }

  impl Default for IndicesPutMappingPostResponseContent {
    fn default() -> Self {
      Self {
        acknowledged: Ok(Default::default()),
      }
    }
  }

  impl IndicesPutMappingPostResponseContent {
    pub fn acknowledged<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.acknowledged = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesPutMappingPostResponseContent> for super::IndicesPutMappingPostResponseContent {
    type Error = String;

    fn try_from(value: IndicesPutMappingPostResponseContent) -> Result<Self, String> {
      Ok(Self {
        acknowledged: value.acknowledged?,
      })
    }
  }

  impl From<super::IndicesPutMappingPostResponseContent> for IndicesPutMappingPostResponseContent {
    fn from(value: super::IndicesPutMappingPostResponseContent) -> Self {
      Self {
        acknowledged: Ok(value.acknowledged),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesPutMappingPutResponseContent {
    acknowledged: Result<Option<bool>, String>,
  }

  impl Default for IndicesPutMappingPutResponseContent {
    fn default() -> Self {
      Self {
        acknowledged: Ok(Default::default()),
      }
    }
  }

  impl IndicesPutMappingPutResponseContent {
    pub fn acknowledged<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.acknowledged = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesPutMappingPutResponseContent> for super::IndicesPutMappingPutResponseContent {
    type Error = String;

    fn try_from(value: IndicesPutMappingPutResponseContent) -> Result<Self, String> {
      Ok(Self {
        acknowledged: value.acknowledged?,
      })
    }
  }

  impl From<super::IndicesPutMappingPutResponseContent> for IndicesPutMappingPutResponseContent {
    fn from(value: super::IndicesPutMappingPutResponseContent) -> Self {
      Self {
        acknowledged: Ok(value.acknowledged),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesUpdateAliasesBodyParams {
    actions: Result<Option<super::ActionObjectStructure>, String>,
  }

  impl Default for IndicesUpdateAliasesBodyParams {
    fn default() -> Self {
      Self {
        actions: Ok(Default::default()),
      }
    }
  }

  impl IndicesUpdateAliasesBodyParams {
    pub fn actions<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::ActionObjectStructure>>,
      T::Error: std::fmt::Display, {
      self.actions = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for actions: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesUpdateAliasesBodyParams> for super::IndicesUpdateAliasesBodyParams {
    type Error = String;

    fn try_from(value: IndicesUpdateAliasesBodyParams) -> Result<Self, String> {
      Ok(Self {
        actions: value.actions?,
      })
    }
  }

  impl From<super::IndicesUpdateAliasesBodyParams> for IndicesUpdateAliasesBodyParams {
    fn from(value: super::IndicesUpdateAliasesBodyParams) -> Self {
      Self {
        actions: Ok(value.actions),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct IndicesUpdateAliasesResponseContent {
    acknowledged: Result<bool, String>,
  }

  impl Default for IndicesUpdateAliasesResponseContent {
    fn default() -> Self {
      Self {
        acknowledged: Err("no value supplied for acknowledged".to_string()),
      }
    }
  }

  impl IndicesUpdateAliasesResponseContent {
    pub fn acknowledged<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<bool>,
      T::Error: std::fmt::Display, {
      self.acknowledged = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndicesUpdateAliasesResponseContent> for super::IndicesUpdateAliasesResponseContent {
    type Error = String;

    fn try_from(value: IndicesUpdateAliasesResponseContent) -> Result<Self, String> {
      Ok(Self {
        acknowledged: value.acknowledged?,
      })
    }
  }

  impl From<super::IndicesUpdateAliasesResponseContent> for IndicesUpdateAliasesResponseContent {
    fn from(value: super::IndicesUpdateAliasesResponseContent) -> Self {
      Self {
        acknowledged: Ok(value.acknowledged),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ActionObjectStructure {
    add: Result<Option<super::UserDefinedStructure>, String>,
    remove: Result<Option<super::UserDefinedStructure>, String>,
    remove_index: Result<Option<super::UserDefinedStructure>, String>,
  }

  impl Default for ActionObjectStructure {
    fn default() -> Self {
      Self {
        add: Ok(Default::default()),
        remove: Ok(Default::default()),
        remove_index: Ok(Default::default()),
      }
    }
  }

  impl ActionObjectStructure {
    pub fn add<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedStructure>>,
      T::Error: std::fmt::Display, {
      self.add = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for add: {}", e));
      self
    }

    pub fn remove<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedStructure>>,
      T::Error: std::fmt::Display, {
      self.remove = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for remove: {}", e));
      self
    }

    pub fn remove_index<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedStructure>>,
      T::Error: std::fmt::Display, {
      self.remove_index = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for remove_index: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ActionObjectStructure> for super::ActionObjectStructure {
    type Error = String;

    fn try_from(value: ActionObjectStructure) -> Result<Self, String> {
      Ok(Self {
        add: value.add?,
        remove: value.remove?,
        remove_index: value.remove_index?,
      })
    }
  }

  impl From<super::ActionObjectStructure> for ActionObjectStructure {
    fn from(value: super::ActionObjectStructure) -> Self {
      Self {
        add: Ok(value.add),
        remove: Ok(value.remove),
        remove_index: Ok(value.remove_index),
      }
    }
  }
}
