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
pub struct Hits {
  #[serde(rename = "_index", skip_serializing_if = "Option::is_none")]
  pub _index: Option<String>,
  #[serde(rename = "_type", skip_serializing_if = "Option::is_none")]
  pub _type: Option<String>,
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub _id: Option<String>,
  #[serde(rename = "_score", skip_serializing_if = "Option::is_none")]
  pub _score: Option<f32>,
  #[serde(
    rename = "_source",
    default,
    with = "::serde_with::rust::double_option",
    skip_serializing_if = "Option::is_none"
  )]
  pub _source: Option<Option<serde_json::Value>>,
  #[serde(
    rename = "fields",
    default,
    with = "::serde_with::rust::double_option",
    skip_serializing_if = "Option::is_none"
  )]
  pub fields: Option<Option<serde_json::Value>>,
}

impl Hits {
  pub fn new() -> Hits {
    Hits {
      _index: None,
      _type: None,
      _id: None,
      _score: None,
      _source: None,
      fields: None,
    }
  }
}
