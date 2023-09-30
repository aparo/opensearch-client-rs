
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PutScriptBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for PutScriptBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<PutScriptBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: PutScriptBodyParams) -> Self {
    value.0
  }
}

impl From<&PutScriptBodyParams> for PutScriptBodyParams {
  fn from(value: &PutScriptBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for PutScriptBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostClusterManagerTimeout(String);
impl std::ops::Deref for PutScriptPostClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostClusterManagerTimeout> for String {
  fn from(value: PutScriptPostClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPostClusterManagerTimeout> for PutScriptPostClusterManagerTimeout {
  fn from(value: &PutScriptPostClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostId(String);
impl std::ops::Deref for PutScriptPostId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostId> for String {
  fn from(value: PutScriptPostId) -> Self {
    value.0
  }
}

impl From<&PutScriptPostId> for PutScriptPostId {
  fn from(value: &PutScriptPostId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostId {
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

impl std::convert::TryFrom<&str> for PutScriptPostId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostMasterTimeout(String);
impl std::ops::Deref for PutScriptPostMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostMasterTimeout> for String {
  fn from(value: PutScriptPostMasterTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPostMasterTimeout> for PutScriptPostMasterTimeout {
  fn from(value: &PutScriptPostMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostMasterTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostTimeout(String);
impl std::ops::Deref for PutScriptPostTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostTimeout> for String {
  fn from(value: PutScriptPostTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPostTimeout> for PutScriptPostTimeout {
  fn from(value: &PutScriptPostTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPostTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostWithContextClusterManagerTimeout(String);
impl std::ops::Deref for PutScriptPostWithContextClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostWithContextClusterManagerTimeout> for String {
  fn from(value: PutScriptPostWithContextClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPostWithContextClusterManagerTimeout> for PutScriptPostWithContextClusterManagerTimeout {
  fn from(value: &PutScriptPostWithContextClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostWithContextClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPostWithContextClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostWithContextClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostWithContextClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostWithContextClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostWithContextContext(String);
impl std::ops::Deref for PutScriptPostWithContextContext {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostWithContextContext> for String {
  fn from(value: PutScriptPostWithContextContext) -> Self {
    value.0
  }
}

impl From<&PutScriptPostWithContextContext> for PutScriptPostWithContextContext {
  fn from(value: &PutScriptPostWithContextContext) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostWithContextContext {
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

impl std::convert::TryFrom<&str> for PutScriptPostWithContextContext {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostWithContextContext {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostWithContextContext {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostWithContextContext {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostWithContextId(String);
impl std::ops::Deref for PutScriptPostWithContextId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostWithContextId> for String {
  fn from(value: PutScriptPostWithContextId) -> Self {
    value.0
  }
}

impl From<&PutScriptPostWithContextId> for PutScriptPostWithContextId {
  fn from(value: &PutScriptPostWithContextId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostWithContextId {
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

impl std::convert::TryFrom<&str> for PutScriptPostWithContextId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostWithContextId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostWithContextId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostWithContextId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostWithContextMasterTimeout(String);
impl std::ops::Deref for PutScriptPostWithContextMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostWithContextMasterTimeout> for String {
  fn from(value: PutScriptPostWithContextMasterTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPostWithContextMasterTimeout> for PutScriptPostWithContextMasterTimeout {
  fn from(value: &PutScriptPostWithContextMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostWithContextMasterTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPostWithContextMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostWithContextMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostWithContextMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostWithContextMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPostWithContextTimeout(String);
impl std::ops::Deref for PutScriptPostWithContextTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPostWithContextTimeout> for String {
  fn from(value: PutScriptPostWithContextTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPostWithContextTimeout> for PutScriptPostWithContextTimeout {
  fn from(value: &PutScriptPostWithContextTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPostWithContextTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPostWithContextTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPostWithContextTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPostWithContextTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPostWithContextTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutClusterManagerTimeout(String);
impl std::ops::Deref for PutScriptPutClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutClusterManagerTimeout> for String {
  fn from(value: PutScriptPutClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPutClusterManagerTimeout> for PutScriptPutClusterManagerTimeout {
  fn from(value: &PutScriptPutClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutId(String);
impl std::ops::Deref for PutScriptPutId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutId> for String {
  fn from(value: PutScriptPutId) -> Self {
    value.0
  }
}

impl From<&PutScriptPutId> for PutScriptPutId {
  fn from(value: &PutScriptPutId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutId {
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

impl std::convert::TryFrom<&str> for PutScriptPutId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutMasterTimeout(String);
impl std::ops::Deref for PutScriptPutMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutMasterTimeout> for String {
  fn from(value: PutScriptPutMasterTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPutMasterTimeout> for PutScriptPutMasterTimeout {
  fn from(value: &PutScriptPutMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutMasterTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutTimeout(String);
impl std::ops::Deref for PutScriptPutTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutTimeout> for String {
  fn from(value: PutScriptPutTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPutTimeout> for PutScriptPutTimeout {
  fn from(value: &PutScriptPutTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPutTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutWithContextClusterManagerTimeout(String);
impl std::ops::Deref for PutScriptPutWithContextClusterManagerTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutWithContextClusterManagerTimeout> for String {
  fn from(value: PutScriptPutWithContextClusterManagerTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPutWithContextClusterManagerTimeout> for PutScriptPutWithContextClusterManagerTimeout {
  fn from(value: &PutScriptPutWithContextClusterManagerTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutWithContextClusterManagerTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPutWithContextClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutWithContextClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutWithContextClusterManagerTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutWithContextClusterManagerTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutWithContextContext(String);
impl std::ops::Deref for PutScriptPutWithContextContext {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutWithContextContext> for String {
  fn from(value: PutScriptPutWithContextContext) -> Self {
    value.0
  }
}

impl From<&PutScriptPutWithContextContext> for PutScriptPutWithContextContext {
  fn from(value: &PutScriptPutWithContextContext) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutWithContextContext {
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

impl std::convert::TryFrom<&str> for PutScriptPutWithContextContext {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutWithContextContext {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutWithContextContext {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutWithContextContext {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutWithContextId(String);
impl std::ops::Deref for PutScriptPutWithContextId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutWithContextId> for String {
  fn from(value: PutScriptPutWithContextId) -> Self {
    value.0
  }
}

impl From<&PutScriptPutWithContextId> for PutScriptPutWithContextId {
  fn from(value: &PutScriptPutWithContextId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutWithContextId {
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

impl std::convert::TryFrom<&str> for PutScriptPutWithContextId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutWithContextId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutWithContextId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutWithContextId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutWithContextMasterTimeout(String);
impl std::ops::Deref for PutScriptPutWithContextMasterTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutWithContextMasterTimeout> for String {
  fn from(value: PutScriptPutWithContextMasterTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPutWithContextMasterTimeout> for PutScriptPutWithContextMasterTimeout {
  fn from(value: &PutScriptPutWithContextMasterTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutWithContextMasterTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPutWithContextMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutWithContextMasterTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutWithContextMasterTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutWithContextMasterTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PutScriptPutWithContextTimeout(String);
impl std::ops::Deref for PutScriptPutWithContextTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<PutScriptPutWithContextTimeout> for String {
  fn from(value: PutScriptPutWithContextTimeout) -> Self {
    value.0
  }
}

impl From<&PutScriptPutWithContextTimeout> for PutScriptPutWithContextTimeout {
  fn from(value: &PutScriptPutWithContextTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for PutScriptPutWithContextTimeout {
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

impl std::convert::TryFrom<&str> for PutScriptPutWithContextTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for PutScriptPutWithContextTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for PutScriptPutWithContextTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for PutScriptPutWithContextTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
