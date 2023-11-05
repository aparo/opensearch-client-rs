#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};









///Pipeline ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IngestDeletePipelineId(String);
impl std::ops::Deref for IngestDeletePipelineId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IngestDeletePipelineId> for String {
  fn from(value: IngestDeletePipelineId) -> Self {
    value.0
  }
}

impl From<&IngestDeletePipelineId> for IngestDeletePipelineId {
  fn from(value: &IngestDeletePipelineId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IngestDeletePipelineId {
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

impl std::convert::TryFrom<&str> for IngestDeletePipelineId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IngestDeletePipelineId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IngestDeletePipelineId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IngestDeletePipelineId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Operation timeout.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IngestDeletePipelineTimeout(String);
impl std::ops::Deref for IngestDeletePipelineTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IngestDeletePipelineTimeout> for String {
  fn from(value: IngestDeletePipelineTimeout) -> Self {
    value.0
  }
}

impl From<&IngestDeletePipelineTimeout> for IngestDeletePipelineTimeout {
  fn from(value: &IngestDeletePipelineTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IngestDeletePipelineTimeout {
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

impl std::convert::TryFrom<&str> for IngestDeletePipelineTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IngestDeletePipelineTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IngestDeletePipelineTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IngestDeletePipelineTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

























///Comma-separated list of pipeline ids. Wildcards supported.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IngestGetPipelineWithIdId(String);
impl std::ops::Deref for IngestGetPipelineWithIdId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IngestGetPipelineWithIdId> for String {
  fn from(value: IngestGetPipelineWithIdId) -> Self {
    value.0
  }
}

impl From<&IngestGetPipelineWithIdId> for IngestGetPipelineWithIdId {
  fn from(value: &IngestGetPipelineWithIdId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IngestGetPipelineWithIdId {
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

impl std::convert::TryFrom<&str> for IngestGetPipelineWithIdId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IngestGetPipelineWithIdId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IngestGetPipelineWithIdId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IngestGetPipelineWithIdId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///The ingest definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IngestPutPipelineBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IngestPutPipelineBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<IngestPutPipelineBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IngestPutPipelineBodyParams) -> Self {
    value.0
  }
}

impl From<&IngestPutPipelineBodyParams> for IngestPutPipelineBodyParams {
  fn from(value: &IngestPutPipelineBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for IngestPutPipelineBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Pipeline ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IngestPutPipelineId(String);
impl std::ops::Deref for IngestPutPipelineId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IngestPutPipelineId> for String {
  fn from(value: IngestPutPipelineId) -> Self {
    value.0
  }
}

impl From<&IngestPutPipelineId> for IngestPutPipelineId {
  fn from(value: &IngestPutPipelineId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IngestPutPipelineId {
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

impl std::convert::TryFrom<&str> for IngestPutPipelineId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IngestPutPipelineId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IngestPutPipelineId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IngestPutPipelineId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}









///Operation timeout.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IngestPutPipelineTimeout(String);
impl std::ops::Deref for IngestPutPipelineTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IngestPutPipelineTimeout> for String {
  fn from(value: IngestPutPipelineTimeout) -> Self {
    value.0
  }
}

impl From<&IngestPutPipelineTimeout> for IngestPutPipelineTimeout {
  fn from(value: &IngestPutPipelineTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IngestPutPipelineTimeout {
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

impl std::convert::TryFrom<&str> for IngestPutPipelineTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IngestPutPipelineTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IngestPutPipelineTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IngestPutPipelineTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///The simulate definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IngestSimulateBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IngestSimulateBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<IngestSimulateBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: IngestSimulateBodyParams) -> Self {
    value.0
  }
}

impl From<&IngestSimulateBodyParams> for IngestSimulateBodyParams {
  fn from(value: &IngestSimulateBodyParams) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for IngestSimulateBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

///Pipeline ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IngestSimulateGetWithIdId(String);
impl std::ops::Deref for IngestSimulateGetWithIdId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IngestSimulateGetWithIdId> for String {
  fn from(value: IngestSimulateGetWithIdId) -> Self {
    value.0
  }
}

impl From<&IngestSimulateGetWithIdId> for IngestSimulateGetWithIdId {
  fn from(value: &IngestSimulateGetWithIdId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IngestSimulateGetWithIdId {
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

impl std::convert::TryFrom<&str> for IngestSimulateGetWithIdId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IngestSimulateGetWithIdId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IngestSimulateGetWithIdId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IngestSimulateGetWithIdId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

///Pipeline ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IngestSimulatePostWithIdId(String);
impl std::ops::Deref for IngestSimulatePostWithIdId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<IngestSimulatePostWithIdId> for String {
  fn from(value: IngestSimulatePostWithIdId) -> Self {
    value.0
  }
}

impl From<&IngestSimulatePostWithIdId> for IngestSimulatePostWithIdId {
  fn from(value: &IngestSimulatePostWithIdId) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for IngestSimulatePostWithIdId {
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

impl std::convert::TryFrom<&str> for IngestSimulatePostWithIdId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for IngestSimulatePostWithIdId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for IngestSimulatePostWithIdId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for IngestSimulatePostWithIdId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
