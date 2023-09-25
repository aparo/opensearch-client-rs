/*
 * OpenSearch
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2021-11-23
 *
 * Generated by: https://openapi-generator.tech
 */

/// VersionType : Specific version type.

/// Specific version type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VersionType {
  #[serde(rename = "internal")]
  Internal,
  #[serde(rename = "external")]
  External,
  #[serde(rename = "external_gte")]
  ExternalGte,
  #[serde(rename = "force")]
  Force,
}

impl ToString for VersionType {
  fn to_string(&self) -> String {
    match self {
      Self::Internal => String::from("internal"),
      Self::External => String::from("external"),
      Self::ExternalGte => String::from("external_gte"),
      Self::Force => String::from("force"),
    }
  }
}

impl Default for VersionType {
  fn default() -> VersionType {
    Self::Internal
  }
}
