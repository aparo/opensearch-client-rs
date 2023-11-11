#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
