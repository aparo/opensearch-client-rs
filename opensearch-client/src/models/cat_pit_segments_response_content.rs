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
pub struct CatPitSegmentsResponseContent {
  #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
  pub content: Option<Box<crate::models::CatPitSegment>>,
}

impl CatPitSegmentsResponseContent {
  pub fn new() -> CatPitSegmentsResponseContent {
    CatPitSegmentsResponseContent { content: None }
  }
}
