
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use super::{builder, DeletedPit};

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
pub struct DeleteActionGroupResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteActionGroupResponseContent> for DeleteActionGroupResponseContent {
  fn from(value: &DeleteActionGroupResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteActionGroupResponseContent {
  pub fn builder() -> builder::DeleteActionGroupResponseContent {
    builder::DeleteActionGroupResponseContent::default()
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

/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteByQueryIndex(String);
impl std::ops::Deref for DeleteByQueryIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteByQueryIndex> for String {
  fn from(value: DeleteByQueryIndex) -> Self {
    value.0
  }
}

impl From<&DeleteByQueryIndex> for DeleteByQueryIndex {
  fn from(value: &DeleteByQueryIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteByQueryIndex {
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

impl std::convert::TryFrom<&str> for DeleteByQueryIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteByQueryIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteByQueryIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteByQueryIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteByQueryRethrottleTaskId(String);
impl std::ops::Deref for DeleteByQueryRethrottleTaskId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteByQueryRethrottleTaskId> for String {
  fn from(value: DeleteByQueryRethrottleTaskId) -> Self {
    value.0
  }
}

impl From<&DeleteByQueryRethrottleTaskId> for DeleteByQueryRethrottleTaskId {
  fn from(value: &DeleteByQueryRethrottleTaskId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteByQueryRethrottleTaskId {
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

impl std::convert::TryFrom<&str> for DeleteByQueryRethrottleTaskId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteByQueryRethrottleTaskId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteByQueryRethrottleTaskId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteByQueryRethrottleTaskId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteByQuerySearchTimeout(String);
impl std::ops::Deref for DeleteByQuerySearchTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteByQuerySearchTimeout> for String {
  fn from(value: DeleteByQuerySearchTimeout) -> Self {
    value.0
  }
}

impl From<&DeleteByQuerySearchTimeout> for DeleteByQuerySearchTimeout {
  fn from(value: &DeleteByQuerySearchTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteByQuerySearchTimeout {
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

impl std::convert::TryFrom<&str> for DeleteByQuerySearchTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteByQuerySearchTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteByQuerySearchTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteByQuerySearchTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

/// unavailable.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteByQueryTimeout(String);
impl std::ops::Deref for DeleteByQueryTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteByQueryTimeout> for String {
  fn from(value: DeleteByQueryTimeout) -> Self {
    value.0
  }
}

impl From<&DeleteByQueryTimeout> for DeleteByQueryTimeout {
  fn from(value: &DeleteByQueryTimeout) -> Self {
    value.clone()
  }
}

impl Default for DeleteByQueryTimeout {
  fn default() -> Self {
    DeleteByQueryTimeout("1m".to_string())
  }
}

impl std::str::FromStr for DeleteByQueryTimeout {
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

impl std::convert::TryFrom<&str> for DeleteByQueryTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteByQueryTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteByQueryTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteByQueryTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteDistinguishedNamesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteDistinguishedNamesResponseContent> for DeleteDistinguishedNamesResponseContent {
  fn from(value: &DeleteDistinguishedNamesResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteDistinguishedNamesResponseContent {
  pub fn builder() -> builder::DeleteDistinguishedNamesResponseContent {
    builder::DeleteDistinguishedNamesResponseContent::default()
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteId(String);
impl std::ops::Deref for DeleteId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteId> for String {
  fn from(value: DeleteId) -> Self {
    value.0
  }
}

impl From<&DeleteId> for DeleteId {
  fn from(value: &DeleteId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteId {
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

impl std::convert::TryFrom<&str> for DeleteId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteIndex(String);
impl std::ops::Deref for DeleteIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteIndex> for String {
  fn from(value: DeleteIndex) -> Self {
    value.0
  }
}

impl From<&DeleteIndex> for DeleteIndex {
  fn from(value: &DeleteIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteIndex {
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

impl std::convert::TryFrom<&str> for DeleteIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteIndex {
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
pub struct DeleteRoleMappingResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteRoleMappingResponseContent> for DeleteRoleMappingResponseContent {
  fn from(value: &DeleteRoleMappingResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteRoleMappingResponseContent {
  pub fn builder() -> builder::DeleteRoleMappingResponseContent {
    builder::DeleteRoleMappingResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteRoleResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteRoleResponseContent> for DeleteRoleResponseContent {
  fn from(value: &DeleteRoleResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteRoleResponseContent {
  pub fn builder() -> builder::DeleteRoleResponseContent {
    builder::DeleteRoleResponseContent::default()
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteScriptClusterManagerTimeout(String);
impl std::ops::Deref for DeleteScriptClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteScriptClusterManagerTimeout> for String {
  fn from(value: DeleteScriptClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&DeleteScriptClusterManagerTimeout> for DeleteScriptClusterManagerTimeout {
  fn from(value: &DeleteScriptClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteScriptClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for DeleteScriptClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteScriptClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteScriptClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteScriptClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteScriptId(String);
impl std::ops::Deref for DeleteScriptId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteScriptId> for String {
  fn from(value: DeleteScriptId) -> Self {
    value.0
  }
}

impl From<&DeleteScriptId> for DeleteScriptId {
  fn from(value: &DeleteScriptId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteScriptId {
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

impl std::convert::TryFrom<&str> for DeleteScriptId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteScriptId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteScriptId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteScriptId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteScriptMasterTimeout(String);
impl std::ops::Deref for DeleteScriptMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteScriptMasterTimeout> for String {
  fn from(value: DeleteScriptMasterTimeout) -> Self {
    value.0
  }
}

impl From<&DeleteScriptMasterTimeout> for DeleteScriptMasterTimeout {
  fn from(value: &DeleteScriptMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteScriptMasterTimeout {
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

impl std::convert::TryFrom<&str> for DeleteScriptMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteScriptMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteScriptMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteScriptMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteScriptTimeout(String);
impl std::ops::Deref for DeleteScriptTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteScriptTimeout> for String {
  fn from(value: DeleteScriptTimeout) -> Self {
    value.0
  }
}

impl From<&DeleteScriptTimeout> for DeleteScriptTimeout {
  fn from(value: &DeleteScriptTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteScriptTimeout {
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

impl std::convert::TryFrom<&str> for DeleteScriptTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteScriptTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteScriptTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteScriptTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteTenantResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteTenantResponseContent> for DeleteTenantResponseContent {
  fn from(value: &DeleteTenantResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteTenantResponseContent {
  pub fn builder() -> builder::DeleteTenantResponseContent {
    builder::DeleteTenantResponseContent::default()
  }
}
