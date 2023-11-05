#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditConfig {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub audit: Option<AuditLogsConfig>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub compliance: Option<ComplianceConfig>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
}

impl From<&AuditConfig> for AuditConfig {
  fn from(value: &AuditConfig) -> Self {
    value.clone()
  }
}

impl AuditConfig {
  pub fn builder() -> builder::AuditConfig {
    builder::AuditConfig::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditConfigWithReadOnly {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub config: Option<AuditConfig>,
  #[serde(rename = "_readonly", default, skip_serializing_if = "Vec::is_empty")]
  pub readonly: Vec<String>,
}

impl From<&AuditConfigWithReadOnly> for AuditConfigWithReadOnly {
  fn from(value: &AuditConfigWithReadOnly) -> Self {
    value.clone()
  }
}

impl AuditConfigWithReadOnly {
  pub fn builder() -> builder::AuditConfigWithReadOnly {
    builder::AuditConfigWithReadOnly::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogsConfig {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub disabled_rest_categories: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub disabled_transport_categories: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub enable_rest: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub enable_transport: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub exclude_sensitive_headers: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub ignore_requests: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub ignore_users: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub log_request_body: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub resolve_bulk_requests: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub resolve_indices: Option<bool>,
}

impl From<&AuditLogsConfig> for AuditLogsConfig {
  fn from(value: &AuditLogsConfig) -> Self {
    value.clone()
  }
}

impl AuditLogsConfig {
  pub fn builder() -> builder::AuditLogsConfig {
    builder::AuditLogsConfig::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAuditConfigurationResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&UpdateAuditConfigurationResponseContent> for UpdateAuditConfigurationResponseContent {
  fn from(value: &UpdateAuditConfigurationResponseContent) -> Self {
    value.clone()
  }
}

impl UpdateAuditConfigurationResponseContent {
  pub fn builder() -> builder::UpdateAuditConfigurationResponseContent {
    builder::UpdateAuditConfigurationResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountDetails {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub backend_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub custom_attribute_names: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_internal_user: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub is_reserved: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub tenants: Option<UserTenants>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub user_name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub user_requested_tenant: Option<String>,
}

impl From<&AccountDetails> for AccountDetails {
  fn from(value: &AccountDetails) -> Self {
    value.clone()
  }
}

impl AccountDetails {
  pub fn builder() -> builder::AccountDetails {
    builder::AccountDetails::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActionGroup {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub allowed_actions: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
  pub static_: Option<bool>,
  #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
  pub type_: Option<String>,
}

impl From<&ActionGroup> for ActionGroup {
  fn from(value: &ActionGroup) -> Self {
    value.clone()
  }
}

impl ActionGroup {
  pub fn builder() -> builder::ActionGroup {
    builder::ActionGroup::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActionGroupsMap(pub std::collections::HashMap<String, ActionGroup>);
impl std::ops::Deref for ActionGroupsMap {
  type Target = std::collections::HashMap<String, ActionGroup>;

  fn deref(&self) -> &std::collections::HashMap<String, ActionGroup> {
    &self.0
  }
}

impl From<ActionGroupsMap> for std::collections::HashMap<String, ActionGroup> {
  fn from(value: ActionGroupsMap) -> Self {
    value.0
  }
}

impl From<&ActionGroupsMap> for ActionGroupsMap {
  fn from(value: &ActionGroupsMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, ActionGroup>> for ActionGroupsMap {
  fn from(value: std::collections::HashMap<String, ActionGroup>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateActionGroupResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateActionGroupResponseContent> for CreateActionGroupResponseContent {
  fn from(value: &CreateActionGroupResponseContent) -> Self {
    value.clone()
  }
}

impl CreateActionGroupResponseContent {
  pub fn builder() -> builder::CreateActionGroupResponseContent {
    builder::CreateActionGroupResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CertificatesDetail {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub issuer_dn: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub not_after: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub not_before: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub san: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub subject_dn: Option<String>,
}

impl From<&CertificatesDetail> for CertificatesDetail {
  fn from(value: &CertificatesDetail) -> Self {
    value.clone()
  }
}

impl CertificatesDetail {
  pub fn builder() -> builder::CertificatesDetail {
    builder::CertificatesDetail::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangePasswordRequestContent {
  ///The current password
  pub current_password: String,
  ///The new password to set
  pub password: String,
}

impl From<&ChangePasswordRequestContent> for ChangePasswordRequestContent {
  fn from(value: &ChangePasswordRequestContent) -> Self {
    value.clone()
  }
}

impl ChangePasswordRequestContent {
  pub fn builder() -> builder::ChangePasswordRequestContent {
    builder::ChangePasswordRequestContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangePasswordResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&ChangePasswordResponseContent> for ChangePasswordResponseContent {
  fn from(value: &ChangePasswordResponseContent) -> Self {
    value.clone()
  }
}

impl ChangePasswordResponseContent {
  pub fn builder() -> builder::ChangePasswordResponseContent {
    builder::ChangePasswordResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateRoleMappingResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateRoleMappingResponseContent> for CreateRoleMappingResponseContent {
  fn from(value: &CreateRoleMappingResponseContent) -> Self {
    value.clone()
  }
}

impl CreateRoleMappingResponseContent {
  pub fn builder() -> builder::CreateRoleMappingResponseContent {
    builder::CreateRoleMappingResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateRoleResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateRoleResponseContent> for CreateRoleResponseContent {
  fn from(value: &CreateRoleResponseContent) -> Self {
    value.clone()
  }
}

impl CreateRoleResponseContent {
  pub fn builder() -> builder::CreateRoleResponseContent {
    builder::CreateRoleResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTenantParams {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

impl From<&CreateTenantParams> for CreateTenantParams {
  fn from(value: &CreateTenantParams) -> Self {
    value.clone()
  }
}

impl CreateTenantParams {
  pub fn builder() -> builder::CreateTenantParams {
    builder::CreateTenantParams::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTenantResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateTenantResponseContent> for CreateTenantResponseContent {
  fn from(value: &CreateTenantResponseContent) -> Self {
    value.clone()
  }
}

impl CreateTenantResponseContent {
  pub fn builder() -> builder::CreateTenantResponseContent {
    builder::CreateTenantResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateUserResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&CreateUserResponseContent> for CreateUserResponseContent {
  fn from(value: &CreateUserResponseContent) -> Self {
    value.clone()
  }
}

impl CreateUserResponseContent {
  pub fn builder() -> builder::CreateUserResponseContent {
    builder::CreateUserResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComplianceConfig {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub external_config: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub internal_config: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub read_ignore_users: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub read_metadata_only: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub read_watched_fields: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub write_ignore_users: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub write_log_diffs: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub write_metadata_only: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub write_watched_indices: Vec<String>,
}

impl From<&ComplianceConfig> for ComplianceConfig {
  fn from(value: &ComplianceConfig) -> Self {
    value.clone()
  }
}

impl ComplianceConfig {
  pub fn builder() -> builder::ComplianceConfig {
    builder::ComplianceConfig::default()
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteTenantResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteTenantResponseContent> for DeleteTenantResponseContent {
  fn from(value: &DeleteTenantResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteTenantResponseContent {
  pub fn builder() -> builder::DeleteTenantResponseContent {
    builder::DeleteTenantResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlushCacheResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&FlushCacheResponseContent> for FlushCacheResponseContent {
  fn from(value: &FlushCacheResponseContent) -> Self {
    value.clone()
  }
}

impl FlushCacheResponseContent {
  pub fn builder() -> builder::FlushCacheResponseContent {
    builder::FlushCacheResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetCertificatesResponseContent {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub http_certificates_list: Vec<CertificatesDetail>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub transport_certificates_list: Vec<CertificatesDetail>,
}

impl From<&GetCertificatesResponseContent> for GetCertificatesResponseContent {
  fn from(value: &GetCertificatesResponseContent) -> Self {
    value.clone()
  }
}

impl GetCertificatesResponseContent {
  pub fn builder() -> builder::GetCertificatesResponseContent {
    builder::GetCertificatesResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteActionGroupResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteActionGroupResponseContent> for DeleteActionGroupResponseContent {
  fn from(value: &DeleteActionGroupResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteActionGroupResponseContent {
  pub fn builder() -> builder::DeleteActionGroupResponseContent {
    builder::DeleteActionGroupResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteRoleMappingResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteRoleMappingResponseContent> for DeleteRoleMappingResponseContent {
  fn from(value: &DeleteRoleMappingResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteRoleMappingResponseContent {
  pub fn builder() -> builder::DeleteRoleMappingResponseContent {
    builder::DeleteRoleMappingResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteRoleResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteRoleResponseContent> for DeleteRoleResponseContent {
  fn from(value: &DeleteRoleResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteRoleResponseContent {
  pub fn builder() -> builder::DeleteRoleResponseContent {
    builder::DeleteRoleResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DynamicConfig {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub dynamic: Option<DynamicOptions>,
}

impl From<&DynamicConfig> for DynamicConfig {
  fn from(value: &DynamicConfig) -> Self {
    value.clone()
  }
}

impl DynamicConfig {
  pub fn builder() -> builder::DynamicConfig {
    builder::DynamicConfig::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DynamicOptions {
  #[serde(rename = "authFailureListeners", default, skip_serializing_if = "Option::is_none")]
  pub auth_failure_listeners: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub authc: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub authz: Option<serde_json::Value>,
  #[serde(
    rename = "disableIntertransportAuth",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub disable_intertransport_auth: Option<bool>,
  #[serde(rename = "disableRestAuth", default, skip_serializing_if = "Option::is_none")]
  pub disable_rest_auth: Option<bool>,
  #[serde(rename = "doNotFailOnForbidden", default, skip_serializing_if = "Option::is_none")]
  pub do_not_fail_on_forbidden: Option<bool>,
  #[serde(
    rename = "doNotFailOnForbiddenEmpty",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub do_not_fail_on_forbidden_empty: Option<bool>,
  #[serde(rename = "filteredAliasMode", default, skip_serializing_if = "Option::is_none")]
  pub filtered_alias_mode: Option<String>,
  #[serde(rename = "hostsResolverMode", default, skip_serializing_if = "Option::is_none")]
  pub hosts_resolver_mode: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub http: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub kibana: Option<serde_json::Value>,
  #[serde(rename = "multiRolespanEnabled", default, skip_serializing_if = "Option::is_none")]
  pub multi_rolespan_enabled: Option<bool>,
  #[serde(
    rename = "respectRequestIndicesOptions",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub respect_request_indices_options: Option<bool>,
}

impl From<&DynamicOptions> for DynamicOptions {
  fn from(value: &DynamicOptions) -> Self {
    value.clone()
  }
}

impl DynamicOptions {
  pub fn builder() -> builder::DynamicOptions {
    builder::DynamicOptions::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteUserResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteUserResponseContent> for DeleteUserResponseContent {
  fn from(value: &DeleteUserResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteUserResponseContent {
  pub fn builder() -> builder::DeleteUserResponseContent {
    builder::DeleteUserResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DistinguishedNames {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub nodes_dn: Vec<String>,
}

impl From<&DistinguishedNames> for DistinguishedNames {
  fn from(value: &DistinguishedNames) -> Self {
    value.clone()
  }
}

impl DistinguishedNames {
  pub fn builder() -> builder::DistinguishedNames {
    builder::DistinguishedNames::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DistinguishedNamesMap(pub std::collections::HashMap<String, DistinguishedNames>);
impl std::ops::Deref for DistinguishedNamesMap {
  type Target = std::collections::HashMap<String, DistinguishedNames>;

  fn deref(&self) -> &std::collections::HashMap<String, DistinguishedNames> {
    &self.0
  }
}

impl From<DistinguishedNamesMap> for std::collections::HashMap<String, DistinguishedNames> {
  fn from(value: DistinguishedNamesMap) -> Self {
    value.0
  }
}

impl From<&DistinguishedNamesMap> for DistinguishedNamesMap {
  fn from(value: &DistinguishedNamesMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, DistinguishedNames>> for DistinguishedNamesMap {
  fn from(value: std::collections::HashMap<String, DistinguishedNames>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndexPermission {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub allowed_actions: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub fls: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub index_patterns: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub masked_fields: Vec<String>,
}

impl From<&IndexPermission> for IndexPermission {
  fn from(value: &IndexPermission) -> Self {
    value.clone()
  }
}

impl IndexPermission {
  pub fn builder() -> builder::IndexPermission {
    builder::IndexPermission::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateConfigurationResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&UpdateConfigurationResponseContent> for UpdateConfigurationResponseContent {
  fn from(value: &UpdateConfigurationResponseContent) -> Self {
    value.clone()
  }
}

impl UpdateConfigurationResponseContent {
  pub fn builder() -> builder::UpdateConfigurationResponseContent {
    builder::UpdateConfigurationResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteDistinguishedNamesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&DeleteDistinguishedNamesResponseContent> for DeleteDistinguishedNamesResponseContent {
  fn from(value: &DeleteDistinguishedNamesResponseContent) -> Self {
    value.clone()
  }
}

impl DeleteDistinguishedNamesResponseContent {
  pub fn builder() -> builder::DeleteDistinguishedNamesResponseContent {
    builder::DeleteDistinguishedNamesResponseContent::default()
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateDistinguishedNamesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&UpdateDistinguishedNamesResponseContent> for UpdateDistinguishedNamesResponseContent {
  fn from(value: &UpdateDistinguishedNamesResponseContent) -> Self {
    value.clone()
  }
}

impl UpdateDistinguishedNamesResponseContent {
  pub fn builder() -> builder::UpdateDistinguishedNamesResponseContent {
    builder::UpdateDistinguishedNamesResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchActionGroupInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchActionGroupInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchActionGroupInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchActionGroupInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchActionGroupInputPayload> for PatchActionGroupInputPayload {
  fn from(value: &PatchActionGroupInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchActionGroupInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchActionGroupResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchActionGroupResponseContent> for PatchActionGroupResponseContent {
  fn from(value: &PatchActionGroupResponseContent) -> Self {
    value.clone()
  }
}

impl PatchActionGroupResponseContent {
  pub fn builder() -> builder::PatchActionGroupResponseContent {
    builder::PatchActionGroupResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchActionGroupsInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchActionGroupsInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchActionGroupsInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchActionGroupsInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchActionGroupsInputPayload> for PatchActionGroupsInputPayload {
  fn from(value: &PatchActionGroupsInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchActionGroupsInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchActionGroupsResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchActionGroupsResponseContent> for PatchActionGroupsResponseContent {
  fn from(value: &PatchActionGroupsResponseContent) -> Self {
    value.clone()
  }
}

impl PatchActionGroupsResponseContent {
  pub fn builder() -> builder::PatchActionGroupsResponseContent {
    builder::PatchActionGroupsResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchAuditConfigurationInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchAuditConfigurationInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchAuditConfigurationInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchAuditConfigurationInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchAuditConfigurationInputPayload> for PatchAuditConfigurationInputPayload {
  fn from(value: &PatchAuditConfigurationInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchAuditConfigurationInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchConfigurationInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchConfigurationInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchConfigurationInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchConfigurationInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchConfigurationInputPayload> for PatchConfigurationInputPayload {
  fn from(value: &PatchConfigurationInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchConfigurationInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchConfigurationResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchConfigurationResponseContent> for PatchConfigurationResponseContent {
  fn from(value: &PatchConfigurationResponseContent) -> Self {
    value.clone()
  }
}

impl PatchConfigurationResponseContent {
  pub fn builder() -> builder::PatchConfigurationResponseContent {
    builder::PatchConfigurationResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchDistinguishedNamesInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchDistinguishedNamesInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchDistinguishedNamesInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchDistinguishedNamesInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchDistinguishedNamesInputPayload> for PatchDistinguishedNamesInputPayload {
  fn from(value: &PatchDistinguishedNamesInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchDistinguishedNamesInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchDistinguishedNamesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchDistinguishedNamesResponseContent> for PatchDistinguishedNamesResponseContent {
  fn from(value: &PatchDistinguishedNamesResponseContent) -> Self {
    value.clone()
  }
}

impl PatchDistinguishedNamesResponseContent {
  pub fn builder() -> builder::PatchDistinguishedNamesResponseContent {
    builder::PatchDistinguishedNamesResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchOperation {
  ///The operation to perform. Possible values: remove,add, replace,
  /// move, copy, test.
  pub op: String,
  ///The path to the resource.
  pub path: String,
  ///The new values used for the update.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub value: Option<serde_json::Value>,
}

impl From<&PatchOperation> for PatchOperation {
  fn from(value: &PatchOperation) -> Self {
    value.clone()
  }
}

impl PatchOperation {
  pub fn builder() -> builder::PatchOperation {
    builder::PatchOperation::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchRoleInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchRoleInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchRoleInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchRoleInputPayload> for PatchRoleInputPayload {
  fn from(value: &PatchRoleInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchRoleInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleMappingInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchRoleMappingInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchRoleMappingInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchRoleMappingInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchRoleMappingInputPayload> for PatchRoleMappingInputPayload {
  fn from(value: &PatchRoleMappingInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchRoleMappingInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleMappingResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchRoleMappingResponseContent> for PatchRoleMappingResponseContent {
  fn from(value: &PatchRoleMappingResponseContent) -> Self {
    value.clone()
  }
}

impl PatchRoleMappingResponseContent {
  pub fn builder() -> builder::PatchRoleMappingResponseContent {
    builder::PatchRoleMappingResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleMappingsInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchRoleMappingsInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchRoleMappingsInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchRoleMappingsInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchRoleMappingsInputPayload> for PatchRoleMappingsInputPayload {
  fn from(value: &PatchRoleMappingsInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchRoleMappingsInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleMappingsResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchRoleMappingsResponseContent> for PatchRoleMappingsResponseContent {
  fn from(value: &PatchRoleMappingsResponseContent) -> Self {
    value.clone()
  }
}

impl PatchRoleMappingsResponseContent {
  pub fn builder() -> builder::PatchRoleMappingsResponseContent {
    builder::PatchRoleMappingsResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRoleResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchRoleResponseContent> for PatchRoleResponseContent {
  fn from(value: &PatchRoleResponseContent) -> Self {
    value.clone()
  }
}

impl PatchRoleResponseContent {
  pub fn builder() -> builder::PatchRoleResponseContent {
    builder::PatchRoleResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRolesInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchRolesInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchRolesInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchRolesInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchRolesInputPayload> for PatchRolesInputPayload {
  fn from(value: &PatchRolesInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchRolesInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchRolesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchRolesResponseContent> for PatchRolesResponseContent {
  fn from(value: &PatchRolesResponseContent) -> Self {
    value.clone()
  }
}

impl PatchRolesResponseContent {
  pub fn builder() -> builder::PatchRolesResponseContent {
    builder::PatchRolesResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchTenantInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchTenantInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchTenantInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchTenantInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchTenantInputPayload> for PatchTenantInputPayload {
  fn from(value: &PatchTenantInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchTenantInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchTenantResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchTenantResponseContent> for PatchTenantResponseContent {
  fn from(value: &PatchTenantResponseContent) -> Self {
    value.clone()
  }
}

impl PatchTenantResponseContent {
  pub fn builder() -> builder::PatchTenantResponseContent {
    builder::PatchTenantResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchTenantsInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchTenantsInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchTenantsInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchTenantsInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchTenantsInputPayload> for PatchTenantsInputPayload {
  fn from(value: &PatchTenantsInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchTenantsInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchTenantsResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchTenantsResponseContent> for PatchTenantsResponseContent {
  fn from(value: &PatchTenantsResponseContent) -> Self {
    value.clone()
  }
}

impl PatchTenantsResponseContent {
  pub fn builder() -> builder::PatchTenantsResponseContent {
    builder::PatchTenantsResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchUserInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchUserInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchUserInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchUserInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchUserInputPayload> for PatchUserInputPayload {
  fn from(value: &PatchUserInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchUserInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchUserResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchUserResponseContent> for PatchUserResponseContent {
  fn from(value: &PatchUserResponseContent) -> Self {
    value.clone()
  }
}

impl PatchUserResponseContent {
  pub fn builder() -> builder::PatchUserResponseContent {
    builder::PatchUserResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchUsersInputPayload(pub Vec<PatchOperation>);
impl std::ops::Deref for PatchUsersInputPayload {
  type Target = Vec<PatchOperation>;

  fn deref(&self) -> &Vec<PatchOperation> {
    &self.0
  }
}

impl From<PatchUsersInputPayload> for Vec<PatchOperation> {
  fn from(value: PatchUsersInputPayload) -> Self {
    value.0
  }
}

impl From<&PatchUsersInputPayload> for PatchUsersInputPayload {
  fn from(value: &PatchUsersInputPayload) -> Self {
    value.clone()
  }
}

impl From<Vec<PatchOperation>> for PatchUsersInputPayload {
  fn from(value: Vec<PatchOperation>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatchUsersResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&PatchUsersResponseContent> for PatchUsersResponseContent {
  fn from(value: &PatchUsersResponseContent) -> Self {
    value.clone()
  }
}

impl PatchUsersResponseContent {
  pub fn builder() -> builder::PatchUsersResponseContent {
    builder::PatchUsersResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Role {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub cluster_permission: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub index_permission: Option<IndexPermission>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
  pub static_: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub tenant_permissions: Vec<String>,
}

impl From<&Role> for Role {
  fn from(value: &Role) -> Self {
    value.clone()
  }
}

impl Role {
  pub fn builder() -> builder::Role {
    builder::Role::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleMapping {
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub and_backend_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub backend_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub hosts: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub users: Vec<String>,
}

impl From<&RoleMapping> for RoleMapping {
  fn from(value: &RoleMapping) -> Self {
    value.clone()
  }
}

impl RoleMapping {
  pub fn builder() -> builder::RoleMapping {
    builder::RoleMapping::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleMappings(pub std::collections::HashMap<String, RoleMapping>);
impl std::ops::Deref for RoleMappings {
  type Target = std::collections::HashMap<String, RoleMapping>;

  fn deref(&self) -> &std::collections::HashMap<String, RoleMapping> {
    &self.0
  }
}

impl From<RoleMappings> for std::collections::HashMap<String, RoleMapping> {
  fn from(value: RoleMappings) -> Self {
    value.0
  }
}

impl From<&RoleMappings> for RoleMappings {
  fn from(value: &RoleMappings) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, RoleMapping>> for RoleMappings {
  fn from(value: std::collections::HashMap<String, RoleMapping>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RolesMap(pub std::collections::HashMap<String, Role>);
impl std::ops::Deref for RolesMap {
  type Target = std::collections::HashMap<String, Role>;

  fn deref(&self) -> &std::collections::HashMap<String, Role> {
    &self.0
  }
}

impl From<RolesMap> for std::collections::HashMap<String, Role> {
  fn from(value: RolesMap) -> Self {
    value.0
  }
}

impl From<&RolesMap> for RolesMap {
  fn from(value: &RolesMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, Role>> for RolesMap {
  fn from(value: std::collections::HashMap<String, Role>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReloadHttpCertificatesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&ReloadHttpCertificatesResponseContent> for ReloadHttpCertificatesResponseContent {
  fn from(value: &ReloadHttpCertificatesResponseContent) -> Self {
    value.clone()
  }
}

impl ReloadHttpCertificatesResponseContent {
  pub fn builder() -> builder::ReloadHttpCertificatesResponseContent {
    builder::ReloadHttpCertificatesResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReloadTransportCertificatesResponseContent {
  ///Security Operation Message
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  ///Security Operation Status
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&ReloadTransportCertificatesResponseContent> for ReloadTransportCertificatesResponseContent {
  fn from(value: &ReloadTransportCertificatesResponseContent) -> Self {
    value.clone()
  }
}

impl ReloadTransportCertificatesResponseContent {
  pub fn builder() -> builder::ReloadTransportCertificatesResponseContent {
    builder::ReloadTransportCertificatesResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecurityHealthResponseContent {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub mode: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
}

impl From<&SecurityHealthResponseContent> for SecurityHealthResponseContent {
  fn from(value: &SecurityHealthResponseContent) -> Self {
    value.clone()
  }
}

impl SecurityHealthResponseContent {
  pub fn builder() -> builder::SecurityHealthResponseContent {
    builder::SecurityHealthResponseContent::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tenant {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
  pub static_: Option<bool>,
}

impl From<&Tenant> for Tenant {
  fn from(value: &Tenant) -> Self {
    value.clone()
  }
}

impl Tenant {
  pub fn builder() -> builder::Tenant {
    builder::Tenant::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TenantsMap(pub std::collections::HashMap<String, Tenant>);
impl std::ops::Deref for TenantsMap {
  type Target = std::collections::HashMap<String, Tenant>;

  fn deref(&self) -> &std::collections::HashMap<String, Tenant> {
    &self.0
  }
}

impl From<TenantsMap> for std::collections::HashMap<String, Tenant> {
  fn from(value: TenantsMap) -> Self {
    value.0
  }
}

impl From<&TenantsMap> for TenantsMap {
  fn from(value: &TenantsMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, Tenant>> for TenantsMap {
  fn from(value: std::collections::HashMap<String, Tenant>) -> Self {
    Self(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserTenants {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub admin: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub admin_tenant: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub global_tenant: Option<bool>,
}

impl From<&UserTenants> for UserTenants {
  fn from(value: &UserTenants) -> Self {
    value.clone()
  }
}

impl UserTenants {
  pub fn builder() -> builder::UserTenants {
    builder::UserTenants::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UsersMap(pub std::collections::HashMap<String, User>);
impl std::ops::Deref for UsersMap {
  type Target = std::collections::HashMap<String, User>;

  fn deref(&self) -> &std::collections::HashMap<String, User> {
    &self.0
  }
}

impl From<UsersMap> for std::collections::HashMap<String, User> {
  fn from(value: UsersMap) -> Self {
    value.0
  }
}

impl From<&UsersMap> for UsersMap {
  fn from(value: &UsersMap) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, User>> for UsersMap {
  fn from(value: std::collections::HashMap<String, User>) -> Self {
    Self(value)
  }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub attributes: Option<UserAttributes>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub backend_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hash: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub hidden: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub opendistro_security_roles: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub reserved: Option<bool>,
  #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
  pub static_: Option<bool>,
}

impl From<&User> for User {
  fn from(value: &User) -> Self {
    value.clone()
  }
}

impl User {
  pub fn builder() -> builder::User {
    builder::User::default()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserAttributes(pub std::collections::HashMap<String, String>);
impl std::ops::Deref for UserAttributes {
  type Target = std::collections::HashMap<String, String>;

  fn deref(&self) -> &std::collections::HashMap<String, String> {
    &self.0
  }
}

impl From<UserAttributes> for std::collections::HashMap<String, String> {
  fn from(value: UserAttributes) -> Self {
    value.0
  }
}

impl From<&UserAttributes> for UserAttributes {
  fn from(value: &UserAttributes) -> Self {
    value.clone()
  }
}

impl From<std::collections::HashMap<String, String>> for UserAttributes {
  fn from(value: std::collections::HashMap<String, String>) -> Self {
    Self(value)
  }
}

pub mod builder {
  #[derive(Clone, Debug)]
  pub struct AccountDetails {
    backend_roles: Result<Vec<String>, String>,
    custom_attribute_names: Result<Vec<String>, String>,
    is_hidden: Result<Option<bool>, String>,
    is_internal_user: Result<Option<bool>, String>,
    is_reserved: Result<Option<bool>, String>,
    roles: Result<Vec<String>, String>,
    tenants: Result<Option<super::UserTenants>, String>,
    user_name: Result<Option<String>, String>,
    user_requested_tenant: Result<Option<String>, String>,
  }

  impl Default for AccountDetails {
    fn default() -> Self {
      Self {
        backend_roles: Ok(Default::default()),
        custom_attribute_names: Ok(Default::default()),
        is_hidden: Ok(Default::default()),
        is_internal_user: Ok(Default::default()),
        is_reserved: Ok(Default::default()),
        roles: Ok(Default::default()),
        tenants: Ok(Default::default()),
        user_name: Ok(Default::default()),
        user_requested_tenant: Ok(Default::default()),
      }
    }
  }

  impl AccountDetails {
    pub fn backend_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.backend_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for backend_roles: {}", e));
      self
    }

    pub fn custom_attribute_names<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.custom_attribute_names = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for custom_attribute_names: {}", e));
      self
    }

    pub fn is_hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.is_hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for is_hidden: {}", e));
      self
    }

    pub fn is_internal_user<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.is_internal_user = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for is_internal_user: {}", e));
      self
    }

    pub fn is_reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.is_reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for is_reserved: {}", e));
      self
    }

    pub fn roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for roles: {}", e));
      self
    }

    pub fn tenants<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserTenants>>,
      T::Error: std::fmt::Display, {
      self.tenants = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for tenants: {}", e));
      self
    }

    pub fn user_name<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.user_name = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for user_name: {}", e));
      self
    }

    pub fn user_requested_tenant<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.user_requested_tenant = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for user_requested_tenant: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<AccountDetails> for super::AccountDetails {
    type Error = String;

    fn try_from(value: AccountDetails) -> Result<Self, String> {
      Ok(Self {
        backend_roles: value.backend_roles?,
        custom_attribute_names: value.custom_attribute_names?,
        is_hidden: value.is_hidden?,
        is_internal_user: value.is_internal_user?,
        is_reserved: value.is_reserved?,
        roles: value.roles?,
        tenants: value.tenants?,
        user_name: value.user_name?,
        user_requested_tenant: value.user_requested_tenant?,
      })
    }
  }

  impl From<super::AccountDetails> for AccountDetails {
    fn from(value: super::AccountDetails) -> Self {
      Self {
        backend_roles: Ok(value.backend_roles),
        custom_attribute_names: Ok(value.custom_attribute_names),
        is_hidden: Ok(value.is_hidden),
        is_internal_user: Ok(value.is_internal_user),
        is_reserved: Ok(value.is_reserved),
        roles: Ok(value.roles),
        tenants: Ok(value.tenants),
        user_name: Ok(value.user_name),
        user_requested_tenant: Ok(value.user_requested_tenant),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ActionGroup {
    allowed_actions: Result<Vec<String>, String>,
    description: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    reserved: Result<Option<bool>, String>,
    static_: Result<Option<bool>, String>,
    type_: Result<Option<String>, String>,
  }

  impl Default for ActionGroup {
    fn default() -> Self {
      Self {
        allowed_actions: Ok(Default::default()),
        description: Ok(Default::default()),
        hidden: Ok(Default::default()),
        reserved: Ok(Default::default()),
        static_: Ok(Default::default()),
        type_: Ok(Default::default()),
      }
    }
  }

  impl ActionGroup {
    pub fn allowed_actions<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.allowed_actions = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for allowed_actions: {}", e));
      self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn static_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.static_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for static_: {}", e));
      self
    }

    pub fn type_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.type_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for type_: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ActionGroup> for super::ActionGroup {
    type Error = String;

    fn try_from(value: ActionGroup) -> Result<Self, String> {
      Ok(Self {
        allowed_actions: value.allowed_actions?,
        description: value.description?,
        hidden: value.hidden?,
        reserved: value.reserved?,
        static_: value.static_?,
        type_: value.type_?,
      })
    }
  }

  impl From<super::ActionGroup> for ActionGroup {
    fn from(value: super::ActionGroup) -> Self {
      Self {
        allowed_actions: Ok(value.allowed_actions),
        description: Ok(value.description),
        hidden: Ok(value.hidden),
        reserved: Ok(value.reserved),
        static_: Ok(value.static_),
        type_: Ok(value.type_),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct AuditConfig {
    audit: Result<Option<super::AuditLogsConfig>, String>,
    compliance: Result<Option<super::ComplianceConfig>, String>,
    enabled: Result<Option<bool>, String>,
  }

  impl Default for AuditConfig {
    fn default() -> Self {
      Self {
        audit: Ok(Default::default()),
        compliance: Ok(Default::default()),
        enabled: Ok(Default::default()),
      }
    }
  }

  impl AuditConfig {
    pub fn audit<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::AuditLogsConfig>>,
      T::Error: std::fmt::Display, {
      self.audit = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for audit: {}", e));
      self
    }

    pub fn compliance<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::ComplianceConfig>>,
      T::Error: std::fmt::Display, {
      self.compliance = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for compliance: {}", e));
      self
    }

    pub fn enabled<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.enabled = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for enabled: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<AuditConfig> for super::AuditConfig {
    type Error = String;

    fn try_from(value: AuditConfig) -> Result<Self, String> {
      Ok(Self {
        audit: value.audit?,
        compliance: value.compliance?,
        enabled: value.enabled?,
      })
    }
  }

  impl From<super::AuditConfig> for AuditConfig {
    fn from(value: super::AuditConfig) -> Self {
      Self {
        audit: Ok(value.audit),
        compliance: Ok(value.compliance),
        enabled: Ok(value.enabled),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct AuditConfigWithReadOnly {
    config: Result<Option<super::AuditConfig>, String>,
    readonly: Result<Vec<String>, String>,
  }

  impl Default for AuditConfigWithReadOnly {
    fn default() -> Self {
      Self {
        config: Ok(Default::default()),
        readonly: Ok(Default::default()),
      }
    }
  }

  impl AuditConfigWithReadOnly {
    pub fn config<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::AuditConfig>>,
      T::Error: std::fmt::Display, {
      self.config = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for config: {}", e));
      self
    }

    pub fn readonly<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.readonly = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for readonly: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<AuditConfigWithReadOnly> for super::AuditConfigWithReadOnly {
    type Error = String;

    fn try_from(value: AuditConfigWithReadOnly) -> Result<Self, String> {
      Ok(Self {
        config: value.config?,
        readonly: value.readonly?,
      })
    }
  }

  impl From<super::AuditConfigWithReadOnly> for AuditConfigWithReadOnly {
    fn from(value: super::AuditConfigWithReadOnly) -> Self {
      Self {
        config: Ok(value.config),
        readonly: Ok(value.readonly),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct AuditLogsConfig {
    disabled_rest_categories: Result<Vec<String>, String>,
    disabled_transport_categories: Result<Vec<String>, String>,
    enable_rest: Result<Option<bool>, String>,
    enable_transport: Result<Option<bool>, String>,
    exclude_sensitive_headers: Result<Option<bool>, String>,
    ignore_requests: Result<Vec<String>, String>,
    ignore_users: Result<Vec<String>, String>,
    log_request_body: Result<Option<bool>, String>,
    resolve_bulk_requests: Result<Option<bool>, String>,
    resolve_indices: Result<Option<bool>, String>,
  }

  impl Default for AuditLogsConfig {
    fn default() -> Self {
      Self {
        disabled_rest_categories: Ok(Default::default()),
        disabled_transport_categories: Ok(Default::default()),
        enable_rest: Ok(Default::default()),
        enable_transport: Ok(Default::default()),
        exclude_sensitive_headers: Ok(Default::default()),
        ignore_requests: Ok(Default::default()),
        ignore_users: Ok(Default::default()),
        log_request_body: Ok(Default::default()),
        resolve_bulk_requests: Ok(Default::default()),
        resolve_indices: Ok(Default::default()),
      }
    }
  }

  impl AuditLogsConfig {
    pub fn disabled_rest_categories<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.disabled_rest_categories = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for disabled_rest_categories: {}", e));
      self
    }

    pub fn disabled_transport_categories<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.disabled_transport_categories = value.try_into().map_err(|e| {
        format!(
          "error converting supplied value for disabled_transport_categories: {}",
          e
        )
      });
      self
    }

    pub fn enable_rest<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.enable_rest = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for enable_rest: {}", e));
      self
    }

    pub fn enable_transport<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.enable_transport = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for enable_transport: {}", e));
      self
    }

    pub fn exclude_sensitive_headers<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.exclude_sensitive_headers = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for exclude_sensitive_headers: {}", e));
      self
    }

    pub fn ignore_requests<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.ignore_requests = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for ignore_requests: {}", e));
      self
    }

    pub fn ignore_users<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.ignore_users = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for ignore_users: {}", e));
      self
    }

    pub fn log_request_body<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.log_request_body = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for log_request_body: {}", e));
      self
    }

    pub fn resolve_bulk_requests<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.resolve_bulk_requests = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for resolve_bulk_requests: {}", e));
      self
    }

    pub fn resolve_indices<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.resolve_indices = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for resolve_indices: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<AuditLogsConfig> for super::AuditLogsConfig {
    type Error = String;

    fn try_from(value: AuditLogsConfig) -> Result<Self, String> {
      Ok(Self {
        disabled_rest_categories: value.disabled_rest_categories?,
        disabled_transport_categories: value.disabled_transport_categories?,
        enable_rest: value.enable_rest?,
        enable_transport: value.enable_transport?,
        exclude_sensitive_headers: value.exclude_sensitive_headers?,
        ignore_requests: value.ignore_requests?,
        ignore_users: value.ignore_users?,
        log_request_body: value.log_request_body?,
        resolve_bulk_requests: value.resolve_bulk_requests?,
        resolve_indices: value.resolve_indices?,
      })
    }
  }

  impl From<super::AuditLogsConfig> for AuditLogsConfig {
    fn from(value: super::AuditLogsConfig) -> Self {
      Self {
        disabled_rest_categories: Ok(value.disabled_rest_categories),
        disabled_transport_categories: Ok(value.disabled_transport_categories),
        enable_rest: Ok(value.enable_rest),
        enable_transport: Ok(value.enable_transport),
        exclude_sensitive_headers: Ok(value.exclude_sensitive_headers),
        ignore_requests: Ok(value.ignore_requests),
        ignore_users: Ok(value.ignore_users),
        log_request_body: Ok(value.log_request_body),
        resolve_bulk_requests: Ok(value.resolve_bulk_requests),
        resolve_indices: Ok(value.resolve_indices),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct UpdateAuditConfigurationResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for UpdateAuditConfigurationResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl UpdateAuditConfigurationResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UpdateAuditConfigurationResponseContent> for super::UpdateAuditConfigurationResponseContent {
    type Error = String;

    fn try_from(value: UpdateAuditConfigurationResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::UpdateAuditConfigurationResponseContent> for UpdateAuditConfigurationResponseContent {
    fn from(value: super::UpdateAuditConfigurationResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct UpdateConfigurationResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for UpdateConfigurationResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl UpdateConfigurationResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UpdateConfigurationResponseContent> for super::UpdateConfigurationResponseContent {
    type Error = String;

    fn try_from(value: UpdateConfigurationResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::UpdateConfigurationResponseContent> for UpdateConfigurationResponseContent {
    fn from(value: super::UpdateConfigurationResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct UpdateDistinguishedNamesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for UpdateDistinguishedNamesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl UpdateDistinguishedNamesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UpdateDistinguishedNamesResponseContent> for super::UpdateDistinguishedNamesResponseContent {
    type Error = String;

    fn try_from(value: UpdateDistinguishedNamesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::UpdateDistinguishedNamesResponseContent> for UpdateDistinguishedNamesResponseContent {
    fn from(value: super::UpdateDistinguishedNamesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct User {
    attributes: Result<Option<super::UserAttributes>, String>,
    backend_roles: Result<Vec<String>, String>,
    description: Result<Option<String>, String>,
    hash: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    opendistro_security_roles: Result<Vec<String>, String>,
    reserved: Result<Option<bool>, String>,
    static_: Result<Option<bool>, String>,
  }

  impl Default for User {
    fn default() -> Self {
      Self {
        attributes: Ok(Default::default()),
        backend_roles: Ok(Default::default()),
        description: Ok(Default::default()),
        hash: Ok(Default::default()),
        hidden: Ok(Default::default()),
        opendistro_security_roles: Ok(Default::default()),
        reserved: Ok(Default::default()),
        static_: Ok(Default::default()),
      }
    }
  }

  impl User {
    pub fn attributes<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::UserAttributes>>,
      T::Error: std::fmt::Display, {
      self.attributes = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for attributes: {}", e));
      self
    }

    pub fn backend_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.backend_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for backend_roles: {}", e));
      self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hash<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.hash = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hash: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn opendistro_security_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.opendistro_security_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for opendistro_security_roles: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn static_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.static_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for static_: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<User> for super::User {
    type Error = String;

    fn try_from(value: User) -> Result<Self, String> {
      Ok(Self {
        attributes: value.attributes?,
        backend_roles: value.backend_roles?,
        description: value.description?,
        hash: value.hash?,
        hidden: value.hidden?,
        opendistro_security_roles: value.opendistro_security_roles?,
        reserved: value.reserved?,
        static_: value.static_?,
      })
    }
  }

  impl From<super::User> for User {
    fn from(value: super::User) -> Self {
      Self {
        attributes: Ok(value.attributes),
        backend_roles: Ok(value.backend_roles),
        description: Ok(value.description),
        hash: Ok(value.hash),
        hidden: Ok(value.hidden),
        opendistro_security_roles: Ok(value.opendistro_security_roles),
        reserved: Ok(value.reserved),
        static_: Ok(value.static_),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CertificatesDetail {
    issuer_dn: Result<Option<String>, String>,
    not_after: Result<Option<String>, String>,
    not_before: Result<Option<String>, String>,
    san: Result<Option<String>, String>,
    subject_dn: Result<Option<String>, String>,
  }

  impl Default for CertificatesDetail {
    fn default() -> Self {
      Self {
        issuer_dn: Ok(Default::default()),
        not_after: Ok(Default::default()),
        not_before: Ok(Default::default()),
        san: Ok(Default::default()),
        subject_dn: Ok(Default::default()),
      }
    }
  }

  impl CertificatesDetail {
    pub fn issuer_dn<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.issuer_dn = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for issuer_dn: {}", e));
      self
    }

    pub fn not_after<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.not_after = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for not_after: {}", e));
      self
    }

    pub fn not_before<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.not_before = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for not_before: {}", e));
      self
    }

    pub fn san<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.san = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for san: {}", e));
      self
    }

    pub fn subject_dn<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.subject_dn = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for subject_dn: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CertificatesDetail> for super::CertificatesDetail {
    type Error = String;

    fn try_from(value: CertificatesDetail) -> Result<Self, String> {
      Ok(Self {
        issuer_dn: value.issuer_dn?,
        not_after: value.not_after?,
        not_before: value.not_before?,
        san: value.san?,
        subject_dn: value.subject_dn?,
      })
    }
  }

  impl From<super::CertificatesDetail> for CertificatesDetail {
    fn from(value: super::CertificatesDetail) -> Self {
      Self {
        issuer_dn: Ok(value.issuer_dn),
        not_after: Ok(value.not_after),
        not_before: Ok(value.not_before),
        san: Ok(value.san),
        subject_dn: Ok(value.subject_dn),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ChangePasswordRequestContent {
    current_password: Result<String, String>,
    password: Result<String, String>,
  }

  impl Default for ChangePasswordRequestContent {
    fn default() -> Self {
      Self {
        current_password: Err("no value supplied for current_password".to_string()),
        password: Err("no value supplied for password".to_string()),
      }
    }
  }

  impl ChangePasswordRequestContent {
    pub fn current_password<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.current_password = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for current_password: {}", e));
      self
    }

    pub fn password<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.password = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for password: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ChangePasswordRequestContent> for super::ChangePasswordRequestContent {
    type Error = String;

    fn try_from(value: ChangePasswordRequestContent) -> Result<Self, String> {
      Ok(Self {
        current_password: value.current_password?,
        password: value.password?,
      })
    }
  }

  impl From<super::ChangePasswordRequestContent> for ChangePasswordRequestContent {
    fn from(value: super::ChangePasswordRequestContent) -> Self {
      Self {
        current_password: Ok(value.current_password),
        password: Ok(value.password),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ChangePasswordResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for ChangePasswordResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl ChangePasswordResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ChangePasswordResponseContent> for super::ChangePasswordResponseContent {
    type Error = String;

    fn try_from(value: ChangePasswordResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::ChangePasswordResponseContent> for ChangePasswordResponseContent {
    fn from(value: super::ChangePasswordResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateActionGroupResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateActionGroupResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateActionGroupResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateActionGroupResponseContent> for super::CreateActionGroupResponseContent {
    type Error = String;

    fn try_from(value: CreateActionGroupResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateActionGroupResponseContent> for CreateActionGroupResponseContent {
    fn from(value: super::CreateActionGroupResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }
  #[derive(Clone, Debug)]
  pub struct CreateRoleMappingResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateRoleMappingResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateRoleMappingResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateRoleMappingResponseContent> for super::CreateRoleMappingResponseContent {
    type Error = String;

    fn try_from(value: CreateRoleMappingResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateRoleMappingResponseContent> for CreateRoleMappingResponseContent {
    fn from(value: super::CreateRoleMappingResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateRoleResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateRoleResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateRoleResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateRoleResponseContent> for super::CreateRoleResponseContent {
    type Error = String;

    fn try_from(value: CreateRoleResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateRoleResponseContent> for CreateRoleResponseContent {
    fn from(value: super::CreateRoleResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateTenantParams {
    description: Result<Option<String>, String>,
  }

  impl Default for CreateTenantParams {
    fn default() -> Self {
      Self {
        description: Ok(Default::default()),
      }
    }
  }

  impl CreateTenantParams {
    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateTenantParams> for super::CreateTenantParams {
    type Error = String;

    fn try_from(value: CreateTenantParams) -> Result<Self, String> {
      Ok(Self {
        description: value.description?,
      })
    }
  }

  impl From<super::CreateTenantParams> for CreateTenantParams {
    fn from(value: super::CreateTenantParams) -> Self {
      Self {
        description: Ok(value.description),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateTenantResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateTenantResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateTenantResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateTenantResponseContent> for super::CreateTenantResponseContent {
    type Error = String;

    fn try_from(value: CreateTenantResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateTenantResponseContent> for CreateTenantResponseContent {
    fn from(value: super::CreateTenantResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct CreateUserResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for CreateUserResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl CreateUserResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<CreateUserResponseContent> for super::CreateUserResponseContent {
    type Error = String;

    fn try_from(value: CreateUserResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::CreateUserResponseContent> for CreateUserResponseContent {
    fn from(value: super::CreateUserResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ComplianceConfig {
    enabled: Result<Option<bool>, String>,
    external_config: Result<Option<bool>, String>,
    internal_config: Result<Option<bool>, String>,
    read_ignore_users: Result<Vec<String>, String>,
    read_metadata_only: Result<Option<bool>, String>,
    read_watched_fields: Result<Option<serde_json::Value>, String>,
    write_ignore_users: Result<Vec<String>, String>,
    write_log_diffs: Result<Option<bool>, String>,
    write_metadata_only: Result<Option<bool>, String>,
    write_watched_indices: Result<Vec<String>, String>,
  }

  impl Default for ComplianceConfig {
    fn default() -> Self {
      Self {
        enabled: Ok(Default::default()),
        external_config: Ok(Default::default()),
        internal_config: Ok(Default::default()),
        read_ignore_users: Ok(Default::default()),
        read_metadata_only: Ok(Default::default()),
        read_watched_fields: Ok(Default::default()),
        write_ignore_users: Ok(Default::default()),
        write_log_diffs: Ok(Default::default()),
        write_metadata_only: Ok(Default::default()),
        write_watched_indices: Ok(Default::default()),
      }
    }
  }

  impl ComplianceConfig {
    pub fn enabled<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.enabled = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for enabled: {}", e));
      self
    }

    pub fn external_config<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.external_config = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for external_config: {}", e));
      self
    }

    pub fn internal_config<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.internal_config = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for internal_config: {}", e));
      self
    }

    pub fn read_ignore_users<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.read_ignore_users = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for read_ignore_users: {}", e));
      self
    }

    pub fn read_metadata_only<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.read_metadata_only = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for read_metadata_only: {}", e));
      self
    }

    pub fn read_watched_fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.read_watched_fields = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for read_watched_fields: {}", e));
      self
    }

    pub fn write_ignore_users<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.write_ignore_users = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for write_ignore_users: {}", e));
      self
    }

    pub fn write_log_diffs<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.write_log_diffs = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for write_log_diffs: {}", e));
      self
    }

    pub fn write_metadata_only<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.write_metadata_only = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for write_metadata_only: {}", e));
      self
    }

    pub fn write_watched_indices<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.write_watched_indices = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for write_watched_indices: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ComplianceConfig> for super::ComplianceConfig {
    type Error = String;

    fn try_from(value: ComplianceConfig) -> Result<Self, String> {
      Ok(Self {
        enabled: value.enabled?,
        external_config: value.external_config?,
        internal_config: value.internal_config?,
        read_ignore_users: value.read_ignore_users?,
        read_metadata_only: value.read_metadata_only?,
        read_watched_fields: value.read_watched_fields?,
        write_ignore_users: value.write_ignore_users?,
        write_log_diffs: value.write_log_diffs?,
        write_metadata_only: value.write_metadata_only?,
        write_watched_indices: value.write_watched_indices?,
      })
    }
  }

  impl From<super::ComplianceConfig> for ComplianceConfig {
    fn from(value: super::ComplianceConfig) -> Self {
      Self {
        enabled: Ok(value.enabled),
        external_config: Ok(value.external_config),
        internal_config: Ok(value.internal_config),
        read_ignore_users: Ok(value.read_ignore_users),
        read_metadata_only: Ok(value.read_metadata_only),
        read_watched_fields: Ok(value.read_watched_fields),
        write_ignore_users: Ok(value.write_ignore_users),
        write_log_diffs: Ok(value.write_log_diffs),
        write_metadata_only: Ok(value.write_metadata_only),
        write_watched_indices: Ok(value.write_watched_indices),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteActionGroupResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteActionGroupResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteActionGroupResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteActionGroupResponseContent> for super::DeleteActionGroupResponseContent {
    type Error = String;

    fn try_from(value: DeleteActionGroupResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteActionGroupResponseContent> for DeleteActionGroupResponseContent {
    fn from(value: super::DeleteActionGroupResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteRoleMappingResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteRoleMappingResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteRoleMappingResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteRoleMappingResponseContent> for super::DeleteRoleMappingResponseContent {
    type Error = String;

    fn try_from(value: DeleteRoleMappingResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteRoleMappingResponseContent> for DeleteRoleMappingResponseContent {
    fn from(value: super::DeleteRoleMappingResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteRoleResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteRoleResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteRoleResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteRoleResponseContent> for super::DeleteRoleResponseContent {
    type Error = String;

    fn try_from(value: DeleteRoleResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteRoleResponseContent> for DeleteRoleResponseContent {
    fn from(value: super::DeleteRoleResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteTenantResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteTenantResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteTenantResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteTenantResponseContent> for super::DeleteTenantResponseContent {
    type Error = String;

    fn try_from(value: DeleteTenantResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteTenantResponseContent> for DeleteTenantResponseContent {
    fn from(value: super::DeleteTenantResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteUserResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteUserResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteUserResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteUserResponseContent> for super::DeleteUserResponseContent {
    type Error = String;

    fn try_from(value: DeleteUserResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteUserResponseContent> for DeleteUserResponseContent {
    fn from(value: super::DeleteUserResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DeleteDistinguishedNamesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for DeleteDistinguishedNamesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl DeleteDistinguishedNamesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DeleteDistinguishedNamesResponseContent> for super::DeleteDistinguishedNamesResponseContent {
    type Error = String;

    fn try_from(value: DeleteDistinguishedNamesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::DeleteDistinguishedNamesResponseContent> for DeleteDistinguishedNamesResponseContent {
    fn from(value: super::DeleteDistinguishedNamesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DistinguishedNames {
    nodes_dn: Result<Vec<String>, String>,
  }

  impl Default for DistinguishedNames {
    fn default() -> Self {
      Self {
        nodes_dn: Ok(Default::default()),
      }
    }
  }

  impl DistinguishedNames {
    pub fn nodes_dn<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.nodes_dn = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for nodes_dn: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DistinguishedNames> for super::DistinguishedNames {
    type Error = String;

    fn try_from(value: DistinguishedNames) -> Result<Self, String> {
      Ok(Self {
        nodes_dn: value.nodes_dn?,
      })
    }
  }

  impl From<super::DistinguishedNames> for DistinguishedNames {
    fn from(value: super::DistinguishedNames) -> Self {
      Self {
        nodes_dn: Ok(value.nodes_dn),
      }
    }
  }
  #[derive(Clone, Debug)]
  pub struct DynamicConfig {
    dynamic: Result<Option<super::DynamicOptions>, String>,
  }

  impl Default for DynamicConfig {
    fn default() -> Self {
      Self {
        dynamic: Ok(Default::default()),
      }
    }
  }

  impl DynamicConfig {
    pub fn dynamic<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::DynamicOptions>>,
      T::Error: std::fmt::Display, {
      self.dynamic = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for dynamic: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<DynamicConfig> for super::DynamicConfig {
    type Error = String;

    fn try_from(value: DynamicConfig) -> Result<Self, String> {
      Ok(Self {
        dynamic: value.dynamic?,
      })
    }
  }

  impl From<super::DynamicConfig> for DynamicConfig {
    fn from(value: super::DynamicConfig) -> Self {
      Self {
        dynamic: Ok(value.dynamic),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct DynamicOptions {
    auth_failure_listeners: Result<Option<serde_json::Value>, String>,
    authc: Result<Option<serde_json::Value>, String>,
    authz: Result<Option<serde_json::Value>, String>,
    disable_intertransport_auth: Result<Option<bool>, String>,
    disable_rest_auth: Result<Option<bool>, String>,
    do_not_fail_on_forbidden: Result<Option<bool>, String>,
    do_not_fail_on_forbidden_empty: Result<Option<bool>, String>,
    filtered_alias_mode: Result<Option<String>, String>,
    hosts_resolver_mode: Result<Option<String>, String>,
    http: Result<Option<serde_json::Value>, String>,
    kibana: Result<Option<serde_json::Value>, String>,
    multi_rolespan_enabled: Result<Option<bool>, String>,
    respect_request_indices_options: Result<Option<bool>, String>,
  }

  impl Default for DynamicOptions {
    fn default() -> Self {
      Self {
        auth_failure_listeners: Ok(Default::default()),
        authc: Ok(Default::default()),
        authz: Ok(Default::default()),
        disable_intertransport_auth: Ok(Default::default()),
        disable_rest_auth: Ok(Default::default()),
        do_not_fail_on_forbidden: Ok(Default::default()),
        do_not_fail_on_forbidden_empty: Ok(Default::default()),
        filtered_alias_mode: Ok(Default::default()),
        hosts_resolver_mode: Ok(Default::default()),
        http: Ok(Default::default()),
        kibana: Ok(Default::default()),
        multi_rolespan_enabled: Ok(Default::default()),
        respect_request_indices_options: Ok(Default::default()),
      }
    }
  }

  impl DynamicOptions {
    pub fn auth_failure_listeners<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.auth_failure_listeners = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for auth_failure_listeners: {}", e));
      self
    }

    pub fn authc<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.authc = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for authc: {}", e));
      self
    }

    pub fn authz<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.authz = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for authz: {}", e));
      self
    }

    pub fn disable_intertransport_auth<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.disable_intertransport_auth = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for disable_intertransport_auth: {}", e));
      self
    }

    pub fn disable_rest_auth<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.disable_rest_auth = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for disable_rest_auth: {}", e));
      self
    }

    pub fn do_not_fail_on_forbidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.do_not_fail_on_forbidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for do_not_fail_on_forbidden: {}", e));
      self
    }

    pub fn do_not_fail_on_forbidden_empty<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.do_not_fail_on_forbidden_empty = value.try_into().map_err(|e| {
        format!(
          "error converting supplied value for do_not_fail_on_forbidden_empty: {}",
          e
        )
      });
      self
    }

    pub fn filtered_alias_mode<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.filtered_alias_mode = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for filtered_alias_mode: {}", e));
      self
    }

    pub fn hosts_resolver_mode<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.hosts_resolver_mode = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hosts_resolver_mode: {}", e));
      self
    }

    pub fn http<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.http = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for http: {}", e));
      self
    }

    pub fn kibana<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.kibana = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for kibana: {}", e));
      self
    }

    pub fn multi_rolespan_enabled<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.multi_rolespan_enabled = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for multi_rolespan_enabled: {}", e));
      self
    }

    pub fn respect_request_indices_options<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.respect_request_indices_options = value.try_into().map_err(|e| {
        format!(
          "error converting supplied value for respect_request_indices_options: {}",
          e
        )
      });
      self
    }
  }

  impl std::convert::TryFrom<DynamicOptions> for super::DynamicOptions {
    type Error = String;

    fn try_from(value: DynamicOptions) -> Result<Self, String> {
      Ok(Self {
        auth_failure_listeners: value.auth_failure_listeners?,
        authc: value.authc?,
        authz: value.authz?,
        disable_intertransport_auth: value.disable_intertransport_auth?,
        disable_rest_auth: value.disable_rest_auth?,
        do_not_fail_on_forbidden: value.do_not_fail_on_forbidden?,
        do_not_fail_on_forbidden_empty: value.do_not_fail_on_forbidden_empty?,
        filtered_alias_mode: value.filtered_alias_mode?,
        hosts_resolver_mode: value.hosts_resolver_mode?,
        http: value.http?,
        kibana: value.kibana?,
        multi_rolespan_enabled: value.multi_rolespan_enabled?,
        respect_request_indices_options: value.respect_request_indices_options?,
      })
    }
  }

  impl From<super::DynamicOptions> for DynamicOptions {
    fn from(value: super::DynamicOptions) -> Self {
      Self {
        auth_failure_listeners: Ok(value.auth_failure_listeners),
        authc: Ok(value.authc),
        authz: Ok(value.authz),
        disable_intertransport_auth: Ok(value.disable_intertransport_auth),
        disable_rest_auth: Ok(value.disable_rest_auth),
        do_not_fail_on_forbidden: Ok(value.do_not_fail_on_forbidden),
        do_not_fail_on_forbidden_empty: Ok(value.do_not_fail_on_forbidden_empty),
        filtered_alias_mode: Ok(value.filtered_alias_mode),
        hosts_resolver_mode: Ok(value.hosts_resolver_mode),
        http: Ok(value.http),
        kibana: Ok(value.kibana),
        multi_rolespan_enabled: Ok(value.multi_rolespan_enabled),
        respect_request_indices_options: Ok(value.respect_request_indices_options),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct FlushCacheResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for FlushCacheResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl FlushCacheResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<FlushCacheResponseContent> for super::FlushCacheResponseContent {
    type Error = String;

    fn try_from(value: FlushCacheResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::FlushCacheResponseContent> for FlushCacheResponseContent {
    fn from(value: super::FlushCacheResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }
  #[derive(Clone, Debug)]
  pub struct GetCertificatesResponseContent {
    http_certificates_list: Result<Vec<super::CertificatesDetail>, String>,
    transport_certificates_list: Result<Vec<super::CertificatesDetail>, String>,
  }

  impl Default for GetCertificatesResponseContent {
    fn default() -> Self {
      Self {
        http_certificates_list: Ok(Default::default()),
        transport_certificates_list: Ok(Default::default()),
      }
    }
  }

  impl GetCertificatesResponseContent {
    pub fn http_certificates_list<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::CertificatesDetail>>,
      T::Error: std::fmt::Display, {
      self.http_certificates_list = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for http_certificates_list: {}", e));
      self
    }

    pub fn transport_certificates_list<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<super::CertificatesDetail>>,
      T::Error: std::fmt::Display, {
      self.transport_certificates_list = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for transport_certificates_list: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<GetCertificatesResponseContent> for super::GetCertificatesResponseContent {
    type Error = String;

    fn try_from(value: GetCertificatesResponseContent) -> Result<Self, String> {
      Ok(Self {
        http_certificates_list: value.http_certificates_list?,
        transport_certificates_list: value.transport_certificates_list?,
      })
    }
  }

  impl From<super::GetCertificatesResponseContent> for GetCertificatesResponseContent {
    fn from(value: super::GetCertificatesResponseContent) -> Self {
      Self {
        http_certificates_list: Ok(value.http_certificates_list),
        transport_certificates_list: Ok(value.transport_certificates_list),
      }
    }
  }
  #[derive(Clone, Debug)]
  pub struct IndexPermission {
    allowed_actions: Result<Vec<String>, String>,
    fls: Result<Vec<String>, String>,
    index_patterns: Result<Vec<String>, String>,
    masked_fields: Result<Vec<String>, String>,
  }

  impl Default for IndexPermission {
    fn default() -> Self {
      Self {
        allowed_actions: Ok(Default::default()),
        fls: Ok(Default::default()),
        index_patterns: Ok(Default::default()),
        masked_fields: Ok(Default::default()),
      }
    }
  }

  impl IndexPermission {
    pub fn allowed_actions<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.allowed_actions = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for allowed_actions: {}", e));
      self
    }

    pub fn fls<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.fls = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for fls: {}", e));
      self
    }

    pub fn index_patterns<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.index_patterns = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index_patterns: {}", e));
      self
    }

    pub fn masked_fields<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.masked_fields = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for masked_fields: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<IndexPermission> for super::IndexPermission {
    type Error = String;

    fn try_from(value: IndexPermission) -> Result<Self, String> {
      Ok(Self {
        allowed_actions: value.allowed_actions?,
        fls: value.fls?,
        index_patterns: value.index_patterns?,
        masked_fields: value.masked_fields?,
      })
    }
  }

  impl From<super::IndexPermission> for IndexPermission {
    fn from(value: super::IndexPermission) -> Self {
      Self {
        allowed_actions: Ok(value.allowed_actions),
        fls: Ok(value.fls),
        index_patterns: Ok(value.index_patterns),
        masked_fields: Ok(value.masked_fields),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchActionGroupResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchActionGroupResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchActionGroupResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchActionGroupResponseContent> for super::PatchActionGroupResponseContent {
    type Error = String;

    fn try_from(value: PatchActionGroupResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchActionGroupResponseContent> for PatchActionGroupResponseContent {
    fn from(value: super::PatchActionGroupResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchActionGroupsResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchActionGroupsResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchActionGroupsResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchActionGroupsResponseContent> for super::PatchActionGroupsResponseContent {
    type Error = String;

    fn try_from(value: PatchActionGroupsResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchActionGroupsResponseContent> for PatchActionGroupsResponseContent {
    fn from(value: super::PatchActionGroupsResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchConfigurationResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchConfigurationResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchConfigurationResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchConfigurationResponseContent> for super::PatchConfigurationResponseContent {
    type Error = String;

    fn try_from(value: PatchConfigurationResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchConfigurationResponseContent> for PatchConfigurationResponseContent {
    fn from(value: super::PatchConfigurationResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchDistinguishedNamesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchDistinguishedNamesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchDistinguishedNamesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchDistinguishedNamesResponseContent> for super::PatchDistinguishedNamesResponseContent {
    type Error = String;

    fn try_from(value: PatchDistinguishedNamesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchDistinguishedNamesResponseContent> for PatchDistinguishedNamesResponseContent {
    fn from(value: super::PatchDistinguishedNamesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchOperation {
    op: Result<String, String>,
    path: Result<String, String>,
    value: Result<Option<serde_json::Value>, String>,
  }

  impl Default for PatchOperation {
    fn default() -> Self {
      Self {
        op: Err("no value supplied for op".to_string()),
        path: Err("no value supplied for path".to_string()),
        value: Ok(Default::default()),
      }
    }
  }

  impl PatchOperation {
    pub fn op<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.op = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for op: {}", e));
      self
    }

    pub fn path<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display, {
      self.path = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for path: {}", e));
      self
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<serde_json::Value>>,
      T::Error: std::fmt::Display, {
      self.value = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for value: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchOperation> for super::PatchOperation {
    type Error = String;

    fn try_from(value: PatchOperation) -> Result<Self, String> {
      Ok(Self {
        op: value.op?,
        path: value.path?,
        value: value.value?,
      })
    }
  }

  impl From<super::PatchOperation> for PatchOperation {
    fn from(value: super::PatchOperation) -> Self {
      Self {
        op: Ok(value.op),
        path: Ok(value.path),
        value: Ok(value.value),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchRoleMappingResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchRoleMappingResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchRoleMappingResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchRoleMappingResponseContent> for super::PatchRoleMappingResponseContent {
    type Error = String;

    fn try_from(value: PatchRoleMappingResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchRoleMappingResponseContent> for PatchRoleMappingResponseContent {
    fn from(value: super::PatchRoleMappingResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchRoleMappingsResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchRoleMappingsResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchRoleMappingsResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchRoleMappingsResponseContent> for super::PatchRoleMappingsResponseContent {
    type Error = String;

    fn try_from(value: PatchRoleMappingsResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchRoleMappingsResponseContent> for PatchRoleMappingsResponseContent {
    fn from(value: super::PatchRoleMappingsResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchRoleResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchRoleResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchRoleResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchRoleResponseContent> for super::PatchRoleResponseContent {
    type Error = String;

    fn try_from(value: PatchRoleResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchRoleResponseContent> for PatchRoleResponseContent {
    fn from(value: super::PatchRoleResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchRolesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchRolesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchRolesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchRolesResponseContent> for super::PatchRolesResponseContent {
    type Error = String;

    fn try_from(value: PatchRolesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchRolesResponseContent> for PatchRolesResponseContent {
    fn from(value: super::PatchRolesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchTenantResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchTenantResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchTenantResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchTenantResponseContent> for super::PatchTenantResponseContent {
    type Error = String;

    fn try_from(value: PatchTenantResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchTenantResponseContent> for PatchTenantResponseContent {
    fn from(value: super::PatchTenantResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchTenantsResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchTenantsResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchTenantsResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchTenantsResponseContent> for super::PatchTenantsResponseContent {
    type Error = String;

    fn try_from(value: PatchTenantsResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchTenantsResponseContent> for PatchTenantsResponseContent {
    fn from(value: super::PatchTenantsResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchUserResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchUserResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchUserResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchUserResponseContent> for super::PatchUserResponseContent {
    type Error = String;

    fn try_from(value: PatchUserResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchUserResponseContent> for PatchUserResponseContent {
    fn from(value: super::PatchUserResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct PatchUsersResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for PatchUsersResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl PatchUsersResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<PatchUsersResponseContent> for super::PatchUsersResponseContent {
    type Error = String;

    fn try_from(value: PatchUsersResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::PatchUsersResponseContent> for PatchUsersResponseContent {
    fn from(value: super::PatchUsersResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ReloadHttpCertificatesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for ReloadHttpCertificatesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl ReloadHttpCertificatesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ReloadHttpCertificatesResponseContent> for super::ReloadHttpCertificatesResponseContent {
    type Error = String;

    fn try_from(value: ReloadHttpCertificatesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::ReloadHttpCertificatesResponseContent> for ReloadHttpCertificatesResponseContent {
    fn from(value: super::ReloadHttpCertificatesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct ReloadTransportCertificatesResponseContent {
    message: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for ReloadTransportCertificatesResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl ReloadTransportCertificatesResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<ReloadTransportCertificatesResponseContent>
    for super::ReloadTransportCertificatesResponseContent
  {
    type Error = String;

    fn try_from(value: ReloadTransportCertificatesResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        status: value.status?,
      })
    }
  }

  impl From<super::ReloadTransportCertificatesResponseContent> for ReloadTransportCertificatesResponseContent {
    fn from(value: super::ReloadTransportCertificatesResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct Role {
    cluster_permission: Result<Vec<String>, String>,
    description: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    index_permission: Result<Option<super::IndexPermission>, String>,
    reserved: Result<Option<bool>, String>,
    static_: Result<Option<bool>, String>,
    tenant_permissions: Result<Vec<String>, String>,
  }

  impl Default for Role {
    fn default() -> Self {
      Self {
        cluster_permission: Ok(Default::default()),
        description: Ok(Default::default()),
        hidden: Ok(Default::default()),
        index_permission: Ok(Default::default()),
        reserved: Ok(Default::default()),
        static_: Ok(Default::default()),
        tenant_permissions: Ok(Default::default()),
      }
    }
  }

  impl Role {
    pub fn cluster_permission<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.cluster_permission = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for cluster_permission: {}", e));
      self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn index_permission<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<super::IndexPermission>>,
      T::Error: std::fmt::Display, {
      self.index_permission = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for index_permission: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn static_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.static_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for static_: {}", e));
      self
    }

    pub fn tenant_permissions<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.tenant_permissions = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for tenant_permissions: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<Role> for super::Role {
    type Error = String;

    fn try_from(value: Role) -> Result<Self, String> {
      Ok(Self {
        cluster_permission: value.cluster_permission?,
        description: value.description?,
        hidden: value.hidden?,
        index_permission: value.index_permission?,
        reserved: value.reserved?,
        static_: value.static_?,
        tenant_permissions: value.tenant_permissions?,
      })
    }
  }

  impl From<super::Role> for Role {
    fn from(value: super::Role) -> Self {
      Self {
        cluster_permission: Ok(value.cluster_permission),
        description: Ok(value.description),
        hidden: Ok(value.hidden),
        index_permission: Ok(value.index_permission),
        reserved: Ok(value.reserved),
        static_: Ok(value.static_),
        tenant_permissions: Ok(value.tenant_permissions),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct RoleMapping {
    and_backend_roles: Result<Vec<String>, String>,
    backend_roles: Result<Vec<String>, String>,
    description: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    hosts: Result<Vec<String>, String>,
    reserved: Result<Option<bool>, String>,
    users: Result<Vec<String>, String>,
  }

  impl Default for RoleMapping {
    fn default() -> Self {
      Self {
        and_backend_roles: Ok(Default::default()),
        backend_roles: Ok(Default::default()),
        description: Ok(Default::default()),
        hidden: Ok(Default::default()),
        hosts: Ok(Default::default()),
        reserved: Ok(Default::default()),
        users: Ok(Default::default()),
      }
    }
  }

  impl RoleMapping {
    pub fn and_backend_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.and_backend_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for and_backend_roles: {}", e));
      self
    }

    pub fn backend_roles<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.backend_roles = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for backend_roles: {}", e));
      self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn hosts<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.hosts = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hosts: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn users<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Vec<String>>,
      T::Error: std::fmt::Display, {
      self.users = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for users: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<RoleMapping> for super::RoleMapping {
    type Error = String;

    fn try_from(value: RoleMapping) -> Result<Self, String> {
      Ok(Self {
        and_backend_roles: value.and_backend_roles?,
        backend_roles: value.backend_roles?,
        description: value.description?,
        hidden: value.hidden?,
        hosts: value.hosts?,
        reserved: value.reserved?,
        users: value.users?,
      })
    }
  }

  impl From<super::RoleMapping> for RoleMapping {
    fn from(value: super::RoleMapping) -> Self {
      Self {
        and_backend_roles: Ok(value.and_backend_roles),
        backend_roles: Ok(value.backend_roles),
        description: Ok(value.description),
        hidden: Ok(value.hidden),
        hosts: Ok(value.hosts),
        reserved: Ok(value.reserved),
        users: Ok(value.users),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct SecurityHealthResponseContent {
    message: Result<Option<String>, String>,
    mode: Result<Option<String>, String>,
    status: Result<Option<String>, String>,
  }

  impl Default for SecurityHealthResponseContent {
    fn default() -> Self {
      Self {
        message: Ok(Default::default()),
        mode: Ok(Default::default()),
        status: Ok(Default::default()),
      }
    }
  }

  impl SecurityHealthResponseContent {
    pub fn message<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.message = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for message: {}", e));
      self
    }

    pub fn mode<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.mode = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for mode: {}", e));
      self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.status = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for status: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<SecurityHealthResponseContent> for super::SecurityHealthResponseContent {
    type Error = String;

    fn try_from(value: SecurityHealthResponseContent) -> Result<Self, String> {
      Ok(Self {
        message: value.message?,
        mode: value.mode?,
        status: value.status?,
      })
    }
  }

  impl From<super::SecurityHealthResponseContent> for SecurityHealthResponseContent {
    fn from(value: super::SecurityHealthResponseContent) -> Self {
      Self {
        message: Ok(value.message),
        mode: Ok(value.mode),
        status: Ok(value.status),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct Tenant {
    description: Result<Option<String>, String>,
    hidden: Result<Option<bool>, String>,
    reserved: Result<Option<bool>, String>,
    static_: Result<Option<bool>, String>,
  }

  impl Default for Tenant {
    fn default() -> Self {
      Self {
        description: Ok(Default::default()),
        hidden: Ok(Default::default()),
        reserved: Ok(Default::default()),
        static_: Ok(Default::default()),
      }
    }
  }

  impl Tenant {
    pub fn description<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<String>>,
      T::Error: std::fmt::Display, {
      self.description = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for description: {}", e));
      self
    }

    pub fn hidden<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.hidden = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for hidden: {}", e));
      self
    }

    pub fn reserved<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.reserved = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for reserved: {}", e));
      self
    }

    pub fn static_<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.static_ = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for static_: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<Tenant> for super::Tenant {
    type Error = String;

    fn try_from(value: Tenant) -> Result<Self, String> {
      Ok(Self {
        description: value.description?,
        hidden: value.hidden?,
        reserved: value.reserved?,
        static_: value.static_?,
      })
    }
  }

  impl From<super::Tenant> for Tenant {
    fn from(value: super::Tenant) -> Self {
      Self {
        description: Ok(value.description),
        hidden: Ok(value.hidden),
        reserved: Ok(value.reserved),
        static_: Ok(value.static_),
      }
    }
  }

  #[derive(Clone, Debug)]
  pub struct UserTenants {
    admin: Result<Option<bool>, String>,
    admin_tenant: Result<Option<bool>, String>,
    global_tenant: Result<Option<bool>, String>,
  }

  impl Default for UserTenants {
    fn default() -> Self {
      Self {
        admin: Ok(Default::default()),
        admin_tenant: Ok(Default::default()),
        global_tenant: Ok(Default::default()),
      }
    }
  }

  impl UserTenants {
    pub fn admin<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.admin = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for admin: {}", e));
      self
    }

    pub fn admin_tenant<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.admin_tenant = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for admin_tenant: {}", e));
      self
    }

    pub fn global_tenant<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<Option<bool>>,
      T::Error: std::fmt::Display, {
      self.global_tenant = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for global_tenant: {}", e));
      self
    }
  }

  impl std::convert::TryFrom<UserTenants> for super::UserTenants {
    type Error = String;

    fn try_from(value: UserTenants) -> Result<Self, String> {
      Ok(Self {
        admin: value.admin?,
        admin_tenant: value.admin_tenant?,
        global_tenant: value.global_tenant?,
      })
    }
  }

  impl From<super::UserTenants> for UserTenants {
    fn from(value: super::UserTenants) -> Self {
      Self {
        admin: Ok(value.admin),
        admin_tenant: Ok(value.admin_tenant),
        global_tenant: Ok(value.global_tenant),
      }
    }
  }
}
