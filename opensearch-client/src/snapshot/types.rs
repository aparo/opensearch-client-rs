#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
