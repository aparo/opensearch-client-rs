#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub mod bulk;
pub use bulk::{BulkAction, BulkError, BulkItemResponse, BulkResponse, IndexResponse, UpdateAction};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountDetails {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub backend_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub custom_attribute_names: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_internal_user: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_reserved: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub tenants: Option<UserTenants>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub user_name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub user_requested_tenant: Option<String>,
}

impl From<&AccountDetails> for AccountDetails {
  fn from(value: &AccountDetails) -> Self {
    value.clone()
  }
}

impl AccountDetails {
  pub fn builder() -> builder::AccountDetails {
    builder::AccountDetails::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActionGroup {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub allowed_actions: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
  pub static_: Option<bool>,
  #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
  pub type_: Option<String>,
}

impl From<&ActionGroup> for ActionGroup {
  fn from(value: &ActionGroup) -> Self {
    value.clone()
  }
}

impl ActionGroup {
  pub fn builder() -> builder::ActionGroup {
    builder::ActionGroup::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActionGroupsMap(pub std::collections::HashMap<String, ActionGroup>);
impl std::ops::Deref for ActionGroupsMap {
  type Target = std::collections::HashMap<String, ActionGroup>;

  fn deref(&self) -> &std::collections::HashMap<String, ActionGroup> {
    &self.0
  }
}

impl From<ActionGroupsMap> for std::collections::HashMap<String, ActionGroup> {
  fn from(value: ActionGroupsMap) -> Self {
    value.0
  }
}

impl From<&ActionGroupsMap> for ActionGroupsMap {
  fn from(value: &ActionGroupsMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, ActionGroup>> for ActionGroupsMap {
  fn from(value: std::collections::HashMap<String, ActionGroup>) -> Self {
    Self(value)
  }
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditConfig {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub audit: Option<AuditLogsConfig>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub compliance: Option<ComplianceConfig>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
}

impl From<&AuditConfig> for AuditConfig {
  fn from(value: &AuditConfig) -> Self {
    value.clone()
  }
}

impl AuditConfig {
  pub fn builder() -> builder::AuditConfig {
    builder::AuditConfig::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditConfigWithReadOnly {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub config: Option<AuditConfig>,
  #[serde(rename = "_readonly", default, skip_serializing_if = "Vec::is_empty")]
  pub readonly: Vec<String>,
}

impl From<&AuditConfigWithReadOnly> for AuditConfigWithReadOnly {
  fn from(value: &AuditConfigWithReadOnly) -> Self {
    value.clone()
  }
}

impl AuditConfigWithReadOnly {
  pub fn builder() -> builder::AuditConfigWithReadOnly {
    builder::AuditConfigWithReadOnly::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogsConfig {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub disabled_rest_categories: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub disabled_transport_categories: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub enable_rest: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub enable_transport: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub exclude_sensitive_headers: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub ignore_requests: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub ignore_users: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub log_request_body: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub resolve_bulk_requests: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub resolve_indices: Option<bool>,
}

impl From<&AuditLogsConfig> for AuditLogsConfig {
  fn from(value: &AuditLogsConfig) -> Self {
    value.clone()
  }
}

impl AuditLogsConfig {
  pub fn builder() -> builder::AuditLogsConfig {
    builder::AuditLogsConfig::default()
  }
}

























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

///Comma-separated list of alias names.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CatAliasesWithNameName(String);
impl std::ops::Deref for CatAliasesWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CatAliasesWithNameName> for String {
  fn from(value: CatAliasesWithNameName) -> Self {
    value.0
  }
}

impl From<&CatAliasesWithNameName> for CatAliasesWithNameName {
  fn from(value: &CatAliasesWithNameName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CatAliasesWithNameName {
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

impl std::convert::TryFrom<&str> for CatAliasesWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CatAliasesWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CatAliasesWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CatAliasesWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatAllPitSegmentsResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub content: Option<CatPitSegment>,
}

impl From<&CatAllPitSegmentsResponseContent> for CatAllPitSegmentsResponseContent {
  fn from(value: &CatAllPitSegmentsResponseContent) -> Self {
    value.clone()
  }
}

impl CatAllPitSegmentsResponseContent {
  pub fn builder() -> builder::CatAllPitSegmentsResponseContent {
    builder::CatAllPitSegmentsResponseContent::default()
  }
}

///Comma-separated list of node IDs or names to limit the returned
/// information.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CatAllocationWithNodeIdNodeId(String);
impl std::ops::Deref for CatAllocationWithNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CatAllocationWithNodeIdNodeId> for String {
  fn from(value: CatAllocationWithNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&CatAllocationWithNodeIdNodeId> for CatAllocationWithNodeIdNodeId {
  fn from(value: &CatAllocationWithNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CatAllocationWithNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for CatAllocationWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CatAllocationWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CatAllocationWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CatAllocationWithNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices to limit the returned information.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CatCountWithIndexIndex(String);
impl std::ops::Deref for CatCountWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CatCountWithIndexIndex> for String {
  fn from(value: CatCountWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&CatCountWithIndexIndex> for CatCountWithIndexIndex {
  fn from(value: &CatCountWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CatCountWithIndexIndex {
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

impl std::convert::TryFrom<&str> for CatCountWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CatCountWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CatCountWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CatCountWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices to limit the returned information.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CatIndicesWithIndexIndex(String);
impl std::ops::Deref for CatIndicesWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CatIndicesWithIndexIndex> for String {
  fn from(value: CatIndicesWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&CatIndicesWithIndexIndex> for CatIndicesWithIndexIndex {
  fn from(value: &CatIndicesWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CatIndicesWithIndexIndex {
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

impl std::convert::TryFrom<&str> for CatIndicesWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CatIndicesWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CatIndicesWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CatIndicesWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatPitSegment {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub committed: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub compound: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub docs_count: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub docs_deleted: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub generation: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ip: Option<String>,
  ///Set to true to return stats only for primary shards.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub prirep: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub searchable: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub segment: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub shard: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub size: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub size_memory: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
}

impl From<&CatPitSegment> for CatPitSegment {
  fn from(value: &CatPitSegment) -> Self {
    value.clone()
  }
}

impl CatPitSegment {
  pub fn builder() -> builder::CatPitSegment {
    builder::CatPitSegment::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatPitSegmentsBodyParams {
  pub pit_id: Vec<String>,
}

impl From<&CatPitSegmentsBodyParams> for CatPitSegmentsBodyParams {
  fn from(value: &CatPitSegmentsBodyParams) -> Self {
    value.clone()
  }
}

impl CatPitSegmentsBodyParams {
  pub fn builder() -> builder::CatPitSegmentsBodyParams {
    builder::CatPitSegmentsBodyParams::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatPitSegmentsResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub content: Option<CatPitSegment>,
}

impl From<&CatPitSegmentsResponseContent> for CatPitSegmentsResponseContent {
  fn from(value: &CatPitSegmentsResponseContent) -> Self {
    value.clone()
  }
}

impl CatPitSegmentsResponseContent {
  pub fn builder() -> builder::CatPitSegmentsResponseContent {
    builder::CatPitSegmentsResponseContent::default()
  }
}

///Comma-separated list of indices to limit the returned information.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CatSegmentsWithIndexIndex(String);
impl std::ops::Deref for CatSegmentsWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CatSegmentsWithIndexIndex> for String {
  fn from(value: CatSegmentsWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&CatSegmentsWithIndexIndex> for CatSegmentsWithIndexIndex {
  fn from(value: &CatSegmentsWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CatSegmentsWithIndexIndex {
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

impl std::convert::TryFrom<&str> for CatSegmentsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CatSegmentsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CatSegmentsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CatSegmentsWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices to limit the returned information.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CatShardsWithIndexIndex(String);
impl std::ops::Deref for CatShardsWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CatShardsWithIndexIndex> for String {
  fn from(value: CatShardsWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&CatShardsWithIndexIndex> for CatShardsWithIndexIndex {
  fn from(value: &CatShardsWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CatShardsWithIndexIndex {
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

impl std::convert::TryFrom<&str> for CatShardsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CatShardsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CatShardsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CatShardsWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of repository names.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CatSnapshotsWithRepositoryRepository(String);
impl std::ops::Deref for CatSnapshotsWithRepositoryRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CatSnapshotsWithRepositoryRepository> for String {
  fn from(value: CatSnapshotsWithRepositoryRepository) -> Self {
    value.0
  }
}

impl From<&CatSnapshotsWithRepositoryRepository> for CatSnapshotsWithRepositoryRepository {
  fn from(value: &CatSnapshotsWithRepositoryRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CatSnapshotsWithRepositoryRepository {
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

impl std::convert::TryFrom<&str> for CatSnapshotsWithRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CatSnapshotsWithRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CatSnapshotsWithRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CatSnapshotsWithRepositoryRepository {
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
pub struct CatTemplatesWithNameName(String);
impl std::ops::Deref for CatTemplatesWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CatTemplatesWithNameName> for String {
  fn from(value: CatTemplatesWithNameName) -> Self {
    value.0
  }
}

impl From<&CatTemplatesWithNameName> for CatTemplatesWithNameName {
  fn from(value: &CatTemplatesWithNameName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CatTemplatesWithNameName {
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

impl std::convert::TryFrom<&str> for CatTemplatesWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CatTemplatesWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CatTemplatesWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CatTemplatesWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of regular-expressions to filter the thread pools
/// in the output.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns(String);
impl std::ops::Deref for CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns> for String {
  fn from(value: CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns) -> Self {
    value.0
  }
}

impl From<&CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns>
  for CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns
{
  fn from(value: &CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns {
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

impl std::convert::TryFrom<&str> for CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CertificatesDetail {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub issuer_dn: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub not_after: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub not_before: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub san: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub subject_dn: Option<String>,
}

impl From<&CertificatesDetail> for CertificatesDetail {
  fn from(value: &CertificatesDetail) -> Self {
    value.clone()
  }
}

impl CertificatesDetail {
  pub fn builder() -> builder::CertificatesDetail {
    builder::CertificatesDetail::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangePasswordRequestContent {
  ///The current password
  pub current_password: String,
  ///The new password to set
  pub password: String,
}

impl From<&ChangePasswordRequestContent> for ChangePasswordRequestContent {
  fn from(value: &ChangePasswordRequestContent) -> Self {
    value.clone()
  }
}

impl ChangePasswordRequestContent {
  pub fn builder() -> builder::ChangePasswordRequestContent {
    builder::ChangePasswordRequestContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangePasswordResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&ChangePasswordResponseContent> for ChangePasswordResponseContent {
  fn from(value: &ChangePasswordResponseContent) -> Self {
    value.clone()
  }
}

impl ChangePasswordResponseContent {
  pub fn builder() -> builder::ChangePasswordResponseContent {
    builder::ChangePasswordResponseContent::default()
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

///Comma-separated list of scroll IDs to clear.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClearScrollWithScrollIdScrollId(String);
impl std::ops::Deref for ClearScrollWithScrollIdScrollId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClearScrollWithScrollIdScrollId> for String {
  fn from(value: ClearScrollWithScrollIdScrollId) -> Self {
    value.0
  }
}

impl From<&ClearScrollWithScrollIdScrollId> for ClearScrollWithScrollIdScrollId {
  fn from(value: &ClearScrollWithScrollIdScrollId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClearScrollWithScrollIdScrollId {
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

impl std::convert::TryFrom<&str> for ClearScrollWithScrollIdScrollId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClearScrollWithScrollIdScrollId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClearScrollWithScrollIdScrollId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClearScrollWithScrollIdScrollId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The index, shard, and primary flag to explain. Empty means 'explain the
/// first unassigned shard'
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterAllocationExplainBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for ClusterAllocationExplainBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<ClusterAllocationExplainBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: ClusterAllocationExplainBodyParams) -> Self {
    value.0
  }
}

impl From<&ClusterAllocationExplainBodyParams> for ClusterAllocationExplainBodyParams {
  fn from(value: &ClusterAllocationExplainBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for ClusterAllocationExplainBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The name of the template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterDeleteComponentTemplateName(String);
impl std::ops::Deref for ClusterDeleteComponentTemplateName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterDeleteComponentTemplateName> for String {
  fn from(value: ClusterDeleteComponentTemplateName) -> Self {
    value.0
  }
}

impl From<&ClusterDeleteComponentTemplateName> for ClusterDeleteComponentTemplateName {
  fn from(value: &ClusterDeleteComponentTemplateName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterDeleteComponentTemplateName {
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

impl std::convert::TryFrom<&str> for ClusterDeleteComponentTemplateName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterDeleteComponentTemplateName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterDeleteComponentTemplateName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterDeleteComponentTemplateName {
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
pub struct ClusterExistsComponentTemplateName(String);
impl std::ops::Deref for ClusterExistsComponentTemplateName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterExistsComponentTemplateName> for String {
  fn from(value: ClusterExistsComponentTemplateName) -> Self {
    value.0
  }
}

impl From<&ClusterExistsComponentTemplateName> for ClusterExistsComponentTemplateName {
  fn from(value: &ClusterExistsComponentTemplateName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterExistsComponentTemplateName {
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

impl std::convert::TryFrom<&str> for ClusterExistsComponentTemplateName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterExistsComponentTemplateName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterExistsComponentTemplateName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterExistsComponentTemplateName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The Comma-separated names of the component templates.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetComponentTemplateWithNameName(String);
impl std::ops::Deref for ClusterGetComponentTemplateWithNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetComponentTemplateWithNameName> for String {
  fn from(value: ClusterGetComponentTemplateWithNameName) -> Self {
    value.0
  }
}

impl From<&ClusterGetComponentTemplateWithNameName> for ClusterGetComponentTemplateWithNameName {
  fn from(value: &ClusterGetComponentTemplateWithNameName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetComponentTemplateWithNameName {
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

impl std::convert::TryFrom<&str> for ClusterGetComponentTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetComponentTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetComponentTemplateWithNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetComponentTemplateWithNameName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Awareness attribute name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetDecommissionAwarenessAwarenessAttributeName(String);
impl std::ops::Deref for ClusterGetDecommissionAwarenessAwarenessAttributeName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetDecommissionAwarenessAwarenessAttributeName> for String {
  fn from(value: ClusterGetDecommissionAwarenessAwarenessAttributeName) -> Self {
    value.0
  }
}

impl From<&ClusterGetDecommissionAwarenessAwarenessAttributeName>
  for ClusterGetDecommissionAwarenessAwarenessAttributeName
{
  fn from(value: &ClusterGetDecommissionAwarenessAwarenessAttributeName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetDecommissionAwarenessAwarenessAttributeName {
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

impl std::convert::TryFrom<&str> for ClusterGetDecommissionAwarenessAwarenessAttributeName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetDecommissionAwarenessAwarenessAttributeName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetDecommissionAwarenessAwarenessAttributeName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetDecommissionAwarenessAwarenessAttributeName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterGetSettingsResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub defaults: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub persistent: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub transient: Option<UserDefinedValueMap>,
}

impl From<&ClusterGetSettingsResponseContent> for ClusterGetSettingsResponseContent {
  fn from(value: &ClusterGetSettingsResponseContent) -> Self {
    value.clone()
  }
}

impl ClusterGetSettingsResponseContent {
  pub fn builder() -> builder::ClusterGetSettingsResponseContent {
    builder::ClusterGetSettingsResponseContent::default()
  }
}









///Awareness attribute name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetWeightedRoutingAttribute(String);
impl std::ops::Deref for ClusterGetWeightedRoutingAttribute {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetWeightedRoutingAttribute> for String {
  fn from(value: ClusterGetWeightedRoutingAttribute) -> Self {
    value.0
  }
}

impl From<&ClusterGetWeightedRoutingAttribute> for ClusterGetWeightedRoutingAttribute {
  fn from(value: &ClusterGetWeightedRoutingAttribute) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetWeightedRoutingAttribute {
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

impl std::convert::TryFrom<&str> for ClusterGetWeightedRoutingAttribute {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetWeightedRoutingAttribute {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetWeightedRoutingAttribute {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetWeightedRoutingAttribute {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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









///Limit the information returned to specific indicies.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterHealthWithIndexIndex(String);
impl std::ops::Deref for ClusterHealthWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterHealthWithIndexIndex> for String {
  fn from(value: ClusterHealthWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&ClusterHealthWithIndexIndex> for ClusterHealthWithIndexIndex {
  fn from(value: &ClusterHealthWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterHealthWithIndexIndex {
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

impl std::convert::TryFrom<&str> for ClusterHealthWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterHealthWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterHealthWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterHealthWithIndexIndex {
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
pub struct ClusterPutComponentTemplateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for ClusterPutComponentTemplateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<ClusterPutComponentTemplateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: ClusterPutComponentTemplateBodyParams) -> Self {
    value.0
  }
}

impl From<&ClusterPutComponentTemplateBodyParams> for ClusterPutComponentTemplateBodyParams {
  fn from(value: &ClusterPutComponentTemplateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for ClusterPutComponentTemplateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///The name of the template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutComponentTemplatePostName(String);
impl std::ops::Deref for ClusterPutComponentTemplatePostName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutComponentTemplatePostName> for String {
  fn from(value: ClusterPutComponentTemplatePostName) -> Self {
    value.0
  }
}

impl From<&ClusterPutComponentTemplatePostName> for ClusterPutComponentTemplatePostName {
  fn from(value: &ClusterPutComponentTemplatePostName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutComponentTemplatePostName {
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

impl std::convert::TryFrom<&str> for ClusterPutComponentTemplatePostName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutComponentTemplatePostName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutComponentTemplatePostName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutComponentTemplatePostName {
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
pub struct ClusterPutComponentTemplatePutName(String);
impl std::ops::Deref for ClusterPutComponentTemplatePutName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutComponentTemplatePutName> for String {
  fn from(value: ClusterPutComponentTemplatePutName) -> Self {
    value.0
  }
}

impl From<&ClusterPutComponentTemplatePutName> for ClusterPutComponentTemplatePutName {
  fn from(value: &ClusterPutComponentTemplatePutName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutComponentTemplatePutName {
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

impl std::convert::TryFrom<&str> for ClusterPutComponentTemplatePutName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutComponentTemplatePutName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutComponentTemplatePutName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutComponentTemplatePutName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Awareness attribute name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutDecommissionAwarenessAwarenessAttributeName(String);
impl std::ops::Deref for ClusterPutDecommissionAwarenessAwarenessAttributeName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutDecommissionAwarenessAwarenessAttributeName> for String {
  fn from(value: ClusterPutDecommissionAwarenessAwarenessAttributeName) -> Self {
    value.0
  }
}

impl From<&ClusterPutDecommissionAwarenessAwarenessAttributeName>
  for ClusterPutDecommissionAwarenessAwarenessAttributeName
{
  fn from(value: &ClusterPutDecommissionAwarenessAwarenessAttributeName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutDecommissionAwarenessAwarenessAttributeName {
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

impl std::convert::TryFrom<&str> for ClusterPutDecommissionAwarenessAwarenessAttributeName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutDecommissionAwarenessAwarenessAttributeName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutDecommissionAwarenessAwarenessAttributeName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutDecommissionAwarenessAwarenessAttributeName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Awareness attribute value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutDecommissionAwarenessAwarenessAttributeValue(String);
impl std::ops::Deref for ClusterPutDecommissionAwarenessAwarenessAttributeValue {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutDecommissionAwarenessAwarenessAttributeValue> for String {
  fn from(value: ClusterPutDecommissionAwarenessAwarenessAttributeValue) -> Self {
    value.0
  }
}

impl From<&ClusterPutDecommissionAwarenessAwarenessAttributeValue>
  for ClusterPutDecommissionAwarenessAwarenessAttributeValue
{
  fn from(value: &ClusterPutDecommissionAwarenessAwarenessAttributeValue) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutDecommissionAwarenessAwarenessAttributeValue {
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

impl std::convert::TryFrom<&str> for ClusterPutDecommissionAwarenessAwarenessAttributeValue {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutDecommissionAwarenessAwarenessAttributeValue {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutDecommissionAwarenessAwarenessAttributeValue {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutDecommissionAwarenessAwarenessAttributeValue {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The settings to be updated. Can be either `transient` or `persistent`
/// (survives cluster restart).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterPutSettingsBodyParams {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub persistent: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub transient: Option<UserDefinedValueMap>,
}

impl From<&ClusterPutSettingsBodyParams> for ClusterPutSettingsBodyParams {
  fn from(value: &ClusterPutSettingsBodyParams) -> Self {
    value.clone()
  }
}

impl ClusterPutSettingsBodyParams {
  pub fn builder() -> builder::ClusterPutSettingsBodyParams {
    builder::ClusterPutSettingsBodyParams::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterPutSettingsResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub acknowledged: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub persistent: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub transient: Option<UserDefinedValueMap>,
}

impl From<&ClusterPutSettingsResponseContent> for ClusterPutSettingsResponseContent {
  fn from(value: &ClusterPutSettingsResponseContent) -> Self {
    value.clone()
  }
}

impl ClusterPutSettingsResponseContent {
  pub fn builder() -> builder::ClusterPutSettingsResponseContent {
    builder::ClusterPutSettingsResponseContent::default()
  }
}









///Awareness attribute name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutWeightedRoutingAttribute(String);
impl std::ops::Deref for ClusterPutWeightedRoutingAttribute {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutWeightedRoutingAttribute> for String {
  fn from(value: ClusterPutWeightedRoutingAttribute) -> Self {
    value.0
  }
}

impl From<&ClusterPutWeightedRoutingAttribute> for ClusterPutWeightedRoutingAttribute {
  fn from(value: &ClusterPutWeightedRoutingAttribute) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutWeightedRoutingAttribute {
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

impl std::convert::TryFrom<&str> for ClusterPutWeightedRoutingAttribute {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutWeightedRoutingAttribute {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutWeightedRoutingAttribute {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutWeightedRoutingAttribute {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The definition of `commands` to perform (`move`, `cancel`, `allocate`)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterRerouteBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for ClusterRerouteBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<ClusterRerouteBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: ClusterRerouteBodyParams) -> Self {
    value.0
  }
}

impl From<&ClusterRerouteBodyParams> for ClusterRerouteBodyParams {
  fn from(value: &ClusterRerouteBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for ClusterRerouteBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ClusterRerouteMetricMember {
  #[serde(rename = "_all")]
  All,
  #[serde(rename = "blocks")]
  Blocks,
  #[serde(rename = "metadata")]
  Metadata,
  #[serde(rename = "nodes")]
  Nodes,
  #[serde(rename = "routing_table")]
  RoutingTable,
  #[serde(rename = "master_node")]
  MasterNode,
  #[serde(rename = "cluster_manager_node")]
  ClusterManagerNode,
  #[serde(rename = "version")]
  Version,
}

impl From<&ClusterRerouteMetricMember> for ClusterRerouteMetricMember {
  fn from(value: &ClusterRerouteMetricMember) -> Self {
    value.clone()
  }
}

impl ToString for ClusterRerouteMetricMember {
  fn to_string(&self) -> String {
    match *self {
      Self::All => "_all".to_string(),
      Self::Blocks => "blocks".to_string(),
      Self::Metadata => "metadata".to_string(),
      Self::Nodes => "nodes".to_string(),
      Self::RoutingTable => "routing_table".to_string(),
      Self::MasterNode => "master_node".to_string(),
      Self::ClusterManagerNode => "cluster_manager_node".to_string(),
      Self::Version => "version".to_string(),
    }
  }
}

impl std::str::FromStr for ClusterRerouteMetricMember {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "_all" => Ok(Self::All),
      "blocks" => Ok(Self::Blocks),
      "metadata" => Ok(Self::Metadata),
      "nodes" => Ok(Self::Nodes),
      "routing_table" => Ok(Self::RoutingTable),
      "master_node" => Ok(Self::MasterNode),
      "cluster_manager_node" => Ok(Self::ClusterManagerNode),
      "version" => Ok(Self::Version),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for ClusterRerouteMetricMember {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterRerouteMetricMember {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterRerouteMetricMember {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}









///The maximum time to wait for wait_for_metadata_version before timing








///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWithIndexMetricIndex(String);
impl std::ops::Deref for ClusterStateWithIndexMetricIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWithIndexMetricIndex> for String {
  fn from(value: ClusterStateWithIndexMetricIndex) -> Self {
    value.0
  }
}

impl From<&ClusterStateWithIndexMetricIndex> for ClusterStateWithIndexMetricIndex {
  fn from(value: &ClusterStateWithIndexMetricIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWithIndexMetricIndex {
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

impl std::convert::TryFrom<&str> for ClusterStateWithIndexMetricIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWithIndexMetricIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWithIndexMetricIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWithIndexMetricIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Limit the information returned to the specified metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWithIndexMetricMetric(String);
impl std::ops::Deref for ClusterStateWithIndexMetricMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWithIndexMetricMetric> for String {
  fn from(value: ClusterStateWithIndexMetricMetric) -> Self {
    value.0
  }
}

impl From<&ClusterStateWithIndexMetricMetric> for ClusterStateWithIndexMetricMetric {
  fn from(value: &ClusterStateWithIndexMetricMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWithIndexMetricMetric {
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

impl std::convert::TryFrom<&str> for ClusterStateWithIndexMetricMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWithIndexMetricMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWithIndexMetricMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWithIndexMetricMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The maximum time to wait for wait_for_metadata_version before timing








///Limit the information returned to the specified metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWithMetricMetric(String);
impl std::ops::Deref for ClusterStateWithMetricMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWithMetricMetric> for String {
  fn from(value: ClusterStateWithMetricMetric) -> Self {
    value.0
  }
}

impl From<&ClusterStateWithMetricMetric> for ClusterStateWithMetricMetric {
  fn from(value: &ClusterStateWithMetricMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWithMetricMetric {
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

impl std::convert::TryFrom<&str> for ClusterStateWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWithMetricMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The maximum time to wait for wait_for_metadata_version before timing
















///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStatsWithNodeIdNodeId(String);
impl std::ops::Deref for ClusterStatsWithNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStatsWithNodeIdNodeId> for String {
  fn from(value: ClusterStatsWithNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&ClusterStatsWithNodeIdNodeId> for ClusterStatsWithNodeIdNodeId {
  fn from(value: &ClusterStatsWithNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStatsWithNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for ClusterStatsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStatsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStatsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStatsWithNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComplianceConfig {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub external_config: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub internal_config: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub read_ignore_users: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub read_metadata_only: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub read_watched_fields: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub write_ignore_users: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub write_log_diffs: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub write_metadata_only: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub write_watched_indices: Vec<String>,
}

impl From<&ComplianceConfig> for ComplianceConfig {
  fn from(value: &ComplianceConfig) -> Self {
    value.clone()
  }
}

impl ComplianceConfig {
  pub fn builder() -> builder::ComplianceConfig {
    builder::ComplianceConfig::default()
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

///Comma-separated list of indices to restrict the results.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CountGetWithIndexIndex(String);
impl std::ops::Deref for CountGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CountGetWithIndexIndex> for String {
  fn from(value: CountGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&CountGetWithIndexIndex> for CountGetWithIndexIndex {
  fn from(value: &CountGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CountGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for CountGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CountGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CountGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CountGetWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices to restrict the results.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CountPostWithIndexIndex(String);
impl std::ops::Deref for CountPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CountPostWithIndexIndex> for String {
  fn from(value: CountPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&CountPostWithIndexIndex> for CountPostWithIndexIndex {
  fn from(value: &CountPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CountPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for CountPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CountPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CountPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CountPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateActionGroupResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateActionGroupResponseContent> for CreateActionGroupResponseContent {
  fn from(value: &CreateActionGroupResponseContent) -> Self {
    value.clone()
  }
}

impl CreateActionGroupResponseContent {
  pub fn builder() -> builder::CreateActionGroupResponseContent {
    builder::CreateActionGroupResponseContent::default()
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreatePitIndex(String);
impl std::ops::Deref for CreatePitIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CreatePitIndex> for String {
  fn from(value: CreatePitIndex) -> Self {
    value.0
  }
}

impl From<&CreatePitIndex> for CreatePitIndex {
  fn from(value: &CreatePitIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CreatePitIndex {
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

impl std::convert::TryFrom<&str> for CreatePitIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CreatePitIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CreatePitIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CreatePitIndex {
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

///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreatePostId(String);
impl std::ops::Deref for CreatePostId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CreatePostId> for String {
  fn from(value: CreatePostId) -> Self {
    value.0
  }
}

impl From<&CreatePostId> for CreatePostId {
  fn from(value: &CreatePostId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CreatePostId {
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

impl std::convert::TryFrom<&str> for CreatePostId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CreatePostId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CreatePostId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CreatePostId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreatePostIndex(String);
impl std::ops::Deref for CreatePostIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CreatePostIndex> for String {
  fn from(value: CreatePostIndex) -> Self {
    value.0
  }
}

impl From<&CreatePostIndex> for CreatePostIndex {
  fn from(value: &CreatePostIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CreatePostIndex {
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

impl std::convert::TryFrom<&str> for CreatePostIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CreatePostIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CreatePostIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CreatePostIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreatePutId(String);
impl std::ops::Deref for CreatePutId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CreatePutId> for String {
  fn from(value: CreatePutId) -> Self {
    value.0
  }
}

impl From<&CreatePutId> for CreatePutId {
  fn from(value: &CreatePutId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CreatePutId {
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

impl std::convert::TryFrom<&str> for CreatePutId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CreatePutId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CreatePutId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CreatePutId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreatePutIndex(String);
impl std::ops::Deref for CreatePutIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CreatePutIndex> for String {
  fn from(value: CreatePutIndex) -> Self {
    value.0
  }
}

impl From<&CreatePutIndex> for CreatePutIndex {
  fn from(value: &CreatePutIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CreatePutIndex {
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

impl std::convert::TryFrom<&str> for CreatePutIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CreatePutIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CreatePutIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CreatePutIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateRoleMappingResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateRoleMappingResponseContent> for CreateRoleMappingResponseContent {
  fn from(value: &CreateRoleMappingResponseContent) -> Self {
    value.clone()
  }
}

impl CreateRoleMappingResponseContent {
  pub fn builder() -> builder::CreateRoleMappingResponseContent {
    builder::CreateRoleMappingResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateRoleResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateRoleResponseContent> for CreateRoleResponseContent {
  fn from(value: &CreateRoleResponseContent) -> Self {
    value.clone()
  }
}

impl CreateRoleResponseContent {
  pub fn builder() -> builder::CreateRoleResponseContent {
    builder::CreateRoleResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTenantParams {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

impl From<&CreateTenantParams> for CreateTenantParams {
  fn from(value: &CreateTenantParams) -> Self {
    value.clone()
  }
}

impl CreateTenantParams {
  pub fn builder() -> builder::CreateTenantParams {
    builder::CreateTenantParams::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTenantResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateTenantResponseContent> for CreateTenantResponseContent {
  fn from(value: &CreateTenantResponseContent) -> Self {
    value.clone()
  }
}

impl CreateTenantResponseContent {
  pub fn builder() -> builder::CreateTenantResponseContent {
    builder::CreateTenantResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateUserResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateUserResponseContent> for CreateUserResponseContent {
  fn from(value: &CreateUserResponseContent) -> Self {
    value.clone()
  }
}

impl CreateUserResponseContent {
  pub fn builder() -> builder::CreateUserResponseContent {
    builder::CreateUserResponseContent::default()
  }
}

///The UUID of the dangling index.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesDeleteDanglingIndexIndexUuid(String);
impl std::ops::Deref for DanglingIndicesDeleteDanglingIndexIndexUuid {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesDeleteDanglingIndexIndexUuid> for String {
  fn from(value: DanglingIndicesDeleteDanglingIndexIndexUuid) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesDeleteDanglingIndexIndexUuid> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  fn from(value: &DanglingIndicesDeleteDanglingIndexIndexUuid) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesDeleteDanglingIndexIndexUuid {
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

impl std::convert::TryFrom<&str> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///The UUID of the dangling index.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesImportDanglingIndexIndexUuid(String);
impl std::ops::Deref for DanglingIndicesImportDanglingIndexIndexUuid {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesImportDanglingIndexIndexUuid> for String {
  fn from(value: DanglingIndicesImportDanglingIndexIndexUuid) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesImportDanglingIndexIndexUuid> for DanglingIndicesImportDanglingIndexIndexUuid {
  fn from(value: &DanglingIndicesImportDanglingIndexIndexUuid) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesImportDanglingIndexIndexUuid {
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

impl std::convert::TryFrom<&str> for DanglingIndicesImportDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesImportDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesImportDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesImportDanglingIndexIndexUuid {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Comma-separated list of indices; use `_all` or empty string to perform
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

///The task id to rethrottle.
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









///Time each individual bulk request should wait for shards that are









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

///Document ID.
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

///Index name.
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

///Script ID.
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









#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteUserResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteUserResponseContent> for DeleteUserResponseContent {
  fn from(value: &DeleteUserResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteUserResponseContent {
  pub fn builder() -> builder::DeleteUserResponseContent {
    builder::DeleteUserResponseContent::default()
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DistinguishedNames {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub nodes_dn: Vec<String>,
}

impl From<&DistinguishedNames> for DistinguishedNames {
  fn from(value: &DistinguishedNames) -> Self {
    value.clone()
  }
}

impl DistinguishedNames {
  pub fn builder() -> builder::DistinguishedNames {
    builder::DistinguishedNames::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DistinguishedNamesMap(pub std::collections::HashMap<String, DistinguishedNames>);
impl std::ops::Deref for DistinguishedNamesMap {
  type Target = std::collections::HashMap<String, DistinguishedNames>;

  fn deref(&self) -> &std::collections::HashMap<String, DistinguishedNames> {
    &self.0
  }
}

impl From<DistinguishedNamesMap> for std::collections::HashMap<String, DistinguishedNames> {
  fn from(value: DistinguishedNamesMap) -> Self {
    value.0
  }
}

impl From<&DistinguishedNamesMap> for DistinguishedNamesMap {
  fn from(value: &DistinguishedNamesMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, DistinguishedNames>> for DistinguishedNamesMap {
  fn from(value: std::collections::HashMap<String, DistinguishedNames>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DynamicConfig {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub dynamic: Option<DynamicOptions>,
}

impl From<&DynamicConfig> for DynamicConfig {
  fn from(value: &DynamicConfig) -> Self {
    value.clone()
  }
}

impl DynamicConfig {
  pub fn builder() -> builder::DynamicConfig {
    builder::DynamicConfig::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DynamicOptions {
  #[serde(rename = "authFailureListeners", default, skip_serializing_if = "Option::is_none")]
  pub auth_failure_listeners: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub authc: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub authz: Option<serde_json::Value>,
  #[serde(
    rename = "disableIntertransportAuth",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub disable_intertransport_auth: Option<bool>,
  #[serde(rename = "disableRestAuth", default, skip_serializing_if = "Option::is_none")]
  pub disable_rest_auth: Option<bool>,
  #[serde(rename = "doNotFailOnForbidden", default, skip_serializing_if = "Option::is_none")]
  pub do_not_fail_on_forbidden: Option<bool>,
  #[serde(
    rename = "doNotFailOnForbiddenEmpty",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub do_not_fail_on_forbidden_empty: Option<bool>,
  #[serde(rename = "filteredAliasMode", default, skip_serializing_if = "Option::is_none")]
  pub filtered_alias_mode: Option<String>,
  #[serde(rename = "hostsResolverMode", default, skip_serializing_if = "Option::is_none")]
  pub hosts_resolver_mode: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub http: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub kibana: Option<serde_json::Value>,
  #[serde(rename = "multiRolespanEnabled", default, skip_serializing_if = "Option::is_none")]
  pub multi_rolespan_enabled: Option<bool>,
  #[serde(
    rename = "respectRequestIndicesOptions",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub respect_request_indices_options: Option<bool>,
}

impl From<&DynamicOptions> for DynamicOptions {
  fn from(value: &DynamicOptions) -> Self {
    value.clone()
  }
}

impl DynamicOptions {
  pub fn builder() -> builder::DynamicOptions {
    builder::DynamicOptions::default()
  }
}

///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExistsId(String);
impl std::ops::Deref for ExistsId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ExistsId> for String {
  fn from(value: ExistsId) -> Self {
    value.0
  }
}

impl From<&ExistsId> for ExistsId {
  fn from(value: &ExistsId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ExistsId {
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

impl std::convert::TryFrom<&str> for ExistsId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ExistsId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ExistsId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ExistsId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExistsIndex(String);
impl std::ops::Deref for ExistsIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ExistsIndex> for String {
  fn from(value: ExistsIndex) -> Self {
    value.0
  }
}

impl From<&ExistsIndex> for ExistsIndex {
  fn from(value: &ExistsIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ExistsIndex {
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

impl std::convert::TryFrom<&str> for ExistsIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ExistsIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ExistsIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ExistsIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExistsSourceId(String);
impl std::ops::Deref for ExistsSourceId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ExistsSourceId> for String {
  fn from(value: ExistsSourceId) -> Self {
    value.0
  }
}

impl From<&ExistsSourceId> for ExistsSourceId {
  fn from(value: &ExistsSourceId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ExistsSourceId {
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

impl std::convert::TryFrom<&str> for ExistsSourceId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ExistsSourceId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ExistsSourceId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ExistsSourceId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExistsSourceIndex(String);
impl std::ops::Deref for ExistsSourceIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ExistsSourceIndex> for String {
  fn from(value: ExistsSourceIndex) -> Self {
    value.0
  }
}

impl From<&ExistsSourceIndex> for ExistsSourceIndex {
  fn from(value: &ExistsSourceIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ExistsSourceIndex {
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

impl std::convert::TryFrom<&str> for ExistsSourceIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ExistsSourceIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ExistsSourceIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ExistsSourceIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExplainGetId(String);
impl std::ops::Deref for ExplainGetId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ExplainGetId> for String {
  fn from(value: ExplainGetId) -> Self {
    value.0
  }
}

impl From<&ExplainGetId> for ExplainGetId {
  fn from(value: &ExplainGetId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ExplainGetId {
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

impl std::convert::TryFrom<&str> for ExplainGetId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ExplainGetId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ExplainGetId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ExplainGetId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExplainGetIndex(String);
impl std::ops::Deref for ExplainGetIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ExplainGetIndex> for String {
  fn from(value: ExplainGetIndex) -> Self {
    value.0
  }
}

impl From<&ExplainGetIndex> for ExplainGetIndex {
  fn from(value: &ExplainGetIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ExplainGetIndex {
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

impl std::convert::TryFrom<&str> for ExplainGetIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ExplainGetIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ExplainGetIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ExplainGetIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExplainPostId(String);
impl std::ops::Deref for ExplainPostId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ExplainPostId> for String {
  fn from(value: ExplainPostId) -> Self {
    value.0
  }
}

impl From<&ExplainPostId> for ExplainPostId {
  fn from(value: &ExplainPostId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ExplainPostId {
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

impl std::convert::TryFrom<&str> for ExplainPostId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ExplainPostId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ExplainPostId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ExplainPostId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExplainPostIndex(String);
impl std::ops::Deref for ExplainPostIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ExplainPostIndex> for String {
  fn from(value: ExplainPostIndex) -> Self {
    value.0
  }
}

impl From<&ExplainPostIndex> for ExplainPostIndex {
  fn from(value: &ExplainPostIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ExplainPostIndex {
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

impl std::convert::TryFrom<&str> for ExplainPostIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ExplainPostIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ExplainPostIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ExplainPostIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FieldCapsGetWithIndexIndex(String);
impl std::ops::Deref for FieldCapsGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<FieldCapsGetWithIndexIndex> for String {
  fn from(value: FieldCapsGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&FieldCapsGetWithIndexIndex> for FieldCapsGetWithIndexIndex {
  fn from(value: &FieldCapsGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for FieldCapsGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for FieldCapsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for FieldCapsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for FieldCapsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for FieldCapsGetWithIndexIndex {
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
pub struct FieldCapsPostWithIndexIndex(String);
impl std::ops::Deref for FieldCapsPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<FieldCapsPostWithIndexIndex> for String {
  fn from(value: FieldCapsPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&FieldCapsPostWithIndexIndex> for FieldCapsPostWithIndexIndex {
  fn from(value: &FieldCapsPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for FieldCapsPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for FieldCapsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for FieldCapsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for FieldCapsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for FieldCapsPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlushCacheResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&FlushCacheResponseContent> for FlushCacheResponseContent {
  fn from(value: &FlushCacheResponseContent) -> Self {
    value.clone()
  }
}

impl FlushCacheResponseContent {
  pub fn builder() -> builder::FlushCacheResponseContent {
    builder::FlushCacheResponseContent::default()
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
pub struct GetCertificatesResponseContent {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub http_certificates_list: Vec<CertificatesDetail>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub transport_certificates_list: Vec<CertificatesDetail>,
}

impl From<&GetCertificatesResponseContent> for GetCertificatesResponseContent {
  fn from(value: &GetCertificatesResponseContent) -> Self {
    value.clone()
  }
}

impl GetCertificatesResponseContent {
  pub fn builder() -> builder::GetCertificatesResponseContent {
    builder::GetCertificatesResponseContent::default()
  }
}

///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GetId(String);
impl std::ops::Deref for GetId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<GetId> for String {
  fn from(value: GetId) -> Self {
    value.0
  }
}

impl From<&GetId> for GetId {
  fn from(value: &GetId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for GetId {
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

impl std::convert::TryFrom<&str> for GetId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for GetId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for GetId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for GetId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GetIndex(String);
impl std::ops::Deref for GetIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<GetIndex> for String {
  fn from(value: GetIndex) -> Self {
    value.0
  }
}

impl From<&GetIndex> for GetIndex {
  fn from(value: &GetIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for GetIndex {
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

impl std::convert::TryFrom<&str> for GetIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for GetIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for GetIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for GetIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Script ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GetScriptId(String);
impl std::ops::Deref for GetScriptId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<GetScriptId> for String {
  fn from(value: GetScriptId) -> Self {
    value.0
  }
}

impl From<&GetScriptId> for GetScriptId {
  fn from(value: &GetScriptId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for GetScriptId {
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

impl std::convert::TryFrom<&str> for GetScriptId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for GetScriptId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for GetScriptId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for GetScriptId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GetSourceId(String);
impl std::ops::Deref for GetSourceId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<GetSourceId> for String {
  fn from(value: GetSourceId) -> Self {
    value.0
  }
}

impl From<&GetSourceId> for GetSourceId {
  fn from(value: &GetSourceId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for GetSourceId {
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

impl std::convert::TryFrom<&str> for GetSourceId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for GetSourceId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for GetSourceId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for GetSourceId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GetSourceIndex(String);
impl std::ops::Deref for GetSourceIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<GetSourceIndex> for String {
  fn from(value: GetSourceIndex) -> Self {
    value.0
  }
}

impl From<&GetSourceIndex> for GetSourceIndex {
  fn from(value: &GetSourceIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for GetSourceIndex {
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

impl std::convert::TryFrom<&str> for GetSourceIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for GetSourceIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for GetSourceIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for GetSourceIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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
  #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(rename = "_index", default, skip_serializing_if = "Option::is_none")]
  pub index: Option<String>,
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

// impl<T: Serialize> From<T> for IndexBodyParams {
//   fn from(value: T) -> Self {
//     Self(serde_json::to_value(value).unwrap())
//   }
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndexPermission {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub allowed_actions: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub fls: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub index_patterns: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub masked_fields: Vec<String>,
}

impl From<&IndexPermission> for IndexPermission {
  fn from(value: &IndexPermission) -> Self {
    value.clone()
  }
}

impl IndexPermission {
  pub fn builder() -> builder::IndexPermission {
    builder::IndexPermission::default()
  }
}

///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexPostId(String);
impl std::ops::Deref for IndexPostId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexPostId> for String {
  fn from(value: IndexPostId) -> Self {
    value.0
  }
}

impl From<&IndexPostId> for IndexPostId {
  fn from(value: &IndexPostId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexPostId {
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

impl std::convert::TryFrom<&str> for IndexPostId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexPostId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexPostId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexPostId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexPostIndex(String);
impl std::ops::Deref for IndexPostIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexPostIndex> for String {
  fn from(value: IndexPostIndex) -> Self {
    value.0
  }
}

impl From<&IndexPostIndex> for IndexPostIndex {
  fn from(value: &IndexPostIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexPostIndex {
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

impl std::convert::TryFrom<&str> for IndexPostIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexPostIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexPostIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexPostIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexPutWithIdId(String);
impl std::ops::Deref for IndexPutWithIdId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexPutWithIdId> for String {
  fn from(value: IndexPutWithIdId) -> Self {
    value.0
  }
}

impl From<&IndexPutWithIdId> for IndexPutWithIdId {
  fn from(value: &IndexPutWithIdId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexPutWithIdId {
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

impl std::convert::TryFrom<&str> for IndexPutWithIdId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexPutWithIdId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexPutWithIdId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexPutWithIdId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexPutWithIdIndex(String);
impl std::ops::Deref for IndexPutWithIdIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexPutWithIdIndex> for String {
  fn from(value: IndexPutWithIdIndex) -> Self {
    value.0
  }
}

impl From<&IndexPutWithIdIndex> for IndexPutWithIdIndex {
  fn from(value: &IndexPutWithIdIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexPutWithIdIndex {
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

impl std::convert::TryFrom<&str> for IndexPutWithIdIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexPutWithIdIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexPutWithIdIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexPutWithIdIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Comma-separated list of indices to add a block to.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesAddBlockIndex(String);
impl std::ops::Deref for IndicesAddBlockIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesAddBlockIndex> for String {
  fn from(value: IndicesAddBlockIndex) -> Self {
    value.0
  }
}

impl From<&IndicesAddBlockIndex> for IndicesAddBlockIndex {
  fn from(value: &IndicesAddBlockIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesAddBlockIndex {
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

impl std::convert::TryFrom<&str> for IndicesAddBlockIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesAddBlockIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesAddBlockIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesAddBlockIndex {
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

///The name of the source index to clone.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePostIndex(String);
impl std::ops::Deref for IndicesClonePostIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesClonePostIndex> for String {
  fn from(value: IndicesClonePostIndex) -> Self {
    value.0
  }
}

impl From<&IndicesClonePostIndex> for IndicesClonePostIndex {
  fn from(value: &IndicesClonePostIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesClonePostIndex {
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

impl std::convert::TryFrom<&str> for IndicesClonePostIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesClonePostIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesClonePostIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesClonePostIndex {
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









///The name of the source index to clone.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePutIndex(String);
impl std::ops::Deref for IndicesClonePutIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesClonePutIndex> for String {
  fn from(value: IndicesClonePutIndex) -> Self {
    value.0
  }
}

impl From<&IndicesClonePutIndex> for IndicesClonePutIndex {
  fn from(value: &IndicesClonePutIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesClonePutIndex {
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

impl std::convert::TryFrom<&str> for IndicesClonePutIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesClonePutIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesClonePutIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesClonePutIndex {
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









///Comma-separated list of indices to close.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesCloseIndex(String);
impl std::ops::Deref for IndicesCloseIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesCloseIndex> for String {
  fn from(value: IndicesCloseIndex) -> Self {
    value.0
  }
}

impl From<&IndicesCloseIndex> for IndicesCloseIndex {
  fn from(value: &IndicesCloseIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesCloseIndex {
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

impl std::convert::TryFrom<&str> for IndicesCloseIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesCloseIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesCloseIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesCloseIndex {
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

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesCreateIndex(String);
impl std::ops::Deref for IndicesCreateIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesCreateIndex> for String {
  fn from(value: IndicesCreateIndex) -> Self {
    value.0
  }
}

impl From<&IndicesCreateIndex> for IndicesCreateIndex {
  fn from(value: &IndicesCreateIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesCreateIndex {
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

impl std::convert::TryFrom<&str> for IndicesCreateIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesCreateIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesCreateIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesCreateIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasIndex(String);
impl std::ops::Deref for IndicesDeleteAliasIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteAliasIndex> for String {
  fn from(value: IndicesDeleteAliasIndex) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteAliasIndex> for IndicesDeleteAliasIndex {
  fn from(value: &IndicesDeleteAliasIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteAliasIndex {
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

impl std::convert::TryFrom<&str> for IndicesDeleteAliasIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteAliasIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteAliasIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasPluralIndex(String);
impl std::ops::Deref for IndicesDeleteAliasPluralIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteAliasPluralIndex> for String {
  fn from(value: IndicesDeleteAliasPluralIndex) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteAliasPluralIndex> for IndicesDeleteAliasPluralIndex {
  fn from(value: &IndicesDeleteAliasPluralIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteAliasPluralIndex {
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

impl std::convert::TryFrom<&str> for IndicesDeleteAliasPluralIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteAliasPluralIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteAliasPluralIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasPluralIndex {
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

///Comma-separated list of indices to delete; use `_all` or `*` string to
/// delete all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteIndex(String);
impl std::ops::Deref for IndicesDeleteIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteIndex> for String {
  fn from(value: IndicesDeleteIndex) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteIndex> for IndicesDeleteIndex {
  fn from(value: &IndicesDeleteIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteIndex {
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

impl std::convert::TryFrom<&str> for IndicesDeleteIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteIndex {
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
pub struct IndicesDeleteIndexTemplateName(String);
impl std::ops::Deref for IndicesDeleteIndexTemplateName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteIndexTemplateName> for String {
  fn from(value: IndicesDeleteIndexTemplateName) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteIndexTemplateName> for IndicesDeleteIndexTemplateName {
  fn from(value: &IndicesDeleteIndexTemplateName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteIndexTemplateName {
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

impl std::convert::TryFrom<&str> for IndicesDeleteIndexTemplateName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteIndexTemplateName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteIndexTemplateName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteIndexTemplateName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Comma-separated list of indices to filter aliases.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesExistsAliasWithIndexIndex(String);
impl std::ops::Deref for IndicesExistsAliasWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesExistsAliasWithIndexIndex> for String {
  fn from(value: IndicesExistsAliasWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesExistsAliasWithIndexIndex> for IndicesExistsAliasWithIndexIndex {
  fn from(value: &IndicesExistsAliasWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesExistsAliasWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesExistsAliasWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesExistsAliasWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesExistsAliasWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesExistsAliasWithIndexIndex {
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
pub struct IndicesExistsAliasWithIndexName(String);
impl std::ops::Deref for IndicesExistsAliasWithIndexName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesExistsAliasWithIndexName> for String {
  fn from(value: IndicesExistsAliasWithIndexName) -> Self {
    value.0
  }
}

impl From<&IndicesExistsAliasWithIndexName> for IndicesExistsAliasWithIndexName {
  fn from(value: &IndicesExistsAliasWithIndexName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesExistsAliasWithIndexName {
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

impl std::convert::TryFrom<&str> for IndicesExistsAliasWithIndexName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesExistsAliasWithIndexName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesExistsAliasWithIndexName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesExistsAliasWithIndexName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesExistsIndex(String);
impl std::ops::Deref for IndicesExistsIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesExistsIndex> for String {
  fn from(value: IndicesExistsIndex) -> Self {
    value.0
  }
}

impl From<&IndicesExistsIndex> for IndicesExistsIndex {
  fn from(value: &IndicesExistsIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesExistsIndex {
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

impl std::convert::TryFrom<&str> for IndicesExistsIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesExistsIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesExistsIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesExistsIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesFlushGetWithIndexIndex(String);
impl std::ops::Deref for IndicesFlushGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesFlushGetWithIndexIndex> for String {
  fn from(value: IndicesFlushGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesFlushGetWithIndexIndex> for IndicesFlushGetWithIndexIndex {
  fn from(value: &IndicesFlushGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesFlushGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesFlushGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesFlushGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesFlushGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesFlushGetWithIndexIndex {
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
pub struct IndicesFlushPostWithIndexIndex(String);
impl std::ops::Deref for IndicesFlushPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesFlushPostWithIndexIndex> for String {
  fn from(value: IndicesFlushPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesFlushPostWithIndexIndex> for IndicesFlushPostWithIndexIndex {
  fn from(value: &IndicesFlushPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesFlushPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesFlushPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesFlushPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesFlushPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesFlushPostWithIndexIndex {
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
pub struct IndicesForcemergeWithIndexIndex(String);
impl std::ops::Deref for IndicesForcemergeWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesForcemergeWithIndexIndex> for String {
  fn from(value: IndicesForcemergeWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesForcemergeWithIndexIndex> for IndicesForcemergeWithIndexIndex {
  fn from(value: &IndicesForcemergeWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesForcemergeWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesForcemergeWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesForcemergeWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesForcemergeWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesForcemergeWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices to filter aliases.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetAliasWithIndexIndex(String);
impl std::ops::Deref for IndicesGetAliasWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetAliasWithIndexIndex> for String {
  fn from(value: IndicesGetAliasWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesGetAliasWithIndexIndex> for IndicesGetAliasWithIndexIndex {
  fn from(value: &IndicesGetAliasWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetAliasWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesGetAliasWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetAliasWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetAliasWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetAliasWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices to filter aliases.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetAliasWithIndexNameIndex(String);
impl std::ops::Deref for IndicesGetAliasWithIndexNameIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetAliasWithIndexNameIndex> for String {
  fn from(value: IndicesGetAliasWithIndexNameIndex) -> Self {
    value.0
  }
}

impl From<&IndicesGetAliasWithIndexNameIndex> for IndicesGetAliasWithIndexNameIndex {
  fn from(value: &IndicesGetAliasWithIndexNameIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetAliasWithIndexNameIndex {
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

impl std::convert::TryFrom<&str> for IndicesGetAliasWithIndexNameIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetAliasWithIndexNameIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetAliasWithIndexNameIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetAliasWithIndexNameIndex {
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
pub struct IndicesGetAliasWithIndexNameName(String);
impl std::ops::Deref for IndicesGetAliasWithIndexNameName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetAliasWithIndexNameName> for String {
  fn from(value: IndicesGetAliasWithIndexNameName) -> Self {
    value.0
  }
}

impl From<&IndicesGetAliasWithIndexNameName> for IndicesGetAliasWithIndexNameName {
  fn from(value: &IndicesGetAliasWithIndexNameName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetAliasWithIndexNameName {
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

impl std::convert::TryFrom<&str> for IndicesGetAliasWithIndexNameName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetAliasWithIndexNameName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetAliasWithIndexNameName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetAliasWithIndexNameName {
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

///Comma-separated list of indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetFieldMappingWithIndexIndex(String);
impl std::ops::Deref for IndicesGetFieldMappingWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetFieldMappingWithIndexIndex> for String {
  fn from(value: IndicesGetFieldMappingWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesGetFieldMappingWithIndexIndex> for IndicesGetFieldMappingWithIndexIndex {
  fn from(value: &IndicesGetFieldMappingWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetFieldMappingWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesGetFieldMappingWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetFieldMappingWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetFieldMappingWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetFieldMappingWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetIndex(String);
impl std::ops::Deref for IndicesGetIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetIndex> for String {
  fn from(value: IndicesGetIndex) -> Self {
    value.0
  }
}

impl From<&IndicesGetIndex> for IndicesGetIndex {
  fn from(value: &IndicesGetIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetIndex {
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

impl std::convert::TryFrom<&str> for IndicesGetIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetIndex {
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

///Comma-separated list of indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetMappingWithIndexIndex(String);
impl std::ops::Deref for IndicesGetMappingWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetMappingWithIndexIndex> for String {
  fn from(value: IndicesGetMappingWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesGetMappingWithIndexIndex> for IndicesGetMappingWithIndexIndex {
  fn from(value: &IndicesGetMappingWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetMappingWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesGetMappingWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetMappingWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetMappingWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetMappingWithIndexIndex {
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
pub struct IndicesGetSettingsWithIndexIndex(String);
impl std::ops::Deref for IndicesGetSettingsWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsWithIndexIndex> for String {
  fn from(value: IndicesGetSettingsWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsWithIndexIndex> for IndicesGetSettingsWithIndexIndex {
  fn from(value: &IndicesGetSettingsWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithIndexIndex {
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
pub struct IndicesGetSettingsWithIndexNameIndex(String);
impl std::ops::Deref for IndicesGetSettingsWithIndexNameIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsWithIndexNameIndex> for String {
  fn from(value: IndicesGetSettingsWithIndexNameIndex) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsWithIndexNameIndex> for IndicesGetSettingsWithIndexNameIndex {
  fn from(value: &IndicesGetSettingsWithIndexNameIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsWithIndexNameIndex {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsWithIndexNameIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsWithIndexNameIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsWithIndexNameIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithIndexNameIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetUpgradeWithIndexIndex(String);
impl std::ops::Deref for IndicesGetUpgradeWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetUpgradeWithIndexIndex> for String {
  fn from(value: IndicesGetUpgradeWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesGetUpgradeWithIndexIndex> for IndicesGetUpgradeWithIndexIndex {
  fn from(value: &IndicesGetUpgradeWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetUpgradeWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesGetUpgradeWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetUpgradeWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetUpgradeWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetUpgradeWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices to open.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesOpenIndex(String);
impl std::ops::Deref for IndicesOpenIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesOpenIndex> for String {
  fn from(value: IndicesOpenIndex) -> Self {
    value.0
  }
}

impl From<&IndicesOpenIndex> for IndicesOpenIndex {
  fn from(value: &IndicesOpenIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesOpenIndex {
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

impl std::convert::TryFrom<&str> for IndicesOpenIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesOpenIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesOpenIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesOpenIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostIndex(String);
impl std::ops::Deref for IndicesPutAliasPostIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPostIndex> for String {
  fn from(value: IndicesPutAliasPostIndex) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPostIndex> for IndicesPutAliasPostIndex {
  fn from(value: &IndicesPutAliasPostIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPostIndex {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPostIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPostIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPostIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostPluralIndex(String);
impl std::ops::Deref for IndicesPutAliasPostPluralIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPostPluralIndex> for String {
  fn from(value: IndicesPutAliasPostPluralIndex) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPostPluralIndex> for IndicesPutAliasPostPluralIndex {
  fn from(value: &IndicesPutAliasPostPluralIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPostPluralIndex {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPostPluralIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPostPluralIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPostPluralIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostPluralIndex {
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

















///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutIndex(String);
impl std::ops::Deref for IndicesPutAliasPutIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPutIndex> for String {
  fn from(value: IndicesPutAliasPutIndex) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPutIndex> for IndicesPutAliasPutIndex {
  fn from(value: &IndicesPutAliasPutIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPutIndex {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPutIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPutIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPutIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutPluralIndex(String);
impl std::ops::Deref for IndicesPutAliasPutPluralIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPutPluralIndex> for String {
  fn from(value: IndicesPutAliasPutPluralIndex) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPutPluralIndex> for IndicesPutAliasPutPluralIndex {
  fn from(value: &IndicesPutAliasPutPluralIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPutPluralIndex {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPutPluralIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPutPluralIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPutPluralIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutPluralIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutMappingPostIndex(String);
impl std::ops::Deref for IndicesPutMappingPostIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutMappingPostIndex> for String {
  fn from(value: IndicesPutMappingPostIndex) -> Self {
    value.0
  }
}

impl From<&IndicesPutMappingPostIndex> for IndicesPutMappingPostIndex {
  fn from(value: &IndicesPutMappingPostIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutMappingPostIndex {
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

impl std::convert::TryFrom<&str> for IndicesPutMappingPostIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutMappingPostIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutMappingPostIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutMappingPostIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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









///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutMappingPutIndex(String);
impl std::ops::Deref for IndicesPutMappingPutIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutMappingPutIndex> for String {
  fn from(value: IndicesPutMappingPutIndex) -> Self {
    value.0
  }
}

impl From<&IndicesPutMappingPutIndex> for IndicesPutMappingPutIndex {
  fn from(value: &IndicesPutMappingPutIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutMappingPutIndex {
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

impl std::convert::TryFrom<&str> for IndicesPutMappingPutIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutMappingPutIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutMappingPutIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutMappingPutIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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









///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutSettingsWithIndexIndex(String);
impl std::ops::Deref for IndicesPutSettingsWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutSettingsWithIndexIndex> for String {
  fn from(value: IndicesPutSettingsWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesPutSettingsWithIndexIndex> for IndicesPutSettingsWithIndexIndex {
  fn from(value: &IndicesPutSettingsWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutSettingsWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesPutSettingsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutSettingsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutSettingsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutSettingsWithIndexIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRecoveryWithIndexIndex(String);
impl std::ops::Deref for IndicesRecoveryWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRecoveryWithIndexIndex> for String {
  fn from(value: IndicesRecoveryWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesRecoveryWithIndexIndex> for IndicesRecoveryWithIndexIndex {
  fn from(value: &IndicesRecoveryWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRecoveryWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesRecoveryWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRecoveryWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRecoveryWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRecoveryWithIndexIndex {
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
pub struct IndicesRefreshGetWithIndexIndex(String);
impl std::ops::Deref for IndicesRefreshGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRefreshGetWithIndexIndex> for String {
  fn from(value: IndicesRefreshGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesRefreshGetWithIndexIndex> for IndicesRefreshGetWithIndexIndex {
  fn from(value: &IndicesRefreshGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRefreshGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesRefreshGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRefreshGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRefreshGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRefreshGetWithIndexIndex {
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
pub struct IndicesRefreshPostWithIndexIndex(String);
impl std::ops::Deref for IndicesRefreshPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRefreshPostWithIndexIndex> for String {
  fn from(value: IndicesRefreshPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesRefreshPostWithIndexIndex> for IndicesRefreshPostWithIndexIndex {
  fn from(value: &IndicesRefreshPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRefreshPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesRefreshPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRefreshPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRefreshPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRefreshPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of names or wildcard expressions.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesResolveIndexName(String);
impl std::ops::Deref for IndicesResolveIndexName {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesResolveIndexName> for String {
  fn from(value: IndicesResolveIndexName) -> Self {
    value.0
  }
}

impl From<&IndicesResolveIndexName> for IndicesResolveIndexName {
  fn from(value: &IndicesResolveIndexName) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesResolveIndexName {
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

impl std::convert::TryFrom<&str> for IndicesResolveIndexName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesResolveIndexName {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesResolveIndexName {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesResolveIndexName {
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









///The name of the alias to rollover.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRolloverWithNewIndexAlias(String);
impl std::ops::Deref for IndicesRolloverWithNewIndexAlias {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRolloverWithNewIndexAlias> for String {
  fn from(value: IndicesRolloverWithNewIndexAlias) -> Self {
    value.0
  }
}

impl From<&IndicesRolloverWithNewIndexAlias> for IndicesRolloverWithNewIndexAlias {
  fn from(value: &IndicesRolloverWithNewIndexAlias) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRolloverWithNewIndexAlias {
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

impl std::convert::TryFrom<&str> for IndicesRolloverWithNewIndexAlias {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRolloverWithNewIndexAlias {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRolloverWithNewIndexAlias {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRolloverWithNewIndexAlias {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The name of the rollover index.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRolloverWithNewIndexNewIndex(String);
impl std::ops::Deref for IndicesRolloverWithNewIndexNewIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRolloverWithNewIndexNewIndex> for String {
  fn from(value: IndicesRolloverWithNewIndexNewIndex) -> Self {
    value.0
  }
}

impl From<&IndicesRolloverWithNewIndexNewIndex> for IndicesRolloverWithNewIndexNewIndex {
  fn from(value: &IndicesRolloverWithNewIndexNewIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRolloverWithNewIndexNewIndex {
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

impl std::convert::TryFrom<&str> for IndicesRolloverWithNewIndexNewIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRolloverWithNewIndexNewIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRolloverWithNewIndexNewIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRolloverWithNewIndexNewIndex {
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
pub struct IndicesSegmentsWithIndexIndex(String);
impl std::ops::Deref for IndicesSegmentsWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSegmentsWithIndexIndex> for String {
  fn from(value: IndicesSegmentsWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesSegmentsWithIndexIndex> for IndicesSegmentsWithIndexIndex {
  fn from(value: &IndicesSegmentsWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSegmentsWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesSegmentsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSegmentsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSegmentsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSegmentsWithIndexIndex {
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
pub struct IndicesShardStoresWithIndexIndex(String);
impl std::ops::Deref for IndicesShardStoresWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesShardStoresWithIndexIndex> for String {
  fn from(value: IndicesShardStoresWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesShardStoresWithIndexIndex> for IndicesShardStoresWithIndexIndex {
  fn from(value: &IndicesShardStoresWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesShardStoresWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesShardStoresWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesShardStoresWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesShardStoresWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesShardStoresWithIndexIndex {
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

///The name of the source index to shrink.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPostIndex(String);
impl std::ops::Deref for IndicesShrinkPostIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesShrinkPostIndex> for String {
  fn from(value: IndicesShrinkPostIndex) -> Self {
    value.0
  }
}

impl From<&IndicesShrinkPostIndex> for IndicesShrinkPostIndex {
  fn from(value: &IndicesShrinkPostIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesShrinkPostIndex {
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

impl std::convert::TryFrom<&str> for IndicesShrinkPostIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesShrinkPostIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesShrinkPostIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesShrinkPostIndex {
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









///The name of the source index to shrink.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPutIndex(String);
impl std::ops::Deref for IndicesShrinkPutIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesShrinkPutIndex> for String {
  fn from(value: IndicesShrinkPutIndex) -> Self {
    value.0
  }
}

impl From<&IndicesShrinkPutIndex> for IndicesShrinkPutIndex {
  fn from(value: &IndicesShrinkPutIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesShrinkPutIndex {
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

impl std::convert::TryFrom<&str> for IndicesShrinkPutIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesShrinkPutIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesShrinkPutIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesShrinkPutIndex {
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

///The name of the source index to split.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPostIndex(String);
impl std::ops::Deref for IndicesSplitPostIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSplitPostIndex> for String {
  fn from(value: IndicesSplitPostIndex) -> Self {
    value.0
  }
}

impl From<&IndicesSplitPostIndex> for IndicesSplitPostIndex {
  fn from(value: &IndicesSplitPostIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSplitPostIndex {
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

impl std::convert::TryFrom<&str> for IndicesSplitPostIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSplitPostIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSplitPostIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSplitPostIndex {
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









///The name of the source index to split.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPutIndex(String);
impl std::ops::Deref for IndicesSplitPutIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSplitPutIndex> for String {
  fn from(value: IndicesSplitPutIndex) -> Self {
    value.0
  }
}

impl From<&IndicesSplitPutIndex> for IndicesSplitPutIndex {
  fn from(value: &IndicesSplitPutIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSplitPutIndex {
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

impl std::convert::TryFrom<&str> for IndicesSplitPutIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSplitPutIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSplitPutIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSplitPutIndex {
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









///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesStatsWithIndexIndex(String);
impl std::ops::Deref for IndicesStatsWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesStatsWithIndexIndex> for String {
  fn from(value: IndicesStatsWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesStatsWithIndexIndex> for IndicesStatsWithIndexIndex {
  fn from(value: &IndicesStatsWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesStatsWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesStatsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesStatsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesStatsWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesStatsWithIndexIndex {
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
pub struct IndicesStatsWithIndexMetricIndex(String);
impl std::ops::Deref for IndicesStatsWithIndexMetricIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesStatsWithIndexMetricIndex> for String {
  fn from(value: IndicesStatsWithIndexMetricIndex) -> Self {
    value.0
  }
}

impl From<&IndicesStatsWithIndexMetricIndex> for IndicesStatsWithIndexMetricIndex {
  fn from(value: &IndicesStatsWithIndexMetricIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesStatsWithIndexMetricIndex {
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

impl std::convert::TryFrom<&str> for IndicesStatsWithIndexMetricIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesStatsWithIndexMetricIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesStatsWithIndexMetricIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesStatsWithIndexMetricIndex {
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









///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesUpgradeWithIndexIndex(String);
impl std::ops::Deref for IndicesUpgradeWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesUpgradeWithIndexIndex> for String {
  fn from(value: IndicesUpgradeWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesUpgradeWithIndexIndex> for IndicesUpgradeWithIndexIndex {
  fn from(value: &IndicesUpgradeWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesUpgradeWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesUpgradeWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesUpgradeWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesUpgradeWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesUpgradeWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesValidateQueryGetWithIndexIndex(String);
impl std::ops::Deref for IndicesValidateQueryGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesValidateQueryGetWithIndexIndex> for String {
  fn from(value: IndicesValidateQueryGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesValidateQueryGetWithIndexIndex> for IndicesValidateQueryGetWithIndexIndex {
  fn from(value: &IndicesValidateQueryGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesValidateQueryGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesValidateQueryGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesValidateQueryGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesValidateQueryGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesValidateQueryGetWithIndexIndex {
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
pub struct IndicesValidateQueryPostWithIndexIndex(String);
impl std::ops::Deref for IndicesValidateQueryPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesValidateQueryPostWithIndexIndex> for String {
  fn from(value: IndicesValidateQueryPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&IndicesValidateQueryPostWithIndexIndex> for IndicesValidateQueryPostWithIndexIndex {
  fn from(value: &IndicesValidateQueryPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesValidateQueryPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for IndicesValidateQueryPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesValidateQueryPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesValidateQueryPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesValidateQueryPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MgetGetWithIndexIndex(String);
impl std::ops::Deref for MgetGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<MgetGetWithIndexIndex> for String {
  fn from(value: MgetGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&MgetGetWithIndexIndex> for MgetGetWithIndexIndex {
  fn from(value: &MgetGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for MgetGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for MgetGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for MgetGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for MgetGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for MgetGetWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MgetPostWithIndexIndex(String);
impl std::ops::Deref for MgetPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<MgetPostWithIndexIndex> for String {
  fn from(value: MgetPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&MgetPostWithIndexIndex> for MgetPostWithIndexIndex {
  fn from(value: &MgetPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for MgetPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for MgetPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for MgetPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for MgetPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for MgetPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Comma-separated list of indices to use as default.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MsearchGetWithIndexIndex(String);
impl std::ops::Deref for MsearchGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<MsearchGetWithIndexIndex> for String {
  fn from(value: MsearchGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&MsearchGetWithIndexIndex> for MsearchGetWithIndexIndex {
  fn from(value: &MsearchGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for MsearchGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for MsearchGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for MsearchGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for MsearchGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for MsearchGetWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices to use as default.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MsearchPostWithIndexIndex(String);
impl std::ops::Deref for MsearchPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<MsearchPostWithIndexIndex> for String {
  fn from(value: MsearchPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&MsearchPostWithIndexIndex> for MsearchPostWithIndexIndex {
  fn from(value: &MsearchPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for MsearchPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for MsearchPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for MsearchPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for MsearchPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for MsearchPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Comma-separated list of indices to use as default.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MsearchTemplateGetWithIndexIndex(String);
impl std::ops::Deref for MsearchTemplateGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<MsearchTemplateGetWithIndexIndex> for String {
  fn from(value: MsearchTemplateGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&MsearchTemplateGetWithIndexIndex> for MsearchTemplateGetWithIndexIndex {
  fn from(value: &MsearchTemplateGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for MsearchTemplateGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for MsearchTemplateGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for MsearchTemplateGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for MsearchTemplateGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for MsearchTemplateGetWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of indices to use as default.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MsearchTemplatePostWithIndexIndex(String);
impl std::ops::Deref for MsearchTemplatePostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<MsearchTemplatePostWithIndexIndex> for String {
  fn from(value: MsearchTemplatePostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&MsearchTemplatePostWithIndexIndex> for MsearchTemplatePostWithIndexIndex {
  fn from(value: &MsearchTemplatePostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for MsearchTemplatePostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for MsearchTemplatePostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for MsearchTemplatePostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for MsearchTemplatePostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for MsearchTemplatePostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

///The index in which the document resides.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MtermvectorsGetWithIndexIndex(String);
impl std::ops::Deref for MtermvectorsGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<MtermvectorsGetWithIndexIndex> for String {
  fn from(value: MtermvectorsGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&MtermvectorsGetWithIndexIndex> for MtermvectorsGetWithIndexIndex {
  fn from(value: &MtermvectorsGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for MtermvectorsGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for MtermvectorsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for MtermvectorsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for MtermvectorsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for MtermvectorsGetWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The index in which the document resides.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MtermvectorsPostWithIndexIndex(String);
impl std::ops::Deref for MtermvectorsPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<MtermvectorsPostWithIndexIndex> for String {
  fn from(value: MtermvectorsPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&MtermvectorsPostWithIndexIndex> for MtermvectorsPostWithIndexIndex {
  fn from(value: &MtermvectorsPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for MtermvectorsPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for MtermvectorsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for MtermvectorsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for MtermvectorsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for MtermvectorsPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The interval for the second sampling of threads.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsDeprecatedClusterInterval(String);
impl std::ops::Deref for NodesHotThreadsDeprecatedClusterInterval {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsDeprecatedClusterInterval> for String {
  fn from(value: NodesHotThreadsDeprecatedClusterInterval) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsDeprecatedClusterInterval> for NodesHotThreadsDeprecatedClusterInterval {
  fn from(value: &NodesHotThreadsDeprecatedClusterInterval) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsDeprecatedClusterInterval {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsDeprecatedClusterInterval {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsDeprecatedClusterInterval {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsDeprecatedClusterInterval {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsDeprecatedClusterInterval {
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

///The interval for the second sampling of threads.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsDeprecatedDashInterval(String);
impl std::ops::Deref for NodesHotThreadsDeprecatedDashInterval {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsDeprecatedDashInterval> for String {
  fn from(value: NodesHotThreadsDeprecatedDashInterval) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsDeprecatedDashInterval> for NodesHotThreadsDeprecatedDashInterval {
  fn from(value: &NodesHotThreadsDeprecatedDashInterval) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsDeprecatedDashInterval {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsDeprecatedDashInterval {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsDeprecatedDashInterval {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsDeprecatedDashInterval {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsDeprecatedDashInterval {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///The interval for the second sampling of threads.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsDeprecatedInterval(String);
impl std::ops::Deref for NodesHotThreadsDeprecatedInterval {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsDeprecatedInterval> for String {
  fn from(value: NodesHotThreadsDeprecatedInterval) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsDeprecatedInterval> for NodesHotThreadsDeprecatedInterval {
  fn from(value: &NodesHotThreadsDeprecatedInterval) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsDeprecatedInterval {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsDeprecatedInterval {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsDeprecatedInterval {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsDeprecatedInterval {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsDeprecatedInterval {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///The interval for the second sampling of threads.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsInterval(String);
impl std::ops::Deref for NodesHotThreadsInterval {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsInterval> for String {
  fn from(value: NodesHotThreadsInterval) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsInterval> for NodesHotThreadsInterval {
  fn from(value: &NodesHotThreadsInterval) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsInterval {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsInterval {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsInterval {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsInterval {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsInterval {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///The interval for the second sampling of threads.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsWithNodeIdDeprecatedClusterInterval(String);
impl std::ops::Deref for NodesHotThreadsWithNodeIdDeprecatedClusterInterval {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsWithNodeIdDeprecatedClusterInterval> for String {
  fn from(value: NodesHotThreadsWithNodeIdDeprecatedClusterInterval) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsWithNodeIdDeprecatedClusterInterval> for NodesHotThreadsWithNodeIdDeprecatedClusterInterval {
  fn from(value: &NodesHotThreadsWithNodeIdDeprecatedClusterInterval) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsWithNodeIdDeprecatedClusterInterval {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsWithNodeIdDeprecatedClusterInterval {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsWithNodeIdDeprecatedClusterInterval {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsWithNodeIdDeprecatedClusterInterval {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsWithNodeIdDeprecatedClusterInterval {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsWithNodeIdDeprecatedClusterNodeId(String);
impl std::ops::Deref for NodesHotThreadsWithNodeIdDeprecatedClusterNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsWithNodeIdDeprecatedClusterNodeId> for String {
  fn from(value: NodesHotThreadsWithNodeIdDeprecatedClusterNodeId) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsWithNodeIdDeprecatedClusterNodeId> for NodesHotThreadsWithNodeIdDeprecatedClusterNodeId {
  fn from(value: &NodesHotThreadsWithNodeIdDeprecatedClusterNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsWithNodeIdDeprecatedClusterNodeId {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsWithNodeIdDeprecatedClusterNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsWithNodeIdDeprecatedClusterNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsWithNodeIdDeprecatedClusterNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsWithNodeIdDeprecatedClusterNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///The interval for the second sampling of threads.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsWithNodeIdDeprecatedDashInterval(String);
impl std::ops::Deref for NodesHotThreadsWithNodeIdDeprecatedDashInterval {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsWithNodeIdDeprecatedDashInterval> for String {
  fn from(value: NodesHotThreadsWithNodeIdDeprecatedDashInterval) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsWithNodeIdDeprecatedDashInterval> for NodesHotThreadsWithNodeIdDeprecatedDashInterval {
  fn from(value: &NodesHotThreadsWithNodeIdDeprecatedDashInterval) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsWithNodeIdDeprecatedDashInterval {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsWithNodeIdDeprecatedDashInterval {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsWithNodeIdDeprecatedDashInterval {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsWithNodeIdDeprecatedDashInterval {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsWithNodeIdDeprecatedDashInterval {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsWithNodeIdDeprecatedDashNodeId(String);
impl std::ops::Deref for NodesHotThreadsWithNodeIdDeprecatedDashNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsWithNodeIdDeprecatedDashNodeId> for String {
  fn from(value: NodesHotThreadsWithNodeIdDeprecatedDashNodeId) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsWithNodeIdDeprecatedDashNodeId> for NodesHotThreadsWithNodeIdDeprecatedDashNodeId {
  fn from(value: &NodesHotThreadsWithNodeIdDeprecatedDashNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsWithNodeIdDeprecatedDashNodeId {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsWithNodeIdDeprecatedDashNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsWithNodeIdDeprecatedDashNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsWithNodeIdDeprecatedDashNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsWithNodeIdDeprecatedDashNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///The interval for the second sampling of threads.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsWithNodeIdDeprecatedInterval(String);
impl std::ops::Deref for NodesHotThreadsWithNodeIdDeprecatedInterval {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsWithNodeIdDeprecatedInterval> for String {
  fn from(value: NodesHotThreadsWithNodeIdDeprecatedInterval) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsWithNodeIdDeprecatedInterval> for NodesHotThreadsWithNodeIdDeprecatedInterval {
  fn from(value: &NodesHotThreadsWithNodeIdDeprecatedInterval) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsWithNodeIdDeprecatedInterval {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsWithNodeIdDeprecatedInterval {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsWithNodeIdDeprecatedInterval {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsWithNodeIdDeprecatedInterval {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsWithNodeIdDeprecatedInterval {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsWithNodeIdDeprecatedNodeId(String);
impl std::ops::Deref for NodesHotThreadsWithNodeIdDeprecatedNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsWithNodeIdDeprecatedNodeId> for String {
  fn from(value: NodesHotThreadsWithNodeIdDeprecatedNodeId) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsWithNodeIdDeprecatedNodeId> for NodesHotThreadsWithNodeIdDeprecatedNodeId {
  fn from(value: &NodesHotThreadsWithNodeIdDeprecatedNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsWithNodeIdDeprecatedNodeId {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsWithNodeIdDeprecatedNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsWithNodeIdDeprecatedNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsWithNodeIdDeprecatedNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsWithNodeIdDeprecatedNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///The interval for the second sampling of threads.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsWithNodeIdInterval(String);
impl std::ops::Deref for NodesHotThreadsWithNodeIdInterval {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsWithNodeIdInterval> for String {
  fn from(value: NodesHotThreadsWithNodeIdInterval) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsWithNodeIdInterval> for NodesHotThreadsWithNodeIdInterval {
  fn from(value: &NodesHotThreadsWithNodeIdInterval) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsWithNodeIdInterval {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsWithNodeIdInterval {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsWithNodeIdInterval {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsWithNodeIdInterval {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsWithNodeIdInterval {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesHotThreadsWithNodeIdNodeId(String);
impl std::ops::Deref for NodesHotThreadsWithNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesHotThreadsWithNodeIdNodeId> for String {
  fn from(value: NodesHotThreadsWithNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&NodesHotThreadsWithNodeIdNodeId> for NodesHotThreadsWithNodeIdNodeId {
  fn from(value: &NodesHotThreadsWithNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesHotThreadsWithNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for NodesHotThreadsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesHotThreadsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesHotThreadsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesHotThreadsWithNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

















///Comma-separated list of metrics you wish returned. Leave empty to return
/// all.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesInfoWithMetricNodeIdMetric(String);
impl std::ops::Deref for NodesInfoWithMetricNodeIdMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesInfoWithMetricNodeIdMetric> for String {
  fn from(value: NodesInfoWithMetricNodeIdMetric) -> Self {
    value.0
  }
}

impl From<&NodesInfoWithMetricNodeIdMetric> for NodesInfoWithMetricNodeIdMetric {
  fn from(value: &NodesInfoWithMetricNodeIdMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesInfoWithMetricNodeIdMetric {
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

impl std::convert::TryFrom<&str> for NodesInfoWithMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesInfoWithMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesInfoWithMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesInfoWithMetricNodeIdMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesInfoWithMetricNodeIdNodeId(String);
impl std::ops::Deref for NodesInfoWithMetricNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesInfoWithMetricNodeIdNodeId> for String {
  fn from(value: NodesInfoWithMetricNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&NodesInfoWithMetricNodeIdNodeId> for NodesInfoWithMetricNodeIdNodeId {
  fn from(value: &NodesInfoWithMetricNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesInfoWithMetricNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for NodesInfoWithMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesInfoWithMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesInfoWithMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesInfoWithMetricNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesInfoWithNodeIdNodeId(String);
impl std::ops::Deref for NodesInfoWithNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesInfoWithNodeIdNodeId> for String {
  fn from(value: NodesInfoWithNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&NodesInfoWithNodeIdNodeId> for NodesInfoWithNodeIdNodeId {
  fn from(value: &NodesInfoWithNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesInfoWithNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for NodesInfoWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesInfoWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesInfoWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesInfoWithNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///An object containing the password for the opensearch keystore
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodesReloadSecureSettingsBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for NodesReloadSecureSettingsBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<NodesReloadSecureSettingsBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: NodesReloadSecureSettingsBodyParams) -> Self {
    value.0
  }
}

impl From<&NodesReloadSecureSettingsBodyParams> for NodesReloadSecureSettingsBodyParams {
  fn from(value: &NodesReloadSecureSettingsBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for NodesReloadSecureSettingsBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}









///Comma-separated list of node IDs to span the reload/reinit call. Should
/// stay empty because reloading usually involves all cluster nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesReloadSecureSettingsWithNodeIdNodeId(String);
impl std::ops::Deref for NodesReloadSecureSettingsWithNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesReloadSecureSettingsWithNodeIdNodeId> for String {
  fn from(value: NodesReloadSecureSettingsWithNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&NodesReloadSecureSettingsWithNodeIdNodeId> for NodesReloadSecureSettingsWithNodeIdNodeId {
  fn from(value: &NodesReloadSecureSettingsWithNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesReloadSecureSettingsWithNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for NodesReloadSecureSettingsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesReloadSecureSettingsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesReloadSecureSettingsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesReloadSecureSettingsWithNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Return indices stats aggregated at index, node or shard level.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NodesStatLevel {
  #[serde(rename = "indices")]
  Indices,
  #[serde(rename = "node")]
  Node,
  #[serde(rename = "shards")]
  Shards,
}

impl From<&NodesStatLevel> for NodesStatLevel {
  fn from(value: &NodesStatLevel) -> Self {
    value.clone()
  }
}

impl ToString for NodesStatLevel {
  fn to_string(&self) -> String {
    match *self {
      Self::Indices => "indices".to_string(),
      Self::Node => "node".to_string(),
      Self::Shards => "shards".to_string(),
    }
  }
}

impl std::str::FromStr for NodesStatLevel {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    match value {
      "indices" => Ok(Self::Indices),
      "node" => Ok(Self::Node),
      "shards" => Ok(Self::Shards),
      _ => Err("invalid value"),
    }
  }
}

impl std::convert::TryFrom<&str> for NodesStatLevel {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatLevel {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatLevel {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}









///Limit the information returned for `indices` metric to the specific
/// index metrics. Isn't used if `indices` (or `all`) metric isn't
/// specified.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesStatsWithIndexMetricMetricIndexMetric(String);
impl std::ops::Deref for NodesStatsWithIndexMetricMetricIndexMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesStatsWithIndexMetricMetricIndexMetric> for String {
  fn from(value: NodesStatsWithIndexMetricMetricIndexMetric) -> Self {
    value.0
  }
}

impl From<&NodesStatsWithIndexMetricMetricIndexMetric> for NodesStatsWithIndexMetricMetricIndexMetric {
  fn from(value: &NodesStatsWithIndexMetricMetricIndexMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesStatsWithIndexMetricMetricIndexMetric {
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

impl std::convert::TryFrom<&str> for NodesStatsWithIndexMetricMetricIndexMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatsWithIndexMetricMetricIndexMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatsWithIndexMetricMetricIndexMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesStatsWithIndexMetricMetricIndexMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Limit the information returned to the specified metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesStatsWithIndexMetricMetricMetric(String);
impl std::ops::Deref for NodesStatsWithIndexMetricMetricMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesStatsWithIndexMetricMetricMetric> for String {
  fn from(value: NodesStatsWithIndexMetricMetricMetric) -> Self {
    value.0
  }
}

impl From<&NodesStatsWithIndexMetricMetricMetric> for NodesStatsWithIndexMetricMetricMetric {
  fn from(value: &NodesStatsWithIndexMetricMetricMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesStatsWithIndexMetricMetricMetric {
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

impl std::convert::TryFrom<&str> for NodesStatsWithIndexMetricMetricMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatsWithIndexMetricMetricMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatsWithIndexMetricMetricMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesStatsWithIndexMetricMetricMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Limit the information returned for `indices` metric to the specific
/// index metrics. Isn't used if `indices` (or `all`) metric isn't
/// specified.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesStatsWithIndexMetricMetricNodeIdIndexMetric(String);
impl std::ops::Deref for NodesStatsWithIndexMetricMetricNodeIdIndexMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesStatsWithIndexMetricMetricNodeIdIndexMetric> for String {
  fn from(value: NodesStatsWithIndexMetricMetricNodeIdIndexMetric) -> Self {
    value.0
  }
}

impl From<&NodesStatsWithIndexMetricMetricNodeIdIndexMetric> for NodesStatsWithIndexMetricMetricNodeIdIndexMetric {
  fn from(value: &NodesStatsWithIndexMetricMetricNodeIdIndexMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesStatsWithIndexMetricMetricNodeIdIndexMetric {
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

impl std::convert::TryFrom<&str> for NodesStatsWithIndexMetricMetricNodeIdIndexMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatsWithIndexMetricMetricNodeIdIndexMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatsWithIndexMetricMetricNodeIdIndexMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesStatsWithIndexMetricMetricNodeIdIndexMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Limit the information returned to the specified metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesStatsWithIndexMetricMetricNodeIdMetric(String);
impl std::ops::Deref for NodesStatsWithIndexMetricMetricNodeIdMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesStatsWithIndexMetricMetricNodeIdMetric> for String {
  fn from(value: NodesStatsWithIndexMetricMetricNodeIdMetric) -> Self {
    value.0
  }
}

impl From<&NodesStatsWithIndexMetricMetricNodeIdMetric> for NodesStatsWithIndexMetricMetricNodeIdMetric {
  fn from(value: &NodesStatsWithIndexMetricMetricNodeIdMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesStatsWithIndexMetricMetricNodeIdMetric {
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

impl std::convert::TryFrom<&str> for NodesStatsWithIndexMetricMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatsWithIndexMetricMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatsWithIndexMetricMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesStatsWithIndexMetricMetricNodeIdMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesStatsWithIndexMetricMetricNodeIdNodeId(String);
impl std::ops::Deref for NodesStatsWithIndexMetricMetricNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesStatsWithIndexMetricMetricNodeIdNodeId> for String {
  fn from(value: NodesStatsWithIndexMetricMetricNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&NodesStatsWithIndexMetricMetricNodeIdNodeId> for NodesStatsWithIndexMetricMetricNodeIdNodeId {
  fn from(value: &NodesStatsWithIndexMetricMetricNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesStatsWithIndexMetricMetricNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for NodesStatsWithIndexMetricMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatsWithIndexMetricMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatsWithIndexMetricMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesStatsWithIndexMetricMetricNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

















///Limit the information returned to the specified metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesStatsWithMetricMetric(String);
impl std::ops::Deref for NodesStatsWithMetricMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesStatsWithMetricMetric> for String {
  fn from(value: NodesStatsWithMetricMetric) -> Self {
    value.0
  }
}

impl From<&NodesStatsWithMetricMetric> for NodesStatsWithMetricMetric {
  fn from(value: &NodesStatsWithMetricMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesStatsWithMetricMetric {
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

impl std::convert::TryFrom<&str> for NodesStatsWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatsWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatsWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesStatsWithMetricMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Limit the information returned to the specified metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesStatsWithMetricNodeIdMetric(String);
impl std::ops::Deref for NodesStatsWithMetricNodeIdMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesStatsWithMetricNodeIdMetric> for String {
  fn from(value: NodesStatsWithMetricNodeIdMetric) -> Self {
    value.0
  }
}

impl From<&NodesStatsWithMetricNodeIdMetric> for NodesStatsWithMetricNodeIdMetric {
  fn from(value: &NodesStatsWithMetricNodeIdMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesStatsWithMetricNodeIdMetric {
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

impl std::convert::TryFrom<&str> for NodesStatsWithMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatsWithMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatsWithMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesStatsWithMetricNodeIdMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesStatsWithMetricNodeIdNodeId(String);
impl std::ops::Deref for NodesStatsWithMetricNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesStatsWithMetricNodeIdNodeId> for String {
  fn from(value: NodesStatsWithMetricNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&NodesStatsWithMetricNodeIdNodeId> for NodesStatsWithMetricNodeIdNodeId {
  fn from(value: &NodesStatsWithMetricNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesStatsWithMetricNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for NodesStatsWithMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatsWithMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatsWithMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesStatsWithMetricNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

















///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesStatsWithNodeIdNodeId(String);
impl std::ops::Deref for NodesStatsWithNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesStatsWithNodeIdNodeId> for String {
  fn from(value: NodesStatsWithNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&NodesStatsWithNodeIdNodeId> for NodesStatsWithNodeIdNodeId {
  fn from(value: &NodesStatsWithNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesStatsWithNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for NodesStatsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesStatsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesStatsWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesStatsWithNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

















///Limit the information returned to the specified metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesUsageWithMetricMetric(String);
impl std::ops::Deref for NodesUsageWithMetricMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesUsageWithMetricMetric> for String {
  fn from(value: NodesUsageWithMetricMetric) -> Self {
    value.0
  }
}

impl From<&NodesUsageWithMetricMetric> for NodesUsageWithMetricMetric {
  fn from(value: &NodesUsageWithMetricMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesUsageWithMetricMetric {
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

impl std::convert::TryFrom<&str> for NodesUsageWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesUsageWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesUsageWithMetricMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesUsageWithMetricMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Limit the information returned to the specified metrics.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesUsageWithMetricNodeIdMetric(String);
impl std::ops::Deref for NodesUsageWithMetricNodeIdMetric {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesUsageWithMetricNodeIdMetric> for String {
  fn from(value: NodesUsageWithMetricNodeIdMetric) -> Self {
    value.0
  }
}

impl From<&NodesUsageWithMetricNodeIdMetric> for NodesUsageWithMetricNodeIdMetric {
  fn from(value: &NodesUsageWithMetricNodeIdMetric) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesUsageWithMetricNodeIdMetric {
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

impl std::convert::TryFrom<&str> for NodesUsageWithMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesUsageWithMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesUsageWithMetricNodeIdMetric {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesUsageWithMetricNodeIdMetric {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesUsageWithMetricNodeIdNodeId(String);
impl std::ops::Deref for NodesUsageWithMetricNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesUsageWithMetricNodeIdNodeId> for String {
  fn from(value: NodesUsageWithMetricNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&NodesUsageWithMetricNodeIdNodeId> for NodesUsageWithMetricNodeIdNodeId {
  fn from(value: &NodesUsageWithMetricNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesUsageWithMetricNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for NodesUsageWithMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesUsageWithMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesUsageWithMetricNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesUsageWithMetricNodeIdNodeId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

















///Comma-separated list of node IDs or names to limit the returned
/// information; use `_local` to return information from the node you're
/// connecting to, leave empty to get information from all nodes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NodesUsageWithNodeIdNodeId(String);
impl std::ops::Deref for NodesUsageWithNodeIdNodeId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<NodesUsageWithNodeIdNodeId> for String {
  fn from(value: NodesUsageWithNodeIdNodeId) -> Self {
    value.0
  }
}

impl From<&NodesUsageWithNodeIdNodeId> for NodesUsageWithNodeIdNodeId {
  fn from(value: &NodesUsageWithNodeIdNodeId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for NodesUsageWithNodeIdNodeId {
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

impl std::convert::TryFrom<&str> for NodesUsageWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for NodesUsageWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for NodesUsageWithNodeIdNodeId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for NodesUsageWithNodeIdNodeId {
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
pub struct PatchActionGroupInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchActionGroupInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchActionGroupInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchActionGroupInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchActionGroupInputPayload> for PatchActionGroupInputPayload {
  fn from(value: &PatchActionGroupInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchActionGroupInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchActionGroupResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchActionGroupResponseContent> for PatchActionGroupResponseContent {
  fn from(value: &PatchActionGroupResponseContent) -> Self {
    value.clone()
  }
}

impl PatchActionGroupResponseContent {
  pub fn builder() -> builder::PatchActionGroupResponseContent {
    builder::PatchActionGroupResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchActionGroupsInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchActionGroupsInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchActionGroupsInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchActionGroupsInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchActionGroupsInputPayload> for PatchActionGroupsInputPayload {
  fn from(value: &PatchActionGroupsInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchActionGroupsInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchActionGroupsResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchActionGroupsResponseContent> for PatchActionGroupsResponseContent {
  fn from(value: &PatchActionGroupsResponseContent) -> Self {
    value.clone()
  }
}

impl PatchActionGroupsResponseContent {
  pub fn builder() -> builder::PatchActionGroupsResponseContent {
    builder::PatchActionGroupsResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchAuditConfigurationInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchAuditConfigurationInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchAuditConfigurationInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchAuditConfigurationInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchAuditConfigurationInputPayload> for PatchAuditConfigurationInputPayload {
  fn from(value: &PatchAuditConfigurationInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchAuditConfigurationInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchConfigurationInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchConfigurationInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchConfigurationInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchConfigurationInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchConfigurationInputPayload> for PatchConfigurationInputPayload {
  fn from(value: &PatchConfigurationInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchConfigurationInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchConfigurationResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchConfigurationResponseContent> for PatchConfigurationResponseContent {
  fn from(value: &PatchConfigurationResponseContent) -> Self {
    value.clone()
  }
}

impl PatchConfigurationResponseContent {
  pub fn builder() -> builder::PatchConfigurationResponseContent {
    builder::PatchConfigurationResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchDistinguishedNamesInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchDistinguishedNamesInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchDistinguishedNamesInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchDistinguishedNamesInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchDistinguishedNamesInputPayload> for PatchDistinguishedNamesInputPayload {
  fn from(value: &PatchDistinguishedNamesInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchDistinguishedNamesInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchDistinguishedNamesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchDistinguishedNamesResponseContent> for PatchDistinguishedNamesResponseContent {
  fn from(value: &PatchDistinguishedNamesResponseContent) -> Self {
    value.clone()
  }
}

impl PatchDistinguishedNamesResponseContent {
  pub fn builder() -> builder::PatchDistinguishedNamesResponseContent {
    builder::PatchDistinguishedNamesResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchOperation {
  ///The operation to perform. Possible values: remove,add, replace,
  /// move, copy, test.
  pub op: String,
  ///The path to the resource.
  pub path: String,
  ///The new values used for the update.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub value: Option<serde_json::Value>,
}

impl From<&PatchOperation> for PatchOperation {
  fn from(value: &PatchOperation) -> Self {
    value.clone()
  }
}

impl PatchOperation {
  pub fn builder() -> builder::PatchOperation {
    builder::PatchOperation::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchRoleInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchRoleInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchRoleInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchRoleInputPayload> for PatchRoleInputPayload {
  fn from(value: &PatchRoleInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchRoleInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleMappingInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchRoleMappingInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchRoleMappingInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchRoleMappingInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchRoleMappingInputPayload> for PatchRoleMappingInputPayload {
  fn from(value: &PatchRoleMappingInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchRoleMappingInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleMappingResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchRoleMappingResponseContent> for PatchRoleMappingResponseContent {
  fn from(value: &PatchRoleMappingResponseContent) -> Self {
    value.clone()
  }
}

impl PatchRoleMappingResponseContent {
  pub fn builder() -> builder::PatchRoleMappingResponseContent {
    builder::PatchRoleMappingResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleMappingsInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchRoleMappingsInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchRoleMappingsInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchRoleMappingsInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchRoleMappingsInputPayload> for PatchRoleMappingsInputPayload {
  fn from(value: &PatchRoleMappingsInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchRoleMappingsInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleMappingsResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchRoleMappingsResponseContent> for PatchRoleMappingsResponseContent {
  fn from(value: &PatchRoleMappingsResponseContent) -> Self {
    value.clone()
  }
}

impl PatchRoleMappingsResponseContent {
  pub fn builder() -> builder::PatchRoleMappingsResponseContent {
    builder::PatchRoleMappingsResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchRoleResponseContent> for PatchRoleResponseContent {
  fn from(value: &PatchRoleResponseContent) -> Self {
    value.clone()
  }
}

impl PatchRoleResponseContent {
  pub fn builder() -> builder::PatchRoleResponseContent {
    builder::PatchRoleResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRolesInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchRolesInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchRolesInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchRolesInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchRolesInputPayload> for PatchRolesInputPayload {
  fn from(value: &PatchRolesInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchRolesInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRolesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchRolesResponseContent> for PatchRolesResponseContent {
  fn from(value: &PatchRolesResponseContent) -> Self {
    value.clone()
  }
}

impl PatchRolesResponseContent {
  pub fn builder() -> builder::PatchRolesResponseContent {
    builder::PatchRolesResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchTenantInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchTenantInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchTenantInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchTenantInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchTenantInputPayload> for PatchTenantInputPayload {
  fn from(value: &PatchTenantInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchTenantInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchTenantResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchTenantResponseContent> for PatchTenantResponseContent {
  fn from(value: &PatchTenantResponseContent) -> Self {
    value.clone()
  }
}

impl PatchTenantResponseContent {
  pub fn builder() -> builder::PatchTenantResponseContent {
    builder::PatchTenantResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchTenantsInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchTenantsInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchTenantsInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchTenantsInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchTenantsInputPayload> for PatchTenantsInputPayload {
  fn from(value: &PatchTenantsInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchTenantsInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchTenantsResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchTenantsResponseContent> for PatchTenantsResponseContent {
  fn from(value: &PatchTenantsResponseContent) -> Self {
    value.clone()
  }
}

impl PatchTenantsResponseContent {
  pub fn builder() -> builder::PatchTenantsResponseContent {
    builder::PatchTenantsResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchUserInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchUserInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchUserInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchUserInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchUserInputPayload> for PatchUserInputPayload {
  fn from(value: &PatchUserInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchUserInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchUserResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchUserResponseContent> for PatchUserResponseContent {
  fn from(value: &PatchUserResponseContent) -> Self {
    value.clone()
  }
}

impl PatchUserResponseContent {
  pub fn builder() -> builder::PatchUserResponseContent {
    builder::PatchUserResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchUsersInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchUsersInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchUsersInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchUsersInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchUsersInputPayload> for PatchUsersInputPayload {
  fn from(value: &PatchUsersInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchUsersInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchUsersResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchUsersResponseContent> for PatchUsersResponseContent {
  fn from(value: &PatchUsersResponseContent) -> Self {
    value.clone()
  }
}

impl PatchUsersResponseContent {
  pub fn builder() -> builder::PatchUsersResponseContent {
    builder::PatchUsersResponseContent::default()
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

///Script ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostId(String);
impl std::ops::Deref for PutScriptPostId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostId> for String {
  fn from(value: PutScriptPostId) -> Self {
    value.0
  }
}

impl From<&PutScriptPostId> for PutScriptPostId {
  fn from(value: &PutScriptPostId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostId {
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

impl std::convert::TryFrom<&str> for PutScriptPostId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Script context.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostWithContextContext(String);
impl std::ops::Deref for PutScriptPostWithContextContext {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostWithContextContext> for String {
  fn from(value: PutScriptPostWithContextContext) -> Self {
    value.0
  }
}

impl From<&PutScriptPostWithContextContext> for PutScriptPostWithContextContext {
  fn from(value: &PutScriptPostWithContextContext) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostWithContextContext {
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

impl std::convert::TryFrom<&str> for PutScriptPostWithContextContext {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostWithContextContext {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostWithContextContext {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostWithContextContext {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Script ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostWithContextId(String);
impl std::ops::Deref for PutScriptPostWithContextId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostWithContextId> for String {
  fn from(value: PutScriptPostWithContextId) -> Self {
    value.0
  }
}

impl From<&PutScriptPostWithContextId> for PutScriptPostWithContextId {
  fn from(value: &PutScriptPostWithContextId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostWithContextId {
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

impl std::convert::TryFrom<&str> for PutScriptPostWithContextId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostWithContextId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostWithContextId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostWithContextId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Script ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutId(String);
impl std::ops::Deref for PutScriptPutId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutId> for String {
  fn from(value: PutScriptPutId) -> Self {
    value.0
  }
}

impl From<&PutScriptPutId> for PutScriptPutId {
  fn from(value: &PutScriptPutId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutId {
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

impl std::convert::TryFrom<&str> for PutScriptPutId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Script context.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutWithContextContext(String);
impl std::ops::Deref for PutScriptPutWithContextContext {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutWithContextContext> for String {
  fn from(value: PutScriptPutWithContextContext) -> Self {
    value.0
  }
}

impl From<&PutScriptPutWithContextContext> for PutScriptPutWithContextContext {
  fn from(value: &PutScriptPutWithContextContext) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutWithContextContext {
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

impl std::convert::TryFrom<&str> for PutScriptPutWithContextContext {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutWithContextContext {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutWithContextContext {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutWithContextContext {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Script ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutWithContextId(String);
impl std::ops::Deref for PutScriptPutWithContextId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutWithContextId> for String {
  fn from(value: PutScriptPutWithContextId) -> Self {
    value.0
  }
}

impl From<&PutScriptPutWithContextId> for PutScriptPutWithContextId {
  fn from(value: &PutScriptPutWithContextId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutWithContextId {
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

impl std::convert::TryFrom<&str> for PutScriptPutWithContextId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutWithContextId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutWithContextId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutWithContextId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RankEvalGetWithIndexIndex(String);
impl std::ops::Deref for RankEvalGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<RankEvalGetWithIndexIndex> for String {
  fn from(value: RankEvalGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&RankEvalGetWithIndexIndex> for RankEvalGetWithIndexIndex {
  fn from(value: &RankEvalGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for RankEvalGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for RankEvalGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for RankEvalGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for RankEvalGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for RankEvalGetWithIndexIndex {
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
pub struct RankEvalPostWithIndexIndex(String);
impl std::ops::Deref for RankEvalPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<RankEvalPostWithIndexIndex> for String {
  fn from(value: RankEvalPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&RankEvalPostWithIndexIndex> for RankEvalPostWithIndexIndex {
  fn from(value: &RankEvalPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for RankEvalPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for RankEvalPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for RankEvalPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for RankEvalPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for RankEvalPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///The task id to rethrottle.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ReindexRethrottleTaskId(String);
impl std::ops::Deref for ReindexRethrottleTaskId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ReindexRethrottleTaskId> for String {
  fn from(value: ReindexRethrottleTaskId) -> Self {
    value.0
  }
}

impl From<&ReindexRethrottleTaskId> for ReindexRethrottleTaskId {
  fn from(value: &ReindexRethrottleTaskId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ReindexRethrottleTaskId {
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

impl std::convert::TryFrom<&str> for ReindexRethrottleTaskId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ReindexRethrottleTaskId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ReindexRethrottleTaskId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ReindexRethrottleTaskId {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReloadHttpCertificatesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&ReloadHttpCertificatesResponseContent> for ReloadHttpCertificatesResponseContent {
  fn from(value: &ReloadHttpCertificatesResponseContent) -> Self {
    value.clone()
  }
}

impl ReloadHttpCertificatesResponseContent {
  pub fn builder() -> builder::ReloadHttpCertificatesResponseContent {
    builder::ReloadHttpCertificatesResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReloadTransportCertificatesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&ReloadTransportCertificatesResponseContent> for ReloadTransportCertificatesResponseContent {
  fn from(value: &ReloadTransportCertificatesResponseContent) -> Self {
    value.clone()
  }
}

impl ReloadTransportCertificatesResponseContent {
  pub fn builder() -> builder::ReloadTransportCertificatesResponseContent {
    builder::ReloadTransportCertificatesResponseContent::default()
  }
}

///Comma-separated list of index IDs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoreRestoreBodyParams {
  pub indices: Vec<String>,
}

impl From<&RemoteStoreRestoreBodyParams> for RemoteStoreRestoreBodyParams {
  fn from(value: &RemoteStoreRestoreBodyParams) -> Self {
    value.clone()
  }
}

impl RemoteStoreRestoreBodyParams {
  pub fn builder() -> builder::RemoteStoreRestoreBodyParams {
    builder::RemoteStoreRestoreBodyParams::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoreRestoreInfo {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub indices: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<RemoteStoreRestoreShardsInfo>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub snapshot: Option<String>,
}

impl From<&RemoteStoreRestoreInfo> for RemoteStoreRestoreInfo {
  fn from(value: &RemoteStoreRestoreInfo) -> Self {
    value.clone()
  }
}

impl RemoteStoreRestoreInfo {
  pub fn builder() -> builder::RemoteStoreRestoreInfo {
    builder::RemoteStoreRestoreInfo::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoreRestoreResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub accepted: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub remote_store: Option<RemoteStoreRestoreInfo>,
}

impl From<&RemoteStoreRestoreResponseContent> for RemoteStoreRestoreResponseContent {
  fn from(value: &RemoteStoreRestoreResponseContent) -> Self {
    value.clone()
  }
}

impl RemoteStoreRestoreResponseContent {
  pub fn builder() -> builder::RemoteStoreRestoreResponseContent {
    builder::RemoteStoreRestoreResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoreRestoreShardsInfo {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub failed: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub successful: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub total: Option<i32>,
}

impl From<&RemoteStoreRestoreShardsInfo> for RemoteStoreRestoreShardsInfo {
  fn from(value: &RemoteStoreRestoreShardsInfo) -> Self {
    value.clone()
  }
}

impl RemoteStoreRestoreShardsInfo {
  pub fn builder() -> builder::RemoteStoreRestoreShardsInfo {
    builder::RemoteStoreRestoreShardsInfo::default()
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

///The id of the stored search template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RenderSearchTemplateGetWithIdId(String);
impl std::ops::Deref for RenderSearchTemplateGetWithIdId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<RenderSearchTemplateGetWithIdId> for String {
  fn from(value: RenderSearchTemplateGetWithIdId) -> Self {
    value.0
  }
}

impl From<&RenderSearchTemplateGetWithIdId> for RenderSearchTemplateGetWithIdId {
  fn from(value: &RenderSearchTemplateGetWithIdId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for RenderSearchTemplateGetWithIdId {
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

impl std::convert::TryFrom<&str> for RenderSearchTemplateGetWithIdId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for RenderSearchTemplateGetWithIdId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for RenderSearchTemplateGetWithIdId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for RenderSearchTemplateGetWithIdId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The id of the stored search template.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RenderSearchTemplatePostWithIdId(String);
impl std::ops::Deref for RenderSearchTemplatePostWithIdId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<RenderSearchTemplatePostWithIdId> for String {
  fn from(value: RenderSearchTemplatePostWithIdId) -> Self {
    value.0
  }
}

impl From<&RenderSearchTemplatePostWithIdId> for RenderSearchTemplatePostWithIdId {
  fn from(value: &RenderSearchTemplatePostWithIdId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for RenderSearchTemplatePostWithIdId {
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

impl std::convert::TryFrom<&str> for RenderSearchTemplatePostWithIdId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for RenderSearchTemplatePostWithIdId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for RenderSearchTemplatePostWithIdId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for RenderSearchTemplatePostWithIdId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Role {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub cluster_permission: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index_permission: Option<IndexPermission>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
  pub static_: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub tenant_permissions: Vec<String>,
}

impl From<&Role> for Role {
  fn from(value: &Role) -> Self {
    value.clone()
  }
}

impl Role {
  pub fn builder() -> builder::Role {
    builder::Role::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleMapping {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub and_backend_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub backend_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub hosts: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub users: Vec<String>,
}

impl From<&RoleMapping> for RoleMapping {
  fn from(value: &RoleMapping) -> Self {
    value.clone()
  }
}

impl RoleMapping {
  pub fn builder() -> builder::RoleMapping {
    builder::RoleMapping::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleMappings(pub std::collections::HashMap<String, RoleMapping>);
impl std::ops::Deref for RoleMappings {
  type Target = std::collections::HashMap<String, RoleMapping>;

  fn deref(&self) -> &std::collections::HashMap<String, RoleMapping> {
    &self.0
  }
}

impl From<RoleMappings> for std::collections::HashMap<String, RoleMapping> {
  fn from(value: RoleMappings) -> Self {
    value.0
  }
}

impl From<&RoleMappings> for RoleMappings {
  fn from(value: &RoleMappings) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, RoleMapping>> for RoleMappings {
  fn from(value: std::collections::HashMap<String, RoleMapping>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RolesMap(pub std::collections::HashMap<String, Role>);
impl std::ops::Deref for RolesMap {
  type Target = std::collections::HashMap<String, Role>;

  fn deref(&self) -> &std::collections::HashMap<String, Role> {
    &self.0
  }
}

impl From<RolesMap> for std::collections::HashMap<String, Role> {
  fn from(value: RolesMap) -> Self {
    value.0
  }
}

impl From<&RolesMap> for RolesMap {
  fn from(value: &RolesMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, Role>> for RolesMap {
  fn from(value: std::collections::HashMap<String, Role>) -> Self {
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









///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchGetWithIndexIndex(String);
impl std::ops::Deref for SearchGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchGetWithIndexIndex> for String {
  fn from(value: SearchGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&SearchGetWithIndexIndex> for SearchGetWithIndexIndex {
  fn from(value: &SearchGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for SearchGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchGetWithIndexIndex {
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









///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchPostWithIndexIndex(String);
impl std::ops::Deref for SearchPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchPostWithIndexIndex> for String {
  fn from(value: SearchPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&SearchPostWithIndexIndex> for SearchPostWithIndexIndex {
  fn from(value: &SearchPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for SearchPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchPostWithIndexResponseContent<T> {
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

impl<T> From<&SearchPostWithIndexResponseContent<T>> for SearchPostWithIndexResponseContent<T> {
  fn from(value: &SearchPostWithIndexResponseContent<T>) -> Self {
    value.into()
  }
}

impl<T> SearchPostWithIndexResponseContent<T> {
  pub fn builder() -> builder::SearchPostWithIndexResponseContent<T> {
    builder::SearchPostWithIndexResponseContent::default()
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









///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchShardsGetWithIndexIndex(String);
impl std::ops::Deref for SearchShardsGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchShardsGetWithIndexIndex> for String {
  fn from(value: SearchShardsGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&SearchShardsGetWithIndexIndex> for SearchShardsGetWithIndexIndex {
  fn from(value: &SearchShardsGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchShardsGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for SearchShardsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchShardsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchShardsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchShardsGetWithIndexIndex {
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
pub struct SearchShardsPostWithIndexIndex(String);
impl std::ops::Deref for SearchShardsPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchShardsPostWithIndexIndex> for String {
  fn from(value: SearchShardsPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&SearchShardsPostWithIndexIndex> for SearchShardsPostWithIndexIndex {
  fn from(value: &SearchShardsPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchShardsPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for SearchShardsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchShardsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchShardsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchShardsPostWithIndexIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchTemplateGetWithIndexIndex(String);
impl std::ops::Deref for SearchTemplateGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchTemplateGetWithIndexIndex> for String {
  fn from(value: SearchTemplateGetWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&SearchTemplateGetWithIndexIndex> for SearchTemplateGetWithIndexIndex {
  fn from(value: &SearchTemplateGetWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchTemplateGetWithIndexIndex {
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

impl std::convert::TryFrom<&str> for SearchTemplateGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchTemplateGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchTemplateGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchTemplateGetWithIndexIndex {
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchTemplatePostWithIndexIndex(String);
impl std::ops::Deref for SearchTemplatePostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchTemplatePostWithIndexIndex> for String {
  fn from(value: SearchTemplatePostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&SearchTemplatePostWithIndexIndex> for SearchTemplatePostWithIndexIndex {
  fn from(value: &SearchTemplatePostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchTemplatePostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for SearchTemplatePostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchTemplatePostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchTemplatePostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchTemplatePostWithIndexIndex {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecurityHealthResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub mode: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&SecurityHealthResponseContent> for SecurityHealthResponseContent {
  fn from(value: &SecurityHealthResponseContent) -> Self {
    value.clone()
  }
}

impl SecurityHealthResponseContent {
  pub fn builder() -> builder::SecurityHealthResponseContent {
    builder::SecurityHealthResponseContent::default()
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

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCleanupRepositoryRepository(String);
impl std::ops::Deref for SnapshotCleanupRepositoryRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCleanupRepositoryRepository> for String {
  fn from(value: SnapshotCleanupRepositoryRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotCleanupRepositoryRepository> for SnapshotCleanupRepositoryRepository {
  fn from(value: &SnapshotCleanupRepositoryRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCleanupRepositoryRepository {
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

impl std::convert::TryFrom<&str> for SnapshotCleanupRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCleanupRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCleanupRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCleanupRepositoryRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///The snapshot clone definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnapshotCloneBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for SnapshotCloneBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<SnapshotCloneBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: SnapshotCloneBodyParams) -> Self {
    value.0
  }
}

impl From<&SnapshotCloneBodyParams> for SnapshotCloneBodyParams {
  fn from(value: &SnapshotCloneBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for SnapshotCloneBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCloneRepository(String);
impl std::ops::Deref for SnapshotCloneRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCloneRepository> for String {
  fn from(value: SnapshotCloneRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotCloneRepository> for SnapshotCloneRepository {
  fn from(value: &SnapshotCloneRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCloneRepository {
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

impl std::convert::TryFrom<&str> for SnapshotCloneRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCloneRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCloneRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCloneRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Snapshot name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCloneSnapshot(String);
impl std::ops::Deref for SnapshotCloneSnapshot {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCloneSnapshot> for String {
  fn from(value: SnapshotCloneSnapshot) -> Self {
    value.0
  }
}

impl From<&SnapshotCloneSnapshot> for SnapshotCloneSnapshot {
  fn from(value: &SnapshotCloneSnapshot) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCloneSnapshot {
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

impl std::convert::TryFrom<&str> for SnapshotCloneSnapshot {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCloneSnapshot {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCloneSnapshot {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCloneSnapshot {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The name of the cloned snapshot to create.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCloneTargetSnapshot(String);
impl std::ops::Deref for SnapshotCloneTargetSnapshot {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCloneTargetSnapshot> for String {
  fn from(value: SnapshotCloneTargetSnapshot) -> Self {
    value.0
  }
}

impl From<&SnapshotCloneTargetSnapshot> for SnapshotCloneTargetSnapshot {
  fn from(value: &SnapshotCloneTargetSnapshot) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCloneTargetSnapshot {
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

impl std::convert::TryFrom<&str> for SnapshotCloneTargetSnapshot {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCloneTargetSnapshot {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCloneTargetSnapshot {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCloneTargetSnapshot {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The snapshot definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnapshotCreateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for SnapshotCreateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<SnapshotCreateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: SnapshotCreateBodyParams) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateBodyParams> for SnapshotCreateBodyParams {
  fn from(value: &SnapshotCreateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for SnapshotCreateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreatePostRepository(String);
impl std::ops::Deref for SnapshotCreatePostRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreatePostRepository> for String {
  fn from(value: SnapshotCreatePostRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotCreatePostRepository> for SnapshotCreatePostRepository {
  fn from(value: &SnapshotCreatePostRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreatePostRepository {
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

impl std::convert::TryFrom<&str> for SnapshotCreatePostRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreatePostRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreatePostRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreatePostRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Snapshot name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreatePostSnapshot(String);
impl std::ops::Deref for SnapshotCreatePostSnapshot {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreatePostSnapshot> for String {
  fn from(value: SnapshotCreatePostSnapshot) -> Self {
    value.0
  }
}

impl From<&SnapshotCreatePostSnapshot> for SnapshotCreatePostSnapshot {
  fn from(value: &SnapshotCreatePostSnapshot) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreatePostSnapshot {
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

impl std::convert::TryFrom<&str> for SnapshotCreatePostSnapshot {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreatePostSnapshot {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreatePostSnapshot {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreatePostSnapshot {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreatePutRepository(String);
impl std::ops::Deref for SnapshotCreatePutRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreatePutRepository> for String {
  fn from(value: SnapshotCreatePutRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotCreatePutRepository> for SnapshotCreatePutRepository {
  fn from(value: &SnapshotCreatePutRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreatePutRepository {
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

impl std::convert::TryFrom<&str> for SnapshotCreatePutRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreatePutRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreatePutRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreatePutRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Snapshot name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreatePutSnapshot(String);
impl std::ops::Deref for SnapshotCreatePutSnapshot {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreatePutSnapshot> for String {
  fn from(value: SnapshotCreatePutSnapshot) -> Self {
    value.0
  }
}

impl From<&SnapshotCreatePutSnapshot> for SnapshotCreatePutSnapshot {
  fn from(value: &SnapshotCreatePutSnapshot) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreatePutSnapshot {
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

impl std::convert::TryFrom<&str> for SnapshotCreatePutSnapshot {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreatePutSnapshot {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreatePutSnapshot {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreatePutSnapshot {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The repository definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnapshotCreateRepositoryBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for SnapshotCreateRepositoryBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<SnapshotCreateRepositoryBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: SnapshotCreateRepositoryBodyParams) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateRepositoryBodyParams> for SnapshotCreateRepositoryBodyParams {
  fn from(value: &SnapshotCreateRepositoryBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for SnapshotCreateRepositoryBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreateRepositoryPostRepository(String);
impl std::ops::Deref for SnapshotCreateRepositoryPostRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreateRepositoryPostRepository> for String {
  fn from(value: SnapshotCreateRepositoryPostRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateRepositoryPostRepository> for SnapshotCreateRepositoryPostRepository {
  fn from(value: &SnapshotCreateRepositoryPostRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreateRepositoryPostRepository {
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

impl std::convert::TryFrom<&str> for SnapshotCreateRepositoryPostRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreateRepositoryPostRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreateRepositoryPostRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreateRepositoryPostRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreateRepositoryPutRepository(String);
impl std::ops::Deref for SnapshotCreateRepositoryPutRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreateRepositoryPutRepository> for String {
  fn from(value: SnapshotCreateRepositoryPutRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateRepositoryPutRepository> for SnapshotCreateRepositoryPutRepository {
  fn from(value: &SnapshotCreateRepositoryPutRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreateRepositoryPutRepository {
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

impl std::convert::TryFrom<&str> for SnapshotCreateRepositoryPutRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreateRepositoryPutRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreateRepositoryPutRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreateRepositoryPutRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotDeleteRepository(String);
impl std::ops::Deref for SnapshotDeleteRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotDeleteRepository> for String {
  fn from(value: SnapshotDeleteRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotDeleteRepository> for SnapshotDeleteRepository {
  fn from(value: &SnapshotDeleteRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotDeleteRepository {
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

impl std::convert::TryFrom<&str> for SnapshotDeleteRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotDeleteRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotDeleteRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotDeleteRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Name of the snapshot repository to unregister. Wildcard (`*`) patterns
/// are supported.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotDeleteRepositoryRepository(String);
impl std::ops::Deref for SnapshotDeleteRepositoryRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotDeleteRepositoryRepository> for String {
  fn from(value: SnapshotDeleteRepositoryRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotDeleteRepositoryRepository> for SnapshotDeleteRepositoryRepository {
  fn from(value: &SnapshotDeleteRepositoryRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotDeleteRepositoryRepository {
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

impl std::convert::TryFrom<&str> for SnapshotDeleteRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotDeleteRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotDeleteRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotDeleteRepositoryRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Snapshot name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotDeleteSnapshot(String);
impl std::ops::Deref for SnapshotDeleteSnapshot {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotDeleteSnapshot> for String {
  fn from(value: SnapshotDeleteSnapshot) -> Self {
    value.0
  }
}

impl From<&SnapshotDeleteSnapshot> for SnapshotDeleteSnapshot {
  fn from(value: &SnapshotDeleteSnapshot) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotDeleteSnapshot {
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

impl std::convert::TryFrom<&str> for SnapshotDeleteSnapshot {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotDeleteSnapshot {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotDeleteSnapshot {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotDeleteSnapshot {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotGetRepository(String);
impl std::ops::Deref for SnapshotGetRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotGetRepository> for String {
  fn from(value: SnapshotGetRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotGetRepository> for SnapshotGetRepository {
  fn from(value: &SnapshotGetRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotGetRepository {
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

impl std::convert::TryFrom<&str> for SnapshotGetRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotGetRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotGetRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotGetRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of repository names.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotGetRepositoryWithRepositoryRepository(String);
impl std::ops::Deref for SnapshotGetRepositoryWithRepositoryRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotGetRepositoryWithRepositoryRepository> for String {
  fn from(value: SnapshotGetRepositoryWithRepositoryRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotGetRepositoryWithRepositoryRepository> for SnapshotGetRepositoryWithRepositoryRepository {
  fn from(value: &SnapshotGetRepositoryWithRepositoryRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotGetRepositoryWithRepositoryRepository {
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

impl std::convert::TryFrom<&str> for SnapshotGetRepositoryWithRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotGetRepositoryWithRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotGetRepositoryWithRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotGetRepositoryWithRepositoryRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of snapshot names.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotGetSnapshot(String);
impl std::ops::Deref for SnapshotGetSnapshot {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotGetSnapshot> for String {
  fn from(value: SnapshotGetSnapshot) -> Self {
    value.0
  }
}

impl From<&SnapshotGetSnapshot> for SnapshotGetSnapshot {
  fn from(value: &SnapshotGetSnapshot) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotGetSnapshot {
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

impl std::convert::TryFrom<&str> for SnapshotGetSnapshot {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotGetSnapshot {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotGetSnapshot {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotGetSnapshot {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Details of what to restore
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnapshotRestoreBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for SnapshotRestoreBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<SnapshotRestoreBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: SnapshotRestoreBodyParams) -> Self {
    value.0
  }
}

impl From<&SnapshotRestoreBodyParams> for SnapshotRestoreBodyParams {
  fn from(value: &SnapshotRestoreBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for SnapshotRestoreBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotRestoreRepository(String);
impl std::ops::Deref for SnapshotRestoreRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotRestoreRepository> for String {
  fn from(value: SnapshotRestoreRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotRestoreRepository> for SnapshotRestoreRepository {
  fn from(value: &SnapshotRestoreRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotRestoreRepository {
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

impl std::convert::TryFrom<&str> for SnapshotRestoreRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotRestoreRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotRestoreRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotRestoreRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Snapshot name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotRestoreSnapshot(String);
impl std::ops::Deref for SnapshotRestoreSnapshot {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotRestoreSnapshot> for String {
  fn from(value: SnapshotRestoreSnapshot) -> Self {
    value.0
  }
}

impl From<&SnapshotRestoreSnapshot> for SnapshotRestoreSnapshot {
  fn from(value: &SnapshotRestoreSnapshot) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotRestoreSnapshot {
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

impl std::convert::TryFrom<&str> for SnapshotRestoreSnapshot {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotRestoreSnapshot {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotRestoreSnapshot {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotRestoreSnapshot {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotStatusWithRepositoryRepository(String);
impl std::ops::Deref for SnapshotStatusWithRepositoryRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotStatusWithRepositoryRepository> for String {
  fn from(value: SnapshotStatusWithRepositoryRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotStatusWithRepositoryRepository> for SnapshotStatusWithRepositoryRepository {
  fn from(value: &SnapshotStatusWithRepositoryRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotStatusWithRepositoryRepository {
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

impl std::convert::TryFrom<&str> for SnapshotStatusWithRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotStatusWithRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotStatusWithRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotStatusWithRepositoryRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotStatusWithRepositorySnapshotRepository(String);
impl std::ops::Deref for SnapshotStatusWithRepositorySnapshotRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotStatusWithRepositorySnapshotRepository> for String {
  fn from(value: SnapshotStatusWithRepositorySnapshotRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotStatusWithRepositorySnapshotRepository> for SnapshotStatusWithRepositorySnapshotRepository {
  fn from(value: &SnapshotStatusWithRepositorySnapshotRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotStatusWithRepositorySnapshotRepository {
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

impl std::convert::TryFrom<&str> for SnapshotStatusWithRepositorySnapshotRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotStatusWithRepositorySnapshotRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotStatusWithRepositorySnapshotRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotStatusWithRepositorySnapshotRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Comma-separated list of snapshot names.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotStatusWithRepositorySnapshotSnapshot(String);
impl std::ops::Deref for SnapshotStatusWithRepositorySnapshotSnapshot {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotStatusWithRepositorySnapshotSnapshot> for String {
  fn from(value: SnapshotStatusWithRepositorySnapshotSnapshot) -> Self {
    value.0
  }
}

impl From<&SnapshotStatusWithRepositorySnapshotSnapshot> for SnapshotStatusWithRepositorySnapshotSnapshot {
  fn from(value: &SnapshotStatusWithRepositorySnapshotSnapshot) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotStatusWithRepositorySnapshotSnapshot {
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

impl std::convert::TryFrom<&str> for SnapshotStatusWithRepositorySnapshotSnapshot {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotStatusWithRepositorySnapshotSnapshot {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotStatusWithRepositorySnapshotSnapshot {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotStatusWithRepositorySnapshotSnapshot {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Repository name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotVerifyRepositoryRepository(String);
impl std::ops::Deref for SnapshotVerifyRepositoryRepository {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotVerifyRepositoryRepository> for String {
  fn from(value: SnapshotVerifyRepositoryRepository) -> Self {
    value.0
  }
}

impl From<&SnapshotVerifyRepositoryRepository> for SnapshotVerifyRepositoryRepository {
  fn from(value: &SnapshotVerifyRepositoryRepository) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotVerifyRepositoryRepository {
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

impl std::convert::TryFrom<&str> for SnapshotVerifyRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotVerifyRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotVerifyRepositoryRepository {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotVerifyRepositoryRepository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

///Cancel the task with specified task id (node_id:task_number).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TasksCancelWithTaskIdTaskId(String);
impl std::ops::Deref for TasksCancelWithTaskIdTaskId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<TasksCancelWithTaskIdTaskId> for String {
  fn from(value: TasksCancelWithTaskIdTaskId) -> Self {
    value.0
  }
}

impl From<&TasksCancelWithTaskIdTaskId> for TasksCancelWithTaskIdTaskId {
  fn from(value: &TasksCancelWithTaskIdTaskId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for TasksCancelWithTaskIdTaskId {
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

impl std::convert::TryFrom<&str> for TasksCancelWithTaskIdTaskId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for TasksCancelWithTaskIdTaskId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for TasksCancelWithTaskIdTaskId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for TasksCancelWithTaskIdTaskId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Return the task with specified id (node_id:task_number).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TasksGetTaskId(String);
impl std::ops::Deref for TasksGetTaskId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<TasksGetTaskId> for String {
  fn from(value: TasksGetTaskId) -> Self {
    value.0
  }
}

impl From<&TasksGetTaskId> for TasksGetTaskId {
  fn from(value: &TasksGetTaskId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for TasksGetTaskId {
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

impl std::convert::TryFrom<&str> for TasksGetTaskId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for TasksGetTaskId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for TasksGetTaskId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for TasksGetTaskId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

















#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tenant {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
  pub static_: Option<bool>,
}

impl From<&Tenant> for Tenant {
  fn from(value: &Tenant) -> Self {
    value.clone()
  }
}

impl Tenant {
  pub fn builder() -> builder::Tenant {
    builder::Tenant::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TenantsMap(pub std::collections::HashMap<String, Tenant>);
impl std::ops::Deref for TenantsMap {
  type Target = std::collections::HashMap<String, Tenant>;

  fn deref(&self) -> &std::collections::HashMap<String, Tenant> {
    &self.0
  }
}

impl From<TenantsMap> for std::collections::HashMap<String, Tenant> {
  fn from(value: TenantsMap) -> Self {
    value.0
  }
}

impl From<&TenantsMap> for TenantsMap {
  fn from(value: &TenantsMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, Tenant>> for TenantsMap {
  fn from(value: std::collections::HashMap<String, Tenant>) -> Self {
    Self(value)
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

///The index in which the document resides.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TermvectorsGetIndex(String);
impl std::ops::Deref for TermvectorsGetIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<TermvectorsGetIndex> for String {
  fn from(value: TermvectorsGetIndex) -> Self {
    value.0
  }
}

impl From<&TermvectorsGetIndex> for TermvectorsGetIndex {
  fn from(value: &TermvectorsGetIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for TermvectorsGetIndex {
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

impl std::convert::TryFrom<&str> for TermvectorsGetIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for TermvectorsGetIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for TermvectorsGetIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for TermvectorsGetIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Document ID. When not specified a doc param should be supplied.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TermvectorsGetWithIdId(String);
impl std::ops::Deref for TermvectorsGetWithIdId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<TermvectorsGetWithIdId> for String {
  fn from(value: TermvectorsGetWithIdId) -> Self {
    value.0
  }
}

impl From<&TermvectorsGetWithIdId> for TermvectorsGetWithIdId {
  fn from(value: &TermvectorsGetWithIdId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for TermvectorsGetWithIdId {
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

impl std::convert::TryFrom<&str> for TermvectorsGetWithIdId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for TermvectorsGetWithIdId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for TermvectorsGetWithIdId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for TermvectorsGetWithIdId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The index in which the document resides.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TermvectorsGetWithIdIndex(String);
impl std::ops::Deref for TermvectorsGetWithIdIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<TermvectorsGetWithIdIndex> for String {
  fn from(value: TermvectorsGetWithIdIndex) -> Self {
    value.0
  }
}

impl From<&TermvectorsGetWithIdIndex> for TermvectorsGetWithIdIndex {
  fn from(value: &TermvectorsGetWithIdIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for TermvectorsGetWithIdIndex {
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

impl std::convert::TryFrom<&str> for TermvectorsGetWithIdIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for TermvectorsGetWithIdIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for TermvectorsGetWithIdIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for TermvectorsGetWithIdIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The index in which the document resides.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TermvectorsPostIndex(String);
impl std::ops::Deref for TermvectorsPostIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<TermvectorsPostIndex> for String {
  fn from(value: TermvectorsPostIndex) -> Self {
    value.0
  }
}

impl From<&TermvectorsPostIndex> for TermvectorsPostIndex {
  fn from(value: &TermvectorsPostIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for TermvectorsPostIndex {
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

impl std::convert::TryFrom<&str> for TermvectorsPostIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for TermvectorsPostIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for TermvectorsPostIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for TermvectorsPostIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Document ID. When not specified a doc param should be supplied.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TermvectorsPostWithIdId(String);
impl std::ops::Deref for TermvectorsPostWithIdId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<TermvectorsPostWithIdId> for String {
  fn from(value: TermvectorsPostWithIdId) -> Self {
    value.0
  }
}

impl From<&TermvectorsPostWithIdId> for TermvectorsPostWithIdId {
  fn from(value: &TermvectorsPostWithIdId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for TermvectorsPostWithIdId {
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

impl std::convert::TryFrom<&str> for TermvectorsPostWithIdId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for TermvectorsPostWithIdId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for TermvectorsPostWithIdId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for TermvectorsPostWithIdId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The index in which the document resides.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TermvectorsPostWithIdIndex(String);
impl std::ops::Deref for TermvectorsPostWithIdIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<TermvectorsPostWithIdIndex> for String {
  fn from(value: TermvectorsPostWithIdIndex) -> Self {
    value.0
  }
}

impl From<&TermvectorsPostWithIdIndex> for TermvectorsPostWithIdIndex {
  fn from(value: &TermvectorsPostWithIdIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for TermvectorsPostWithIdIndex {
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

impl std::convert::TryFrom<&str> for TermvectorsPostWithIdIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for TermvectorsPostWithIdIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for TermvectorsPostWithIdIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for TermvectorsPostWithIdIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAuditConfigurationResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&UpdateAuditConfigurationResponseContent> for UpdateAuditConfigurationResponseContent {
  fn from(value: &UpdateAuditConfigurationResponseContent) -> Self {
    value.clone()
  }
}

impl UpdateAuditConfigurationResponseContent {
  pub fn builder() -> builder::UpdateAuditConfigurationResponseContent {
    builder::UpdateAuditConfigurationResponseContent::default()
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

///Comma-separated list of indices; use `_all` or empty string to perform
/// the operation on all indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UpdateByQueryIndex(String);
impl std::ops::Deref for UpdateByQueryIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<UpdateByQueryIndex> for String {
  fn from(value: UpdateByQueryIndex) -> Self {
    value.0
  }
}

impl From<&UpdateByQueryIndex> for UpdateByQueryIndex {
  fn from(value: &UpdateByQueryIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for UpdateByQueryIndex {
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

impl std::convert::TryFrom<&str> for UpdateByQueryIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for UpdateByQueryIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for UpdateByQueryIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for UpdateByQueryIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The task id to rethrottle.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UpdateByQueryRethrottleTaskId(String);
impl std::ops::Deref for UpdateByQueryRethrottleTaskId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<UpdateByQueryRethrottleTaskId> for String {
  fn from(value: UpdateByQueryRethrottleTaskId) -> Self {
    value.0
  }
}

impl From<&UpdateByQueryRethrottleTaskId> for UpdateByQueryRethrottleTaskId {
  fn from(value: &UpdateByQueryRethrottleTaskId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for UpdateByQueryRethrottleTaskId {
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

impl std::convert::TryFrom<&str> for UpdateByQueryRethrottleTaskId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for UpdateByQueryRethrottleTaskId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for UpdateByQueryRethrottleTaskId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for UpdateByQueryRethrottleTaskId {
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









///Time each individual bulk request should wait for shards that are









#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateConfigurationResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&UpdateConfigurationResponseContent> for UpdateConfigurationResponseContent {
  fn from(value: &UpdateConfigurationResponseContent) -> Self {
    value.clone()
  }
}

impl UpdateConfigurationResponseContent {
  pub fn builder() -> builder::UpdateConfigurationResponseContent {
    builder::UpdateConfigurationResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateDistinguishedNamesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&UpdateDistinguishedNamesResponseContent> for UpdateDistinguishedNamesResponseContent {
  fn from(value: &UpdateDistinguishedNamesResponseContent) -> Self {
    value.clone()
  }
}

impl UpdateDistinguishedNamesResponseContent {
  pub fn builder() -> builder::UpdateDistinguishedNamesResponseContent {
    builder::UpdateDistinguishedNamesResponseContent::default()
  }
}

///Document ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UpdateId(String);
impl std::ops::Deref for UpdateId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<UpdateId> for String {
  fn from(value: UpdateId) -> Self {
    value.0
  }
}

impl From<&UpdateId> for UpdateId {
  fn from(value: &UpdateId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for UpdateId {
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

impl std::convert::TryFrom<&str> for UpdateId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for UpdateId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for UpdateId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for UpdateId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Index name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UpdateIndex(String);
impl std::ops::Deref for UpdateIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<UpdateIndex> for String {
  fn from(value: UpdateIndex) -> Self {
    value.0
  }
}

impl From<&UpdateIndex> for UpdateIndex {
  fn from(value: &UpdateIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for UpdateIndex {
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

impl std::convert::TryFrom<&str> for UpdateIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for UpdateIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for UpdateIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for UpdateIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub attributes: Option<UserAttributes>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub backend_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hash: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub opendistro_security_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
  pub static_: Option<bool>,
}

impl From<&User> for User {
  fn from(value: &User) -> Self {
    value.clone()
  }
}

impl User {
  pub fn builder() -> builder::User {
    builder::User::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserAttributes(pub std::collections::HashMap<String, String>);
impl std::ops::Deref for UserAttributes {
  type Target = std::collections::HashMap<String, String>;

  fn deref(&self) -> &std::collections::HashMap<String, String> {
    &self.0
  }
}

impl From<UserAttributes> for std::collections::HashMap<String, String> {
  fn from(value: UserAttributes) -> Self {
    value.0
  }
}

impl From<&UserAttributes> for UserAttributes {
  fn from(value: &UserAttributes) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, String>> for UserAttributes {
  fn from(value: std::collections::HashMap<String, String>) -> Self {
    Self(value)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserTenants {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub admin: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub admin_tenant: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub global_tenant: Option<bool>,
}

impl From<&UserTenants> for UserTenants {
  fn from(value: &UserTenants) -> Self {
    value.clone()
  }
}

impl UserTenants {
  pub fn builder() -> builder::UserTenants {
    builder::UserTenants::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UsersMap(pub std::collections::HashMap<String, User>);
impl std::ops::Deref for UsersMap {
  type Target = std::collections::HashMap<String, User>;

  fn deref(&self) -> &std::collections::HashMap<String, User> {
    &self.0
  }
}

impl From<UsersMap> for std::collections::HashMap<String, User> {
  fn from(value: UsersMap) -> Self {
    value.0
  }
}

impl From<&UsersMap> for UsersMap {
  fn from(value: &UsersMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, User>> for UsersMap {
  fn from(value: std::collections::HashMap<String, User>) -> Self {
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
  pub struct AccountDetails {
    backend_roles: Result<Vec<String>, String>,
    custom_attribute_names: Result<Vec<String>, String>,
    is_hidden: Result<Option<bool>, String>,
    is_internal_user: Result<Option<bool>, String>,
    is_reserved: Result<Option<bool>, String>,
    roles: Result<Vec<String>, String>,
    tenants: Result<Option<super::UserTenants>, String>,
    user_name: Result<Option<String>, String>,
    user_requested_tenant: Result<Option<String>, String>,
  }

  impl Default for AccountDetails {
    fn default() -> Self {
      Self {
        backend_roles: Ok(Default::default()),
        custom_attribute_names: Ok(Default::default()),
        is_hidden: Ok(Default::default()),
        is_internal_user: Ok(Default::default()),
        is_reserved: Ok(Default::default()),
        roles: Ok(Default::default()),
        tenants: Ok(Default::default()),
        user_name: Ok(Default::default()),
        user_requested_tenant: Ok(Default::default()),
      }
    }
  }

  impl AccountDetails {
    pub fn backend_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.backend_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for backend_roles: {}", e));
      self
    }

    pub fn custom_attribute_names<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.custom_attribute_names = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for custom_attribute_names: {}", e));
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

    pub fn is_internal_user<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.is_internal_user = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for is_internal_user: {}", e));
      self
    }

    pub fn is_reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.is_reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for is_reserved: {}", e));
      self
    }

    pub fn roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for roles: {}", e));
      self
    }

    pub fn tenants<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserTenants>>,
      T::Error: std::fmt::Display, {
      self.tenants = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for tenants: {}", e));
      self
    }

    pub fn user_name<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.user_name = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for user_name: {}", e));
      self
    }

    pub fn user_requested_tenant<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.user_requested_tenant = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for user_requested_tenant: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<AccountDetails> for super::AccountDetails {
    type Error = String;

    fn try_from(value: AccountDetails) -> Result<Self, String> {
      Ok(Self {
        backend_roles: value.backend_roles?,
        custom_attribute_names: value.custom_attribute_names?,
        is_hidden: value.is_hidden?,
        is_internal_user: value.is_internal_user?,
        is_reserved: value.is_reserved?,
        roles: value.roles?,
        tenants: value.tenants?,
        user_name: value.user_name?,
        user_requested_tenant: value.user_requested_tenant?,
      })
    }
  }

  impl From<super::AccountDetails> for AccountDetails {
    fn from(value: super::AccountDetails) -> Self {
      Self {
        backend_roles: Ok(value.backend_roles),
        custom_attribute_names: Ok(value.custom_attribute_names),
        is_hidden: Ok(value.is_hidden),
        is_internal_user: Ok(value.is_internal_user),
        is_reserved: Ok(value.is_reserved),
        roles: Ok(value.roles),
        tenants: Ok(value.tenants),
        user_name: Ok(value.user_name),
        user_requested_tenant: Ok(value.user_requested_tenant),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ActionGroup {
    allowed_actions: Result<Vec<String>, String>,
    description: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    reserved: Result<Option<bool>, String>,
    static_: Result<Option<bool>, String>,
    type_: Result<Option<String>, String>,
  }

  impl Default for ActionGroup {
    fn default() -> Self {
      Self {
        allowed_actions: Ok(Default::default()),
        description: Ok(Default::default()),
        hidden: Ok(Default::default()),
        reserved: Ok(Default::default()),
        static_: Ok(Default::default()),
        type_: Ok(Default::default()),
      }
    }
  }

  impl ActionGroup {
    pub fn allowed_actions<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.allowed_actions = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for allowed_actions: {}", e));
      self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn static_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.static_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for static_: {}", e));
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
  }

  impl std::convert::TryFrom<ActionGroup> for super::ActionGroup {
    type Error = String;

    fn try_from(value: ActionGroup) -> Result<Self, String> {
      Ok(Self {
        allowed_actions: value.allowed_actions?,
        description: value.description?,
        hidden: value.hidden?,
        reserved: value.reserved?,
        static_: value.static_?,
        type_: value.type_?,
      })
    }
  }

  impl From<super::ActionGroup> for ActionGroup {
    fn from(value: super::ActionGroup) -> Self {
      Self {
        allowed_actions: Ok(value.allowed_actions),
        description: Ok(value.description),
        hidden: Ok(value.hidden),
        reserved: Ok(value.reserved),
        static_: Ok(value.static_),
        type_: Ok(value.type_),
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

  #[derive(Clone, Debug)]
  pub struct AuditConfig {
    audit: Result<Option<super::AuditLogsConfig>, String>,
    compliance: Result<Option<super::ComplianceConfig>, String>,
    enabled: Result<Option<bool>, String>,
  }

  impl Default for AuditConfig {
    fn default() -> Self {
      Self {
        audit: Ok(Default::default()),
        compliance: Ok(Default::default()),
        enabled: Ok(Default::default()),
      }
    }
  }

  impl AuditConfig {
    pub fn audit<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::AuditLogsConfig>>,
      T::Error: std::fmt::Display, {
      self.audit = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for audit: {}", e));
      self
    }

    pub fn compliance<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::ComplianceConfig>>,
      T::Error: std::fmt::Display, {
      self.compliance = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for compliance: {}", e));
      self
    }

    pub fn enabled<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.enabled = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for enabled: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<AuditConfig> for super::AuditConfig {
    type Error = String;

    fn try_from(value: AuditConfig) -> Result<Self, String> {
      Ok(Self {
        audit: value.audit?,
        compliance: value.compliance?,
        enabled: value.enabled?,
      })
    }
  }

  impl From<super::AuditConfig> for AuditConfig {
    fn from(value: super::AuditConfig) -> Self {
      Self {
        audit: Ok(value.audit),
        compliance: Ok(value.compliance),
        enabled: Ok(value.enabled),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct AuditConfigWithReadOnly {
    config: Result<Option<super::AuditConfig>, String>,
    readonly: Result<Vec<String>, String>,
  }

  impl Default for AuditConfigWithReadOnly {
    fn default() -> Self {
      Self {
        config: Ok(Default::default()),
        readonly: Ok(Default::default()),
      }
    }
  }

  impl AuditConfigWithReadOnly {
    pub fn config<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::AuditConfig>>,
      T::Error: std::fmt::Display, {
      self.config = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for config: {}", e));
      self
    }

    pub fn readonly<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.readonly = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for readonly: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<AuditConfigWithReadOnly> for super::AuditConfigWithReadOnly {
    type Error = String;

    fn try_from(value: AuditConfigWithReadOnly) -> Result<Self, String> {
      Ok(Self {
        config: value.config?,
        readonly: value.readonly?,
      })
    }
  }

  impl From<super::AuditConfigWithReadOnly> for AuditConfigWithReadOnly {
    fn from(value: super::AuditConfigWithReadOnly) -> Self {
      Self {
        config: Ok(value.config),
        readonly: Ok(value.readonly),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct AuditLogsConfig {
    disabled_rest_categories: Result<Vec<String>, String>,
    disabled_transport_categories: Result<Vec<String>, String>,
    enable_rest: Result<Option<bool>, String>,
    enable_transport: Result<Option<bool>, String>,
    exclude_sensitive_headers: Result<Option<bool>, String>,
    ignore_requests: Result<Vec<String>, String>,
    ignore_users: Result<Vec<String>, String>,
    log_request_body: Result<Option<bool>, String>,
    resolve_bulk_requests: Result<Option<bool>, String>,
    resolve_indices: Result<Option<bool>, String>,
  }

  impl Default for AuditLogsConfig {
    fn default() -> Self {
      Self {
        disabled_rest_categories: Ok(Default::default()),
        disabled_transport_categories: Ok(Default::default()),
        enable_rest: Ok(Default::default()),
        enable_transport: Ok(Default::default()),
        exclude_sensitive_headers: Ok(Default::default()),
        ignore_requests: Ok(Default::default()),
        ignore_users: Ok(Default::default()),
        log_request_body: Ok(Default::default()),
        resolve_bulk_requests: Ok(Default::default()),
        resolve_indices: Ok(Default::default()),
      }
    }
  }

  impl AuditLogsConfig {
    pub fn disabled_rest_categories<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.disabled_rest_categories = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for disabled_rest_categories: {}", e));
      self
    }

    pub fn disabled_transport_categories<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.disabled_transport_categories = value.try_into().map_err(|e| {
        format!(
          "error converting supplied value for disabled_transport_categories: {}",
          e
        )
      });
      self
    }

    pub fn enable_rest<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.enable_rest = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for enable_rest: {}", e));
      self
    }

    pub fn enable_transport<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.enable_transport = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for enable_transport: {}", e));
      self
    }

    pub fn exclude_sensitive_headers<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.exclude_sensitive_headers = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for exclude_sensitive_headers: {}", e));
      self
    }

    pub fn ignore_requests<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.ignore_requests = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for ignore_requests: {}", e));
      self
    }

    pub fn ignore_users<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.ignore_users = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for ignore_users: {}", e));
      self
    }

    pub fn log_request_body<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.log_request_body = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for log_request_body: {}", e));
      self
    }

    pub fn resolve_bulk_requests<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.resolve_bulk_requests = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for resolve_bulk_requests: {}", e));
      self
    }

    pub fn resolve_indices<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.resolve_indices = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for resolve_indices: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<AuditLogsConfig> for super::AuditLogsConfig {
    type Error = String;

    fn try_from(value: AuditLogsConfig) -> Result<Self, String> {
      Ok(Self {
        disabled_rest_categories: value.disabled_rest_categories?,
        disabled_transport_categories: value.disabled_transport_categories?,
        enable_rest: value.enable_rest?,
        enable_transport: value.enable_transport?,
        exclude_sensitive_headers: value.exclude_sensitive_headers?,
        ignore_requests: value.ignore_requests?,
        ignore_users: value.ignore_users?,
        log_request_body: value.log_request_body?,
        resolve_bulk_requests: value.resolve_bulk_requests?,
        resolve_indices: value.resolve_indices?,
      })
    }
  }

  impl From<super::AuditLogsConfig> for AuditLogsConfig {
    fn from(value: super::AuditLogsConfig) -> Self {
      Self {
        disabled_rest_categories: Ok(value.disabled_rest_categories),
        disabled_transport_categories: Ok(value.disabled_transport_categories),
        enable_rest: Ok(value.enable_rest),
        enable_transport: Ok(value.enable_transport),
        exclude_sensitive_headers: Ok(value.exclude_sensitive_headers),
        ignore_requests: Ok(value.ignore_requests),
        ignore_users: Ok(value.ignore_users),
        log_request_body: Ok(value.log_request_body),
        resolve_bulk_requests: Ok(value.resolve_bulk_requests),
        resolve_indices: Ok(value.resolve_indices),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CatAllPitSegmentsResponseContent {
    content: Result<Option<super::CatPitSegment>, String>,
  }

  impl Default for CatAllPitSegmentsResponseContent {
    fn default() -> Self {
      Self {
        content: Ok(Default::default()),
      }
    }
  }

  impl CatAllPitSegmentsResponseContent {
    pub fn content<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::CatPitSegment>>,
      T::Error: std::fmt::Display, {
      self.content = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for content: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CatAllPitSegmentsResponseContent> for super::CatAllPitSegmentsResponseContent {
    type Error = String;

    fn try_from(value: CatAllPitSegmentsResponseContent) -> Result<Self, String> {
      Ok(Self {
        content: value.content?,
      })
    }
  }

  impl From<super::CatAllPitSegmentsResponseContent> for CatAllPitSegmentsResponseContent {
    fn from(value: super::CatAllPitSegmentsResponseContent) -> Self {
      Self {
        content: Ok(value.content),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CatPitSegment {
    committed: Result<Option<bool>, String>,
    compound: Result<Option<bool>, String>,
    docs_count: Result<Option<i32>, String>,
    docs_deleted: Result<Option<i32>, String>,
    generation: Result<Option<i32>, String>,
    index: Result<Option<String>, String>,
    ip: Result<Option<String>, String>,
    prirep: Result<Option<bool>, String>,
    searchable: Result<Option<bool>, String>,
    segment: Result<Option<String>, String>,
    shard: Result<Option<i32>, String>,
    size: Result<Option<String>, String>,
    size_memory: Result<Option<i32>, String>,
    version: Result<Option<String>, String>,
  }

  impl Default for CatPitSegment {
    fn default() -> Self {
      Self {
        committed: Ok(Default::default()),
        compound: Ok(Default::default()),
        docs_count: Ok(Default::default()),
        docs_deleted: Ok(Default::default()),
        generation: Ok(Default::default()),
        index: Ok(Default::default()),
        ip: Ok(Default::default()),
        prirep: Ok(Default::default()),
        searchable: Ok(Default::default()),
        segment: Ok(Default::default()),
        shard: Ok(Default::default()),
        size: Ok(Default::default()),
        size_memory: Ok(Default::default()),
        version: Ok(Default::default()),
      }
    }
  }

  impl CatPitSegment {
    pub fn committed<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.committed = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for committed: {}", e));
      self
    }

    pub fn compound<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.compound = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for compound: {}", e));
      self
    }

    pub fn docs_count<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.docs_count = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for docs_count: {}", e));
      self
    }

    pub fn docs_deleted<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.docs_deleted = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for docs_deleted: {}", e));
      self
    }

    pub fn generation<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.generation = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for generation: {}", e));
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

    pub fn ip<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.ip = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for ip: {}", e));
      self
    }

    pub fn prirep<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.prirep = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for prirep: {}", e));
      self
    }

    pub fn searchable<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.searchable = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for searchable: {}", e));
      self
    }

    pub fn segment<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.segment = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for segment: {}", e));
      self
    }

    pub fn shard<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.shard = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for shard: {}", e));
      self
    }

    pub fn size<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.size = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for size: {}", e));
      self
    }

    pub fn size_memory<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.size_memory = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for size_memory: {}", e));
      self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.version = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for version: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CatPitSegment> for super::CatPitSegment {
    type Error = String;

    fn try_from(value: CatPitSegment) -> Result<Self, String> {
      Ok(Self {
        committed: value.committed?,
        compound: value.compound?,
        docs_count: value.docs_count?,
        docs_deleted: value.docs_deleted?,
        generation: value.generation?,
        index: value.index?,
        ip: value.ip?,
        prirep: value.prirep?,
        searchable: value.searchable?,
        segment: value.segment?,
        shard: value.shard?,
        size: value.size?,
        size_memory: value.size_memory?,
        version: value.version?,
      })
    }
  }

  impl From<super::CatPitSegment> for CatPitSegment {
    fn from(value: super::CatPitSegment) -> Self {
      Self {
        committed: Ok(value.committed),
        compound: Ok(value.compound),
        docs_count: Ok(value.docs_count),
        docs_deleted: Ok(value.docs_deleted),
        generation: Ok(value.generation),
        index: Ok(value.index),
        ip: Ok(value.ip),
        prirep: Ok(value.prirep),
        searchable: Ok(value.searchable),
        segment: Ok(value.segment),
        shard: Ok(value.shard),
        size: Ok(value.size),
        size_memory: Ok(value.size_memory),
        version: Ok(value.version),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CatPitSegmentsBodyParams {
    pit_id: Result<Vec<String>, String>,
  }

  impl Default for CatPitSegmentsBodyParams {
    fn default() -> Self {
      Self {
        pit_id: Err("no value supplied for pit_id".to_string()),
      }
    }
  }

  impl CatPitSegmentsBodyParams {
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

  impl std::convert::TryFrom<CatPitSegmentsBodyParams> for super::CatPitSegmentsBodyParams {
    type Error = String;

    fn try_from(value: CatPitSegmentsBodyParams) -> Result<Self, String> {
      Ok(Self { pit_id: value.pit_id? })
    }
  }

  impl From<super::CatPitSegmentsBodyParams> for CatPitSegmentsBodyParams {
    fn from(value: super::CatPitSegmentsBodyParams) -> Self {
      Self {
        pit_id: Ok(value.pit_id),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CatPitSegmentsResponseContent {
    content: Result<Option<super::CatPitSegment>, String>,
  }

  impl Default for CatPitSegmentsResponseContent {
    fn default() -> Self {
      Self {
        content: Ok(Default::default()),
      }
    }
  }

  impl CatPitSegmentsResponseContent {
    pub fn content<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::CatPitSegment>>,
      T::Error: std::fmt::Display, {
      self.content = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for content: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CatPitSegmentsResponseContent> for super::CatPitSegmentsResponseContent {
    type Error = String;

    fn try_from(value: CatPitSegmentsResponseContent) -> Result<Self, String> {
      Ok(Self {
        content: value.content?,
      })
    }
  }

  impl From<super::CatPitSegmentsResponseContent> for CatPitSegmentsResponseContent {
    fn from(value: super::CatPitSegmentsResponseContent) -> Self {
      Self {
        content: Ok(value.content),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CertificatesDetail {
    issuer_dn: Result<Option<String>, String>,
    not_after: Result<Option<String>, String>,
    not_before: Result<Option<String>, String>,
    san: Result<Option<String>, String>,
    subject_dn: Result<Option<String>, String>,
  }

  impl Default for CertificatesDetail {
    fn default() -> Self {
      Self {
        issuer_dn: Ok(Default::default()),
        not_after: Ok(Default::default()),
        not_before: Ok(Default::default()),
        san: Ok(Default::default()),
        subject_dn: Ok(Default::default()),
      }
    }
  }

  impl CertificatesDetail {
    pub fn issuer_dn<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.issuer_dn = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for issuer_dn: {}", e));
      self
    }

    pub fn not_after<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.not_after = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for not_after: {}", e));
      self
    }

    pub fn not_before<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.not_before = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for not_before: {}", e));
      self
    }

    pub fn san<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.san = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for san: {}", e));
      self
    }

    pub fn subject_dn<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.subject_dn = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for subject_dn: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CertificatesDetail> for super::CertificatesDetail {
    type Error = String;

    fn try_from(value: CertificatesDetail) -> Result<Self, String> {
      Ok(Self {
        issuer_dn: value.issuer_dn?,
        not_after: value.not_after?,
        not_before: value.not_before?,
        san: value.san?,
        subject_dn: value.subject_dn?,
      })
    }
  }

  impl From<super::CertificatesDetail> for CertificatesDetail {
    fn from(value: super::CertificatesDetail) -> Self {
      Self {
        issuer_dn: Ok(value.issuer_dn),
        not_after: Ok(value.not_after),
        not_before: Ok(value.not_before),
        san: Ok(value.san),
        subject_dn: Ok(value.subject_dn),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ChangePasswordRequestContent {
    current_password: Result<String, String>,
    password: Result<String, String>,
  }

  impl Default for ChangePasswordRequestContent {
    fn default() -> Self {
      Self {
        current_password: Err("no value supplied for current_password".to_string()),
        password: Err("no value supplied for password".to_string()),
      }
    }
  }

  impl ChangePasswordRequestContent {
    pub fn current_password<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.current_password = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for current_password: {}", e));
      self
    }

    pub fn password<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.password = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for password: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ChangePasswordRequestContent> for super::ChangePasswordRequestContent {
    type Error = String;

    fn try_from(value: ChangePasswordRequestContent) -> Result<Self, String> {
      Ok(Self {
        current_password: value.current_password?,
        password: value.password?,
      })
    }
  }

  impl From<super::ChangePasswordRequestContent> for ChangePasswordRequestContent {
    fn from(value: super::ChangePasswordRequestContent) -> Self {
      Self {
        current_password: Ok(value.current_password),
        password: Ok(value.password),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ChangePasswordResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for ChangePasswordResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl ChangePasswordResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ChangePasswordResponseContent> for super::ChangePasswordResponseContent {
    type Error = String;

    fn try_from(value: ChangePasswordResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::ChangePasswordResponseContent> for ChangePasswordResponseContent {
    fn from(value: super::ChangePasswordResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ClusterGetSettingsResponseContent {
    defaults: Result<Option<super::UserDefinedValueMap>, String>,
    persistent: Result<Option<super::UserDefinedValueMap>, String>,
    transient: Result<Option<super::UserDefinedValueMap>, String>,
  }

  impl Default for ClusterGetSettingsResponseContent {
    fn default() -> Self {
      Self {
        defaults: Ok(Default::default()),
        persistent: Ok(Default::default()),
        transient: Ok(Default::default()),
      }
    }
  }

  impl ClusterGetSettingsResponseContent {
    pub fn defaults<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.defaults = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for defaults: {}", e));
      self
    }

    pub fn persistent<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.persistent = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for persistent: {}", e));
      self
    }

    pub fn transient<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.transient = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for transient: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ClusterGetSettingsResponseContent> for super::ClusterGetSettingsResponseContent {
    type Error = String;

    fn try_from(value: ClusterGetSettingsResponseContent) -> Result<Self, String> {
      Ok(Self {
        defaults: value.defaults?,
        persistent: value.persistent?,
        transient: value.transient?,
      })
    }
  }

  impl From<super::ClusterGetSettingsResponseContent> for ClusterGetSettingsResponseContent {
    fn from(value: super::ClusterGetSettingsResponseContent) -> Self {
      Self {
        defaults: Ok(value.defaults),
        persistent: Ok(value.persistent),
        transient: Ok(value.transient),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ClusterPutSettingsBodyParams {
    persistent: Result<Option<super::UserDefinedValueMap>, String>,
    transient: Result<Option<super::UserDefinedValueMap>, String>,
  }

  impl Default for ClusterPutSettingsBodyParams {
    fn default() -> Self {
      Self {
        persistent: Ok(Default::default()),
        transient: Ok(Default::default()),
      }
    }
  }

  impl ClusterPutSettingsBodyParams {
    pub fn persistent<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.persistent = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for persistent: {}", e));
      self
    }

    pub fn transient<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.transient = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for transient: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ClusterPutSettingsBodyParams> for super::ClusterPutSettingsBodyParams {
    type Error = String;

    fn try_from(value: ClusterPutSettingsBodyParams) -> Result<Self, String> {
      Ok(Self {
        persistent: value.persistent?,
        transient: value.transient?,
      })
    }
  }

  impl From<super::ClusterPutSettingsBodyParams> for ClusterPutSettingsBodyParams {
    fn from(value: super::ClusterPutSettingsBodyParams) -> Self {
      Self {
        persistent: Ok(value.persistent),
        transient: Ok(value.transient),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ClusterPutSettingsResponseContent {
    acknowledged: Result<Option<bool>, String>,
    persistent: Result<Option<super::UserDefinedValueMap>, String>,
    transient: Result<Option<super::UserDefinedValueMap>, String>,
  }

  impl Default for ClusterPutSettingsResponseContent {
    fn default() -> Self {
      Self {
        acknowledged: Ok(Default::default()),
        persistent: Ok(Default::default()),
        transient: Ok(Default::default()),
      }
    }
  }

  impl ClusterPutSettingsResponseContent {
    pub fn acknowledged<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.acknowledged = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for acknowledged: {}", e));
      self
    }

    pub fn persistent<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.persistent = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for persistent: {}", e));
      self
    }

    pub fn transient<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.transient = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for transient: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ClusterPutSettingsResponseContent> for super::ClusterPutSettingsResponseContent {
    type Error = String;

    fn try_from(value: ClusterPutSettingsResponseContent) -> Result<Self, String> {
      Ok(Self {
        acknowledged: value.acknowledged?,
        persistent: value.persistent?,
        transient: value.transient?,
      })
    }
  }

  impl From<super::ClusterPutSettingsResponseContent> for ClusterPutSettingsResponseContent {
    fn from(value: super::ClusterPutSettingsResponseContent) -> Self {
      Self {
        acknowledged: Ok(value.acknowledged),
        persistent: Ok(value.persistent),
        transient: Ok(value.transient),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ComplianceConfig {
    enabled: Result<Option<bool>, String>,
    external_config: Result<Option<bool>, String>,
    internal_config: Result<Option<bool>, String>,
    read_ignore_users: Result<Vec<String>, String>,
    read_metadata_only: Result<Option<bool>, String>,
    read_watched_fields: Result<Option<serde_json::Value>, String>,
    write_ignore_users: Result<Vec<String>, String>,
    write_log_diffs: Result<Option<bool>, String>,
    write_metadata_only: Result<Option<bool>, String>,
    write_watched_indices: Result<Vec<String>, String>,
  }

  impl Default for ComplianceConfig {
    fn default() -> Self {
      Self {
        enabled: Ok(Default::default()),
        external_config: Ok(Default::default()),
        internal_config: Ok(Default::default()),
        read_ignore_users: Ok(Default::default()),
        read_metadata_only: Ok(Default::default()),
        read_watched_fields: Ok(Default::default()),
        write_ignore_users: Ok(Default::default()),
        write_log_diffs: Ok(Default::default()),
        write_metadata_only: Ok(Default::default()),
        write_watched_indices: Ok(Default::default()),
      }
    }
  }

  impl ComplianceConfig {
    pub fn enabled<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.enabled = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for enabled: {}", e));
      self
    }

    pub fn external_config<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.external_config = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for external_config: {}", e));
      self
    }

    pub fn internal_config<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.internal_config = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for internal_config: {}", e));
      self
    }

    pub fn read_ignore_users<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.read_ignore_users = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for read_ignore_users: {}", e));
      self
    }

    pub fn read_metadata_only<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.read_metadata_only = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for read_metadata_only: {}", e));
      self
    }

    pub fn read_watched_fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.read_watched_fields = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for read_watched_fields: {}", e));
      self
    }

    pub fn write_ignore_users<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.write_ignore_users = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for write_ignore_users: {}", e));
      self
    }

    pub fn write_log_diffs<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.write_log_diffs = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for write_log_diffs: {}", e));
      self
    }

    pub fn write_metadata_only<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.write_metadata_only = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for write_metadata_only: {}", e));
      self
    }

    pub fn write_watched_indices<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.write_watched_indices = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for write_watched_indices: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ComplianceConfig> for super::ComplianceConfig {
    type Error = String;

    fn try_from(value: ComplianceConfig) -> Result<Self, String> {
      Ok(Self {
        enabled: value.enabled?,
        external_config: value.external_config?,
        internal_config: value.internal_config?,
        read_ignore_users: value.read_ignore_users?,
        read_metadata_only: value.read_metadata_only?,
        read_watched_fields: value.read_watched_fields?,
        write_ignore_users: value.write_ignore_users?,
        write_log_diffs: value.write_log_diffs?,
        write_metadata_only: value.write_metadata_only?,
        write_watched_indices: value.write_watched_indices?,
      })
    }
  }

  impl From<super::ComplianceConfig> for ComplianceConfig {
    fn from(value: super::ComplianceConfig) -> Self {
      Self {
        enabled: Ok(value.enabled),
        external_config: Ok(value.external_config),
        internal_config: Ok(value.internal_config),
        read_ignore_users: Ok(value.read_ignore_users),
        read_metadata_only: Ok(value.read_metadata_only),
        read_watched_fields: Ok(value.read_watched_fields),
        write_ignore_users: Ok(value.write_ignore_users),
        write_log_diffs: Ok(value.write_log_diffs),
        write_metadata_only: Ok(value.write_metadata_only),
        write_watched_indices: Ok(value.write_watched_indices),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateActionGroupResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateActionGroupResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateActionGroupResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateActionGroupResponseContent> for super::CreateActionGroupResponseContent {
    type Error = String;

    fn try_from(value: CreateActionGroupResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateActionGroupResponseContent> for CreateActionGroupResponseContent {
    fn from(value: super::CreateActionGroupResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

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
  pub struct CreateRoleMappingResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateRoleMappingResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateRoleMappingResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateRoleMappingResponseContent> for super::CreateRoleMappingResponseContent {
    type Error = String;

    fn try_from(value: CreateRoleMappingResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateRoleMappingResponseContent> for CreateRoleMappingResponseContent {
    fn from(value: super::CreateRoleMappingResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateRoleResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateRoleResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateRoleResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateRoleResponseContent> for super::CreateRoleResponseContent {
    type Error = String;

    fn try_from(value: CreateRoleResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateRoleResponseContent> for CreateRoleResponseContent {
    fn from(value: super::CreateRoleResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateTenantParams {
    description: Result<Option<String>, String>,
  }

  impl Default for CreateTenantParams {
    fn default() -> Self {
      Self {
        description: Ok(Default::default()),
      }
    }
  }

  impl CreateTenantParams {
    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateTenantParams> for super::CreateTenantParams {
    type Error = String;

    fn try_from(value: CreateTenantParams) -> Result<Self, String> {
      Ok(Self {
        description: value.description?,
      })
    }
  }

  impl From<super::CreateTenantParams> for CreateTenantParams {
    fn from(value: super::CreateTenantParams) -> Self {
      Self {
        description: Ok(value.description),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateTenantResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateTenantResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateTenantResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateTenantResponseContent> for super::CreateTenantResponseContent {
    type Error = String;

    fn try_from(value: CreateTenantResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateTenantResponseContent> for CreateTenantResponseContent {
    fn from(value: super::CreateTenantResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateUserResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateUserResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateUserResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateUserResponseContent> for super::CreateUserResponseContent {
    type Error = String;

    fn try_from(value: CreateUserResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateUserResponseContent> for CreateUserResponseContent {
    fn from(value: super::CreateUserResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
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
  pub struct DeleteActionGroupResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteActionGroupResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteActionGroupResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteActionGroupResponseContent> for super::DeleteActionGroupResponseContent {
    type Error = String;

    fn try_from(value: DeleteActionGroupResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteActionGroupResponseContent> for DeleteActionGroupResponseContent {
    fn from(value: super::DeleteActionGroupResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
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
  pub struct DeleteDistinguishedNamesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteDistinguishedNamesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteDistinguishedNamesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteDistinguishedNamesResponseContent> for super::DeleteDistinguishedNamesResponseContent {
    type Error = String;

    fn try_from(value: DeleteDistinguishedNamesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteDistinguishedNamesResponseContent> for DeleteDistinguishedNamesResponseContent {
    fn from(value: super::DeleteDistinguishedNamesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
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
  pub struct DeleteRoleMappingResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteRoleMappingResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteRoleMappingResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteRoleMappingResponseContent> for super::DeleteRoleMappingResponseContent {
    type Error = String;

    fn try_from(value: DeleteRoleMappingResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteRoleMappingResponseContent> for DeleteRoleMappingResponseContent {
    fn from(value: super::DeleteRoleMappingResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteRoleResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteRoleResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteRoleResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteRoleResponseContent> for super::DeleteRoleResponseContent {
    type Error = String;

    fn try_from(value: DeleteRoleResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteRoleResponseContent> for DeleteRoleResponseContent {
    fn from(value: super::DeleteRoleResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteTenantResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteTenantResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteTenantResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteTenantResponseContent> for super::DeleteTenantResponseContent {
    type Error = String;

    fn try_from(value: DeleteTenantResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteTenantResponseContent> for DeleteTenantResponseContent {
    fn from(value: super::DeleteTenantResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteUserResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteUserResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteUserResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteUserResponseContent> for super::DeleteUserResponseContent {
    type Error = String;

    fn try_from(value: DeleteUserResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteUserResponseContent> for DeleteUserResponseContent {
    fn from(value: super::DeleteUserResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
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
  pub struct DistinguishedNames {
    nodes_dn: Result<Vec<String>, String>,
  }

  impl Default for DistinguishedNames {
    fn default() -> Self {
      Self {
        nodes_dn: Ok(Default::default()),
      }
    }
  }

  impl DistinguishedNames {
    pub fn nodes_dn<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.nodes_dn = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for nodes_dn: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DistinguishedNames> for super::DistinguishedNames {
    type Error = String;

    fn try_from(value: DistinguishedNames) -> Result<Self, String> {
      Ok(Self {
        nodes_dn: value.nodes_dn?,
      })
    }
  }

  impl From<super::DistinguishedNames> for DistinguishedNames {
    fn from(value: super::DistinguishedNames) -> Self {
      Self {
        nodes_dn: Ok(value.nodes_dn),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DynamicConfig {
    dynamic: Result<Option<super::DynamicOptions>, String>,
  }

  impl Default for DynamicConfig {
    fn default() -> Self {
      Self {
        dynamic: Ok(Default::default()),
      }
    }
  }

  impl DynamicConfig {
    pub fn dynamic<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::DynamicOptions>>,
      T::Error: std::fmt::Display, {
      self.dynamic = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for dynamic: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DynamicConfig> for super::DynamicConfig {
    type Error = String;

    fn try_from(value: DynamicConfig) -> Result<Self, String> {
      Ok(Self {
        dynamic: value.dynamic?,
      })
    }
  }

  impl From<super::DynamicConfig> for DynamicConfig {
    fn from(value: super::DynamicConfig) -> Self {
      Self {
        dynamic: Ok(value.dynamic),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DynamicOptions {
    auth_failure_listeners: Result<Option<serde_json::Value>, String>,
    authc: Result<Option<serde_json::Value>, String>,
    authz: Result<Option<serde_json::Value>, String>,
    disable_intertransport_auth: Result<Option<bool>, String>,
    disable_rest_auth: Result<Option<bool>, String>,
    do_not_fail_on_forbidden: Result<Option<bool>, String>,
    do_not_fail_on_forbidden_empty: Result<Option<bool>, String>,
    filtered_alias_mode: Result<Option<String>, String>,
    hosts_resolver_mode: Result<Option<String>, String>,
    http: Result<Option<serde_json::Value>, String>,
    kibana: Result<Option<serde_json::Value>, String>,
    multi_rolespan_enabled: Result<Option<bool>, String>,
    respect_request_indices_options: Result<Option<bool>, String>,
  }

  impl Default for DynamicOptions {
    fn default() -> Self {
      Self {
        auth_failure_listeners: Ok(Default::default()),
        authc: Ok(Default::default()),
        authz: Ok(Default::default()),
        disable_intertransport_auth: Ok(Default::default()),
        disable_rest_auth: Ok(Default::default()),
        do_not_fail_on_forbidden: Ok(Default::default()),
        do_not_fail_on_forbidden_empty: Ok(Default::default()),
        filtered_alias_mode: Ok(Default::default()),
        hosts_resolver_mode: Ok(Default::default()),
        http: Ok(Default::default()),
        kibana: Ok(Default::default()),
        multi_rolespan_enabled: Ok(Default::default()),
        respect_request_indices_options: Ok(Default::default()),
      }
    }
  }

  impl DynamicOptions {
    pub fn auth_failure_listeners<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.auth_failure_listeners = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for auth_failure_listeners: {}", e));
      self
    }

    pub fn authc<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.authc = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for authc: {}", e));
      self
    }

    pub fn authz<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.authz = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for authz: {}", e));
      self
    }

    pub fn disable_intertransport_auth<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.disable_intertransport_auth = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for disable_intertransport_auth: {}", e));
      self
    }

    pub fn disable_rest_auth<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.disable_rest_auth = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for disable_rest_auth: {}", e));
      self
    }

    pub fn do_not_fail_on_forbidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.do_not_fail_on_forbidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for do_not_fail_on_forbidden: {}", e));
      self
    }

    pub fn do_not_fail_on_forbidden_empty<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.do_not_fail_on_forbidden_empty = value.try_into().map_err(|e| {
        format!(
          "error converting supplied value for do_not_fail_on_forbidden_empty: {}",
          e
        )
      });
      self
    }

    pub fn filtered_alias_mode<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.filtered_alias_mode = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for filtered_alias_mode: {}", e));
      self
    }

    pub fn hosts_resolver_mode<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.hosts_resolver_mode = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hosts_resolver_mode: {}", e));
      self
    }

    pub fn http<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.http = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for http: {}", e));
      self
    }

    pub fn kibana<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.kibana = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for kibana: {}", e));
      self
    }

    pub fn multi_rolespan_enabled<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.multi_rolespan_enabled = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for multi_rolespan_enabled: {}", e));
      self
    }

    pub fn respect_request_indices_options<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.respect_request_indices_options = value.try_into().map_err(|e| {
        format!(
          "error converting supplied value for respect_request_indices_options: {}",
          e
        )
      });
      self
    }
  }

  impl std::convert::TryFrom<DynamicOptions> for super::DynamicOptions {
    type Error = String;

    fn try_from(value: DynamicOptions) -> Result<Self, String> {
      Ok(Self {
        auth_failure_listeners: value.auth_failure_listeners?,
        authc: value.authc?,
        authz: value.authz?,
        disable_intertransport_auth: value.disable_intertransport_auth?,
        disable_rest_auth: value.disable_rest_auth?,
        do_not_fail_on_forbidden: value.do_not_fail_on_forbidden?,
        do_not_fail_on_forbidden_empty: value.do_not_fail_on_forbidden_empty?,
        filtered_alias_mode: value.filtered_alias_mode?,
        hosts_resolver_mode: value.hosts_resolver_mode?,
        http: value.http?,
        kibana: value.kibana?,
        multi_rolespan_enabled: value.multi_rolespan_enabled?,
        respect_request_indices_options: value.respect_request_indices_options?,
      })
    }
  }

  impl From<super::DynamicOptions> for DynamicOptions {
    fn from(value: super::DynamicOptions) -> Self {
      Self {
        auth_failure_listeners: Ok(value.auth_failure_listeners),
        authc: Ok(value.authc),
        authz: Ok(value.authz),
        disable_intertransport_auth: Ok(value.disable_intertransport_auth),
        disable_rest_auth: Ok(value.disable_rest_auth),
        do_not_fail_on_forbidden: Ok(value.do_not_fail_on_forbidden),
        do_not_fail_on_forbidden_empty: Ok(value.do_not_fail_on_forbidden_empty),
        filtered_alias_mode: Ok(value.filtered_alias_mode),
        hosts_resolver_mode: Ok(value.hosts_resolver_mode),
        http: Ok(value.http),
        kibana: Ok(value.kibana),
        multi_rolespan_enabled: Ok(value.multi_rolespan_enabled),
        respect_request_indices_options: Ok(value.respect_request_indices_options),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct FlushCacheResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for FlushCacheResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl FlushCacheResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<FlushCacheResponseContent> for super::FlushCacheResponseContent {
    type Error = String;

    fn try_from(value: FlushCacheResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::FlushCacheResponseContent> for FlushCacheResponseContent {
    fn from(value: super::FlushCacheResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
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
  pub struct GetCertificatesResponseContent {
    http_certificates_list: Result<Vec<super::CertificatesDetail>, String>,
    transport_certificates_list: Result<Vec<super::CertificatesDetail>, String>,
  }

  impl Default for GetCertificatesResponseContent {
    fn default() -> Self {
      Self {
        http_certificates_list: Ok(Default::default()),
        transport_certificates_list: Ok(Default::default()),
      }
    }
  }

  impl GetCertificatesResponseContent {
    pub fn http_certificates_list<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::CertificatesDetail>>,
      T::Error: std::fmt::Display, {
      self.http_certificates_list = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for http_certificates_list: {}", e));
      self
    }

    pub fn transport_certificates_list<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::CertificatesDetail>>,
      T::Error: std::fmt::Display, {
      self.transport_certificates_list = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for transport_certificates_list: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<GetCertificatesResponseContent> for super::GetCertificatesResponseContent {
    type Error = String;

    fn try_from(value: GetCertificatesResponseContent) -> Result<Self, String> {
      Ok(Self {
        http_certificates_list: value.http_certificates_list?,
        transport_certificates_list: value.transport_certificates_list?,
      })
    }
  }

  impl From<super::GetCertificatesResponseContent> for GetCertificatesResponseContent {
    fn from(value: super::GetCertificatesResponseContent) -> Self {
      Self {
        http_certificates_list: Ok(value.http_certificates_list),
        transport_certificates_list: Ok(value.transport_certificates_list),
      }
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
    id: Result<Option<String>, String>,
    index: Result<Option<String>, String>,
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
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for id: {}", e));
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
  pub struct IndexPermission {
    allowed_actions: Result<Vec<String>, String>,
    fls: Result<Vec<String>, String>,
    index_patterns: Result<Vec<String>, String>,
    masked_fields: Result<Vec<String>, String>,
  }

  impl Default for IndexPermission {
    fn default() -> Self {
      Self {
        allowed_actions: Ok(Default::default()),
        fls: Ok(Default::default()),
        index_patterns: Ok(Default::default()),
        masked_fields: Ok(Default::default()),
      }
    }
  }

  impl IndexPermission {
    pub fn allowed_actions<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.allowed_actions = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for allowed_actions: {}", e));
      self
    }

    pub fn fls<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.fls = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for fls: {}", e));
      self
    }

    pub fn index_patterns<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.index_patterns = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index_patterns: {}", e));
      self
    }

    pub fn masked_fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.masked_fields = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for masked_fields: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndexPermission> for super::IndexPermission {
    type Error = String;

    fn try_from(value: IndexPermission) -> Result<Self, String> {
      Ok(Self {
        allowed_actions: value.allowed_actions?,
        fls: value.fls?,
        index_patterns: value.index_patterns?,
        masked_fields: value.masked_fields?,
      })
    }
  }

  impl From<super::IndexPermission> for IndexPermission {
    fn from(value: super::IndexPermission) -> Self {
      Self {
        allowed_actions: Ok(value.allowed_actions),
        fls: Ok(value.fls),
        index_patterns: Ok(value.index_patterns),
        masked_fields: Ok(value.masked_fields),
      }
    }
  }

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
  pub struct PatchActionGroupResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchActionGroupResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchActionGroupResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchActionGroupResponseContent> for super::PatchActionGroupResponseContent {
    type Error = String;

    fn try_from(value: PatchActionGroupResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchActionGroupResponseContent> for PatchActionGroupResponseContent {
    fn from(value: super::PatchActionGroupResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchActionGroupsResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchActionGroupsResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchActionGroupsResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchActionGroupsResponseContent> for super::PatchActionGroupsResponseContent {
    type Error = String;

    fn try_from(value: PatchActionGroupsResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchActionGroupsResponseContent> for PatchActionGroupsResponseContent {
    fn from(value: super::PatchActionGroupsResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchConfigurationResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchConfigurationResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchConfigurationResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchConfigurationResponseContent> for super::PatchConfigurationResponseContent {
    type Error = String;

    fn try_from(value: PatchConfigurationResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchConfigurationResponseContent> for PatchConfigurationResponseContent {
    fn from(value: super::PatchConfigurationResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchDistinguishedNamesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchDistinguishedNamesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchDistinguishedNamesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchDistinguishedNamesResponseContent> for super::PatchDistinguishedNamesResponseContent {
    type Error = String;

    fn try_from(value: PatchDistinguishedNamesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchDistinguishedNamesResponseContent> for PatchDistinguishedNamesResponseContent {
    fn from(value: super::PatchDistinguishedNamesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchOperation {
    op: Result<String, String>,
    path: Result<String, String>,
    value: Result<Option<serde_json::Value>, String>,
  }

  impl Default for PatchOperation {
    fn default() -> Self {
      Self {
        op: Err("no value supplied for op".to_string()),
        path: Err("no value supplied for path".to_string()),
        value: Ok(Default::default()),
      }
    }
  }

  impl PatchOperation {
    pub fn op<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.op = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for op: {}", e));
      self
    }

    pub fn path<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.path = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for path: {}", e));
      self
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.value = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for value: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchOperation> for super::PatchOperation {
    type Error = String;

    fn try_from(value: PatchOperation) -> Result<Self, String> {
      Ok(Self {
        op: value.op?,
        path: value.path?,
        value: value.value?,
      })
    }
  }

  impl From<super::PatchOperation> for PatchOperation {
    fn from(value: super::PatchOperation) -> Self {
      Self {
        op: Ok(value.op),
        path: Ok(value.path),
        value: Ok(value.value),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchRoleMappingResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchRoleMappingResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchRoleMappingResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchRoleMappingResponseContent> for super::PatchRoleMappingResponseContent {
    type Error = String;

    fn try_from(value: PatchRoleMappingResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchRoleMappingResponseContent> for PatchRoleMappingResponseContent {
    fn from(value: super::PatchRoleMappingResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchRoleMappingsResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchRoleMappingsResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchRoleMappingsResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchRoleMappingsResponseContent> for super::PatchRoleMappingsResponseContent {
    type Error = String;

    fn try_from(value: PatchRoleMappingsResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchRoleMappingsResponseContent> for PatchRoleMappingsResponseContent {
    fn from(value: super::PatchRoleMappingsResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchRoleResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchRoleResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchRoleResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchRoleResponseContent> for super::PatchRoleResponseContent {
    type Error = String;

    fn try_from(value: PatchRoleResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchRoleResponseContent> for PatchRoleResponseContent {
    fn from(value: super::PatchRoleResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchRolesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchRolesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchRolesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchRolesResponseContent> for super::PatchRolesResponseContent {
    type Error = String;

    fn try_from(value: PatchRolesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchRolesResponseContent> for PatchRolesResponseContent {
    fn from(value: super::PatchRolesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchTenantResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchTenantResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchTenantResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchTenantResponseContent> for super::PatchTenantResponseContent {
    type Error = String;

    fn try_from(value: PatchTenantResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchTenantResponseContent> for PatchTenantResponseContent {
    fn from(value: super::PatchTenantResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchTenantsResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchTenantsResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchTenantsResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchTenantsResponseContent> for super::PatchTenantsResponseContent {
    type Error = String;

    fn try_from(value: PatchTenantsResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchTenantsResponseContent> for PatchTenantsResponseContent {
    fn from(value: super::PatchTenantsResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchUserResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchUserResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchUserResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchUserResponseContent> for super::PatchUserResponseContent {
    type Error = String;

    fn try_from(value: PatchUserResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchUserResponseContent> for PatchUserResponseContent {
    fn from(value: super::PatchUserResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchUsersResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchUsersResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchUsersResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchUsersResponseContent> for super::PatchUsersResponseContent {
    type Error = String;

    fn try_from(value: PatchUsersResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchUsersResponseContent> for PatchUsersResponseContent {
    fn from(value: super::PatchUsersResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
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

  #[derive(Clone, Debug)]
  pub struct ReloadHttpCertificatesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for ReloadHttpCertificatesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl ReloadHttpCertificatesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ReloadHttpCertificatesResponseContent> for super::ReloadHttpCertificatesResponseContent {
    type Error = String;

    fn try_from(value: ReloadHttpCertificatesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::ReloadHttpCertificatesResponseContent> for ReloadHttpCertificatesResponseContent {
    fn from(value: super::ReloadHttpCertificatesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ReloadTransportCertificatesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for ReloadTransportCertificatesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl ReloadTransportCertificatesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ReloadTransportCertificatesResponseContent>
    for super::ReloadTransportCertificatesResponseContent
  {
    type Error = String;

    fn try_from(value: ReloadTransportCertificatesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::ReloadTransportCertificatesResponseContent> for ReloadTransportCertificatesResponseContent {
    fn from(value: super::ReloadTransportCertificatesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct RemoteStoreRestoreBodyParams {
    indices: Result<Vec<String>, String>,
  }

  impl Default for RemoteStoreRestoreBodyParams {
    fn default() -> Self {
      Self {
        indices: Err("no value supplied for indices".to_string()),
      }
    }
  }

  impl RemoteStoreRestoreBodyParams {
    pub fn indices<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.indices = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for indices: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<RemoteStoreRestoreBodyParams> for super::RemoteStoreRestoreBodyParams {
    type Error = String;

    fn try_from(value: RemoteStoreRestoreBodyParams) -> Result<Self, String> {
      Ok(Self {
        indices: value.indices?,
      })
    }
  }

  impl From<super::RemoteStoreRestoreBodyParams> for RemoteStoreRestoreBodyParams {
    fn from(value: super::RemoteStoreRestoreBodyParams) -> Self {
      Self {
        indices: Ok(value.indices),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct RemoteStoreRestoreInfo {
    indices: Result<Vec<String>, String>,
    shards: Result<Option<super::RemoteStoreRestoreShardsInfo>, String>,
    snapshot: Result<Option<String>, String>,
  }

  impl Default for RemoteStoreRestoreInfo {
    fn default() -> Self {
      Self {
        indices: Ok(Default::default()),
        shards: Ok(Default::default()),
        snapshot: Ok(Default::default()),
      }
    }
  }

  impl RemoteStoreRestoreInfo {
    pub fn indices<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.indices = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for indices: {}", e));
      self
    }

    pub fn shards<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::RemoteStoreRestoreShardsInfo>>,
      T::Error: std::fmt::Display, {
      self.shards = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for shards: {}", e));
      self
    }

    pub fn snapshot<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.snapshot = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for snapshot: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<RemoteStoreRestoreInfo> for super::RemoteStoreRestoreInfo {
    type Error = String;

    fn try_from(value: RemoteStoreRestoreInfo) -> Result<Self, String> {
      Ok(Self {
        indices: value.indices?,
        shards: value.shards?,
        snapshot: value.snapshot?,
      })
    }
  }

  impl From<super::RemoteStoreRestoreInfo> for RemoteStoreRestoreInfo {
    fn from(value: super::RemoteStoreRestoreInfo) -> Self {
      Self {
        indices: Ok(value.indices),
        shards: Ok(value.shards),
        snapshot: Ok(value.snapshot),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct RemoteStoreRestoreResponseContent {
    accepted: Result<Option<bool>, String>,
    remote_store: Result<Option<super::RemoteStoreRestoreInfo>, String>,
  }

  impl Default for RemoteStoreRestoreResponseContent {
    fn default() -> Self {
      Self {
        accepted: Ok(Default::default()),
        remote_store: Ok(Default::default()),
      }
    }
  }

  impl RemoteStoreRestoreResponseContent {
    pub fn accepted<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.accepted = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for accepted: {}", e));
      self
    }

    pub fn remote_store<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::RemoteStoreRestoreInfo>>,
      T::Error: std::fmt::Display, {
      self.remote_store = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for remote_store: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<RemoteStoreRestoreResponseContent> for super::RemoteStoreRestoreResponseContent {
    type Error = String;

    fn try_from(value: RemoteStoreRestoreResponseContent) -> Result<Self, String> {
      Ok(Self {
        accepted: value.accepted?,
        remote_store: value.remote_store?,
      })
    }
  }

  impl From<super::RemoteStoreRestoreResponseContent> for RemoteStoreRestoreResponseContent {
    fn from(value: super::RemoteStoreRestoreResponseContent) -> Self {
      Self {
        accepted: Ok(value.accepted),
        remote_store: Ok(value.remote_store),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct RemoteStoreRestoreShardsInfo {
    failed: Result<Option<i32>, String>,
    successful: Result<Option<i32>, String>,
    total: Result<Option<i32>, String>,
  }

  impl Default for RemoteStoreRestoreShardsInfo {
    fn default() -> Self {
      Self {
        failed: Ok(Default::default()),
        successful: Ok(Default::default()),
        total: Ok(Default::default()),
      }
    }
  }

  impl RemoteStoreRestoreShardsInfo {
    pub fn failed<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.failed = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for failed: {}", e));
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

  impl std::convert::TryFrom<RemoteStoreRestoreShardsInfo> for super::RemoteStoreRestoreShardsInfo {
    type Error = String;

    fn try_from(value: RemoteStoreRestoreShardsInfo) -> Result<Self, String> {
      Ok(Self {
        failed: value.failed?,
        successful: value.successful?,
        total: value.total?,
      })
    }
  }

  impl From<super::RemoteStoreRestoreShardsInfo> for RemoteStoreRestoreShardsInfo {
    fn from(value: super::RemoteStoreRestoreShardsInfo) -> Self {
      Self {
        failed: Ok(value.failed),
        successful: Ok(value.successful),
        total: Ok(value.total),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct Role {
    cluster_permission: Result<Vec<String>, String>,
    description: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    index_permission: Result<Option<super::IndexPermission>, String>,
    reserved: Result<Option<bool>, String>,
    static_: Result<Option<bool>, String>,
    tenant_permissions: Result<Vec<String>, String>,
  }

  impl Default for Role {
    fn default() -> Self {
      Self {
        cluster_permission: Ok(Default::default()),
        description: Ok(Default::default()),
        hidden: Ok(Default::default()),
        index_permission: Ok(Default::default()),
        reserved: Ok(Default::default()),
        static_: Ok(Default::default()),
        tenant_permissions: Ok(Default::default()),
      }
    }
  }

  impl Role {
    pub fn cluster_permission<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.cluster_permission = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for cluster_permission: {}", e));
      self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn index_permission<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::IndexPermission>>,
      T::Error: std::fmt::Display, {
      self.index_permission = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index_permission: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn static_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.static_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for static_: {}", e));
      self
    }

    pub fn tenant_permissions<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.tenant_permissions = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for tenant_permissions: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<Role> for super::Role {
    type Error = String;

    fn try_from(value: Role) -> Result<Self, String> {
      Ok(Self {
        cluster_permission: value.cluster_permission?,
        description: value.description?,
        hidden: value.hidden?,
        index_permission: value.index_permission?,
        reserved: value.reserved?,
        static_: value.static_?,
        tenant_permissions: value.tenant_permissions?,
      })
    }
  }

  impl From<super::Role> for Role {
    fn from(value: super::Role) -> Self {
      Self {
        cluster_permission: Ok(value.cluster_permission),
        description: Ok(value.description),
        hidden: Ok(value.hidden),
        index_permission: Ok(value.index_permission),
        reserved: Ok(value.reserved),
        static_: Ok(value.static_),
        tenant_permissions: Ok(value.tenant_permissions),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct RoleMapping {
    and_backend_roles: Result<Vec<String>, String>,
    backend_roles: Result<Vec<String>, String>,
    description: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    hosts: Result<Vec<String>, String>,
    reserved: Result<Option<bool>, String>,
    users: Result<Vec<String>, String>,
  }

  impl Default for RoleMapping {
    fn default() -> Self {
      Self {
        and_backend_roles: Ok(Default::default()),
        backend_roles: Ok(Default::default()),
        description: Ok(Default::default()),
        hidden: Ok(Default::default()),
        hosts: Ok(Default::default()),
        reserved: Ok(Default::default()),
        users: Ok(Default::default()),
      }
    }
  }

  impl RoleMapping {
    pub fn and_backend_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.and_backend_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for and_backend_roles: {}", e));
      self
    }

    pub fn backend_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.backend_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for backend_roles: {}", e));
      self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn hosts<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.hosts = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hosts: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn users<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.users = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for users: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<RoleMapping> for super::RoleMapping {
    type Error = String;

    fn try_from(value: RoleMapping) -> Result<Self, String> {
      Ok(Self {
        and_backend_roles: value.and_backend_roles?,
        backend_roles: value.backend_roles?,
        description: value.description?,
        hidden: value.hidden?,
        hosts: value.hosts?,
        reserved: value.reserved?,
        users: value.users?,
      })
    }
  }

  impl From<super::RoleMapping> for RoleMapping {
    fn from(value: super::RoleMapping) -> Self {
      Self {
        and_backend_roles: Ok(value.and_backend_roles),
        backend_roles: Ok(value.backend_roles),
        description: Ok(value.description),
        hidden: Ok(value.hidden),
        hosts: Ok(value.hosts),
        reserved: Ok(value.reserved),
        users: Ok(value.users),
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
  pub struct SearchPostWithIndexResponseContent<T> {
    hits: Result<Option<super::HitsMetadata<T>>, String>,
    scroll_id: Result<Option<String>, String>,
    shards: Result<Option<super::ShardStatistics>, String>,
    timed_out: Result<Option<bool>, String>,
    took: Result<Option<i64>, String>,
  }

  impl<T> Default for SearchPostWithIndexResponseContent<T> {
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

  impl<T2> SearchPostWithIndexResponseContent<T2> {
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

  impl<T> std::convert::TryFrom<SearchPostWithIndexResponseContent<T>> for super::SearchPostWithIndexResponseContent<T> {
    type Error = String;

    fn try_from(value: SearchPostWithIndexResponseContent<T>) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        scroll_id: value.scroll_id?,
        shards: value.shards?,
        timed_out: value.timed_out?,
        took: value.took?,
      })
    }
  }

  impl<T> From<super::SearchPostWithIndexResponseContent<T>> for SearchPostWithIndexResponseContent<T> {
    fn from(value: super::SearchPostWithIndexResponseContent<T>) -> Self {
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
  pub struct SecurityHealthResponseContent {
    message: Result<Option<String>, String>,
    mode: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for SecurityHealthResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        mode: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl SecurityHealthResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn mode<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.mode = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for mode: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<SecurityHealthResponseContent> for super::SecurityHealthResponseContent {
    type Error = String;

    fn try_from(value: SecurityHealthResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        mode: value.mode?,
        status: value.status?,
      })
    }
  }

  impl From<super::SecurityHealthResponseContent> for SecurityHealthResponseContent {
    fn from(value: super::SecurityHealthResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        mode: Ok(value.mode),
        status: Ok(value.status),
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
  pub struct Tenant {
    description: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    reserved: Result<Option<bool>, String>,
    static_: Result<Option<bool>, String>,
  }

  impl Default for Tenant {
    fn default() -> Self {
      Self {
        description: Ok(Default::default()),
        hidden: Ok(Default::default()),
        reserved: Ok(Default::default()),
        static_: Ok(Default::default()),
      }
    }
  }

  impl Tenant {
    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn static_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.static_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for static_: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<Tenant> for super::Tenant {
    type Error = String;

    fn try_from(value: Tenant) -> Result<Self, String> {
      Ok(Self {
        description: value.description?,
        hidden: value.hidden?,
        reserved: value.reserved?,
        static_: value.static_?,
      })
    }
  }

  impl From<super::Tenant> for Tenant {
    fn from(value: super::Tenant) -> Self {
      Self {
        description: Ok(value.description),
        hidden: Ok(value.hidden),
        reserved: Ok(value.reserved),
        static_: Ok(value.static_),
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
  pub struct UpdateAuditConfigurationResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for UpdateAuditConfigurationResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl UpdateAuditConfigurationResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UpdateAuditConfigurationResponseContent> for super::UpdateAuditConfigurationResponseContent {
    type Error = String;

    fn try_from(value: UpdateAuditConfigurationResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::UpdateAuditConfigurationResponseContent> for UpdateAuditConfigurationResponseContent {
    fn from(value: super::UpdateAuditConfigurationResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct UpdateConfigurationResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for UpdateConfigurationResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl UpdateConfigurationResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UpdateConfigurationResponseContent> for super::UpdateConfigurationResponseContent {
    type Error = String;

    fn try_from(value: UpdateConfigurationResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::UpdateConfigurationResponseContent> for UpdateConfigurationResponseContent {
    fn from(value: super::UpdateConfigurationResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct UpdateDistinguishedNamesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for UpdateDistinguishedNamesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl UpdateDistinguishedNamesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UpdateDistinguishedNamesResponseContent> for super::UpdateDistinguishedNamesResponseContent {
    type Error = String;

    fn try_from(value: UpdateDistinguishedNamesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::UpdateDistinguishedNamesResponseContent> for UpdateDistinguishedNamesResponseContent {
    fn from(value: super::UpdateDistinguishedNamesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct User {
    attributes: Result<Option<super::UserAttributes>, String>,
    backend_roles: Result<Vec<String>, String>,
    description: Result<Option<String>, String>,
    hash: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    opendistro_security_roles: Result<Vec<String>, String>,
    reserved: Result<Option<bool>, String>,
    static_: Result<Option<bool>, String>,
  }

  impl Default for User {
    fn default() -> Self {
      Self {
        attributes: Ok(Default::default()),
        backend_roles: Ok(Default::default()),
        description: Ok(Default::default()),
        hash: Ok(Default::default()),
        hidden: Ok(Default::default()),
        opendistro_security_roles: Ok(Default::default()),
        reserved: Ok(Default::default()),
        static_: Ok(Default::default()),
      }
    }
  }

  impl User {
    pub fn attributes<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserAttributes>>,
      T::Error: std::fmt::Display, {
      self.attributes = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for attributes: {}", e));
      self
    }

    pub fn backend_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.backend_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for backend_roles: {}", e));
      self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hash<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.hash = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hash: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn opendistro_security_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.opendistro_security_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for opendistro_security_roles: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn static_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.static_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for static_: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<User> for super::User {
    type Error = String;

    fn try_from(value: User) -> Result<Self, String> {
      Ok(Self {
        attributes: value.attributes?,
        backend_roles: value.backend_roles?,
        description: value.description?,
        hash: value.hash?,
        hidden: value.hidden?,
        opendistro_security_roles: value.opendistro_security_roles?,
        reserved: value.reserved?,
        static_: value.static_?,
      })
    }
  }

  impl From<super::User> for User {
    fn from(value: super::User) -> Self {
      Self {
        attributes: Ok(value.attributes),
        backend_roles: Ok(value.backend_roles),
        description: Ok(value.description),
        hash: Ok(value.hash),
        hidden: Ok(value.hidden),
        opendistro_security_roles: Ok(value.opendistro_security_roles),
        reserved: Ok(value.reserved),
        static_: Ok(value.static_),
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

  #[derive(Clone, Debug)]
  pub struct UserTenants {
    admin: Result<Option<bool>, String>,
    admin_tenant: Result<Option<bool>, String>,
    global_tenant: Result<Option<bool>, String>,
  }

  impl Default for UserTenants {
    fn default() -> Self {
      Self {
        admin: Ok(Default::default()),
        admin_tenant: Ok(Default::default()),
        global_tenant: Ok(Default::default()),
      }
    }
  }

  impl UserTenants {
    pub fn admin<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.admin = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for admin: {}", e));
      self
    }

    pub fn admin_tenant<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.admin_tenant = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for admin_tenant: {}", e));
      self
    }

    pub fn global_tenant<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.global_tenant = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for global_tenant: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UserTenants> for super::UserTenants {
    type Error = String;

    fn try_from(value: UserTenants) -> Result<Self, String> {
      Ok(Self {
        admin: value.admin?,
        admin_tenant: value.admin_tenant?,
        global_tenant: value.global_tenant?,
      })
    }
  }

  impl From<super::UserTenants> for UserTenants {
    fn from(value: super::UserTenants) -> Self {
      Self {
        admin: Ok(value.admin),
        admin_tenant: Ok(value.admin_tenant),
        global_tenant: Ok(value.global_tenant),
      }
    }
  }
}
