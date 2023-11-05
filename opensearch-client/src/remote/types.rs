#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

///Comma-separated list of index IDs
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

pub mod builder {
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
}
