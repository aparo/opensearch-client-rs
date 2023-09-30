
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use super::{builder, UserDefinedValueMap, DataStream, ActionObjectStructure};

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesAddBlockClusterManagerTimeout(String);
impl std::ops::Deref for IndicesAddBlockClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesAddBlockClusterManagerTimeout> for String {
  fn from(value: IndicesAddBlockClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesAddBlockClusterManagerTimeout> for IndicesAddBlockClusterManagerTimeout {
  fn from(value: &IndicesAddBlockClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesAddBlockClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesAddBlockClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesAddBlockClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesAddBlockClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesAddBlockClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesAddBlockMasterTimeout(String);
impl std::ops::Deref for IndicesAddBlockMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesAddBlockMasterTimeout> for String {
  fn from(value: IndicesAddBlockMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesAddBlockMasterTimeout> for IndicesAddBlockMasterTimeout {
  fn from(value: &IndicesAddBlockMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesAddBlockMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesAddBlockMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesAddBlockMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesAddBlockMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesAddBlockMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesAddBlockTimeout(String);
impl std::ops::Deref for IndicesAddBlockTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesAddBlockTimeout> for String {
  fn from(value: IndicesAddBlockTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesAddBlockTimeout> for IndicesAddBlockTimeout {
  fn from(value: &IndicesAddBlockTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesAddBlockTimeout {
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

impl std::convert::TryFrom<&str> for IndicesAddBlockTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesAddBlockTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesAddBlockTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesAddBlockTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePostClusterManagerTimeout(String);
impl std::ops::Deref for IndicesClonePostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesClonePostClusterManagerTimeout> for String {
  fn from(value: IndicesClonePostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesClonePostClusterManagerTimeout> for IndicesClonePostClusterManagerTimeout {
  fn from(value: &IndicesClonePostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesClonePostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesClonePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesClonePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesClonePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesClonePostClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePostMasterTimeout(String);
impl std::ops::Deref for IndicesClonePostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesClonePostMasterTimeout> for String {
  fn from(value: IndicesClonePostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesClonePostMasterTimeout> for IndicesClonePostMasterTimeout {
  fn from(value: &IndicesClonePostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesClonePostMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesClonePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesClonePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesClonePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesClonePostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePostTimeout(String);
impl std::ops::Deref for IndicesClonePostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesClonePostTimeout> for String {
  fn from(value: IndicesClonePostTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesClonePostTimeout> for IndicesClonePostTimeout {
  fn from(value: &IndicesClonePostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesClonePostTimeout {
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

impl std::convert::TryFrom<&str> for IndicesClonePostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesClonePostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesClonePostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesClonePostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePutClusterManagerTimeout(String);
impl std::ops::Deref for IndicesClonePutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesClonePutClusterManagerTimeout> for String {
  fn from(value: IndicesClonePutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesClonePutClusterManagerTimeout> for IndicesClonePutClusterManagerTimeout {
  fn from(value: &IndicesClonePutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesClonePutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesClonePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesClonePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesClonePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesClonePutClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePutMasterTimeout(String);
impl std::ops::Deref for IndicesClonePutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesClonePutMasterTimeout> for String {
  fn from(value: IndicesClonePutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesClonePutMasterTimeout> for IndicesClonePutMasterTimeout {
  fn from(value: &IndicesClonePutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesClonePutMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesClonePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesClonePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesClonePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesClonePutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesClonePutTimeout(String);
impl std::ops::Deref for IndicesClonePutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesClonePutTimeout> for String {
  fn from(value: IndicesClonePutTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesClonePutTimeout> for IndicesClonePutTimeout {
  fn from(value: &IndicesClonePutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesClonePutTimeout {
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

impl std::convert::TryFrom<&str> for IndicesClonePutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesClonePutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesClonePutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesClonePutTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesCloseClusterManagerTimeout(String);
impl std::ops::Deref for IndicesCloseClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesCloseClusterManagerTimeout> for String {
  fn from(value: IndicesCloseClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesCloseClusterManagerTimeout> for IndicesCloseClusterManagerTimeout {
  fn from(value: &IndicesCloseClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesCloseClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesCloseClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesCloseClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesCloseClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesCloseClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesCloseMasterTimeout(String);
impl std::ops::Deref for IndicesCloseMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesCloseMasterTimeout> for String {
  fn from(value: IndicesCloseMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesCloseMasterTimeout> for IndicesCloseMasterTimeout {
  fn from(value: &IndicesCloseMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesCloseMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesCloseMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesCloseMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesCloseMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesCloseMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesCloseTimeout(String);
impl std::ops::Deref for IndicesCloseTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesCloseTimeout> for String {
  fn from(value: IndicesCloseTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesCloseTimeout> for IndicesCloseTimeout {
  fn from(value: &IndicesCloseTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesCloseTimeout {
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

impl std::convert::TryFrom<&str> for IndicesCloseTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesCloseTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesCloseTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesCloseTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesCreateClusterManagerTimeout(String);
impl std::ops::Deref for IndicesCreateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesCreateClusterManagerTimeout> for String {
  fn from(value: IndicesCreateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesCreateClusterManagerTimeout> for IndicesCreateClusterManagerTimeout {
  fn from(value: &IndicesCreateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesCreateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesCreateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesCreateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesCreateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesCreateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesCreateMasterTimeout(String);
impl std::ops::Deref for IndicesCreateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesCreateMasterTimeout> for String {
  fn from(value: IndicesCreateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesCreateMasterTimeout> for IndicesCreateMasterTimeout {
  fn from(value: &IndicesCreateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesCreateMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesCreateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesCreateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesCreateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesCreateMasterTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesCreateTimeout(String);
impl std::ops::Deref for IndicesCreateTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesCreateTimeout> for String {
  fn from(value: IndicesCreateTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesCreateTimeout> for IndicesCreateTimeout {
  fn from(value: &IndicesCreateTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesCreateTimeout {
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

impl std::convert::TryFrom<&str> for IndicesCreateTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesCreateTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesCreateTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesCreateTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasClusterManagerTimeout(String);
impl std::ops::Deref for IndicesDeleteAliasClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteAliasClusterManagerTimeout> for String {
  fn from(value: IndicesDeleteAliasClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteAliasClusterManagerTimeout> for IndicesDeleteAliasClusterManagerTimeout {
  fn from(value: &IndicesDeleteAliasClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteAliasClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteAliasClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteAliasClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteAliasClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasMasterTimeout(String);
impl std::ops::Deref for IndicesDeleteAliasMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteAliasMasterTimeout> for String {
  fn from(value: IndicesDeleteAliasMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteAliasMasterTimeout> for IndicesDeleteAliasMasterTimeout {
  fn from(value: &IndicesDeleteAliasMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteAliasMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteAliasMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteAliasMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteAliasMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasPluralClusterManagerTimeout(String);
impl std::ops::Deref for IndicesDeleteAliasPluralClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteAliasPluralClusterManagerTimeout> for String {
  fn from(value: IndicesDeleteAliasPluralClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteAliasPluralClusterManagerTimeout> for IndicesDeleteAliasPluralClusterManagerTimeout {
  fn from(value: &IndicesDeleteAliasPluralClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteAliasPluralClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteAliasPluralClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteAliasPluralClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteAliasPluralClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasPluralClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasPluralMasterTimeout(String);
impl std::ops::Deref for IndicesDeleteAliasPluralMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteAliasPluralMasterTimeout> for String {
  fn from(value: IndicesDeleteAliasPluralMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteAliasPluralMasterTimeout> for IndicesDeleteAliasPluralMasterTimeout {
  fn from(value: &IndicesDeleteAliasPluralMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteAliasPluralMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteAliasPluralMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteAliasPluralMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteAliasPluralMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasPluralMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasPluralTimeout(String);
impl std::ops::Deref for IndicesDeleteAliasPluralTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteAliasPluralTimeout> for String {
  fn from(value: IndicesDeleteAliasPluralTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteAliasPluralTimeout> for IndicesDeleteAliasPluralTimeout {
  fn from(value: &IndicesDeleteAliasPluralTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteAliasPluralTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteAliasPluralTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteAliasPluralTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteAliasPluralTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasPluralTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteAliasTimeout(String);
impl std::ops::Deref for IndicesDeleteAliasTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteAliasTimeout> for String {
  fn from(value: IndicesDeleteAliasTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteAliasTimeout> for IndicesDeleteAliasTimeout {
  fn from(value: &IndicesDeleteAliasTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteAliasTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteAliasTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteAliasTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteAliasTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteAliasTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteIndexTemplateClusterManagerTimeout(String);
impl std::ops::Deref for IndicesDeleteIndexTemplateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteIndexTemplateClusterManagerTimeout> for String {
  fn from(value: IndicesDeleteIndexTemplateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteIndexTemplateClusterManagerTimeout> for IndicesDeleteIndexTemplateClusterManagerTimeout {
  fn from(value: &IndicesDeleteIndexTemplateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteIndexTemplateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteIndexTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteIndexTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteIndexTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteIndexTemplateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteIndexTemplateMasterTimeout(String);
impl std::ops::Deref for IndicesDeleteIndexTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteIndexTemplateMasterTimeout> for String {
  fn from(value: IndicesDeleteIndexTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteIndexTemplateMasterTimeout> for IndicesDeleteIndexTemplateMasterTimeout {
  fn from(value: &IndicesDeleteIndexTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteIndexTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteIndexTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteIndexTemplateTimeout(String);
impl std::ops::Deref for IndicesDeleteIndexTemplateTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteIndexTemplateTimeout> for String {
  fn from(value: IndicesDeleteIndexTemplateTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteIndexTemplateTimeout> for IndicesDeleteIndexTemplateTimeout {
  fn from(value: &IndicesDeleteIndexTemplateTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteIndexTemplateTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteIndexTemplateTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteIndexTemplateTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteIndexTemplateTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteIndexTemplateTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteMasterTimeout(String);
impl std::ops::Deref for IndicesDeleteMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteMasterTimeout> for String {
  fn from(value: IndicesDeleteMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteMasterTimeout> for IndicesDeleteMasterTimeout {
  fn from(value: &IndicesDeleteMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteMasterTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteTemplateClusterManagerTimeout(String);
impl std::ops::Deref for IndicesDeleteTemplateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteTemplateClusterManagerTimeout> for String {
  fn from(value: IndicesDeleteTemplateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteTemplateClusterManagerTimeout> for IndicesDeleteTemplateClusterManagerTimeout {
  fn from(value: &IndicesDeleteTemplateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteTemplateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteTemplateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteTemplateMasterTimeout(String);
impl std::ops::Deref for IndicesDeleteTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteTemplateMasterTimeout> for String {
  fn from(value: IndicesDeleteTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteTemplateMasterTimeout> for IndicesDeleteTemplateMasterTimeout {
  fn from(value: &IndicesDeleteTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteTemplateTimeout(String);
impl std::ops::Deref for IndicesDeleteTemplateTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteTemplateTimeout> for String {
  fn from(value: IndicesDeleteTemplateTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteTemplateTimeout> for IndicesDeleteTemplateTimeout {
  fn from(value: &IndicesDeleteTemplateTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteTemplateTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteTemplateTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteTemplateTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteTemplateTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteTemplateTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesDeleteTimeout(String);
impl std::ops::Deref for IndicesDeleteTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesDeleteTimeout> for String {
  fn from(value: IndicesDeleteTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesDeleteTimeout> for IndicesDeleteTimeout {
  fn from(value: &IndicesDeleteTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesDeleteTimeout {
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

impl std::convert::TryFrom<&str> for IndicesDeleteTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesDeleteTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesDeleteTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesDeleteTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesExistsIndexTemplateMasterTimeout(String);
impl std::ops::Deref for IndicesExistsIndexTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesExistsIndexTemplateMasterTimeout> for String {
  fn from(value: IndicesExistsIndexTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesExistsIndexTemplateMasterTimeout> for IndicesExistsIndexTemplateMasterTimeout {
  fn from(value: &IndicesExistsIndexTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesExistsIndexTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesExistsIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesExistsIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesExistsIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesExistsIndexTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesExistsTemplateMasterTimeout(String);
impl std::ops::Deref for IndicesExistsTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesExistsTemplateMasterTimeout> for String {
  fn from(value: IndicesExistsTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesExistsTemplateMasterTimeout> for IndicesExistsTemplateMasterTimeout {
  fn from(value: &IndicesExistsTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesExistsTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesExistsTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesExistsTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesExistsTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesExistsTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetClusterManagerTimeout> for String {
  fn from(value: IndicesGetClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetClusterManagerTimeout> for IndicesGetClusterManagerTimeout {
  fn from(value: &IndicesGetClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetIndexTemplateClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetIndexTemplateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetIndexTemplateClusterManagerTimeout> for String {
  fn from(value: IndicesGetIndexTemplateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetIndexTemplateClusterManagerTimeout> for IndicesGetIndexTemplateClusterManagerTimeout {
  fn from(value: &IndicesGetIndexTemplateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetIndexTemplateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetIndexTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetIndexTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetIndexTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetIndexTemplateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetIndexTemplateMasterTimeout(String);
impl std::ops::Deref for IndicesGetIndexTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetIndexTemplateMasterTimeout> for String {
  fn from(value: IndicesGetIndexTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetIndexTemplateMasterTimeout> for IndicesGetIndexTemplateMasterTimeout {
  fn from(value: &IndicesGetIndexTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetIndexTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetIndexTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetIndexTemplateWithNameClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetIndexTemplateWithNameClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetIndexTemplateWithNameClusterManagerTimeout> for String {
  fn from(value: IndicesGetIndexTemplateWithNameClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetIndexTemplateWithNameClusterManagerTimeout>
  for IndicesGetIndexTemplateWithNameClusterManagerTimeout
{
  fn from(value: &IndicesGetIndexTemplateWithNameClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetIndexTemplateWithNameClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetIndexTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetIndexTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetIndexTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetIndexTemplateWithNameClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetIndexTemplateWithNameMasterTimeout(String);
impl std::ops::Deref for IndicesGetIndexTemplateWithNameMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetIndexTemplateWithNameMasterTimeout> for String {
  fn from(value: IndicesGetIndexTemplateWithNameMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetIndexTemplateWithNameMasterTimeout> for IndicesGetIndexTemplateWithNameMasterTimeout {
  fn from(value: &IndicesGetIndexTemplateWithNameMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetIndexTemplateWithNameMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetIndexTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetIndexTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetIndexTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetIndexTemplateWithNameMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetMappingClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetMappingClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetMappingClusterManagerTimeout> for String {
  fn from(value: IndicesGetMappingClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetMappingClusterManagerTimeout> for IndicesGetMappingClusterManagerTimeout {
  fn from(value: &IndicesGetMappingClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetMappingClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetMappingClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetMappingClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetMappingClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetMappingClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetMappingMasterTimeout(String);
impl std::ops::Deref for IndicesGetMappingMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetMappingMasterTimeout> for String {
  fn from(value: IndicesGetMappingMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetMappingMasterTimeout> for IndicesGetMappingMasterTimeout {
  fn from(value: &IndicesGetMappingMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetMappingMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetMappingMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetMappingMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetMappingMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetMappingMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetMappingWithIndexClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetMappingWithIndexClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetMappingWithIndexClusterManagerTimeout> for String {
  fn from(value: IndicesGetMappingWithIndexClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetMappingWithIndexClusterManagerTimeout> for IndicesGetMappingWithIndexClusterManagerTimeout {
  fn from(value: &IndicesGetMappingWithIndexClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetMappingWithIndexClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetMappingWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetMappingWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetMappingWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetMappingWithIndexClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetMappingWithIndexMasterTimeout(String);
impl std::ops::Deref for IndicesGetMappingWithIndexMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetMappingWithIndexMasterTimeout> for String {
  fn from(value: IndicesGetMappingWithIndexMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetMappingWithIndexMasterTimeout> for IndicesGetMappingWithIndexMasterTimeout {
  fn from(value: &IndicesGetMappingWithIndexMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetMappingWithIndexMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetMappingWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetMappingWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetMappingWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetMappingWithIndexMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetMasterTimeout(String);
impl std::ops::Deref for IndicesGetMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetMasterTimeout> for String {
  fn from(value: IndicesGetMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetMasterTimeout> for IndicesGetMasterTimeout {
  fn from(value: &IndicesGetMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetSettingsClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsClusterManagerTimeout> for String {
  fn from(value: IndicesGetSettingsClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsClusterManagerTimeout> for IndicesGetSettingsClusterManagerTimeout {
  fn from(value: &IndicesGetSettingsClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsMasterTimeout(String);
impl std::ops::Deref for IndicesGetSettingsMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsMasterTimeout> for String {
  fn from(value: IndicesGetSettingsMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsMasterTimeout> for IndicesGetSettingsMasterTimeout {
  fn from(value: &IndicesGetSettingsMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsWithIndexClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetSettingsWithIndexClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsWithIndexClusterManagerTimeout> for String {
  fn from(value: IndicesGetSettingsWithIndexClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsWithIndexClusterManagerTimeout> for IndicesGetSettingsWithIndexClusterManagerTimeout {
  fn from(value: &IndicesGetSettingsWithIndexClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsWithIndexClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithIndexClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsWithIndexMasterTimeout(String);
impl std::ops::Deref for IndicesGetSettingsWithIndexMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsWithIndexMasterTimeout> for String {
  fn from(value: IndicesGetSettingsWithIndexMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsWithIndexMasterTimeout> for IndicesGetSettingsWithIndexMasterTimeout {
  fn from(value: &IndicesGetSettingsWithIndexMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsWithIndexMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithIndexMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsWithIndexNameClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetSettingsWithIndexNameClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsWithIndexNameClusterManagerTimeout> for String {
  fn from(value: IndicesGetSettingsWithIndexNameClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsWithIndexNameClusterManagerTimeout>
  for IndicesGetSettingsWithIndexNameClusterManagerTimeout
{
  fn from(value: &IndicesGetSettingsWithIndexNameClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsWithIndexNameClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsWithIndexNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsWithIndexNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsWithIndexNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithIndexNameClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsWithIndexNameMasterTimeout(String);
impl std::ops::Deref for IndicesGetSettingsWithIndexNameMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsWithIndexNameMasterTimeout> for String {
  fn from(value: IndicesGetSettingsWithIndexNameMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsWithIndexNameMasterTimeout> for IndicesGetSettingsWithIndexNameMasterTimeout {
  fn from(value: &IndicesGetSettingsWithIndexNameMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsWithIndexNameMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsWithIndexNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsWithIndexNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsWithIndexNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithIndexNameMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsWithNameClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetSettingsWithNameClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsWithNameClusterManagerTimeout> for String {
  fn from(value: IndicesGetSettingsWithNameClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsWithNameClusterManagerTimeout> for IndicesGetSettingsWithNameClusterManagerTimeout {
  fn from(value: &IndicesGetSettingsWithNameClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsWithNameClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithNameClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetSettingsWithNameMasterTimeout(String);
impl std::ops::Deref for IndicesGetSettingsWithNameMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetSettingsWithNameMasterTimeout> for String {
  fn from(value: IndicesGetSettingsWithNameMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetSettingsWithNameMasterTimeout> for IndicesGetSettingsWithNameMasterTimeout {
  fn from(value: &IndicesGetSettingsWithNameMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetSettingsWithNameMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetSettingsWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetSettingsWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetSettingsWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetSettingsWithNameMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetTemplateClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetTemplateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetTemplateClusterManagerTimeout> for String {
  fn from(value: IndicesGetTemplateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetTemplateClusterManagerTimeout> for IndicesGetTemplateClusterManagerTimeout {
  fn from(value: &IndicesGetTemplateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetTemplateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetTemplateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetTemplateMasterTimeout(String);
impl std::ops::Deref for IndicesGetTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetTemplateMasterTimeout> for String {
  fn from(value: IndicesGetTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetTemplateMasterTimeout> for IndicesGetTemplateMasterTimeout {
  fn from(value: &IndicesGetTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetTemplateWithNameClusterManagerTimeout(String);
impl std::ops::Deref for IndicesGetTemplateWithNameClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetTemplateWithNameClusterManagerTimeout> for String {
  fn from(value: IndicesGetTemplateWithNameClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetTemplateWithNameClusterManagerTimeout> for IndicesGetTemplateWithNameClusterManagerTimeout {
  fn from(value: &IndicesGetTemplateWithNameClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetTemplateWithNameClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetTemplateWithNameClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesGetTemplateWithNameMasterTimeout(String);
impl std::ops::Deref for IndicesGetTemplateWithNameMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesGetTemplateWithNameMasterTimeout> for String {
  fn from(value: IndicesGetTemplateWithNameMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesGetTemplateWithNameMasterTimeout> for IndicesGetTemplateWithNameMasterTimeout {
  fn from(value: &IndicesGetTemplateWithNameMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesGetTemplateWithNameMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesGetTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesGetTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesGetTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesGetTemplateWithNameMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesOpenMasterTimeout(String);
impl std::ops::Deref for IndicesOpenMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesOpenMasterTimeout> for String {
  fn from(value: IndicesOpenMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesOpenMasterTimeout> for IndicesOpenMasterTimeout {
  fn from(value: &IndicesOpenMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesOpenMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesOpenMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesOpenMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesOpenMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesOpenMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesOpenTimeout(String);
impl std::ops::Deref for IndicesOpenTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesOpenTimeout> for String {
  fn from(value: IndicesOpenTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesOpenTimeout> for IndicesOpenTimeout {
  fn from(value: &IndicesOpenTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesOpenTimeout {
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

impl std::convert::TryFrom<&str> for IndicesOpenTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesOpenTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesOpenTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesOpenTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutAliasPostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPostClusterManagerTimeout> for String {
  fn from(value: IndicesPutAliasPostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPostClusterManagerTimeout> for IndicesPutAliasPostClusterManagerTimeout {
  fn from(value: &IndicesPutAliasPostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostMasterTimeout(String);
impl std::ops::Deref for IndicesPutAliasPostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPostMasterTimeout> for String {
  fn from(value: IndicesPutAliasPostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPostMasterTimeout> for IndicesPutAliasPostMasterTimeout {
  fn from(value: &IndicesPutAliasPostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPostMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostPluralClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutAliasPostPluralClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPostPluralClusterManagerTimeout> for String {
  fn from(value: IndicesPutAliasPostPluralClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPostPluralClusterManagerTimeout> for IndicesPutAliasPostPluralClusterManagerTimeout {
  fn from(value: &IndicesPutAliasPostPluralClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPostPluralClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPostPluralClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPostPluralClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPostPluralClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostPluralClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostPluralMasterTimeout(String);
impl std::ops::Deref for IndicesPutAliasPostPluralMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPostPluralMasterTimeout> for String {
  fn from(value: IndicesPutAliasPostPluralMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPostPluralMasterTimeout> for IndicesPutAliasPostPluralMasterTimeout {
  fn from(value: &IndicesPutAliasPostPluralMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPostPluralMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPostPluralMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPostPluralMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPostPluralMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostPluralMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostPluralTimeout(String);
impl std::ops::Deref for IndicesPutAliasPostPluralTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPostPluralTimeout> for String {
  fn from(value: IndicesPutAliasPostPluralTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPostPluralTimeout> for IndicesPutAliasPostPluralTimeout {
  fn from(value: &IndicesPutAliasPostPluralTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPostPluralTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPostPluralTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPostPluralTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPostPluralTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostPluralTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPostTimeout(String);
impl std::ops::Deref for IndicesPutAliasPostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPostTimeout> for String {
  fn from(value: IndicesPutAliasPostTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPostTimeout> for IndicesPutAliasPostTimeout {
  fn from(value: &IndicesPutAliasPostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPostTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutAliasPutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPutClusterManagerTimeout> for String {
  fn from(value: IndicesPutAliasPutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPutClusterManagerTimeout> for IndicesPutAliasPutClusterManagerTimeout {
  fn from(value: &IndicesPutAliasPutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutMasterTimeout(String);
impl std::ops::Deref for IndicesPutAliasPutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPutMasterTimeout> for String {
  fn from(value: IndicesPutAliasPutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPutMasterTimeout> for IndicesPutAliasPutMasterTimeout {
  fn from(value: &IndicesPutAliasPutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPutMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutPluralClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutAliasPutPluralClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPutPluralClusterManagerTimeout> for String {
  fn from(value: IndicesPutAliasPutPluralClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPutPluralClusterManagerTimeout> for IndicesPutAliasPutPluralClusterManagerTimeout {
  fn from(value: &IndicesPutAliasPutPluralClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPutPluralClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPutPluralClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPutPluralClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPutPluralClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutPluralClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutPluralMasterTimeout(String);
impl std::ops::Deref for IndicesPutAliasPutPluralMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPutPluralMasterTimeout> for String {
  fn from(value: IndicesPutAliasPutPluralMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPutPluralMasterTimeout> for IndicesPutAliasPutPluralMasterTimeout {
  fn from(value: &IndicesPutAliasPutPluralMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPutPluralMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPutPluralMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPutPluralMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPutPluralMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutPluralMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutPluralTimeout(String);
impl std::ops::Deref for IndicesPutAliasPutPluralTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPutPluralTimeout> for String {
  fn from(value: IndicesPutAliasPutPluralTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPutPluralTimeout> for IndicesPutAliasPutPluralTimeout {
  fn from(value: &IndicesPutAliasPutPluralTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPutPluralTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPutPluralTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPutPluralTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPutPluralTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutPluralTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutAliasPutTimeout(String);
impl std::ops::Deref for IndicesPutAliasPutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutAliasPutTimeout> for String {
  fn from(value: IndicesPutAliasPutTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutAliasPutTimeout> for IndicesPutAliasPutTimeout {
  fn from(value: &IndicesPutAliasPutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutAliasPutTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutAliasPutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutAliasPutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutAliasPutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutAliasPutTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutIndexTemplatePostClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutIndexTemplatePostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutIndexTemplatePostClusterManagerTimeout> for String {
  fn from(value: IndicesPutIndexTemplatePostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutIndexTemplatePostClusterManagerTimeout> for IndicesPutIndexTemplatePostClusterManagerTimeout {
  fn from(value: &IndicesPutIndexTemplatePostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutIndexTemplatePostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutIndexTemplatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutIndexTemplatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutIndexTemplatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutIndexTemplatePostClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutIndexTemplatePostMasterTimeout(String);
impl std::ops::Deref for IndicesPutIndexTemplatePostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutIndexTemplatePostMasterTimeout> for String {
  fn from(value: IndicesPutIndexTemplatePostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutIndexTemplatePostMasterTimeout> for IndicesPutIndexTemplatePostMasterTimeout {
  fn from(value: &IndicesPutIndexTemplatePostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutIndexTemplatePostMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutIndexTemplatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutIndexTemplatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutIndexTemplatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutIndexTemplatePostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutIndexTemplatePutClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutIndexTemplatePutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutIndexTemplatePutClusterManagerTimeout> for String {
  fn from(value: IndicesPutIndexTemplatePutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutIndexTemplatePutClusterManagerTimeout> for IndicesPutIndexTemplatePutClusterManagerTimeout {
  fn from(value: &IndicesPutIndexTemplatePutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutIndexTemplatePutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutIndexTemplatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutIndexTemplatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutIndexTemplatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutIndexTemplatePutClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutIndexTemplatePutMasterTimeout(String);
impl std::ops::Deref for IndicesPutIndexTemplatePutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutIndexTemplatePutMasterTimeout> for String {
  fn from(value: IndicesPutIndexTemplatePutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutIndexTemplatePutMasterTimeout> for IndicesPutIndexTemplatePutMasterTimeout {
  fn from(value: &IndicesPutIndexTemplatePutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutIndexTemplatePutMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutIndexTemplatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutIndexTemplatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutIndexTemplatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutIndexTemplatePutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutMappingPostClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutMappingPostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutMappingPostClusterManagerTimeout> for String {
  fn from(value: IndicesPutMappingPostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutMappingPostClusterManagerTimeout> for IndicesPutMappingPostClusterManagerTimeout {
  fn from(value: &IndicesPutMappingPostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutMappingPostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutMappingPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutMappingPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutMappingPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutMappingPostClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutMappingPostMasterTimeout(String);
impl std::ops::Deref for IndicesPutMappingPostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutMappingPostMasterTimeout> for String {
  fn from(value: IndicesPutMappingPostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutMappingPostMasterTimeout> for IndicesPutMappingPostMasterTimeout {
  fn from(value: &IndicesPutMappingPostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutMappingPostMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutMappingPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutMappingPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutMappingPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutMappingPostMasterTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutMappingPostTimeout(String);
impl std::ops::Deref for IndicesPutMappingPostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutMappingPostTimeout> for String {
  fn from(value: IndicesPutMappingPostTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutMappingPostTimeout> for IndicesPutMappingPostTimeout {
  fn from(value: &IndicesPutMappingPostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutMappingPostTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutMappingPostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutMappingPostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutMappingPostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutMappingPostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutMappingPutClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutMappingPutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutMappingPutClusterManagerTimeout> for String {
  fn from(value: IndicesPutMappingPutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutMappingPutClusterManagerTimeout> for IndicesPutMappingPutClusterManagerTimeout {
  fn from(value: &IndicesPutMappingPutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutMappingPutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutMappingPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutMappingPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutMappingPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutMappingPutClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutMappingPutMasterTimeout(String);
impl std::ops::Deref for IndicesPutMappingPutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutMappingPutMasterTimeout> for String {
  fn from(value: IndicesPutMappingPutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutMappingPutMasterTimeout> for IndicesPutMappingPutMasterTimeout {
  fn from(value: &IndicesPutMappingPutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutMappingPutMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutMappingPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutMappingPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutMappingPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutMappingPutMasterTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutMappingPutTimeout(String);
impl std::ops::Deref for IndicesPutMappingPutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutMappingPutTimeout> for String {
  fn from(value: IndicesPutMappingPutTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutMappingPutTimeout> for IndicesPutMappingPutTimeout {
  fn from(value: &IndicesPutMappingPutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutMappingPutTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutMappingPutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutMappingPutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutMappingPutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutMappingPutTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutSettingsClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutSettingsClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutSettingsClusterManagerTimeout> for String {
  fn from(value: IndicesPutSettingsClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutSettingsClusterManagerTimeout> for IndicesPutSettingsClusterManagerTimeout {
  fn from(value: &IndicesPutSettingsClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutSettingsClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutSettingsClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutSettingsClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutSettingsMasterTimeout(String);
impl std::ops::Deref for IndicesPutSettingsMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutSettingsMasterTimeout> for String {
  fn from(value: IndicesPutSettingsMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutSettingsMasterTimeout> for IndicesPutSettingsMasterTimeout {
  fn from(value: &IndicesPutSettingsMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutSettingsMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutSettingsMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutSettingsMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutSettingsTimeout(String);
impl std::ops::Deref for IndicesPutSettingsTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutSettingsTimeout> for String {
  fn from(value: IndicesPutSettingsTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutSettingsTimeout> for IndicesPutSettingsTimeout {
  fn from(value: &IndicesPutSettingsTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutSettingsTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutSettingsTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutSettingsTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutSettingsTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutSettingsTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutSettingsWithIndexClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutSettingsWithIndexClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutSettingsWithIndexClusterManagerTimeout> for String {
  fn from(value: IndicesPutSettingsWithIndexClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutSettingsWithIndexClusterManagerTimeout> for IndicesPutSettingsWithIndexClusterManagerTimeout {
  fn from(value: &IndicesPutSettingsWithIndexClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutSettingsWithIndexClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutSettingsWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutSettingsWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutSettingsWithIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutSettingsWithIndexClusterManagerTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutSettingsWithIndexMasterTimeout(String);
impl std::ops::Deref for IndicesPutSettingsWithIndexMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutSettingsWithIndexMasterTimeout> for String {
  fn from(value: IndicesPutSettingsWithIndexMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutSettingsWithIndexMasterTimeout> for IndicesPutSettingsWithIndexMasterTimeout {
  fn from(value: &IndicesPutSettingsWithIndexMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutSettingsWithIndexMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutSettingsWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutSettingsWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutSettingsWithIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutSettingsWithIndexMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutSettingsWithIndexTimeout(String);
impl std::ops::Deref for IndicesPutSettingsWithIndexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutSettingsWithIndexTimeout> for String {
  fn from(value: IndicesPutSettingsWithIndexTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutSettingsWithIndexTimeout> for IndicesPutSettingsWithIndexTimeout {
  fn from(value: &IndicesPutSettingsWithIndexTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutSettingsWithIndexTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutSettingsWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutSettingsWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutSettingsWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutSettingsWithIndexTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutTemplatePostClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutTemplatePostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutTemplatePostClusterManagerTimeout> for String {
  fn from(value: IndicesPutTemplatePostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutTemplatePostClusterManagerTimeout> for IndicesPutTemplatePostClusterManagerTimeout {
  fn from(value: &IndicesPutTemplatePostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutTemplatePostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutTemplatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutTemplatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutTemplatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutTemplatePostClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutTemplatePostMasterTimeout(String);
impl std::ops::Deref for IndicesPutTemplatePostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutTemplatePostMasterTimeout> for String {
  fn from(value: IndicesPutTemplatePostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutTemplatePostMasterTimeout> for IndicesPutTemplatePostMasterTimeout {
  fn from(value: &IndicesPutTemplatePostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutTemplatePostMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutTemplatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutTemplatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutTemplatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutTemplatePostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutTemplatePutClusterManagerTimeout(String);
impl std::ops::Deref for IndicesPutTemplatePutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutTemplatePutClusterManagerTimeout> for String {
  fn from(value: IndicesPutTemplatePutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutTemplatePutClusterManagerTimeout> for IndicesPutTemplatePutClusterManagerTimeout {
  fn from(value: &IndicesPutTemplatePutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutTemplatePutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutTemplatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutTemplatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutTemplatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutTemplatePutClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesPutTemplatePutMasterTimeout(String);
impl std::ops::Deref for IndicesPutTemplatePutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesPutTemplatePutMasterTimeout> for String {
  fn from(value: IndicesPutTemplatePutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesPutTemplatePutMasterTimeout> for IndicesPutTemplatePutMasterTimeout {
  fn from(value: &IndicesPutTemplatePutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesPutTemplatePutMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesPutTemplatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesPutTemplatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesPutTemplatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesPutTemplatePutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRolloverClusterManagerTimeout(String);
impl std::ops::Deref for IndicesRolloverClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRolloverClusterManagerTimeout> for String {
  fn from(value: IndicesRolloverClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesRolloverClusterManagerTimeout> for IndicesRolloverClusterManagerTimeout {
  fn from(value: &IndicesRolloverClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRolloverClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesRolloverClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRolloverClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRolloverClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRolloverClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRolloverMasterTimeout(String);
impl std::ops::Deref for IndicesRolloverMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRolloverMasterTimeout> for String {
  fn from(value: IndicesRolloverMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesRolloverMasterTimeout> for IndicesRolloverMasterTimeout {
  fn from(value: &IndicesRolloverMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRolloverMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesRolloverMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRolloverMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRolloverMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRolloverMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRolloverTimeout(String);
impl std::ops::Deref for IndicesRolloverTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRolloverTimeout> for String {
  fn from(value: IndicesRolloverTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesRolloverTimeout> for IndicesRolloverTimeout {
  fn from(value: &IndicesRolloverTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRolloverTimeout {
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

impl std::convert::TryFrom<&str> for IndicesRolloverTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRolloverTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRolloverTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRolloverTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRolloverWithNewIndexClusterManagerTimeout(String);
impl std::ops::Deref for IndicesRolloverWithNewIndexClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRolloverWithNewIndexClusterManagerTimeout> for String {
  fn from(value: IndicesRolloverWithNewIndexClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesRolloverWithNewIndexClusterManagerTimeout> for IndicesRolloverWithNewIndexClusterManagerTimeout {
  fn from(value: &IndicesRolloverWithNewIndexClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRolloverWithNewIndexClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesRolloverWithNewIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRolloverWithNewIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRolloverWithNewIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRolloverWithNewIndexClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRolloverWithNewIndexMasterTimeout(String);
impl std::ops::Deref for IndicesRolloverWithNewIndexMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRolloverWithNewIndexMasterTimeout> for String {
  fn from(value: IndicesRolloverWithNewIndexMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesRolloverWithNewIndexMasterTimeout> for IndicesRolloverWithNewIndexMasterTimeout {
  fn from(value: &IndicesRolloverWithNewIndexMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRolloverWithNewIndexMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesRolloverWithNewIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRolloverWithNewIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRolloverWithNewIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRolloverWithNewIndexMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesRolloverWithNewIndexTimeout(String);
impl std::ops::Deref for IndicesRolloverWithNewIndexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesRolloverWithNewIndexTimeout> for String {
  fn from(value: IndicesRolloverWithNewIndexTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesRolloverWithNewIndexTimeout> for IndicesRolloverWithNewIndexTimeout {
  fn from(value: &IndicesRolloverWithNewIndexTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesRolloverWithNewIndexTimeout {
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

impl std::convert::TryFrom<&str> for IndicesRolloverWithNewIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesRolloverWithNewIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesRolloverWithNewIndexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesRolloverWithNewIndexTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPostClusterManagerTimeout(String);
impl std::ops::Deref for IndicesShrinkPostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesShrinkPostClusterManagerTimeout> for String {
  fn from(value: IndicesShrinkPostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesShrinkPostClusterManagerTimeout> for IndicesShrinkPostClusterManagerTimeout {
  fn from(value: &IndicesShrinkPostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesShrinkPostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesShrinkPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesShrinkPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesShrinkPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesShrinkPostClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPostMasterTimeout(String);
impl std::ops::Deref for IndicesShrinkPostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesShrinkPostMasterTimeout> for String {
  fn from(value: IndicesShrinkPostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesShrinkPostMasterTimeout> for IndicesShrinkPostMasterTimeout {
  fn from(value: &IndicesShrinkPostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesShrinkPostMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesShrinkPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesShrinkPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesShrinkPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesShrinkPostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPostTimeout(String);
impl std::ops::Deref for IndicesShrinkPostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesShrinkPostTimeout> for String {
  fn from(value: IndicesShrinkPostTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesShrinkPostTimeout> for IndicesShrinkPostTimeout {
  fn from(value: &IndicesShrinkPostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesShrinkPostTimeout {
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

impl std::convert::TryFrom<&str> for IndicesShrinkPostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesShrinkPostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesShrinkPostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesShrinkPostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPutClusterManagerTimeout(String);
impl std::ops::Deref for IndicesShrinkPutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesShrinkPutClusterManagerTimeout> for String {
  fn from(value: IndicesShrinkPutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesShrinkPutClusterManagerTimeout> for IndicesShrinkPutClusterManagerTimeout {
  fn from(value: &IndicesShrinkPutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesShrinkPutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesShrinkPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesShrinkPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesShrinkPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesShrinkPutClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPutMasterTimeout(String);
impl std::ops::Deref for IndicesShrinkPutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesShrinkPutMasterTimeout> for String {
  fn from(value: IndicesShrinkPutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesShrinkPutMasterTimeout> for IndicesShrinkPutMasterTimeout {
  fn from(value: &IndicesShrinkPutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesShrinkPutMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesShrinkPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesShrinkPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesShrinkPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesShrinkPutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesShrinkPutTimeout(String);
impl std::ops::Deref for IndicesShrinkPutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesShrinkPutTimeout> for String {
  fn from(value: IndicesShrinkPutTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesShrinkPutTimeout> for IndicesShrinkPutTimeout {
  fn from(value: &IndicesShrinkPutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesShrinkPutTimeout {
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

impl std::convert::TryFrom<&str> for IndicesShrinkPutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesShrinkPutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesShrinkPutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesShrinkPutTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSimulateIndexTemplateClusterManagerTimeout(String);
impl std::ops::Deref for IndicesSimulateIndexTemplateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSimulateIndexTemplateClusterManagerTimeout> for String {
  fn from(value: IndicesSimulateIndexTemplateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSimulateIndexTemplateClusterManagerTimeout> for IndicesSimulateIndexTemplateClusterManagerTimeout {
  fn from(value: &IndicesSimulateIndexTemplateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSimulateIndexTemplateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSimulateIndexTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSimulateIndexTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSimulateIndexTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSimulateIndexTemplateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSimulateIndexTemplateMasterTimeout(String);
impl std::ops::Deref for IndicesSimulateIndexTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSimulateIndexTemplateMasterTimeout> for String {
  fn from(value: IndicesSimulateIndexTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSimulateIndexTemplateMasterTimeout> for IndicesSimulateIndexTemplateMasterTimeout {
  fn from(value: &IndicesSimulateIndexTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSimulateIndexTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSimulateIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSimulateIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSimulateIndexTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSimulateIndexTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSimulateTemplateClusterManagerTimeout(String);
impl std::ops::Deref for IndicesSimulateTemplateClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSimulateTemplateClusterManagerTimeout> for String {
  fn from(value: IndicesSimulateTemplateClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSimulateTemplateClusterManagerTimeout> for IndicesSimulateTemplateClusterManagerTimeout {
  fn from(value: &IndicesSimulateTemplateClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSimulateTemplateClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSimulateTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSimulateTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSimulateTemplateClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSimulateTemplateClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSimulateTemplateMasterTimeout(String);
impl std::ops::Deref for IndicesSimulateTemplateMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSimulateTemplateMasterTimeout> for String {
  fn from(value: IndicesSimulateTemplateMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSimulateTemplateMasterTimeout> for IndicesSimulateTemplateMasterTimeout {
  fn from(value: &IndicesSimulateTemplateMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSimulateTemplateMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSimulateTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSimulateTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSimulateTemplateMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSimulateTemplateMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSimulateTemplateWithNameClusterManagerTimeout(String);
impl std::ops::Deref for IndicesSimulateTemplateWithNameClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSimulateTemplateWithNameClusterManagerTimeout> for String {
  fn from(value: IndicesSimulateTemplateWithNameClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSimulateTemplateWithNameClusterManagerTimeout>
  for IndicesSimulateTemplateWithNameClusterManagerTimeout
{
  fn from(value: &IndicesSimulateTemplateWithNameClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSimulateTemplateWithNameClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSimulateTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSimulateTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSimulateTemplateWithNameClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSimulateTemplateWithNameClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSimulateTemplateWithNameMasterTimeout(String);
impl std::ops::Deref for IndicesSimulateTemplateWithNameMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSimulateTemplateWithNameMasterTimeout> for String {
  fn from(value: IndicesSimulateTemplateWithNameMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSimulateTemplateWithNameMasterTimeout> for IndicesSimulateTemplateWithNameMasterTimeout {
  fn from(value: &IndicesSimulateTemplateWithNameMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSimulateTemplateWithNameMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSimulateTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSimulateTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSimulateTemplateWithNameMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSimulateTemplateWithNameMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPostClusterManagerTimeout(String);
impl std::ops::Deref for IndicesSplitPostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSplitPostClusterManagerTimeout> for String {
  fn from(value: IndicesSplitPostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSplitPostClusterManagerTimeout> for IndicesSplitPostClusterManagerTimeout {
  fn from(value: &IndicesSplitPostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSplitPostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSplitPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSplitPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSplitPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSplitPostClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPostMasterTimeout(String);
impl std::ops::Deref for IndicesSplitPostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSplitPostMasterTimeout> for String {
  fn from(value: IndicesSplitPostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSplitPostMasterTimeout> for IndicesSplitPostMasterTimeout {
  fn from(value: &IndicesSplitPostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSplitPostMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSplitPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSplitPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSplitPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSplitPostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPostTimeout(String);
impl std::ops::Deref for IndicesSplitPostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSplitPostTimeout> for String {
  fn from(value: IndicesSplitPostTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSplitPostTimeout> for IndicesSplitPostTimeout {
  fn from(value: &IndicesSplitPostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSplitPostTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSplitPostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSplitPostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSplitPostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSplitPostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPutClusterManagerTimeout(String);
impl std::ops::Deref for IndicesSplitPutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSplitPutClusterManagerTimeout> for String {
  fn from(value: IndicesSplitPutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSplitPutClusterManagerTimeout> for IndicesSplitPutClusterManagerTimeout {
  fn from(value: &IndicesSplitPutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSplitPutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSplitPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSplitPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSplitPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSplitPutClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPutMasterTimeout(String);
impl std::ops::Deref for IndicesSplitPutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSplitPutMasterTimeout> for String {
  fn from(value: IndicesSplitPutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSplitPutMasterTimeout> for IndicesSplitPutMasterTimeout {
  fn from(value: &IndicesSplitPutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSplitPutMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSplitPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSplitPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSplitPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSplitPutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesSplitPutTimeout(String);
impl std::ops::Deref for IndicesSplitPutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesSplitPutTimeout> for String {
  fn from(value: IndicesSplitPutTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesSplitPutTimeout> for IndicesSplitPutTimeout {
  fn from(value: &IndicesSplitPutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesSplitPutTimeout {
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

impl std::convert::TryFrom<&str> for IndicesSplitPutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesSplitPutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesSplitPutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesSplitPutTimeout {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesUpdateAliasesClusterManagerTimeout(String);
impl std::ops::Deref for IndicesUpdateAliasesClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesUpdateAliasesClusterManagerTimeout> for String {
  fn from(value: IndicesUpdateAliasesClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesUpdateAliasesClusterManagerTimeout> for IndicesUpdateAliasesClusterManagerTimeout {
  fn from(value: &IndicesUpdateAliasesClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesUpdateAliasesClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for IndicesUpdateAliasesClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesUpdateAliasesClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesUpdateAliasesClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesUpdateAliasesClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesUpdateAliasesMasterTimeout(String);
impl std::ops::Deref for IndicesUpdateAliasesMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesUpdateAliasesMasterTimeout> for String {
  fn from(value: IndicesUpdateAliasesMasterTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesUpdateAliasesMasterTimeout> for IndicesUpdateAliasesMasterTimeout {
  fn from(value: &IndicesUpdateAliasesMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesUpdateAliasesMasterTimeout {
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

impl std::convert::TryFrom<&str> for IndicesUpdateAliasesMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesUpdateAliasesMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesUpdateAliasesMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesUpdateAliasesMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndicesUpdateAliasesTimeout(String);
impl std::ops::Deref for IndicesUpdateAliasesTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IndicesUpdateAliasesTimeout> for String {
  fn from(value: IndicesUpdateAliasesTimeout) -> Self {
    value.0
  }
}

impl From<&IndicesUpdateAliasesTimeout> for IndicesUpdateAliasesTimeout {
  fn from(value: &IndicesUpdateAliasesTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IndicesUpdateAliasesTimeout {
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

impl std::convert::TryFrom<&str> for IndicesUpdateAliasesTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IndicesUpdateAliasesTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IndicesUpdateAliasesTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IndicesUpdateAliasesTimeout {
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
