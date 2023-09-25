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
pub struct CreateTenantResponseContent {
  /// Security Operation Status
  #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
  /// Security Operation Message
  #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
}

impl CreateTenantResponseContent {
  pub fn new() -> CreateTenantResponseContent {
    CreateTenantResponseContent {
      status: None,
      message: None,
    }
  }
}
