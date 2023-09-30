
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

/// newlines
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BulkBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for BulkBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<BulkBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: BulkBodyParams) -> Self {
    value.0
  }
}

impl From<&BulkBodyParams> for BulkBodyParams {
  fn from(value: &BulkBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for BulkBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BulkPostTimeout(String);
impl std::ops::Deref for BulkPostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<BulkPostTimeout> for String {
  fn from(value: BulkPostTimeout) -> Self {
    value.0
  }
}

impl From<&BulkPostTimeout> for BulkPostTimeout {
  fn from(value: &BulkPostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for BulkPostTimeout {
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

impl std::convert::TryFrom<&str> for BulkPostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for BulkPostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for BulkPostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for BulkPostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BulkPostWithIndexIndex(String);
impl std::ops::Deref for BulkPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<BulkPostWithIndexIndex> for String {
  fn from(value: BulkPostWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&BulkPostWithIndexIndex> for BulkPostWithIndexIndex {
  fn from(value: &BulkPostWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for BulkPostWithIndexIndex {
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

impl std::convert::TryFrom<&str> for BulkPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for BulkPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for BulkPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for BulkPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BulkPostWithIndexTimeout(String);
impl std::ops::Deref for BulkPostWithIndexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<BulkPostWithIndexTimeout> for String {
  fn from(value: BulkPostWithIndexTimeout) -> Self {
    value.0
  }
}

impl From<&BulkPostWithIndexTimeout> for BulkPostWithIndexTimeout {
  fn from(value: &BulkPostWithIndexTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for BulkPostWithIndexTimeout {
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

impl std::convert::TryFrom<&str> for BulkPostWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for BulkPostWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for BulkPostWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for BulkPostWithIndexTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BulkPutTimeout(String);
impl std::ops::Deref for BulkPutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<BulkPutTimeout> for String {
  fn from(value: BulkPutTimeout) -> Self {
    value.0
  }
}

impl From<&BulkPutTimeout> for BulkPutTimeout {
  fn from(value: &BulkPutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for BulkPutTimeout {
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

impl std::convert::TryFrom<&str> for BulkPutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for BulkPutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for BulkPutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for BulkPutTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BulkPutWithIndexIndex(String);
impl std::ops::Deref for BulkPutWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<BulkPutWithIndexIndex> for String {
  fn from(value: BulkPutWithIndexIndex) -> Self {
    value.0
  }
}

impl From<&BulkPutWithIndexIndex> for BulkPutWithIndexIndex {
  fn from(value: &BulkPutWithIndexIndex) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for BulkPutWithIndexIndex {
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

impl std::convert::TryFrom<&str> for BulkPutWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for BulkPutWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for BulkPutWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for BulkPutWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BulkPutWithIndexTimeout(String);
impl std::ops::Deref for BulkPutWithIndexTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<BulkPutWithIndexTimeout> for String {
  fn from(value: BulkPutWithIndexTimeout) -> Self {
    value.0
  }
}

impl From<&BulkPutWithIndexTimeout> for BulkPutWithIndexTimeout {
  fn from(value: &BulkPutWithIndexTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for BulkPutWithIndexTimeout {
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

impl std::convert::TryFrom<&str> for BulkPutWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for BulkPutWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for BulkPutWithIndexTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for BulkPutWithIndexTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
