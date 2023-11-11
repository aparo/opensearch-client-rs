#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatAllPitSegmentsResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub content: Option<CatPitSegment>,
}
impl From<&CatAllPitSegmentsResponseContent> for CatAllPitSegmentsResponseContent {
  fn from(value: &CatAllPitSegmentsResponseContent) -> Self {
    value.clone()
  }
}
impl CatAllPitSegmentsResponseContent {
  pub fn builder() -> builder::CatAllPitSegmentsResponseContent {
    builder::CatAllPitSegmentsResponseContent::default()
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatPitSegment {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub committed: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub compound: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub docs_count: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub docs_deleted: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub generation: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ip: Option<String>,
  ///Set to true to return stats only for primary shards.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub prirep: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub searchable: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub segment: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub shard: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub size: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub size_memory: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
}
impl From<&CatPitSegment> for CatPitSegment {
  fn from(value: &CatPitSegment) -> Self {
    value.clone()
  }
}
impl CatPitSegment {
  pub fn builder() -> builder::CatPitSegment {
    builder::CatPitSegment::default()
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatPitSegmentsBodyParams {
  pub pit_id: Vec<String>,
}
impl From<&CatPitSegmentsBodyParams> for CatPitSegmentsBodyParams {
  fn from(value: &CatPitSegmentsBodyParams) -> Self {
    value.clone()
  }
}
impl CatPitSegmentsBodyParams {
  pub fn builder() -> builder::CatPitSegmentsBodyParams {
    builder::CatPitSegmentsBodyParams::default()
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatPitSegmentsResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub content: Option<CatPitSegment>,
}
impl From<&CatPitSegmentsResponseContent> for CatPitSegmentsResponseContent {
  fn from(value: &CatPitSegmentsResponseContent) -> Self {
    value.clone()
  }
}
impl CatPitSegmentsResponseContent {
  pub fn builder() -> builder::CatPitSegmentsResponseContent {
    builder::CatPitSegmentsResponseContent::default()
  }
}

pub mod builder {
  #[derive(Clone, Debug)]
  pub struct CatAllPitSegmentsResponseContent {
    content: Result<Option<super::CatPitSegment>, String>,
  }

  impl Default for CatAllPitSegmentsResponseContent {
    fn default() -> Self {
      Self {
        content: Ok(Default::default()),
      }
    }
  }

  impl CatAllPitSegmentsResponseContent {
    pub fn content<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::CatPitSegment>>,
      T::Error: std::fmt::Display, {
      self.content = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for content: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CatAllPitSegmentsResponseContent> for super::CatAllPitSegmentsResponseContent {
    type Error = String;

    fn try_from(value: CatAllPitSegmentsResponseContent) -> Result<Self, String> {
      Ok(Self {
        content: value.content?,
      })
    }
  }

  impl From<super::CatAllPitSegmentsResponseContent> for CatAllPitSegmentsResponseContent {
    fn from(value: super::CatAllPitSegmentsResponseContent) -> Self {
      Self {
        content: Ok(value.content),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CatPitSegment {
    committed: Result<Option<bool>, String>,
    compound: Result<Option<bool>, String>,
    docs_count: Result<Option<i32>, String>,
    docs_deleted: Result<Option<i32>, String>,
    generation: Result<Option<i32>, String>,
    index: Result<Option<String>, String>,
    ip: Result<Option<String>, String>,
    prirep: Result<Option<bool>, String>,
    searchable: Result<Option<bool>, String>,
    segment: Result<Option<String>, String>,
    shard: Result<Option<i32>, String>,
    size: Result<Option<String>, String>,
    size_memory: Result<Option<i32>, String>,
    version: Result<Option<String>, String>,
  }

  impl Default for CatPitSegment {
    fn default() -> Self {
      Self {
        committed: Ok(Default::default()),
        compound: Ok(Default::default()),
        docs_count: Ok(Default::default()),
        docs_deleted: Ok(Default::default()),
        generation: Ok(Default::default()),
        index: Ok(Default::default()),
        ip: Ok(Default::default()),
        prirep: Ok(Default::default()),
        searchable: Ok(Default::default()),
        segment: Ok(Default::default()),
        shard: Ok(Default::default()),
        size: Ok(Default::default()),
        size_memory: Ok(Default::default()),
        version: Ok(Default::default()),
      }
    }
  }

  impl CatPitSegment {
    pub fn committed<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.committed = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for committed: {}", e));
      self
    }

    pub fn compound<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.compound = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for compound: {}", e));
      self
    }

    pub fn docs_count<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.docs_count = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for docs_count: {}", e));
      self
    }

    pub fn docs_deleted<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.docs_deleted = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for docs_deleted: {}", e));
      self
    }

    pub fn generation<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.generation = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for generation: {}", e));
      self
    }

    pub fn index<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.index = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index: {}", e));
      self
    }

    pub fn ip<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.ip = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for ip: {}", e));
      self
    }

    pub fn prirep<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.prirep = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for prirep: {}", e));
      self
    }

    pub fn searchable<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.searchable = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for searchable: {}", e));
      self
    }

    pub fn segment<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.segment = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for segment: {}", e));
      self
    }

    pub fn shard<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.shard = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for shard: {}", e));
      self
    }

