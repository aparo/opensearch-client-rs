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
pub struct RoleMapping {
  #[serde(rename = "hosts", skip_serializing_if = "Option::is_none")]
  pub hosts: Option<Vec<String>>,
  #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
  pub users: Option<Vec<String>>,
  #[serde(rename = "reserved", skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(rename = "backend_roles", skip_serializing_if = "Option::is_none")]
  pub backend_roles: Option<Vec<String>>,
  #[serde(rename = "and_backend_roles", skip_serializing_if = "Option::is_none")]
  pub and_backend_roles: Option<Vec<String>>,
  #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

impl RoleMapping {
  pub fn new() -> RoleMapping {
    RoleMapping {
      hosts: None,
      users: None,
      reserved: None,
      hidden: None,
      backend_roles: None,
      and_backend_roles: None,
      description: None,
    }
  }
}
