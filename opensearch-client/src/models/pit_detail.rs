/*
 * OpenSearch
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2021-11-23
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PitDetail {
  #[serde(rename = "pit_id", skip_serializing_if = "Option::is_none")]
  pub pit_id: Option<String>,
  #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
  pub creation_time: Option<i64>,
  #[serde(rename = "keep_alive", skip_serializing_if = "Option::is_none")]
  pub keep_alive: Option<i64>,
}

impl PitDetail {
  pub fn new() -> PitDetail {
    PitDetail {
      pit_id: None,
      creation_time: None,
      keep_alive: None,
    }
  }
}
