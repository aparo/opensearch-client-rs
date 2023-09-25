/*
 * OpenSearch
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2021-11-23
 *
 * Generated by: https://openapi-generator.tech
 */

/// SuggestMode : Specify suggest mode.

/// Specify suggest mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SuggestMode {
  #[serde(rename = "missing")]
  Missing,
  #[serde(rename = "popular")]
  Popular,
  #[serde(rename = "always")]
  Always,
}

impl ToString for SuggestMode {
  fn to_string(&self) -> String {
    match self {
      Self::Missing => String::from("missing"),
      Self::Popular => String::from("popular"),
      Self::Always => String::from("always"),
    }
  }
}

impl Default for SuggestMode {
  fn default() -> SuggestMode {
    Self::Missing
  }
}
