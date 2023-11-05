#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::types::UserDefinedValueMap;

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

pub mod builder {
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
}
