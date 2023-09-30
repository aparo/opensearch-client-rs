
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCleanupRepositoryClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotCleanupRepositoryClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCleanupRepositoryClusterManagerTimeout> for String {
  fn from(value: SnapshotCleanupRepositoryClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCleanupRepositoryClusterManagerTimeout> for SnapshotCleanupRepositoryClusterManagerTimeout {
  fn from(value: &SnapshotCleanupRepositoryClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCleanupRepositoryClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCleanupRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCleanupRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCleanupRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCleanupRepositoryClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCleanupRepositoryMasterTimeout(String);
impl std::ops::Deref for SnapshotCleanupRepositoryMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCleanupRepositoryMasterTimeout> for String {
  fn from(value: SnapshotCleanupRepositoryMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCleanupRepositoryMasterTimeout> for SnapshotCleanupRepositoryMasterTimeout {
  fn from(value: &SnapshotCleanupRepositoryMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCleanupRepositoryMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCleanupRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCleanupRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCleanupRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCleanupRepositoryMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCleanupRepositoryTimeout(String);
impl std::ops::Deref for SnapshotCleanupRepositoryTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCleanupRepositoryTimeout> for String {
  fn from(value: SnapshotCleanupRepositoryTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCleanupRepositoryTimeout> for SnapshotCleanupRepositoryTimeout {
  fn from(value: &SnapshotCleanupRepositoryTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCleanupRepositoryTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCleanupRepositoryTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCleanupRepositoryTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCleanupRepositoryTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCleanupRepositoryTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCloneClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotCloneClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCloneClusterManagerTimeout> for String {
  fn from(value: SnapshotCloneClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCloneClusterManagerTimeout> for SnapshotCloneClusterManagerTimeout {
  fn from(value: &SnapshotCloneClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCloneClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCloneClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCloneClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCloneClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCloneClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCloneMasterTimeout(String);
impl std::ops::Deref for SnapshotCloneMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCloneMasterTimeout> for String {
  fn from(value: SnapshotCloneMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCloneMasterTimeout> for SnapshotCloneMasterTimeout {
  fn from(value: &SnapshotCloneMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCloneMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCloneMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCloneMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCloneMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCloneMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreatePostClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotCreatePostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreatePostClusterManagerTimeout> for String {
  fn from(value: SnapshotCreatePostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreatePostClusterManagerTimeout> for SnapshotCreatePostClusterManagerTimeout {
  fn from(value: &SnapshotCreatePostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreatePostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreatePostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreatePostClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreatePostMasterTimeout(String);
impl std::ops::Deref for SnapshotCreatePostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreatePostMasterTimeout> for String {
  fn from(value: SnapshotCreatePostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreatePostMasterTimeout> for SnapshotCreatePostMasterTimeout {
  fn from(value: &SnapshotCreatePostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreatePostMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreatePostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreatePostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreatePutClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotCreatePutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreatePutClusterManagerTimeout> for String {
  fn from(value: SnapshotCreatePutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreatePutClusterManagerTimeout> for SnapshotCreatePutClusterManagerTimeout {
  fn from(value: &SnapshotCreatePutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreatePutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreatePutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreatePutClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreatePutMasterTimeout(String);
impl std::ops::Deref for SnapshotCreatePutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreatePutMasterTimeout> for String {
  fn from(value: SnapshotCreatePutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreatePutMasterTimeout> for SnapshotCreatePutMasterTimeout {
  fn from(value: &SnapshotCreatePutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreatePutMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreatePutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreatePutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreateRepositoryPostClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotCreateRepositoryPostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreateRepositoryPostClusterManagerTimeout> for String {
  fn from(value: SnapshotCreateRepositoryPostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateRepositoryPostClusterManagerTimeout> for SnapshotCreateRepositoryPostClusterManagerTimeout {
  fn from(value: &SnapshotCreateRepositoryPostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreateRepositoryPostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreateRepositoryPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreateRepositoryPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreateRepositoryPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreateRepositoryPostClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreateRepositoryPostMasterTimeout(String);
impl std::ops::Deref for SnapshotCreateRepositoryPostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreateRepositoryPostMasterTimeout> for String {
  fn from(value: SnapshotCreateRepositoryPostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateRepositoryPostMasterTimeout> for SnapshotCreateRepositoryPostMasterTimeout {
  fn from(value: &SnapshotCreateRepositoryPostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreateRepositoryPostMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreateRepositoryPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreateRepositoryPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreateRepositoryPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreateRepositoryPostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreateRepositoryPostTimeout(String);
impl std::ops::Deref for SnapshotCreateRepositoryPostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreateRepositoryPostTimeout> for String {
  fn from(value: SnapshotCreateRepositoryPostTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateRepositoryPostTimeout> for SnapshotCreateRepositoryPostTimeout {
  fn from(value: &SnapshotCreateRepositoryPostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreateRepositoryPostTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreateRepositoryPostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreateRepositoryPostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreateRepositoryPostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreateRepositoryPostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreateRepositoryPutClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotCreateRepositoryPutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreateRepositoryPutClusterManagerTimeout> for String {
  fn from(value: SnapshotCreateRepositoryPutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateRepositoryPutClusterManagerTimeout> for SnapshotCreateRepositoryPutClusterManagerTimeout {
  fn from(value: &SnapshotCreateRepositoryPutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreateRepositoryPutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreateRepositoryPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreateRepositoryPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreateRepositoryPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreateRepositoryPutClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreateRepositoryPutMasterTimeout(String);
impl std::ops::Deref for SnapshotCreateRepositoryPutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreateRepositoryPutMasterTimeout> for String {
  fn from(value: SnapshotCreateRepositoryPutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateRepositoryPutMasterTimeout> for SnapshotCreateRepositoryPutMasterTimeout {
  fn from(value: &SnapshotCreateRepositoryPutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreateRepositoryPutMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreateRepositoryPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreateRepositoryPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreateRepositoryPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreateRepositoryPutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotCreateRepositoryPutTimeout(String);
impl std::ops::Deref for SnapshotCreateRepositoryPutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotCreateRepositoryPutTimeout> for String {
  fn from(value: SnapshotCreateRepositoryPutTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotCreateRepositoryPutTimeout> for SnapshotCreateRepositoryPutTimeout {
  fn from(value: &SnapshotCreateRepositoryPutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotCreateRepositoryPutTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotCreateRepositoryPutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotCreateRepositoryPutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotCreateRepositoryPutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotCreateRepositoryPutTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotDeleteClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotDeleteClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotDeleteClusterManagerTimeout> for String {
  fn from(value: SnapshotDeleteClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotDeleteClusterManagerTimeout> for SnapshotDeleteClusterManagerTimeout {
  fn from(value: &SnapshotDeleteClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotDeleteClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotDeleteClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotDeleteClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotDeleteClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotDeleteClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotDeleteMasterTimeout(String);
impl std::ops::Deref for SnapshotDeleteMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotDeleteMasterTimeout> for String {
  fn from(value: SnapshotDeleteMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotDeleteMasterTimeout> for SnapshotDeleteMasterTimeout {
  fn from(value: &SnapshotDeleteMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotDeleteMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotDeleteMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotDeleteMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotDeleteMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotDeleteMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotDeleteRepositoryClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotDeleteRepositoryClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotDeleteRepositoryClusterManagerTimeout> for String {
  fn from(value: SnapshotDeleteRepositoryClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotDeleteRepositoryClusterManagerTimeout> for SnapshotDeleteRepositoryClusterManagerTimeout {
  fn from(value: &SnapshotDeleteRepositoryClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotDeleteRepositoryClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotDeleteRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotDeleteRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotDeleteRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotDeleteRepositoryClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotDeleteRepositoryMasterTimeout(String);
impl std::ops::Deref for SnapshotDeleteRepositoryMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotDeleteRepositoryMasterTimeout> for String {
  fn from(value: SnapshotDeleteRepositoryMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotDeleteRepositoryMasterTimeout> for SnapshotDeleteRepositoryMasterTimeout {
  fn from(value: &SnapshotDeleteRepositoryMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotDeleteRepositoryMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotDeleteRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotDeleteRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotDeleteRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotDeleteRepositoryMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotDeleteRepositoryTimeout(String);
impl std::ops::Deref for SnapshotDeleteRepositoryTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotDeleteRepositoryTimeout> for String {
  fn from(value: SnapshotDeleteRepositoryTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotDeleteRepositoryTimeout> for SnapshotDeleteRepositoryTimeout {
  fn from(value: &SnapshotDeleteRepositoryTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotDeleteRepositoryTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotDeleteRepositoryTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotDeleteRepositoryTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotDeleteRepositoryTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotDeleteRepositoryTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotGetClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotGetClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotGetClusterManagerTimeout> for String {
  fn from(value: SnapshotGetClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotGetClusterManagerTimeout> for SnapshotGetClusterManagerTimeout {
  fn from(value: &SnapshotGetClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotGetClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotGetClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotGetClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotGetClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotGetClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotGetMasterTimeout(String);
impl std::ops::Deref for SnapshotGetMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotGetMasterTimeout> for String {
  fn from(value: SnapshotGetMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotGetMasterTimeout> for SnapshotGetMasterTimeout {
  fn from(value: &SnapshotGetMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotGetMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotGetMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotGetMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotGetMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotGetMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotGetRepositoryClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotGetRepositoryClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotGetRepositoryClusterManagerTimeout> for String {
  fn from(value: SnapshotGetRepositoryClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotGetRepositoryClusterManagerTimeout> for SnapshotGetRepositoryClusterManagerTimeout {
  fn from(value: &SnapshotGetRepositoryClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotGetRepositoryClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotGetRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotGetRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotGetRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotGetRepositoryClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotGetRepositoryMasterTimeout(String);
impl std::ops::Deref for SnapshotGetRepositoryMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotGetRepositoryMasterTimeout> for String {
  fn from(value: SnapshotGetRepositoryMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotGetRepositoryMasterTimeout> for SnapshotGetRepositoryMasterTimeout {
  fn from(value: &SnapshotGetRepositoryMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotGetRepositoryMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotGetRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotGetRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotGetRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotGetRepositoryMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotGetRepositoryWithRepositoryClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotGetRepositoryWithRepositoryClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotGetRepositoryWithRepositoryClusterManagerTimeout> for String {
  fn from(value: SnapshotGetRepositoryWithRepositoryClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotGetRepositoryWithRepositoryClusterManagerTimeout>
  for SnapshotGetRepositoryWithRepositoryClusterManagerTimeout
{
  fn from(value: &SnapshotGetRepositoryWithRepositoryClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotGetRepositoryWithRepositoryClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotGetRepositoryWithRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotGetRepositoryWithRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotGetRepositoryWithRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotGetRepositoryWithRepositoryClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotGetRepositoryWithRepositoryMasterTimeout(String);
impl std::ops::Deref for SnapshotGetRepositoryWithRepositoryMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotGetRepositoryWithRepositoryMasterTimeout> for String {
  fn from(value: SnapshotGetRepositoryWithRepositoryMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotGetRepositoryWithRepositoryMasterTimeout> for SnapshotGetRepositoryWithRepositoryMasterTimeout {
  fn from(value: &SnapshotGetRepositoryWithRepositoryMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotGetRepositoryWithRepositoryMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotGetRepositoryWithRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotGetRepositoryWithRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotGetRepositoryWithRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotGetRepositoryWithRepositoryMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotRestoreClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotRestoreClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotRestoreClusterManagerTimeout> for String {
  fn from(value: SnapshotRestoreClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotRestoreClusterManagerTimeout> for SnapshotRestoreClusterManagerTimeout {
  fn from(value: &SnapshotRestoreClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotRestoreClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotRestoreClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotRestoreClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotRestoreClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotRestoreClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotRestoreMasterTimeout(String);
impl std::ops::Deref for SnapshotRestoreMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotRestoreMasterTimeout> for String {
  fn from(value: SnapshotRestoreMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotRestoreMasterTimeout> for SnapshotRestoreMasterTimeout {
  fn from(value: &SnapshotRestoreMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotRestoreMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotRestoreMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotRestoreMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotRestoreMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotRestoreMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotStatusClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotStatusClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotStatusClusterManagerTimeout> for String {
  fn from(value: SnapshotStatusClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotStatusClusterManagerTimeout> for SnapshotStatusClusterManagerTimeout {
  fn from(value: &SnapshotStatusClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotStatusClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotStatusClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotStatusClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotStatusClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotStatusClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotStatusMasterTimeout(String);
impl std::ops::Deref for SnapshotStatusMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotStatusMasterTimeout> for String {
  fn from(value: SnapshotStatusMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotStatusMasterTimeout> for SnapshotStatusMasterTimeout {
  fn from(value: &SnapshotStatusMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotStatusMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotStatusMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotStatusMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotStatusMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotStatusMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotStatusWithRepositoryClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotStatusWithRepositoryClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotStatusWithRepositoryClusterManagerTimeout> for String {
  fn from(value: SnapshotStatusWithRepositoryClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotStatusWithRepositoryClusterManagerTimeout> for SnapshotStatusWithRepositoryClusterManagerTimeout {
  fn from(value: &SnapshotStatusWithRepositoryClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotStatusWithRepositoryClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotStatusWithRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotStatusWithRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotStatusWithRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotStatusWithRepositoryClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotStatusWithRepositoryMasterTimeout(String);
impl std::ops::Deref for SnapshotStatusWithRepositoryMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotStatusWithRepositoryMasterTimeout> for String {
  fn from(value: SnapshotStatusWithRepositoryMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotStatusWithRepositoryMasterTimeout> for SnapshotStatusWithRepositoryMasterTimeout {
  fn from(value: &SnapshotStatusWithRepositoryMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotStatusWithRepositoryMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotStatusWithRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotStatusWithRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotStatusWithRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotStatusWithRepositoryMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotStatusWithRepositorySnapshotClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotStatusWithRepositorySnapshotClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotStatusWithRepositorySnapshotClusterManagerTimeout> for String {
  fn from(value: SnapshotStatusWithRepositorySnapshotClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotStatusWithRepositorySnapshotClusterManagerTimeout>
  for SnapshotStatusWithRepositorySnapshotClusterManagerTimeout
{
  fn from(value: &SnapshotStatusWithRepositorySnapshotClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotStatusWithRepositorySnapshotClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotStatusWithRepositorySnapshotClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotStatusWithRepositorySnapshotClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotStatusWithRepositorySnapshotClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotStatusWithRepositorySnapshotClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotStatusWithRepositorySnapshotMasterTimeout(String);
impl std::ops::Deref for SnapshotStatusWithRepositorySnapshotMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotStatusWithRepositorySnapshotMasterTimeout> for String {
  fn from(value: SnapshotStatusWithRepositorySnapshotMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotStatusWithRepositorySnapshotMasterTimeout> for SnapshotStatusWithRepositorySnapshotMasterTimeout {
  fn from(value: &SnapshotStatusWithRepositorySnapshotMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotStatusWithRepositorySnapshotMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotStatusWithRepositorySnapshotMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotStatusWithRepositorySnapshotMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotStatusWithRepositorySnapshotMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotStatusWithRepositorySnapshotMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotVerifyRepositoryClusterManagerTimeout(String);
impl std::ops::Deref for SnapshotVerifyRepositoryClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotVerifyRepositoryClusterManagerTimeout> for String {
  fn from(value: SnapshotVerifyRepositoryClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotVerifyRepositoryClusterManagerTimeout> for SnapshotVerifyRepositoryClusterManagerTimeout {
  fn from(value: &SnapshotVerifyRepositoryClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotVerifyRepositoryClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotVerifyRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotVerifyRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotVerifyRepositoryClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotVerifyRepositoryClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotVerifyRepositoryMasterTimeout(String);
impl std::ops::Deref for SnapshotVerifyRepositoryMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotVerifyRepositoryMasterTimeout> for String {
  fn from(value: SnapshotVerifyRepositoryMasterTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotVerifyRepositoryMasterTimeout> for SnapshotVerifyRepositoryMasterTimeout {
  fn from(value: &SnapshotVerifyRepositoryMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotVerifyRepositoryMasterTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotVerifyRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotVerifyRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotVerifyRepositoryMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotVerifyRepositoryMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotVerifyRepositoryTimeout(String);
impl std::ops::Deref for SnapshotVerifyRepositoryTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<SnapshotVerifyRepositoryTimeout> for String {
  fn from(value: SnapshotVerifyRepositoryTimeout) -> Self {
    value.0
  }
}

impl From<&SnapshotVerifyRepositoryTimeout> for SnapshotVerifyRepositoryTimeout {
  fn from(value: &SnapshotVerifyRepositoryTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for SnapshotVerifyRepositoryTimeout {
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

impl std::convert::TryFrom<&str> for SnapshotVerifyRepositoryTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for SnapshotVerifyRepositoryTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for SnapshotVerifyRepositoryTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for SnapshotVerifyRepositoryTimeout {
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
