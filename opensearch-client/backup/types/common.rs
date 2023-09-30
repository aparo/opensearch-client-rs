
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use super::{user::UserTenants, UserDefinedStructure};

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStatsWithNodeIdTimeout(String);
impl std::ops::Deref for ClusterStatsWithNodeIdTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStatsWithNodeIdTimeout> for String {
  fn from(value: ClusterStatsWithNodeIdTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStatsWithNodeIdTimeout> for ClusterStatsWithNodeIdTimeout {
  fn from(value: &ClusterStatsWithNodeIdTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStatsWithNodeIdTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStatsWithNodeIdTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStatsWithNodeIdTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStatsWithNodeIdTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStatsWithNodeIdTimeout {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for CreateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<CreateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: CreateBodyParams) -> Self {
    value.0
  }
}

impl From<&CreateBodyParams> for CreateBodyParams {
  fn from(value: &CreateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for CreateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreatePostTimeout(String);
impl std::ops::Deref for CreatePostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CreatePostTimeout> for String {
  fn from(value: CreatePostTimeout) -> Self {
    value.0
  }
}

impl From<&CreatePostTimeout> for CreatePostTimeout {
  fn from(value: &CreatePostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CreatePostTimeout {
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

impl std::convert::TryFrom<&str> for CreatePostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CreatePostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CreatePostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CreatePostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreatePutTimeout(String);
impl std::ops::Deref for CreatePutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<CreatePutTimeout> for String {
  fn from(value: CreatePutTimeout) -> Self {
    value.0
  }
}

impl From<&CreatePutTimeout> for CreatePutTimeout {
  fn from(value: &CreatePutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for CreatePutTimeout {
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

impl std::convert::TryFrom<&str> for CreatePutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for CreatePutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for CreatePutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for CreatePutTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesImportDanglingIndexTimeout(String);
impl std::ops::Deref for DanglingIndicesImportDanglingIndexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesImportDanglingIndexTimeout> for String {
  fn from(value: DanglingIndicesImportDanglingIndexTimeout) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesImportDanglingIndexTimeout> for DanglingIndicesImportDanglingIndexTimeout {
  fn from(value: &DanglingIndicesImportDanglingIndexTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesImportDanglingIndexTimeout {
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

impl std::convert::TryFrom<&str> for DanglingIndicesImportDanglingIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesImportDanglingIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesImportDanglingIndexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesImportDanglingIndexTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeleteTimeout(String);
impl std::ops::Deref for DeleteTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DeleteTimeout> for String {
  fn from(value: DeleteTimeout) -> Self {
    value.0
  }
}

impl From<&DeleteTimeout> for DeleteTimeout {
  fn from(value: &DeleteTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DeleteTimeout {
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

impl std::convert::TryFrom<&str> for DeleteTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DeleteTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DeleteTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DeleteTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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
pub struct GetResponseContent {
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
  pub source: Option<UserDefinedValueMap>,
  #[serde(rename = "_type", default, skip_serializing_if = "Option::is_none")]
  pub type_: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub version: Option<i32>,
}

impl From<&GetResponseContent> for GetResponseContent {
  fn from(value: &GetResponseContent) -> Self {
    value.clone()
  }
}

impl GetResponseContent {
  pub fn builder() -> builder::GetResponseContent {
    builder::GetResponseContent::default()
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GetScriptClusterManagerTimeout(String);
impl std::ops::Deref for GetScriptClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<GetScriptClusterManagerTimeout> for String {
  fn from(value: GetScriptClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&GetScriptClusterManagerTimeout> for GetScriptClusterManagerTimeout {
  fn from(value: &GetScriptClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for GetScriptClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for GetScriptClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for GetScriptClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for GetScriptClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for GetScriptClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GetScriptMasterTimeout(String);
impl std::ops::Deref for GetScriptMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<GetScriptMasterTimeout> for String {
  fn from(value: GetScriptMasterTimeout) -> Self {
    value.0
  }
}

impl From<&GetScriptMasterTimeout> for GetScriptMasterTimeout {
  fn from(value: &GetScriptMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for GetScriptMasterTimeout {
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

impl std::convert::TryFrom<&str> for GetScriptMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for GetScriptMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for GetScriptMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for GetScriptMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hits {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub fields: Option<serde_json::Value>,
  #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(rename = "_index", default, skip_serializing_if = "Option::is_none")]
  pub index: Option<String>,
  #[serde(rename = "_score", default, skip_serializing_if = "Option::is_none")]
  pub score: Option<f64>,
  #[serde(rename = "_source", default, skip_serializing_if = "Option::is_none")]
  pub source: Option<serde_json::Value>,
  #[serde(rename = "_type", default, skip_serializing_if = "Option::is_none")]
  pub type_: Option<String>,
}

impl From<&Hits> for Hits {
  fn from(value: &Hits) -> Self {
    value.clone()
  }
}

impl Hits {
  pub fn builder() -> builder::Hits {
    builder::Hits::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HitsMetadata {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub hits: Vec<Hits>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub max_score: Option<f64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub total: Option<Total>,
}

impl From<&HitsMetadata> for HitsMetadata {
  fn from(value: &HitsMetadata) -> Self {
    value.clone()
  }
}

impl HitsMetadata {
  pub fn builder() -> builder::HitsMetadata {
    builder::HitsMetadata::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndexBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IndexBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<IndexBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IndexBodyParams) -> Self {
    value.0
  }
}

impl From<&IndexBodyParams> for IndexBodyParams {
  fn from(value: &IndexBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for IndexBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexPostTimeout(String);
impl std::ops::Deref for IndexPostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexPostTimeout> for String {
  fn from(value: IndexPostTimeout) -> Self {
    value.0
  }
}

impl From<&IndexPostTimeout> for IndexPostTimeout {
  fn from(value: &IndexPostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexPostTimeout {
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

impl std::convert::TryFrom<&str> for IndexPostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexPostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexPostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexPostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexPostWithIdId(String);
impl std::ops::Deref for IndexPostWithIdId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexPostWithIdId> for String {
  fn from(value: IndexPostWithIdId) -> Self {
    value.0
  }
}

impl From<&IndexPostWithIdId> for IndexPostWithIdId {
  fn from(value: &IndexPostWithIdId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexPostWithIdId {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err(
        "doesn't match pattern \"^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$\"",
      );
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for IndexPostWithIdId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexPostWithIdId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexPostWithIdId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexPostWithIdId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexPostWithIdIndex(String);
impl std::ops::Deref for IndexPostWithIdIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexPostWithIdIndex> for String {
  fn from(value: IndexPostWithIdIndex) -> Self {
    value.0
  }
}

impl From<&IndexPostWithIdIndex> for IndexPostWithIdIndex {
  fn from(value: &IndexPostWithIdIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexPostWithIdIndex {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err(
        "doesn't match pattern \"^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$\"",
      );
    }
    Ok(Self(value.to_string()))
  }
}

impl std::convert::TryFrom<&str> for IndexPostWithIdIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexPostWithIdIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexPostWithIdIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexPostWithIdIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexPostWithIdTimeout(String);
impl std::ops::Deref for IndexPostWithIdTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexPostWithIdTimeout> for String {
  fn from(value: IndexPostWithIdTimeout) -> Self {
    value.0
  }
}

impl From<&IndexPostWithIdTimeout> for IndexPostWithIdTimeout {
  fn from(value: &IndexPostWithIdTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexPostWithIdTimeout {
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

impl std::convert::TryFrom<&str> for IndexPostWithIdTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexPostWithIdTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexPostWithIdTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexPostWithIdTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexPutWithIdTimeout(String);
impl std::ops::Deref for IndexPutWithIdTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndexPutWithIdTimeout> for String {
  fn from(value: IndexPutWithIdTimeout) -> Self {
    value.0
  }
}

impl From<&IndexPutWithIdTimeout> for IndexPutWithIdTimeout {
  fn from(value: &IndexPutWithIdTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndexPutWithIdTimeout {
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

impl std::convert::TryFrom<&str> for IndexPutWithIdTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndexPutWithIdTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndexPutWithIdTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndexPutWithIdTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

/// unavailable.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ReindexTimeout(String);
impl std::ops::Deref for ReindexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ReindexTimeout> for String {
  fn from(value: ReindexTimeout) -> Self {
    value.0
  }
}

impl From<&ReindexTimeout> for ReindexTimeout {
  fn from(value: &ReindexTimeout) -> Self {
    value.clone()
  }
}

impl Default for ReindexTimeout {
  fn default() -> Self {
    ReindexTimeout("1m".to_string())
  }
}

impl std::str::FromStr for ReindexTimeout {
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

impl std::convert::TryFrom<&str> for ReindexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ReindexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ReindexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ReindexTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RemoteStoreRestoreClusterManagerTimeout(String);
impl std::ops::Deref for RemoteStoreRestoreClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<RemoteStoreRestoreClusterManagerTimeout> for String {
  fn from(value: RemoteStoreRestoreClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&RemoteStoreRestoreClusterManagerTimeout> for RemoteStoreRestoreClusterManagerTimeout {
  fn from(value: &RemoteStoreRestoreClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for RemoteStoreRestoreClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for RemoteStoreRestoreClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for RemoteStoreRestoreClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for RemoteStoreRestoreClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for RemoteStoreRestoreClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchBodyParams {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub docvalue_fields: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub explain: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub fields: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub from: Option<i32>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub indices_boost: Vec<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub min_score: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub query: Option<UserDefinedObjectStructure>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub seq_no_primary_term: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub size: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub source: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub stats: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub terminate_after: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub timeout: Option<Time>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub version: Option<bool>,
}

impl From<&SearchBodyParams> for SearchBodyParams {
  fn from(value: &SearchBodyParams) -> Self {
    value.clone()
  }
}

impl SearchBodyParams {
  pub fn builder() -> builder::SearchBodyParams {
    builder::SearchBodyParams::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchGetResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hits: Option<HitsMetadata>,
  #[serde(rename = "_scroll_id", default, skip_serializing_if = "Option::is_none")]
  pub scroll_id: Option<String>,
  #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<ShardStatistics>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub timed_out: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub took: Option<i64>,
}

impl From<&SearchGetResponseContent> for SearchGetResponseContent {
  fn from(value: &SearchGetResponseContent) -> Self {
    value.clone()
  }
}

impl SearchGetResponseContent {
  pub fn builder() -> builder::SearchGetResponseContent {
    builder::SearchGetResponseContent::default()
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchGetTimeout(String);
impl std::ops::Deref for SearchGetTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchGetTimeout> for String {
  fn from(value: SearchGetTimeout) -> Self {
    value.0
  }
}

impl From<&SearchGetTimeout> for SearchGetTimeout {
  fn from(value: &SearchGetTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchGetTimeout {
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

impl std::convert::TryFrom<&str> for SearchGetTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchGetTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchGetTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchGetTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchGetWithIndexResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hits: Option<HitsMetadata>,
  #[serde(rename = "_scroll_id", default, skip_serializing_if = "Option::is_none")]
  pub scroll_id: Option<String>,
  #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<ShardStatistics>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub timed_out: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub took: Option<i64>,
}

impl From<&SearchGetWithIndexResponseContent> for SearchGetWithIndexResponseContent {
  fn from(value: &SearchGetWithIndexResponseContent) -> Self {
    value.clone()
  }
}

impl SearchGetWithIndexResponseContent {
  pub fn builder() -> builder::SearchGetWithIndexResponseContent {
    builder::SearchGetWithIndexResponseContent::default()
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchGetWithIndexTimeout(String);
impl std::ops::Deref for SearchGetWithIndexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchGetWithIndexTimeout> for String {
  fn from(value: SearchGetWithIndexTimeout) -> Self {
    value.0
  }
}

impl From<&SearchGetWithIndexTimeout> for SearchGetWithIndexTimeout {
  fn from(value: &SearchGetWithIndexTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchGetWithIndexTimeout {
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

impl std::convert::TryFrom<&str> for SearchGetWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchGetWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchGetWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchGetWithIndexTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchPostResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hits: Option<HitsMetadata>,
  #[serde(rename = "_scroll_id", default, skip_serializing_if = "Option::is_none")]
  pub scroll_id: Option<String>,
  #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<ShardStatistics>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub timed_out: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub took: Option<i64>,
}

impl From<&SearchPostResponseContent> for SearchPostResponseContent {
  fn from(value: &SearchPostResponseContent) -> Self {
    value.clone()
  }
}

impl SearchPostResponseContent {
  pub fn builder() -> builder::SearchPostResponseContent {
    builder::SearchPostResponseContent::default()
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchPostTimeout(String);
impl std::ops::Deref for SearchPostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchPostTimeout> for String {
  fn from(value: SearchPostTimeout) -> Self {
    value.0
  }
}

impl From<&SearchPostTimeout> for SearchPostTimeout {
  fn from(value: &SearchPostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchPostTimeout {
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

impl std::convert::TryFrom<&str> for SearchPostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchPostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchPostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchPostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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
pub struct SearchPostWithIndexResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hits: Option<HitsMetadata>,
  #[serde(rename = "_scroll_id", default, skip_serializing_if = "Option::is_none")]
  pub scroll_id: Option<String>,
  #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
  pub shards: Option<ShardStatistics>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub timed_out: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub took: Option<i64>,
}

impl From<&SearchPostWithIndexResponseContent> for SearchPostWithIndexResponseContent {
  fn from(value: &SearchPostWithIndexResponseContent) -> Self {
    value.clone()
  }
}

impl SearchPostWithIndexResponseContent {
  pub fn builder() -> builder::SearchPostWithIndexResponseContent {
    builder::SearchPostWithIndexResponseContent::default()
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchPostWithIndexTimeout(String);
impl std::ops::Deref for SearchPostWithIndexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SearchPostWithIndexTimeout> for String {
  fn from(value: SearchPostWithIndexTimeout) -> Self {
    value.0
  }
}

impl From<&SearchPostWithIndexTimeout> for SearchPostWithIndexTimeout {
  fn from(value: &SearchPostWithIndexTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SearchPostWithIndexTimeout {
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

impl std::convert::TryFrom<&str> for SearchPostWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SearchPostWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SearchPostWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SearchPostWithIndexTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TasksListTimeout(String);
impl std::ops::Deref for TasksListTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<TasksListTimeout> for String {
  fn from(value: TasksListTimeout) -> Self {
    value.0
  }
}

impl From<&TasksListTimeout> for TasksListTimeout {
  fn from(value: &TasksListTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for TasksListTimeout {
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

impl std::convert::TryFrom<&str> for TasksListTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for TasksListTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for TasksListTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for TasksListTimeout {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for UpdateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<UpdateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: UpdateBodyParams) -> Self {
    value.0
  }
}

impl From<&UpdateBodyParams> for UpdateBodyParams {
  fn from(value: &UpdateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for UpdateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UpdateByQuerySearchTimeout(String);
impl std::ops::Deref for UpdateByQuerySearchTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<UpdateByQuerySearchTimeout> for String {
  fn from(value: UpdateByQuerySearchTimeout) -> Self {
    value.0
  }
}

impl From<&UpdateByQuerySearchTimeout> for UpdateByQuerySearchTimeout {
  fn from(value: &UpdateByQuerySearchTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for UpdateByQuerySearchTimeout {
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

impl std::convert::TryFrom<&str> for UpdateByQuerySearchTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for UpdateByQuerySearchTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for UpdateByQuerySearchTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for UpdateByQuerySearchTimeout {
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
pub struct UpdateByQueryTimeout(String);
impl std::ops::Deref for UpdateByQueryTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<UpdateByQueryTimeout> for String {
  fn from(value: UpdateByQueryTimeout) -> Self {
    value.0
  }
}

impl From<&UpdateByQueryTimeout> for UpdateByQueryTimeout {
  fn from(value: &UpdateByQueryTimeout) -> Self {
    value.clone()
  }
}

impl Default for UpdateByQueryTimeout {
  fn default() -> Self {
    UpdateByQueryTimeout("1m".to_string())
  }
}

impl std::str::FromStr for UpdateByQueryTimeout {
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

impl std::convert::TryFrom<&str> for UpdateByQueryTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for UpdateByQueryTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for UpdateByQueryTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for UpdateByQueryTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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
    use crate::types::UserDefinedValueMap;

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
    defaults: Result<Option<UserDefinedValueMap>, String>,
    persistent: Result<Option<UserDefinedValueMap>, String>,
    transient: Result<Option<UserDefinedValueMap>, String>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.defaults = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for defaults: {}", e));
      self
    }

    pub fn persistent<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.persistent = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for persistent: {}", e));
      self
    }

    pub fn transient<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
    persistent: Result<Option<UserDefinedValueMap>, String>,
    transient: Result<Option<UserDefinedValueMap>, String>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.persistent = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for persistent: {}", e));
      self
    }

    pub fn transient<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
    persistent: Result<Option<UserDefinedValueMap>, String>,
    transient: Result<Option<UserDefinedValueMap>, String>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.persistent = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for persistent: {}", e));
      self
    }

    pub fn transient<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
  pub struct GetResponseContent {
    fields: Result<Option<UserDefinedValueMap>, String>,
    found: Result<bool, String>,
    id: Result<String, String>,
    index: Result<String, String>,
    primary_term: Result<Option<i64>, String>,
    routing: Result<Option<String>, String>,
    seq_no: Result<Option<i64>, String>,
    source: Result<Option<UserDefinedValueMap>, String>,
    type_: Result<Option<String>, String>,
    version: Result<Option<i32>, String>,
  }

  impl Default for GetResponseContent {
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

  impl GetResponseContent {
    pub fn fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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

    pub fn source<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
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

  impl std::convert::TryFrom<GetResponseContent> for super::GetResponseContent {
    type Error = String;

    fn try_from(value: GetResponseContent) -> Result<Self, String> {
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

  impl From<super::GetResponseContent> for GetResponseContent {
    fn from(value: super::GetResponseContent) -> Self {
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
  pub struct Hits {
    fields: Result<Option<serde_json::Value>, String>,
    id: Result<Option<String>, String>,
    index: Result<Option<String>, String>,
    score: Result<Option<f64>, String>,
    source: Result<Option<serde_json::Value>, String>,
    type_: Result<Option<String>, String>,
  }

  impl Default for Hits {
    fn default() -> Self {
      Self {
        fields: Ok(Default::default()),
        id: Ok(Default::default()),
        index: Ok(Default::default()),
        score: Ok(Default::default()),
        source: Ok(Default::default()),
        type_: Ok(Default::default()),
      }
    }
  }

  impl Hits {
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

    pub fn source<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
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
  }

  impl std::convert::TryFrom<Hits> for super::Hits {
    type Error = String;

    fn try_from(value: Hits) -> Result<Self, String> {
      Ok(Self {
        fields: value.fields?,
        id: value.id?,
        index: value.index?,
        score: value.score?,
        source: value.source?,
        type_: value.type_?,
      })
    }
  }

  impl From<super::Hits> for Hits {
    fn from(value: super::Hits) -> Self {
      Self {
        fields: Ok(value.fields),
        id: Ok(value.id),
        index: Ok(value.index),
        score: Ok(value.score),
        source: Ok(value.source),
        type_: Ok(value.type_),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct HitsMetadata {
    hits: Result<Vec<super::Hits>, String>,
    max_score: Result<Option<f64>, String>,
    total: Result<Option<super::Total>, String>,
  }

  impl Default for HitsMetadata {
    fn default() -> Self {
      Self {
        hits: Ok(Default::default()),
        max_score: Ok(Default::default()),
        total: Ok(Default::default()),
      }
    }
  }

  impl HitsMetadata {
    pub fn hits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::Hits>>,
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

  impl std::convert::TryFrom<HitsMetadata> for super::HitsMetadata {
    type Error = String;

    fn try_from(value: HitsMetadata) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        max_score: value.max_score?,
        total: value.total?,
      })
    }
  }

  impl From<super::HitsMetadata> for HitsMetadata {
    fn from(value: super::HitsMetadata) -> Self {
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
    aliases: Result<Option<UserDefinedValueMap>, String>,
    mapping: Result<Option<UserDefinedValueMap>, String>,
    settings: Result<Option<UserDefinedValueMap>, String>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.aliases = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for aliases: {}", e));
      self
    }

    pub fn mapping<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.mapping = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for mapping: {}", e));
      self
    }

    pub fn settings<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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

  #[derive(Clone, Debug)]
  pub struct SearchBodyParams {
    docvalue_fields: Result<Option<String>, String>,
    explain: Result<Option<bool>, String>,
    fields: Result<Vec<String>, String>,
    from: Result<Option<i32>, String>,
    indices_boost: Result<Vec<serde_json::Value>, String>,
    min_score: Result<Option<i32>, String>,
    query: Result<Option<super::UserDefinedObjectStructure>, String>,
    seq_no_primary_term: Result<Option<bool>, String>,
    size: Result<Option<i32>, String>,
    source: Result<Option<String>, String>,
    stats: Result<Option<String>, String>,
    terminate_after: Result<Option<i32>, String>,
    timeout: Result<Option<super::Time>, String>,
    version: Result<Option<bool>, String>,
  }

  impl Default for SearchBodyParams {
    fn default() -> Self {
      Self {
        docvalue_fields: Ok(Default::default()),
        explain: Ok(Default::default()),
        fields: Ok(Default::default()),
        from: Ok(Default::default()),
        indices_boost: Ok(Default::default()),
        min_score: Ok(Default::default()),
        query: Ok(Default::default()),
        seq_no_primary_term: Ok(Default::default()),
        size: Ok(Default::default()),
        source: Ok(Default::default()),
        stats: Ok(Default::default()),
        terminate_after: Ok(Default::default()),
        timeout: Ok(Default::default()),
        version: Ok(Default::default()),
      }
    }
  }

  impl SearchBodyParams {
    pub fn docvalue_fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.docvalue_fields = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for docvalue_fields: {}", e));
      self
    }

    pub fn explain<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.explain = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for explain: {}", e));
      self
    }

    pub fn fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.fields = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for fields: {}", e));
      self
    }

    pub fn from<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.from = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for from: {}", e));
      self
    }

    pub fn indices_boost<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.indices_boost = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for indices_boost: {}", e));
      self
    }

    pub fn min_score<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.min_score = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for min_score: {}", e));
      self
    }

    pub fn query<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserDefinedObjectStructure>>,
      T::Error: std::fmt::Display, {
      self.query = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for query: {}", e));
      self
    }

    pub fn seq_no_primary_term<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.seq_no_primary_term = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for seq_no_primary_term: {}", e));
      self
    }

    pub fn size<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.size = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for size: {}", e));
      self
    }

    pub fn source<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.source = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for source: {}", e));
      self
    }

    pub fn stats<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.stats = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for stats: {}", e));
      self
    }

    pub fn terminate_after<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.terminate_after = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for terminate_after: {}", e));
      self
    }

    pub fn timeout<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::Time>>,
      T::Error: std::fmt::Display, {
      self.timeout = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for timeout: {}", e));
      self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.version = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for version: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<SearchBodyParams> for super::SearchBodyParams {
    type Error = String;

    fn try_from(value: SearchBodyParams) -> Result<Self, String> {
      Ok(Self {
        docvalue_fields: value.docvalue_fields?,
        explain: value.explain?,
        fields: value.fields?,
        from: value.from?,
        indices_boost: value.indices_boost?,
        min_score: value.min_score?,
        query: value.query?,
        seq_no_primary_term: value.seq_no_primary_term?,
        size: value.size?,
        source: value.source?,
        stats: value.stats?,
        terminate_after: value.terminate_after?,
        timeout: value.timeout?,
        version: value.version?,
      })
    }
  }

  impl From<super::SearchBodyParams> for SearchBodyParams {
    fn from(value: super::SearchBodyParams) -> Self {
      Self {
        docvalue_fields: Ok(value.docvalue_fields),
        explain: Ok(value.explain),
        fields: Ok(value.fields),
        from: Ok(value.from),
        indices_boost: Ok(value.indices_boost),
        min_score: Ok(value.min_score),
        query: Ok(value.query),
        seq_no_primary_term: Ok(value.seq_no_primary_term),
        size: Ok(value.size),
        source: Ok(value.source),
        stats: Ok(value.stats),
        terminate_after: Ok(value.terminate_after),
        timeout: Ok(value.timeout),
        version: Ok(value.version),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct SearchGetResponseContent {
    hits: Result<Option<super::HitsMetadata>, String>,
    scroll_id: Result<Option<String>, String>,
    shards: Result<Option<super::ShardStatistics>, String>,
    timed_out: Result<Option<bool>, String>,
    took: Result<Option<i64>, String>,
  }

  impl Default for SearchGetResponseContent {
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

  impl SearchGetResponseContent {
    pub fn hits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::HitsMetadata>>,
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

  impl std::convert::TryFrom<SearchGetResponseContent> for super::SearchGetResponseContent {
    type Error = String;

    fn try_from(value: SearchGetResponseContent) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        scroll_id: value.scroll_id?,
        shards: value.shards?,
        timed_out: value.timed_out?,
        took: value.took?,
      })
    }
  }

  impl From<super::SearchGetResponseContent> for SearchGetResponseContent {
    fn from(value: super::SearchGetResponseContent) -> Self {
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
  pub struct SearchGetWithIndexResponseContent {
    hits: Result<Option<super::HitsMetadata>, String>,
    scroll_id: Result<Option<String>, String>,
    shards: Result<Option<super::ShardStatistics>, String>,
    timed_out: Result<Option<bool>, String>,
    took: Result<Option<i64>, String>,
  }

  impl Default for SearchGetWithIndexResponseContent {
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

  impl SearchGetWithIndexResponseContent {
    pub fn hits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::HitsMetadata>>,
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

  impl std::convert::TryFrom<SearchGetWithIndexResponseContent> for super::SearchGetWithIndexResponseContent {
    type Error = String;

    fn try_from(value: SearchGetWithIndexResponseContent) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        scroll_id: value.scroll_id?,
        shards: value.shards?,
        timed_out: value.timed_out?,
        took: value.took?,
      })
    }
  }

  impl From<super::SearchGetWithIndexResponseContent> for SearchGetWithIndexResponseContent {
    fn from(value: super::SearchGetWithIndexResponseContent) -> Self {
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
  pub struct SearchPostResponseContent {
    hits: Result<Option<super::HitsMetadata>, String>,
    scroll_id: Result<Option<String>, String>,
    shards: Result<Option<super::ShardStatistics>, String>,
    timed_out: Result<Option<bool>, String>,
    took: Result<Option<i64>, String>,
  }

  impl Default for SearchPostResponseContent {
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

  impl SearchPostResponseContent {
    pub fn hits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::HitsMetadata>>,
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

  impl std::convert::TryFrom<SearchPostResponseContent> for super::SearchPostResponseContent {
    type Error = String;

    fn try_from(value: SearchPostResponseContent) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        scroll_id: value.scroll_id?,
        shards: value.shards?,
        timed_out: value.timed_out?,
        took: value.took?,
      })
    }
  }

  impl From<super::SearchPostResponseContent> for SearchPostResponseContent {
    fn from(value: super::SearchPostResponseContent) -> Self {
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
  pub struct SearchPostWithIndexResponseContent {
    hits: Result<Option<super::HitsMetadata>, String>,
    scroll_id: Result<Option<String>, String>,
    shards: Result<Option<super::ShardStatistics>, String>,
    timed_out: Result<Option<bool>, String>,
    took: Result<Option<i64>, String>,
  }

  impl Default for SearchPostWithIndexResponseContent {
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

  impl SearchPostWithIndexResponseContent {
    pub fn hits<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::HitsMetadata>>,
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

  impl std::convert::TryFrom<SearchPostWithIndexResponseContent> for super::SearchPostWithIndexResponseContent {
    type Error = String;

    fn try_from(value: SearchPostWithIndexResponseContent) -> Result<Self, String> {
      Ok(Self {
        hits: value.hits?,
        scroll_id: value.scroll_id?,
        shards: value.shards?,
        timed_out: value.timed_out?,
        took: value.took?,
      })
    }
  }

  impl From<super::SearchPostWithIndexResponseContent> for SearchPostWithIndexResponseContent {
    fn from(value: super::SearchPostWithIndexResponseContent) -> Self {
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
    fuzzy: Result<Option<UserDefinedValueMap>, String>,
    geo_bounding_box: Result<Option<serde_json::Value>, String>,
    geo_distance: Result<Option<serde_json::Value>, String>,
    geo_polygon: Result<Option<serde_json::Value>, String>,
    geo_shape: Result<Option<serde_json::Value>, String>,
    has_child: Result<Option<serde_json::Value>, String>,
    has_parent: Result<Option<serde_json::Value>, String>,
    ids: Result<Option<serde_json::Value>, String>,
    intervals: Result<Option<UserDefinedValueMap>, String>,
    knn: Result<Option<serde_json::Value>, String>,
    match_: Result<Option<UserDefinedValueMap>, String>,
    match_all: Result<Option<serde_json::Value>, String>,
    match_bool_prefix: Result<Option<UserDefinedValueMap>, String>,
    match_none: Result<Option<serde_json::Value>, String>,
    match_phrase: Result<Option<UserDefinedValueMap>, String>,
    match_phrase_prefix: Result<Option<UserDefinedValueMap>, String>,
    more_like_this: Result<Option<serde_json::Value>, String>,
    multi_match: Result<Option<serde_json::Value>, String>,
    nested: Result<Option<serde_json::Value>, String>,
    parent_id: Result<Option<serde_json::Value>, String>,
    percolate: Result<Option<serde_json::Value>, String>,
    pinned: Result<Option<serde_json::Value>, String>,
    prefix: Result<Option<UserDefinedValueMap>, String>,
    query_string: Result<Option<serde_json::Value>, String>,
    range: Result<Option<UserDefinedValueMap>, String>,
    rank_feature: Result<Option<serde_json::Value>, String>,
    regexp: Result<Option<UserDefinedValueMap>, String>,
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
    span_term: Result<Option<UserDefinedValueMap>, String>,
    span_within: Result<Option<serde_json::Value>, String>,
    term: Result<Option<UserDefinedValueMap>, String>,
    terms: Result<Option<serde_json::Value>, String>,
    terms_set: Result<Option<UserDefinedValueMap>, String>,
    wildcard: Result<Option<UserDefinedValueMap>, String>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.match_phrase = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for match_phrase: {}", e));
      self
    }

    pub fn match_phrase_prefix<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
      T::Error: std::fmt::Display, {
      self.terms_set = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for terms_set: {}", e));
      self
    }

    pub fn wildcard<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<UserDefinedValueMap>>,
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