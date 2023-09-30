
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use super::{UserDefinedValueMap, builder};

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterDeleteComponentTemplateClusterManagerTimeout(String);
impl std::ops::Deref for ClusterDeleteComponentTemplateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterDeleteComponentTemplateClusterManagerTimeout> for String {
  fn from(value: ClusterDeleteComponentTemplateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterDeleteComponentTemplateClusterManagerTimeout>
  for ClusterDeleteComponentTemplateClusterManagerTimeout
{
  fn from(value: &ClusterDeleteComponentTemplateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterDeleteComponentTemplateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterDeleteComponentTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterDeleteComponentTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterDeleteComponentTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterDeleteComponentTemplateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterDeleteComponentTemplateMasterTimeout(String);
impl std::ops::Deref for ClusterDeleteComponentTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterDeleteComponentTemplateMasterTimeout> for String {
  fn from(value: ClusterDeleteComponentTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterDeleteComponentTemplateMasterTimeout> for ClusterDeleteComponentTemplateMasterTimeout {
  fn from(value: &ClusterDeleteComponentTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterDeleteComponentTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterDeleteComponentTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterDeleteComponentTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterDeleteComponentTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterDeleteComponentTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterDeleteComponentTemplateTimeout(String);
impl std::ops::Deref for ClusterDeleteComponentTemplateTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterDeleteComponentTemplateTimeout> for String {
  fn from(value: ClusterDeleteComponentTemplateTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterDeleteComponentTemplateTimeout> for ClusterDeleteComponentTemplateTimeout {
  fn from(value: &ClusterDeleteComponentTemplateTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterDeleteComponentTemplateTimeout {
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

impl std::convert::TryFrom<&str> for ClusterDeleteComponentTemplateTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterDeleteComponentTemplateTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterDeleteComponentTemplateTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterDeleteComponentTemplateTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterExistsComponentTemplateMasterTimeout(String);
impl std::ops::Deref for ClusterExistsComponentTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterExistsComponentTemplateMasterTimeout> for String {
  fn from(value: ClusterExistsComponentTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterExistsComponentTemplateMasterTimeout> for ClusterExistsComponentTemplateMasterTimeout {
  fn from(value: &ClusterExistsComponentTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterExistsComponentTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterExistsComponentTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterExistsComponentTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterExistsComponentTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterExistsComponentTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetComponentTemplateClusterManagerTimeout(String);
impl std::ops::Deref for ClusterGetComponentTemplateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetComponentTemplateClusterManagerTimeout> for String {
  fn from(value: ClusterGetComponentTemplateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterGetComponentTemplateClusterManagerTimeout> for ClusterGetComponentTemplateClusterManagerTimeout {
  fn from(value: &ClusterGetComponentTemplateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetComponentTemplateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterGetComponentTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetComponentTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetComponentTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetComponentTemplateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetComponentTemplateMasterTimeout(String);
impl std::ops::Deref for ClusterGetComponentTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetComponentTemplateMasterTimeout> for String {
  fn from(value: ClusterGetComponentTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterGetComponentTemplateMasterTimeout> for ClusterGetComponentTemplateMasterTimeout {
  fn from(value: &ClusterGetComponentTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetComponentTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterGetComponentTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetComponentTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetComponentTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetComponentTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetComponentTemplateWithNameClusterManagerTimeout(String);
impl std::ops::Deref for ClusterGetComponentTemplateWithNameClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetComponentTemplateWithNameClusterManagerTimeout> for String {
  fn from(value: ClusterGetComponentTemplateWithNameClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterGetComponentTemplateWithNameClusterManagerTimeout>
  for ClusterGetComponentTemplateWithNameClusterManagerTimeout
{
  fn from(value: &ClusterGetComponentTemplateWithNameClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetComponentTemplateWithNameClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterGetComponentTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetComponentTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetComponentTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetComponentTemplateWithNameClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetComponentTemplateWithNameMasterTimeout(String);
impl std::ops::Deref for ClusterGetComponentTemplateWithNameMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetComponentTemplateWithNameMasterTimeout> for String {
  fn from(value: ClusterGetComponentTemplateWithNameMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterGetComponentTemplateWithNameMasterTimeout> for ClusterGetComponentTemplateWithNameMasterTimeout {
  fn from(value: &ClusterGetComponentTemplateWithNameMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetComponentTemplateWithNameMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterGetComponentTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetComponentTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetComponentTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetComponentTemplateWithNameMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetSettingsClusterManagerTimeout(String);
impl std::ops::Deref for ClusterGetSettingsClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetSettingsClusterManagerTimeout> for String {
  fn from(value: ClusterGetSettingsClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterGetSettingsClusterManagerTimeout> for ClusterGetSettingsClusterManagerTimeout {
  fn from(value: &ClusterGetSettingsClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetSettingsClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterGetSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetSettingsClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetSettingsMasterTimeout(String);
impl std::ops::Deref for ClusterGetSettingsMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetSettingsMasterTimeout> for String {
  fn from(value: ClusterGetSettingsMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterGetSettingsMasterTimeout> for ClusterGetSettingsMasterTimeout {
  fn from(value: &ClusterGetSettingsMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetSettingsMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterGetSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetSettingsMasterTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterGetSettingsTimeout(String);
impl std::ops::Deref for ClusterGetSettingsTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterGetSettingsTimeout> for String {
  fn from(value: ClusterGetSettingsTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterGetSettingsTimeout> for ClusterGetSettingsTimeout {
  fn from(value: &ClusterGetSettingsTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterGetSettingsTimeout {
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

impl std::convert::TryFrom<&str> for ClusterGetSettingsTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterGetSettingsTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterGetSettingsTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterGetSettingsTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterHealthClusterManagerTimeout(String);
impl std::ops::Deref for ClusterHealthClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterHealthClusterManagerTimeout> for String {
  fn from(value: ClusterHealthClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterHealthClusterManagerTimeout> for ClusterHealthClusterManagerTimeout {
  fn from(value: &ClusterHealthClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterHealthClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterHealthClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterHealthClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterHealthClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterHealthClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterHealthMasterTimeout(String);
impl std::ops::Deref for ClusterHealthMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterHealthMasterTimeout> for String {
  fn from(value: ClusterHealthMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterHealthMasterTimeout> for ClusterHealthMasterTimeout {
  fn from(value: &ClusterHealthMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterHealthMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterHealthMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterHealthMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterHealthMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterHealthMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterHealthTimeout(String);
impl std::ops::Deref for ClusterHealthTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterHealthTimeout> for String {
  fn from(value: ClusterHealthTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterHealthTimeout> for ClusterHealthTimeout {
  fn from(value: &ClusterHealthTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterHealthTimeout {
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

impl std::convert::TryFrom<&str> for ClusterHealthTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterHealthTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterHealthTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterHealthTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterHealthWithIndexClusterManagerTimeout(String);
impl std::ops::Deref for ClusterHealthWithIndexClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterHealthWithIndexClusterManagerTimeout> for String {
  fn from(value: ClusterHealthWithIndexClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterHealthWithIndexClusterManagerTimeout> for ClusterHealthWithIndexClusterManagerTimeout {
  fn from(value: &ClusterHealthWithIndexClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterHealthWithIndexClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterHealthWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterHealthWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterHealthWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterHealthWithIndexClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterHealthWithIndexMasterTimeout(String);
impl std::ops::Deref for ClusterHealthWithIndexMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterHealthWithIndexMasterTimeout> for String {
  fn from(value: ClusterHealthWithIndexMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterHealthWithIndexMasterTimeout> for ClusterHealthWithIndexMasterTimeout {
  fn from(value: &ClusterHealthWithIndexMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterHealthWithIndexMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterHealthWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterHealthWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterHealthWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterHealthWithIndexMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterHealthWithIndexTimeout(String);
impl std::ops::Deref for ClusterHealthWithIndexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterHealthWithIndexTimeout> for String {
  fn from(value: ClusterHealthWithIndexTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterHealthWithIndexTimeout> for ClusterHealthWithIndexTimeout {
  fn from(value: &ClusterHealthWithIndexTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterHealthWithIndexTimeout {
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

impl std::convert::TryFrom<&str> for ClusterHealthWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterHealthWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterHealthWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterHealthWithIndexTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPendingTasksClusterManagerTimeout(String);
impl std::ops::Deref for ClusterPendingTasksClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPendingTasksClusterManagerTimeout> for String {
  fn from(value: ClusterPendingTasksClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPendingTasksClusterManagerTimeout> for ClusterPendingTasksClusterManagerTimeout {
  fn from(value: &ClusterPendingTasksClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPendingTasksClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPendingTasksClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPendingTasksClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPendingTasksClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPendingTasksClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPendingTasksMasterTimeout(String);
impl std::ops::Deref for ClusterPendingTasksMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPendingTasksMasterTimeout> for String {
  fn from(value: ClusterPendingTasksMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPendingTasksMasterTimeout> for ClusterPendingTasksMasterTimeout {
  fn from(value: &ClusterPendingTasksMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPendingTasksMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPendingTasksMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPendingTasksMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPendingTasksMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPendingTasksMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPostVotingConfigExclusionsTimeout(String);
impl std::ops::Deref for ClusterPostVotingConfigExclusionsTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPostVotingConfigExclusionsTimeout> for String {
  fn from(value: ClusterPostVotingConfigExclusionsTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPostVotingConfigExclusionsTimeout> for ClusterPostVotingConfigExclusionsTimeout {
  fn from(value: &ClusterPostVotingConfigExclusionsTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPostVotingConfigExclusionsTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPostVotingConfigExclusionsTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPostVotingConfigExclusionsTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPostVotingConfigExclusionsTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPostVotingConfigExclusionsTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutComponentTemplatePostClusterManagerTimeout(String);
impl std::ops::Deref for ClusterPutComponentTemplatePostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutComponentTemplatePostClusterManagerTimeout> for String {
  fn from(value: ClusterPutComponentTemplatePostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPutComponentTemplatePostClusterManagerTimeout>
  for ClusterPutComponentTemplatePostClusterManagerTimeout
{
  fn from(value: &ClusterPutComponentTemplatePostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutComponentTemplatePostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPutComponentTemplatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutComponentTemplatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutComponentTemplatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutComponentTemplatePostClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutComponentTemplatePostMasterTimeout(String);
impl std::ops::Deref for ClusterPutComponentTemplatePostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutComponentTemplatePostMasterTimeout> for String {
  fn from(value: ClusterPutComponentTemplatePostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPutComponentTemplatePostMasterTimeout> for ClusterPutComponentTemplatePostMasterTimeout {
  fn from(value: &ClusterPutComponentTemplatePostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutComponentTemplatePostMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPutComponentTemplatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutComponentTemplatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutComponentTemplatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutComponentTemplatePostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutComponentTemplatePostTimeout(String);
impl std::ops::Deref for ClusterPutComponentTemplatePostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutComponentTemplatePostTimeout> for String {
  fn from(value: ClusterPutComponentTemplatePostTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPutComponentTemplatePostTimeout> for ClusterPutComponentTemplatePostTimeout {
  fn from(value: &ClusterPutComponentTemplatePostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutComponentTemplatePostTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPutComponentTemplatePostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutComponentTemplatePostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutComponentTemplatePostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutComponentTemplatePostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutComponentTemplatePutClusterManagerTimeout(String);
impl std::ops::Deref for ClusterPutComponentTemplatePutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutComponentTemplatePutClusterManagerTimeout> for String {
  fn from(value: ClusterPutComponentTemplatePutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPutComponentTemplatePutClusterManagerTimeout>
  for ClusterPutComponentTemplatePutClusterManagerTimeout
{
  fn from(value: &ClusterPutComponentTemplatePutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutComponentTemplatePutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPutComponentTemplatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutComponentTemplatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutComponentTemplatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutComponentTemplatePutClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutComponentTemplatePutMasterTimeout(String);
impl std::ops::Deref for ClusterPutComponentTemplatePutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutComponentTemplatePutMasterTimeout> for String {
  fn from(value: ClusterPutComponentTemplatePutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPutComponentTemplatePutMasterTimeout> for ClusterPutComponentTemplatePutMasterTimeout {
  fn from(value: &ClusterPutComponentTemplatePutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutComponentTemplatePutMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPutComponentTemplatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutComponentTemplatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutComponentTemplatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutComponentTemplatePutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutComponentTemplatePutTimeout(String);
impl std::ops::Deref for ClusterPutComponentTemplatePutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutComponentTemplatePutTimeout> for String {
  fn from(value: ClusterPutComponentTemplatePutTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPutComponentTemplatePutTimeout> for ClusterPutComponentTemplatePutTimeout {
  fn from(value: &ClusterPutComponentTemplatePutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutComponentTemplatePutTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPutComponentTemplatePutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutComponentTemplatePutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutComponentTemplatePutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutComponentTemplatePutTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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
  pub fn builder() -> super::builder::ClusterPutSettingsBodyParams {
    super::builder::ClusterPutSettingsBodyParams::default()
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutSettingsClusterManagerTimeout(String);
impl std::ops::Deref for ClusterPutSettingsClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutSettingsClusterManagerTimeout> for String {
  fn from(value: ClusterPutSettingsClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPutSettingsClusterManagerTimeout> for ClusterPutSettingsClusterManagerTimeout {
  fn from(value: &ClusterPutSettingsClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutSettingsClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPutSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutSettingsClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutSettingsMasterTimeout(String);
impl std::ops::Deref for ClusterPutSettingsMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutSettingsMasterTimeout> for String {
  fn from(value: ClusterPutSettingsMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPutSettingsMasterTimeout> for ClusterPutSettingsMasterTimeout {
  fn from(value: &ClusterPutSettingsMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutSettingsMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPutSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutSettingsMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterPutSettingsTimeout(String);
impl std::ops::Deref for ClusterPutSettingsTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterPutSettingsTimeout> for String {
  fn from(value: ClusterPutSettingsTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterPutSettingsTimeout> for ClusterPutSettingsTimeout {
  fn from(value: &ClusterPutSettingsTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterPutSettingsTimeout {
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

impl std::convert::TryFrom<&str> for ClusterPutSettingsTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterPutSettingsTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterPutSettingsTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterPutSettingsTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterRerouteClusterManagerTimeout(String);
impl std::ops::Deref for ClusterRerouteClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterRerouteClusterManagerTimeout> for String {
  fn from(value: ClusterRerouteClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterRerouteClusterManagerTimeout> for ClusterRerouteClusterManagerTimeout {
  fn from(value: &ClusterRerouteClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterRerouteClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterRerouteClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterRerouteClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterRerouteClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterRerouteClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterRerouteMasterTimeout(String);
impl std::ops::Deref for ClusterRerouteMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterRerouteMasterTimeout> for String {
  fn from(value: ClusterRerouteMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterRerouteMasterTimeout> for ClusterRerouteMasterTimeout {
  fn from(value: &ClusterRerouteMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterRerouteMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterRerouteMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterRerouteMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterRerouteMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterRerouteMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterRerouteTimeout(String);
impl std::ops::Deref for ClusterRerouteTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterRerouteTimeout> for String {
  fn from(value: ClusterRerouteTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterRerouteTimeout> for ClusterRerouteTimeout {
  fn from(value: &ClusterRerouteTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterRerouteTimeout {
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

impl std::convert::TryFrom<&str> for ClusterRerouteTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterRerouteTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterRerouteTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterRerouteTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateClusterManagerTimeout(String);
impl std::ops::Deref for ClusterStateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateClusterManagerTimeout> for String {
  fn from(value: ClusterStateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStateClusterManagerTimeout> for ClusterStateClusterManagerTimeout {
  fn from(value: &ClusterStateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateMasterTimeout(String);
impl std::ops::Deref for ClusterStateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateMasterTimeout> for String {
  fn from(value: ClusterStateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStateMasterTimeout> for ClusterStateMasterTimeout {
  fn from(value: &ClusterStateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

/// out.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWaitForTimeout(String);
impl std::ops::Deref for ClusterStateWaitForTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWaitForTimeout> for String {
  fn from(value: ClusterStateWaitForTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStateWaitForTimeout> for ClusterStateWaitForTimeout {
  fn from(value: &ClusterStateWaitForTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWaitForTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStateWaitForTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWaitForTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWaitForTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWaitForTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWithIndexMetricClusterManagerTimeout(String);
impl std::ops::Deref for ClusterStateWithIndexMetricClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWithIndexMetricClusterManagerTimeout> for String {
  fn from(value: ClusterStateWithIndexMetricClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStateWithIndexMetricClusterManagerTimeout> for ClusterStateWithIndexMetricClusterManagerTimeout {
  fn from(value: &ClusterStateWithIndexMetricClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWithIndexMetricClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStateWithIndexMetricClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWithIndexMetricClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWithIndexMetricClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWithIndexMetricClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWithIndexMetricMasterTimeout(String);
impl std::ops::Deref for ClusterStateWithIndexMetricMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWithIndexMetricMasterTimeout> for String {
  fn from(value: ClusterStateWithIndexMetricMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStateWithIndexMetricMasterTimeout> for ClusterStateWithIndexMetricMasterTimeout {
  fn from(value: &ClusterStateWithIndexMetricMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWithIndexMetricMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStateWithIndexMetricMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWithIndexMetricMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWithIndexMetricMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWithIndexMetricMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

/// out.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWithIndexMetricWaitForTimeout(String);
impl std::ops::Deref for ClusterStateWithIndexMetricWaitForTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWithIndexMetricWaitForTimeout> for String {
  fn from(value: ClusterStateWithIndexMetricWaitForTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStateWithIndexMetricWaitForTimeout> for ClusterStateWithIndexMetricWaitForTimeout {
  fn from(value: &ClusterStateWithIndexMetricWaitForTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWithIndexMetricWaitForTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStateWithIndexMetricWaitForTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWithIndexMetricWaitForTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWithIndexMetricWaitForTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWithIndexMetricWaitForTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWithMetricClusterManagerTimeout(String);
impl std::ops::Deref for ClusterStateWithMetricClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWithMetricClusterManagerTimeout> for String {
  fn from(value: ClusterStateWithMetricClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStateWithMetricClusterManagerTimeout> for ClusterStateWithMetricClusterManagerTimeout {
  fn from(value: &ClusterStateWithMetricClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWithMetricClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStateWithMetricClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWithMetricClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWithMetricClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWithMetricClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWithMetricMasterTimeout(String);
impl std::ops::Deref for ClusterStateWithMetricMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWithMetricMasterTimeout> for String {
  fn from(value: ClusterStateWithMetricMasterTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStateWithMetricMasterTimeout> for ClusterStateWithMetricMasterTimeout {
  fn from(value: &ClusterStateWithMetricMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWithMetricMasterTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStateWithMetricMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWithMetricMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWithMetricMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWithMetricMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

/// out.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStateWithMetricWaitForTimeout(String);
impl std::ops::Deref for ClusterStateWithMetricWaitForTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStateWithMetricWaitForTimeout> for String {
  fn from(value: ClusterStateWithMetricWaitForTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStateWithMetricWaitForTimeout> for ClusterStateWithMetricWaitForTimeout {
  fn from(value: &ClusterStateWithMetricWaitForTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStateWithMetricWaitForTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStateWithMetricWaitForTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStateWithMetricWaitForTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStateWithMetricWaitForTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStateWithMetricWaitForTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterStatsTimeout(String);
impl std::ops::Deref for ClusterStatsTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<ClusterStatsTimeout> for String {
  fn from(value: ClusterStatsTimeout) -> Self {
    value.0
  }
}

impl From<&ClusterStatsTimeout> for ClusterStatsTimeout {
  fn from(value: &ClusterStatsTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for ClusterStatsTimeout {
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

impl std::convert::TryFrom<&str> for ClusterStatsTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for ClusterStatsTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for ClusterStatsTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for ClusterStatsTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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
