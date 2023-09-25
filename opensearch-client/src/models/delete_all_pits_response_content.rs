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
pub struct DeleteAllPitsResponseContent {
  #[serde(rename = "pits", skip_serializing_if = "Option::is_none")]
  pub pits: Option<Vec<crate::models::PitsDetailsDeleteAll>>,
}

impl DeleteAllPitsResponseContent {
  pub fn new() -> DeleteAllPitsResponseContent {
    DeleteAllPitsResponseContent { pits: None }
  }
}
