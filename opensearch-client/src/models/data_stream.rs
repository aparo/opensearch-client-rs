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
pub struct DataStream {
  #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(rename = "timestamp_field", skip_serializing_if = "Option::is_none")]
  pub timestamp_field: Option<Box<crate::models::DataStreamTimestampField>>,
  #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
  pub indices: Option<Vec<crate::models::DataStreamIndex>>,
  #[serde(rename = "generation", skip_serializing_if = "Option::is_none")]
  pub generation: Option<i64>,
  #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
  pub status: Option<crate::models::DataStreamStatus>,
  #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
  pub template: Option<String>,
}

impl DataStream {
  pub fn new() -> DataStream {
    DataStream {
      name: None,
      timestamp_field: None,
      indices: None,
      generation: None,
      status: None,
      template: None,
    }
  }
}
