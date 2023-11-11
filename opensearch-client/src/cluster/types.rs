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
