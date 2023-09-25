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
pub struct Total {
  #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
  pub value: Option<i32>,
  #[serde(rename = "relation", skip_serializing_if = "Option::is_none")]
  pub relation: Option<crate::models::Relation>,
}

impl Total {
  pub fn new() -> Total {
    Total {
      value: None,
      relation: None,
    }
  }
}
