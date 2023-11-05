#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::types::UserDefinedValueMap;
///Define ids, documents, parameters or a list of parameters per document
/// here. You must at least provide a list of document ids. See
/// documentation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MtermvectorsBodyParams(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for MtermvectorsBodyParams {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}
impl From<MtermvectorsBodyParams> for serde_json::Map<String, serde_json::Value> {
  fn from(value: MtermvectorsBodyParams) -> Self {
    value.0
  }
}
impl From<&MtermvectorsBodyParams> for MtermvectorsBodyParams {
  fn from(value: &MtermvectorsBodyParams) -> Self {
    value.clone()
  }
}
impl From<serde_json::Map<String, serde_json::Value>> for MtermvectorsBodyParams {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}
///The index in which the document resides.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MtermvectorsGetWithIndexIndex(String);
impl std::ops::Deref for MtermvectorsGetWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<MtermvectorsGetWithIndexIndex> for String {
  fn from(value: MtermvectorsGetWithIndexIndex) -> Self {
    value.0
  }
}
impl From<&MtermvectorsGetWithIndexIndex> for MtermvectorsGetWithIndexIndex {
  fn from(value: &MtermvectorsGetWithIndexIndex) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for MtermvectorsGetWithIndexIndex {
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
impl std::convert::TryFrom<&str> for MtermvectorsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for MtermvectorsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for MtermvectorsGetWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for MtermvectorsGetWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///The index in which the document resides.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MtermvectorsPostWithIndexIndex(String);
impl std::ops::Deref for MtermvectorsPostWithIndexIndex {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<MtermvectorsPostWithIndexIndex> for String {
  fn from(value: MtermvectorsPostWithIndexIndex) -> Self {
    value.0
  }
}
impl From<&MtermvectorsPostWithIndexIndex> for MtermvectorsPostWithIndexIndex {
  fn from(value: &MtermvectorsPostWithIndexIndex) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for MtermvectorsPostWithIndexIndex {
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
impl std::convert::TryFrom<&str> for MtermvectorsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for MtermvectorsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for MtermvectorsPostWithIndexIndex {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for MtermvectorsPostWithIndexIndex {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
