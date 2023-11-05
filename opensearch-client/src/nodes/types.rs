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

pub mod builder {}
