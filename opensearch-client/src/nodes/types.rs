#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

pub mod builder {}
