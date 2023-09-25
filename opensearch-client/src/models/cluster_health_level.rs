/*
 * OpenSearch
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2021-11-23
 *
 * Generated by: https://openapi-generator.tech
 */

/// ClusterHealthLevel : Specify the level of detail for returned information.

/// Specify the level of detail for returned information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClusterHealthLevel {
  #[serde(rename = "cluster")]
  Cluster,
  #[serde(rename = "indices")]
  Indices,
  #[serde(rename = "shards")]
  Shards,
  #[serde(rename = "awareness_attributes")]
  AwarenessAttributes,
}

impl ToString for ClusterHealthLevel {
  fn to_string(&self) -> String {
    match self {
      Self::Cluster => String::from("cluster"),
      Self::Indices => String::from("indices"),
      Self::Shards => String::from("shards"),
      Self::AwarenessAttributes => String::from("awareness_attributes"),
    }
  }
}

impl Default for ClusterHealthLevel {
  fn default() -> ClusterHealthLevel {
    Self::Cluster
  }
}
