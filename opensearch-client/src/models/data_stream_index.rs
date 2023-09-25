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
pub struct DataStreamIndex {
  #[serde(rename = "index_name", skip_serializing_if = "Option::is_none")]
  pub index_name: Option<String>,
  #[serde(rename = "index_uuid", skip_serializing_if = "Option::is_none")]
  pub index_uuid: Option<String>,
}

impl DataStreamIndex {
  pub fn new() -> DataStreamIndex {
    DataStreamIndex {
      index_name: None,
      index_uuid: None,
    }
  }
}