    pub fn size<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.size = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for size: {}", e));
      self
    }

    pub fn size_memory<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<i32>>,
      T::Error: std::fmt::Display, {
      self.size_memory = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for size_memory: {}", e));
      self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.version = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for version: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CatPitSegment> for super::CatPitSegment {
    type Error = String;

    fn try_from(value: CatPitSegment) -> Result<Self, String> {
      Ok(Self {
        committed: value.committed?,
        compound: value.compound?,
        docs_count: value.docs_count?,
        docs_deleted: value.docs_deleted?,
        generation: value.generation?,
        index: value.index?,
        ip: value.ip?,
        prirep: value.prirep?,
        searchable: value.searchable?,
        segment: value.segment?,
        shard: value.shard?,
        size: value.size?,
        size_memory: value.size_memory?,
        version: value.version?,
      })
    }
  }

  impl From<super::CatPitSegment> for CatPitSegment {
    fn from(value: super::CatPitSegment) -> Self {
      Self {
        committed: Ok(value.committed),
        compound: Ok(value.compound),
        docs_count: Ok(value.docs_count),
        docs_deleted: Ok(value.docs_deleted),
        generation: Ok(value.generation),
        index: Ok(value.index),
        ip: Ok(value.ip),
        prirep: Ok(value.prirep),
        searchable: Ok(value.searchable),
        segment: Ok(value.segment),
        shard: Ok(value.shard),
        size: Ok(value.size),
        size_memory: Ok(value.size_memory),
        version: Ok(value.version),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CatPitSegmentsBodyParams {
    pit_id: Result<Vec<String>, String>,
  }

  impl Default for CatPitSegmentsBodyParams {
    fn default() -> Self {
      Self {
        pit_id: Err("no value supplied for pit_id".to_string()),
      }
    }
  }

  impl CatPitSegmentsBodyParams {
    pub fn pit_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.pit_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for pit_id: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CatPitSegmentsBodyParams> for super::CatPitSegmentsBodyParams {
    type Error = String;

    fn try_from(value: CatPitSegmentsBodyParams) -> Result<Self, String> {
      Ok(Self { pit_id: value.pit_id? })
    }
  }

  impl From<super::CatPitSegmentsBodyParams> for CatPitSegmentsBodyParams {
    fn from(value: super::CatPitSegmentsBodyParams) -> Self {
      Self {
        pit_id: Ok(value.pit_id),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CatPitSegmentsResponseContent {
    content: Result<Option<super::CatPitSegment>, String>,
  }

  impl Default for CatPitSegmentsResponseContent {
    fn default() -> Self {
      Self {
        content: Ok(Default::default()),
      }
    }
  }

  impl CatPitSegmentsResponseContent {
    pub fn content<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::CatPitSegment>>,
      T::Error: std::fmt::Display, {
      self.content = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for content: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CatPitSegmentsResponseContent> for super::CatPitSegmentsResponseContent {
    type Error = String;

    fn try_from(value: CatPitSegmentsResponseContent) -> Result<Self, String> {
      Ok(Self {
        content: value.content?,
      })
    }
  }

  impl From<super::CatPitSegmentsResponseContent> for CatPitSegmentsResponseContent {
    fn from(value: super::CatPitSegmentsResponseContent) -> Self {
      Self {
        content: Ok(value.content),
      }
    }
  }
}
