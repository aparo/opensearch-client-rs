
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use super::builder;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UpdateTimeout(String);
impl std::ops::Deref for UpdateTimeout {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl From<UpdateTimeout> for String {
  fn from(value: UpdateTimeout) -> Self {
    value.0
  }
}

impl From<&UpdateTimeout> for UpdateTimeout {
  fn from(value: &UpdateTimeout) -> Self {
    value.clone()
  }
}

impl std::str::FromStr for UpdateTimeout {
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

impl std::convert::TryFrom<&str> for UpdateTimeout {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<&String> for UpdateTimeout {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl std::convert::TryFrom<String> for UpdateTimeout {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}

impl<'de> serde::Deserialize<'de> for UpdateTimeout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub attributes: Option<UserAttributes>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub backend_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hash: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub opendistro_security_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
  pub static_: Option<bool>,
}

impl From<&User> for User {
  fn from(value: &User) -> Self {
    value.clone()
  }
}

impl User {
  pub fn builder() -> builder::User {
    builder::User::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserAttributes(pub std::collections::HashMap<String, String>);
impl std::ops::Deref for UserAttributes {
  type Target = std::collections::HashMap<String, String>;

  fn deref(&self) -> &std::collections::HashMap<String, String> {
    &self.0
  }
}

impl From<UserAttributes> for std::collections::HashMap<String, String> {
  fn from(value: UserAttributes) -> Self {
    value.0
  }
}

impl From<&UserAttributes> for UserAttributes {
  fn from(value: &UserAttributes) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, String>> for UserAttributes {
  fn from(value: std::collections::HashMap<String, String>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserDefinedObjectStructure {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub bool: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub boosting: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub combined_fields: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub constant_score: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub dis_max: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub distance_feature: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub exists: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub field_masking_span: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub function_score: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub fuzzy: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub geo_bounding_box: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub geo_distance: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub geo_polygon: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub geo_shape: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub has_child: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub has_parent: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub ids: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub intervals: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub knn: Option<serde_json::Value>,
  #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
  pub match_: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_all: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_bool_prefix: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_none: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_phrase: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub match_phrase_prefix: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub more_like_this: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub multi_match: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub nested: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub parent_id: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub percolate: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub pinned: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub prefix: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub query_string: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub range: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub rank_feature: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub regexp: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub script: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub script_score: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub shape: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub simple_query_string: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_containing: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_first: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_multi: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_near: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_not: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_or: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_term: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub span_within: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub term: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub terms: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub terms_set: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub wildcard: Option<UserDefinedValueMap>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub wrapper: Option<serde_json::Value>,
}

impl From<&UserDefinedObjectStructure> for UserDefinedObjectStructure {
  fn from(value: &UserDefinedObjectStructure) -> Self {
    value.clone()
  }
}

impl UserDefinedObjectStructure {
  pub fn builder() -> builder::UserDefinedObjectStructure {
    builder::UserDefinedObjectStructure::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserDefinedStructure {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub alias: Option<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub aliases: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub filter: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index_routing: Option<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub indices: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_write_index: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub must_exist: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub routing: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub search_routing: Option<String>,
}

impl From<&UserDefinedStructure> for UserDefinedStructure {
  fn from(value: &UserDefinedStructure) -> Self {
    value.clone()
  }
}

impl UserDefinedStructure {
  pub fn builder() -> builder::UserDefinedStructure {
    builder::UserDefinedStructure::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserDefinedValueMap(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for UserDefinedValueMap {
  type Target = serde_json::Map<String, serde_json::Value>;

  fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
    &self.0
  }
}

impl From<UserDefinedValueMap> for serde_json::Map<String, serde_json::Value> {
  fn from(value: UserDefinedValueMap) -> Self {
    value.0
  }
}

impl From<&UserDefinedValueMap> for UserDefinedValueMap {
  fn from(value: &UserDefinedValueMap) -> Self {
    value.clone()
  }
}

impl From<serde_json::Map<String, serde_json::Value>> for UserDefinedValueMap {
  fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserTenants {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub admin: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub admin_tenant: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub global_tenant: Option<bool>,
}

impl From<&UserTenants> for UserTenants {
  fn from(value: &UserTenants) -> Self {
    value.clone()
  }
}

impl UserTenants {
  pub fn builder() -> builder::UserTenants {
    builder::UserTenants::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UsersMap(pub std::collections::HashMap<String, User>);
impl std::ops::Deref for UsersMap {
  type Target = std::collections::HashMap<String, User>;

  fn deref(&self) -> &std::collections::HashMap<String, User> {
    &self.0
  }
}

impl From<UsersMap> for std::collections::HashMap<String, User> {
  fn from(value: UsersMap) -> Self {
    value.0
  }
}

impl From<&UsersMap> for UsersMap {
  fn from(value: &UsersMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, User>> for UsersMap {
  fn from(value: std::collections::HashMap<String, User>) -> Self {
    Self(value)
  }
}
