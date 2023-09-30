
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesDeleteDanglingIndexClusterManagerTimeout(String);
impl std::ops::Deref for DanglingIndicesDeleteDanglingIndexClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesDeleteDanglingIndexClusterManagerTimeout> for String {
  fn from(value: DanglingIndicesDeleteDanglingIndexClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesDeleteDanglingIndexClusterManagerTimeout>
  for DanglingIndicesDeleteDanglingIndexClusterManagerTimeout
{
  fn from(value: &DanglingIndicesDeleteDanglingIndexClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesDeleteDanglingIndexClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for DanglingIndicesDeleteDanglingIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesDeleteDanglingIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesDeleteDanglingIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesDeleteDanglingIndexClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesDeleteDanglingIndexIndexUuid(String);
impl std::ops::Deref for DanglingIndicesDeleteDanglingIndexIndexUuid {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesDeleteDanglingIndexIndexUuid> for String {
  fn from(value: DanglingIndicesDeleteDanglingIndexIndexUuid) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesDeleteDanglingIndexIndexUuid> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  fn from(value: &DanglingIndicesDeleteDanglingIndexIndexUuid) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesDeleteDanglingIndexIndexUuid {
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

impl std::convert::TryFrom<&str> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesDeleteDanglingIndexIndexUuid {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesDeleteDanglingIndexMasterTimeout(String);
impl std::ops::Deref for DanglingIndicesDeleteDanglingIndexMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesDeleteDanglingIndexMasterTimeout> for String {
  fn from(value: DanglingIndicesDeleteDanglingIndexMasterTimeout) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesDeleteDanglingIndexMasterTimeout> for DanglingIndicesDeleteDanglingIndexMasterTimeout {
  fn from(value: &DanglingIndicesDeleteDanglingIndexMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesDeleteDanglingIndexMasterTimeout {
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

impl std::convert::TryFrom<&str> for DanglingIndicesDeleteDanglingIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesDeleteDanglingIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesDeleteDanglingIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesDeleteDanglingIndexMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesDeleteDanglingIndexTimeout(String);
impl std::ops::Deref for DanglingIndicesDeleteDanglingIndexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesDeleteDanglingIndexTimeout> for String {
  fn from(value: DanglingIndicesDeleteDanglingIndexTimeout) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesDeleteDanglingIndexTimeout> for DanglingIndicesDeleteDanglingIndexTimeout {
  fn from(value: &DanglingIndicesDeleteDanglingIndexTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesDeleteDanglingIndexTimeout {
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

impl std::convert::TryFrom<&str> for DanglingIndicesDeleteDanglingIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesDeleteDanglingIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesDeleteDanglingIndexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesDeleteDanglingIndexTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesImportDanglingIndexClusterManagerTimeout(String);
impl std::ops::Deref for DanglingIndicesImportDanglingIndexClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesImportDanglingIndexClusterManagerTimeout> for String {
  fn from(value: DanglingIndicesImportDanglingIndexClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesImportDanglingIndexClusterManagerTimeout>
  for DanglingIndicesImportDanglingIndexClusterManagerTimeout
{
  fn from(value: &DanglingIndicesImportDanglingIndexClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesImportDanglingIndexClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for DanglingIndicesImportDanglingIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesImportDanglingIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesImportDanglingIndexClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesImportDanglingIndexClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesImportDanglingIndexIndexUuid(String);
impl std::ops::Deref for DanglingIndicesImportDanglingIndexIndexUuid {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesImportDanglingIndexIndexUuid> for String {
  fn from(value: DanglingIndicesImportDanglingIndexIndexUuid) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesImportDanglingIndexIndexUuid> for DanglingIndicesImportDanglingIndexIndexUuid {
  fn from(value: &DanglingIndicesImportDanglingIndexIndexUuid) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesImportDanglingIndexIndexUuid {
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

impl std::convert::TryFrom<&str> for DanglingIndicesImportDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesImportDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesImportDanglingIndexIndexUuid {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesImportDanglingIndexIndexUuid {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DanglingIndicesImportDanglingIndexMasterTimeout(String);
impl std::ops::Deref for DanglingIndicesImportDanglingIndexMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<DanglingIndicesImportDanglingIndexMasterTimeout> for String {
  fn from(value: DanglingIndicesImportDanglingIndexMasterTimeout) -> Self {
    value.0
  }
}

impl From<&DanglingIndicesImportDanglingIndexMasterTimeout> for DanglingIndicesImportDanglingIndexMasterTimeout {
  fn from(value: &DanglingIndicesImportDanglingIndexMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for DanglingIndicesImportDanglingIndexMasterTimeout {
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

impl std::convert::TryFrom<&str> for DanglingIndicesImportDanglingIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for DanglingIndicesImportDanglingIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for DanglingIndicesImportDanglingIndexMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for DanglingIndicesImportDanglingIndexMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
