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

///The template definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndicesPutComponentTemplateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndicesPutComponentTemplateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<IndicesPutComponentTemplateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndicesPutComponentTemplateBodyParams) -> Self {
    value.0
  }
}
impl From<&IndicesPutComponentTemplateBodyParams> for IndicesPutComponentTemplateBodyParams {
  fn from(value: &IndicesPutComponentTemplateBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for IndicesPutComponentTemplateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
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
