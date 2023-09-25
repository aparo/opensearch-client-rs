/*
 * OpenSearch
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2021-11-23
 *
 * Generated by: https://openapi-generator.tech
 */

/// WaitForEvents : Wait until all currently queued events with the given
/// priority are processed.

/// Wait until all currently queued events with the given priority are
/// processed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WaitForEvents {
  #[serde(rename = "immediate")]
  Immediate,
  #[serde(rename = "urgent")]
  Urgent,
  #[serde(rename = "high")]
  High,
  #[serde(rename = "normal")]
  Normal,
  #[serde(rename = "low")]
  Low,
  #[serde(rename = "languid")]
  Languid,
}

impl ToString for WaitForEvents {
  fn to_string(&self) -> String {
    match self {
      Self::Immediate => String::from("immediate"),
      Self::Urgent => String::from("urgent"),
      Self::High => String::from("high"),
      Self::Normal => String::from("normal"),
      Self::Low => String::from("low"),
      Self::Languid => String::from("languid"),
    }
  }
}

impl Default for WaitForEvents {
  fn default() -> WaitForEvents {
    Self::Immediate
  }
}
