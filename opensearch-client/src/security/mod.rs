use crate::OsClient;
mod builder;
mod types;
pub struct Security<'a> {
  os_client: &'a OsClient,
}

impl<'a> Security<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Returns account details for the current user.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/account`
  ///
  ///```ignore
  /// let response = client.security().get_account_details()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_account_details(&self) -> builder::GetAccountDetails {
    builder::GetAccountDetails::new(self.os_client)
  }

  ///Changes the password for the current user.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/account`
  ///
  ///```ignore
  /// let response = client.security().change_password()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn change_password(&self) -> builder::ChangePassword {
    builder::ChangePassword::new(self.os_client)
  }

  ///Creates, updates, or deletes multiple action groups in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/actiongroups`
  ///
  ///```ignore
  /// let response = client.security().patch_action_groups()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_action_groups(&self) -> builder::PatchActionGroups {
    builder::PatchActionGroups::new(self.os_client)
  }

  ///Retrieves all action groups.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/actiongroups/`
  ///
  ///```ignore
  /// let response = client.security().get_action_groups()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_action_groups(&self) -> builder::GetActionGroups {
    builder::GetActionGroups::new(self.os_client)
  }

  ///Retrieves one action group.
  ///
  ///Sends a `GET` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  ///
  ///Arguments:
  /// - `action_group`: Action group to retrieve.
  ///```ignore
  /// let response = client.security().get_action_group()
  ///    .action_group(action_group)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_action_group(&self) -> builder::GetActionGroup {
    builder::GetActionGroup::new(self.os_client)
  }

  ///Creates or replaces the specified action group.
  ///
  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  ///
  ///Arguments:
  /// - `action_group`: The name of the action group to create or replace
  /// - `body`
  ///```ignore
  /// let response = client.security().create_action_group()
  ///    .action_group(action_group)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_action_group(&self) -> builder::CreateActionGroup {
    builder::CreateActionGroup::new(self.os_client)
  }

  ///Delete a specified action group.
  ///
  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  ///
  ///Arguments:
  /// - `action_group`: Action group to delete.
  ///```ignore
  /// let response = client.security().delete_action_group()
  ///    .action_group(action_group)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_action_group(&self) -> builder::DeleteActionGroup {
    builder::DeleteActionGroup::new(self.os_client)
  }

  ///Updates individual attributes of an action group.
  ///
  ///Sends a `PATCH` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  ///
  ///```ignore
  /// let response = client.security().patch_action_group()
  ///    .action_group(action_group)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_action_group(&self) -> builder::PatchActionGroup {
    builder::PatchActionGroup::new(self.os_client)
  }

  ///Retrieves the audit configuration.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/audit`
  ///
  ///```ignore
  /// let response = client.security().get_audit_configuration()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_audit_configuration(&self) -> builder::GetAuditConfiguration {
    builder::GetAuditConfiguration::new(self.os_client)
  }

  ///A PATCH call is used to update specified fields in the audit
  /// configuration.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/audit`
  ///
  ///```ignore
  /// let response = client.security().patch_audit_configuration()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_audit_configuration(&self) -> builder::PatchAuditConfiguration {
    builder::PatchAuditConfiguration::new(self.os_client)
  }

  ///Updates the audit configuration.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/audit/config`
  ///
  ///```ignore
  /// let response = client.security().update_audit_configuration()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update_audit_configuration(&self) -> builder::UpdateAuditConfiguration {
    builder::UpdateAuditConfiguration::new(self.os_client)
  }

  ///Flushes the Security plugin user, authentication, and authorization
  /// cache.
  ///
  ///Sends a `DELETE` request to `/_plugins/_security/api/cache`
  ///
  ///```ignore
  /// let response = client.security().flush_cache()
  ///    .send()
  ///    .await;
  /// ```
  pub fn flush_cache(&self) -> builder::FlushCache {
    builder::FlushCache::new(self.os_client)
  }

  ///Retrieve all internal users.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/internalusers`
  ///
  ///```ignore
  /// let response = client.security().get_users()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_users(&self) -> builder::GetUsers {
    builder::GetUsers::new(self.os_client)
  }

  ///Creates, updates, or deletes multiple internal users in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/internalusers`
  ///
  ///```ignore
  /// let response = client.security().patch_users()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_users(&self) -> builder::PatchUsers {
    builder::PatchUsers::new(self.os_client)
  }

  ///Retrieve one internal user.
  ///
  ///Sends a `GET` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  ///
  ///```ignore
  /// let response = client.security().get_user()
  ///    .username(username)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_user(&self) -> builder::GetUser {
    builder::GetUser::new(self.os_client)
  }

  ///Creates or replaces the specified user.
  ///
  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  ///
  ///```ignore
  /// let response = client.security().create_user()
  ///    .username(username)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_user(&self) -> builder::CreateUser {
    builder::CreateUser::new(self.os_client)
  }

  ///Delete the specified user.
  ///
  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  ///
  ///```ignore
  /// let response = client.security().delete_user()
  ///    .username(username)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_user(&self) -> builder::DeleteUser {
    builder::DeleteUser::new(self.os_client)
  }

  ///Updates individual attributes of an internal user.
  ///
  ///Sends a `PATCH` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  ///
  ///```ignore
  /// let response = client.security().patch_user()
  ///    .username(username)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_user(&self) -> builder::PatchUser {
    builder::PatchUser::new(self.os_client)
  }

  ///Retrieves all distinguished names in the allow list.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/nodesdn`
  ///
  ///```ignore
  /// let response = client.security().get_distinguished_names()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_distinguished_names(&self) -> builder::GetDistinguishedNames {
    builder::GetDistinguishedNames::new(self.os_client)
  }

  ///Bulk update of distinguished names.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/nodesdn`
  ///
  ///```ignore
  /// let response = client.security().patch_distinguished_names()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_distinguished_names(&self) -> builder::PatchDistinguishedNames {
    builder::PatchDistinguishedNames::new(self.os_client)
  }

  ///Retrieve distinguished names of a specified cluster.
  ///
  ///Sends a `GET` request to
  /// `/_plugins/_security/api/nodesdn/{cluster_name}`
  ///
  ///```ignore
  /// let response = client.security().get_distinguished_names_with_cluster_name()
  ///    .cluster_name(cluster_name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_distinguished_names_with_cluster_name(&self) -> builder::GetDistinguishedNamesWithClusterName {
    builder::GetDistinguishedNamesWithClusterName::new(self.os_client)
  }

  ///Adds or updates the specified distinguished names in the cluster’s or
  /// node’s allow list.
  ///
  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/nodesdn/{cluster_name}`
  ///
  ///```ignore
  /// let response = client.security().update_distinguished_names()
  ///    .cluster_name(cluster_name)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update_distinguished_names(&self) -> builder::UpdateDistinguishedNames {
    builder::UpdateDistinguishedNames::new(self.os_client)
  }

  ///Deletes all distinguished names in the specified cluster’s or node’s
  /// allow list.
  ///
  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/nodesdn/{cluster_name}`
  ///
  ///```ignore
  /// let response = client.security().delete_distinguished_names()
  ///    .cluster_name(cluster_name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_distinguished_names(&self) -> builder::DeleteDistinguishedNames {
    builder::DeleteDistinguishedNames::new(self.os_client)
  }

  ///Creates, updates, or deletes multiple roles in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/roles`
  ///
  ///```ignore
  /// let response = client.security().patch_roles()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_roles(&self) -> builder::PatchRoles {
    builder::PatchRoles::new(self.os_client)
  }

  ///Retrieves all roles.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/roles/`
  ///
  ///```ignore
  /// let response = client.security().get_roles()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_roles(&self) -> builder::GetRoles {
    builder::GetRoles::new(self.os_client)
  }

  ///Retrieves one role.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/roles/{role}`
  ///
  ///```ignore
  /// let response = client.security().get_role()
  ///    .role(role)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_role(&self) -> builder::GetRole {
    builder::GetRole::new(self.os_client)
  }

  ///Creates or replaces the specified role.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/roles/{role}`
  ///
  ///```ignore
  /// let response = client.security().create_role()
  ///    .role(role)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_role(&self) -> builder::CreateRole {
    builder::CreateRole::new(self.os_client)
  }

  ///Delete the specified role.
  ///
  ///Sends a `DELETE` request to `/_plugins/_security/api/roles/{role}`
  ///
  ///```ignore
  /// let response = client.security().delete_role()
  ///    .role(role)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_role(&self) -> builder::DeleteRole {
    builder::DeleteRole::new(self.os_client)
  }

  ///Updates individual attributes of a role.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/roles/{role}`
  ///
  ///```ignore
  /// let response = client.security().patch_role()
  ///    .role(role)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_role(&self) -> builder::PatchRole {
    builder::PatchRole::new(self.os_client)
  }

  ///Retrieves all role mappings.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/rolesmapping`
  ///
  ///```ignore
  /// let response = client.security().get_role_mappings()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_role_mappings(&self) -> builder::GetRoleMappings {
    builder::GetRoleMappings::new(self.os_client)
  }

  ///Creates or updates multiple role mappings in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/rolesmapping`
  ///
  ///```ignore
  /// let response = client.security().patch_role_mappings()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_role_mappings(&self) -> builder::PatchRoleMappings {
    builder::PatchRoleMappings::new(self.os_client)
  }

  ///Retrieves one role mapping.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/rolesmapping/{role}`
  ///
  ///```ignore
  /// let response = client.security().get_role_mapping()
  ///    .role(role)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_role_mapping(&self) -> builder::GetRoleMapping {
    builder::GetRoleMapping::new(self.os_client)
  }

  ///Creates or replaces the specified role mapping.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/rolesmapping/{role}`
  ///
  ///```ignore
  /// let response = client.security().create_role_mapping()
  ///    .role(role)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_role_mapping(&self) -> builder::CreateRoleMapping {
    builder::CreateRoleMapping::new(self.os_client)
  }

  ///Deletes the specified role mapping.
  ///
  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/rolesmapping/{role}`
  ///
  ///```ignore
  /// let response = client.security().delete_role_mapping()
  ///    .role(role)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_role_mapping(&self) -> builder::DeleteRoleMapping {
    builder::DeleteRoleMapping::new(self.os_client)
  }

  ///Updates individual attributes of a role mapping.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/rolesmapping/{role}`
  ///
  ///```ignore
  /// let response = client.security().patch_role_mapping()
  ///    .role(role)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_role_mapping(&self) -> builder::PatchRoleMapping {
    builder::PatchRoleMapping::new(self.os_client)
  }

  ///Returns the current Security plugin configuration in JSON format.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/securityconfig`
  ///
  ///```ignore
  /// let response = client.security().get_configuration()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_configuration(&self) -> builder::GetConfiguration {
    builder::GetConfiguration::new(self.os_client)
  }

  ///A PATCH call is used to update the existing configuration using the REST
  /// API.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/securityconfig`
  ///
  ///```ignore
  /// let response = client.security().patch_configuration()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_configuration(&self) -> builder::PatchConfiguration {
    builder::PatchConfiguration::new(self.os_client)
  }

  ///Adds or updates the existing configuration using the REST API.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/securityconfig/config`
  ///
  ///```ignore
  /// let response = client.security().update_configuration()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update_configuration(&self) -> builder::UpdateConfiguration {
    builder::UpdateConfiguration::new(self.os_client)
  }

  ///Retrieves the cluster’s security certificates.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/ssl/certs`
  ///
  ///```ignore
  /// let response = client.security().get_certificates()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_certificates(&self) -> builder::GetCertificates {
    builder::GetCertificates::new(self.os_client)
  }

  ///Reload HTTP layer communication certificates.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/ssl/http/reloadcerts`
  ///
  ///```ignore
  /// let response = client.security().reload_http_certificates()
  ///    .send()
  ///    .await;
  /// ```
  pub fn reload_http_certificates(&self) -> builder::ReloadHttpCertificates {
    builder::ReloadHttpCertificates::new(self.os_client)
  }

  ///Reload transport layer communication certificates.
  ///
  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/ssl/transport/reloadcerts`
  ///
  ///```ignore
  /// let response = client.security().reload_transport_certificates()
  ///    .send()
  ///    .await;
  /// ```
  pub fn reload_transport_certificates(&self) -> builder::ReloadTransportCertificates {
    builder::ReloadTransportCertificates::new(self.os_client)
  }

  ///Retrieves all tenants.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/tenants/`
  ///
  ///```ignore
  /// let response = client.security().get_tenants()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_tenants(&self) -> builder::GetTenants {
    builder::GetTenants::new(self.os_client)
  }

  ///Add, delete, or modify multiple tenants in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/tenants/`
  ///
  ///```ignore
  /// let response = client.security().patch_tenants()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_tenants(&self) -> builder::PatchTenants {
    builder::PatchTenants::new(self.os_client)
  }

  ///Retrieves one tenant.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/tenants/{tenant}`
  ///
  ///```ignore
  /// let response = client.security().get_tenant()
  ///    .tenant(tenant)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_tenant(&self) -> builder::GetTenant {
    builder::GetTenant::new(self.os_client)
  }

  ///Creates or replaces the specified tenant.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/tenants/{tenant}`
  ///
  ///```ignore
  /// let response = client.security().create_tenant()
  ///    .tenant(tenant)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_tenant(&self) -> builder::CreateTenant {
    builder::CreateTenant::new(self.os_client)
  }

  ///Delete the specified tenant.
  ///
  ///Sends a `DELETE` request to `/_plugins/_security/api/tenants/{tenant}`
  ///
  ///```ignore
  /// let response = client.security().delete_tenant()
  ///    .tenant(tenant)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_tenant(&self) -> builder::DeleteTenant {
    builder::DeleteTenant::new(self.os_client)
  }

  ///Add, delete, or modify a single tenant.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/tenants/{tenant}`
  ///
  ///```ignore
  /// let response = client.security().patch_tenant()
  ///    .tenant(tenant)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_tenant(&self) -> builder::PatchTenant {
    builder::PatchTenant::new(self.os_client)
  }

  ///Checks to see if the Security plugin is up and running.
  ///
  ///Sends a `GET` request to `/_plugins/_security/health`
  ///
  ///```ignore
  /// let response = client.security().security_health()
  ///    .send()
  ///    .await;
  /// ```
  pub fn security_health(&self) -> builder::SecurityHealth {
    builder::SecurityHealth::new(self.os_client)
  }
}
