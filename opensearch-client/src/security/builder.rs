use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::types::*;
use super::types;
#[allow(unused_imports)]
use crate::{
  encode_path, encode_path_option_vec_string, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
  ReqwestResponse, ResponseValue,
};

///Builder for [`Client::get_account_details`]
///
///[`Client::get_account_details`]: super::OsClient::get_account_details
#[derive(Debug, Clone)]
pub struct GetAccountDetails<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetAccountDetails<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/account`
  pub async fn send(self) -> Result<ResponseValue<types::AccountDetails>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/account", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::change_password`]
///
///[`Client::change_password`]: super::OsClient::change_password
#[derive(Debug, Clone)]
pub struct ChangePassword<'a> {
  client: &'a super::OsClient,
  body: Result<types::builder::ChangePasswordRequestContent, String>,
}

impl<'a> ChangePassword<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Ok(types::builder::ChangePasswordRequestContent::default()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ChangePasswordRequestContent>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `ChangePasswordRequestContent` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::ChangePasswordRequestContent) -> types::builder::ChangePasswordRequestContent,
  {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to `/_plugins/_security/api/account`
  pub async fn send(self) -> Result<ResponseValue<types::ChangePasswordResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body
      .and_then(std::convert::TryInto::<types::ChangePasswordRequestContent>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/account", client.baseurl,);
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_action_groups`]
///
///[`Client::patch_action_groups`]: super::OsClient::patch_action_groups
#[derive(Debug, Clone)]
pub struct PatchActionGroups<'a> {
  client: &'a super::OsClient,
  body: Result<types::PatchActionGroupsInputPayload, String>,
}

impl<'a> PatchActionGroups<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchActionGroupsInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchActionGroupsInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to `/_plugins/_security/api/actiongroups`
  pub async fn send(self) -> Result<ResponseValue<types::PatchActionGroupsResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/actiongroups", client.baseurl,);
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_action_groups`]
///
///[`Client::get_action_groups`]: super::OsClient::get_action_groups
#[derive(Debug, Clone)]
pub struct GetActionGroups<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetActionGroups<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/actiongroups/`
  pub async fn send(self) -> Result<ResponseValue<types::ActionGroupsMap>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/actiongroups/", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_action_group`]
///
///[`Client::get_action_group`]: super::OsClient::get_action_group
#[derive(Debug, Clone)]
pub struct GetActionGroup<'a> {
  client: &'a super::OsClient,
  action_group: Result<String, String>,
}

impl<'a> GetActionGroup<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      action_group: Err("action_group was not initialized".to_string()),
    }
  }

  pub fn action_group<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.action_group = value
      .try_into()
      .map_err(|_| "conversion to `String` for action_group failed".to_string());
    self
  }

  ///Sends a `GET` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  pub async fn send(self) -> Result<ResponseValue<types::ActionGroupsMap>, Error> {
    let Self { client, action_group } = self;
    let action_group = action_group.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/actiongroups/{}",
      client.baseurl,
      encode_path(&action_group.to_string()),
    );
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::create_action_group`]
///
///[`Client::create_action_group`]: super::OsClient::create_action_group
#[derive(Debug, Clone)]
pub struct CreateActionGroup<'a> {
  client: &'a super::OsClient,
  action_group: Result<String, String>,
  body: Result<types::builder::ActionGroup, String>,
}

impl<'a> CreateActionGroup<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      action_group: Err("action_group was not initialized".to_string()),
      body: Ok(types::builder::ActionGroup::default()),
    }
  }

  pub fn action_group<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.action_group = value
      .try_into()
      .map_err(|_| "conversion to `String` for action_group failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ActionGroup>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `ActionGroup` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::ActionGroup) -> types::builder::ActionGroup, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  pub async fn send(self) -> Result<ResponseValue<types::CreateActionGroupResponseContent>, Error> {
    let Self {
      client,
      action_group,
      body,
    } = self;
    let action_group = action_group.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::ActionGroup>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/actiongroups/{}",
      client.baseurl,
      encode_path(&action_group.to_string()),
    );
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::delete_action_group`]
///
///[`Client::delete_action_group`]: super::OsClient::delete_action_group
#[derive(Debug, Clone)]
pub struct DeleteActionGroup<'a> {
  client: &'a super::OsClient,
  action_group: Result<String, String>,
}

impl<'a> DeleteActionGroup<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      action_group: Err("action_group was not initialized".to_string()),
    }
  }

  pub fn action_group<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.action_group = value
      .try_into()
      .map_err(|_| "conversion to `String` for action_group failed".to_string());
    self
  }

  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  pub async fn send(self) -> Result<ResponseValue<types::DeleteActionGroupResponseContent>, Error> {
    let Self { client, action_group } = self;
    let action_group = action_group.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/actiongroups/{}",
      client.baseurl,
      encode_path(&action_group.to_string()),
    );
    let request = client
      .client
      .delete(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_action_group`]
///
///[`Client::patch_action_group`]: super::OsClient::patch_action_group
#[derive(Debug, Clone)]
pub struct PatchActionGroup<'a> {
  client: &'a super::OsClient,
  action_group: Result<String, String>,
  body: Result<types::PatchActionGroupInputPayload, String>,
}

impl<'a> PatchActionGroup<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      action_group: Err("action_group was not initialized".to_string()),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn action_group<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.action_group = value
      .try_into()
      .map_err(|_| "conversion to `String` for action_group failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchActionGroupInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchActionGroupInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  pub async fn send(self) -> Result<ResponseValue<types::PatchActionGroupResponseContent>, Error> {
    let Self {
      client,
      action_group,
      body,
    } = self;
    let action_group = action_group.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/actiongroups/{}",
      client.baseurl,
      encode_path(&action_group.to_string()),
    );
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_audit_configuration`]
///
///[`Client::get_audit_configuration`]: super::OsClient::get_audit_configuration
#[derive(Debug, Clone)]
pub struct GetAuditConfiguration<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetAuditConfiguration<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/audit`
  pub async fn send(self) -> Result<ResponseValue<types::AuditConfigWithReadOnly>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/audit", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_audit_configuration`]
///
///[`Client::patch_audit_configuration`]: super::OsClient::patch_audit_configuration
#[derive(Debug, Clone)]
pub struct PatchAuditConfiguration<'a> {
  client: &'a super::OsClient,
  body: Result<types::PatchAuditConfigurationInputPayload, String>,
}

impl<'a> PatchAuditConfiguration<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchAuditConfigurationInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchAuditConfigurationInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to `/_plugins/_security/api/audit`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/audit", client.baseurl,);
    let request = client.client.patch(url).json(&body).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => Ok(ResponseValue::empty(response)),
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::update_audit_configuration`]
///
///[`Client::update_audit_configuration`]: super::OsClient::update_audit_configuration
#[derive(Debug, Clone)]
pub struct UpdateAuditConfiguration<'a> {
  client: &'a super::OsClient,
  body: Result<types::builder::AuditConfig, String>,
}

impl<'a> UpdateAuditConfiguration<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Ok(types::builder::AuditConfig::default()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::AuditConfig>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `AuditConfig` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::AuditConfig) -> types::builder::AuditConfig, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to `/_plugins/_security/api/audit/config`
  pub async fn send(self) -> Result<ResponseValue<types::UpdateAuditConfigurationResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body
      .and_then(std::convert::TryInto::<types::AuditConfig>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/audit/config", client.baseurl,);
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::flush_cache`]
///
///[`Client::flush_cache`]: super::OsClient::flush_cache
#[derive(Debug, Clone)]
pub struct FlushCache<'a> {
  client: &'a super::OsClient,
}

impl<'a> FlushCache<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `DELETE` request to `/_plugins/_security/api/cache`
  pub async fn send(self) -> Result<ResponseValue<types::FlushCacheResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/cache", client.baseurl,);
    let request = client
      .client
      .delete(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_users`]
///
///[`Client::get_users`]: super::OsClient::get_users
#[derive(Debug, Clone)]
pub struct GetUsers<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetUsers<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/internalusers`
  pub async fn send(self) -> Result<ResponseValue<types::UsersMap>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/internalusers", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_users`]
///
///[`Client::patch_users`]: super::OsClient::patch_users
#[derive(Debug, Clone)]
pub struct PatchUsers<'a> {
  client: &'a super::OsClient,
  body: Result<types::PatchUsersInputPayload, String>,
}

impl<'a> PatchUsers<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchUsersInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchUsersInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to `/_plugins/_security/api/internalusers`
  pub async fn send(self) -> Result<ResponseValue<types::PatchUsersResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/internalusers", client.baseurl,);
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_user`]
///
///[`Client::get_user`]: super::OsClient::get_user
#[derive(Debug, Clone)]
pub struct GetUser<'a> {
  client: &'a super::OsClient,
  username: Result<String, String>,
}

impl<'a> GetUser<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      username: Err("username was not initialized".to_string()),
    }
  }

  pub fn username<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.username = value
      .try_into()
      .map_err(|_| "conversion to `String` for username failed".to_string());
    self
  }

  ///Sends a `GET` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  pub async fn send(self) -> Result<ResponseValue<types::UsersMap>, Error> {
    let Self { client, username } = self;
    let username = username.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/internalusers/{}",
      client.baseurl,
      encode_path(&username.to_string()),
    );
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::create_user`]
///
///[`Client::create_user`]: super::OsClient::create_user
#[derive(Debug, Clone)]
pub struct CreateUser<'a> {
  client: &'a super::OsClient,
  username: Result<String, String>,
  body: Result<types::builder::User, String>,
}

impl<'a> CreateUser<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      username: Err("username was not initialized".to_string()),
      body: Ok(types::builder::User::default()),
    }
  }

  pub fn username<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.username = value
      .try_into()
      .map_err(|_| "conversion to `String` for username failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::User>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `User` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::User) -> types::builder::User, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  pub async fn send(self) -> Result<ResponseValue<types::CreateUserResponseContent>, Error> {
    let Self { client, username, body } = self;
    let username = username.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::User>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/internalusers/{}",
      client.baseurl,
      encode_path(&username.to_string()),
    );
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::delete_user`]
///
///[`Client::delete_user`]: super::OsClient::delete_user
#[derive(Debug, Clone)]
pub struct DeleteUser<'a> {
  client: &'a super::OsClient,
  username: Result<String, String>,
}

impl<'a> DeleteUser<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      username: Err("username was not initialized".to_string()),
    }
  }

  pub fn username<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.username = value
      .try_into()
      .map_err(|_| "conversion to `String` for username failed".to_string());
    self
  }

  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  pub async fn send(self) -> Result<ResponseValue<types::DeleteUserResponseContent>, Error> {
    let Self { client, username } = self;
    let username = username.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/internalusers/{}",
      client.baseurl,
      encode_path(&username.to_string()),
    );
    let request = client
      .client
      .delete(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_user`]
///
///[`Client::patch_user`]: super::OsClient::patch_user
#[derive(Debug, Clone)]
pub struct PatchUser<'a> {
  client: &'a super::OsClient,
  username: Result<String, String>,
  body: Result<types::PatchUserInputPayload, String>,
}

impl<'a> PatchUser<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      username: Err("username was not initialized".to_string()),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn username<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.username = value
      .try_into()
      .map_err(|_| "conversion to `String` for username failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchUserInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchUserInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  pub async fn send(self) -> Result<ResponseValue<types::PatchUserResponseContent>, Error> {
    let Self { client, username, body } = self;
    let username = username.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/internalusers/{}",
      client.baseurl,
      encode_path(&username.to_string()),
    );
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_distinguished_names`]
///
///[`Client::get_distinguished_names`]: super::OsClient::get_distinguished_names
#[derive(Debug, Clone)]
pub struct GetDistinguishedNames<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetDistinguishedNames<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/nodesdn`
  pub async fn send(self) -> Result<ResponseValue<types::DistinguishedNamesMap>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/nodesdn", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_distinguished_names`]
///
///[`Client::patch_distinguished_names`]: super::OsClient::patch_distinguished_names
#[derive(Debug, Clone)]
pub struct PatchDistinguishedNames<'a> {
  client: &'a super::OsClient,
  body: Result<types::PatchDistinguishedNamesInputPayload, String>,
}

impl<'a> PatchDistinguishedNames<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchDistinguishedNamesInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchDistinguishedNamesInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to `/_plugins/_security/api/nodesdn`
  pub async fn send(self) -> Result<ResponseValue<types::PatchDistinguishedNamesResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/nodesdn", client.baseurl,);
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_distinguished_names_with_cluster_name`]
///
///[`Client::get_distinguished_names_with_cluster_name`]: super::OsClient::get_distinguished_names_with_cluster_name
#[derive(Debug, Clone)]
pub struct GetDistinguishedNamesWithClusterName<'a> {
  client: &'a super::OsClient,
  cluster_name: Result<String, String>,
}

impl<'a> GetDistinguishedNamesWithClusterName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_name: Err("cluster_name was not initialized".to_string()),
    }
  }

  pub fn cluster_name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.cluster_name = value
      .try_into()
      .map_err(|_| "conversion to `String` for cluster_name failed".to_string());
    self
  }

  ///Sends a `GET` request to
  /// `/_plugins/_security/api/nodesdn/{cluster_name}`
  pub async fn send(self) -> Result<ResponseValue<types::DistinguishedNamesMap>, Error> {
    let Self { client, cluster_name } = self;
    let cluster_name = cluster_name.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/nodesdn/{}",
      client.baseurl,
      encode_path(&cluster_name.to_string()),
    );
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::update_distinguished_names`]
///
///[`Client::update_distinguished_names`]: super::OsClient::update_distinguished_names
#[derive(Debug, Clone)]
pub struct UpdateDistinguishedNames<'a> {
  client: &'a super::OsClient,
  cluster_name: Result<String, String>,
  body: Result<types::builder::DistinguishedNames, String>,
}

impl<'a> UpdateDistinguishedNames<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_name: Err("cluster_name was not initialized".to_string()),
      body: Ok(types::builder::DistinguishedNames::default()),
    }
  }

  pub fn cluster_name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.cluster_name = value
      .try_into()
      .map_err(|_| "conversion to `String` for cluster_name failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::DistinguishedNames>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `DistinguishedNames` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::DistinguishedNames) -> types::builder::DistinguishedNames, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/nodesdn/{cluster_name}`
  pub async fn send(self) -> Result<ResponseValue<types::UpdateDistinguishedNamesResponseContent>, Error> {
    let Self {
      client,
      cluster_name,
      body,
    } = self;
    let cluster_name = cluster_name.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::DistinguishedNames>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/nodesdn/{}",
      client.baseurl,
      encode_path(&cluster_name.to_string()),
    );
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::delete_distinguished_names`]
///
///[`Client::delete_distinguished_names`]: super::OsClient::delete_distinguished_names
#[derive(Debug, Clone)]
pub struct DeleteDistinguishedNames<'a> {
  client: &'a super::OsClient,
  cluster_name: Result<String, String>,
}

impl<'a> DeleteDistinguishedNames<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_name: Err("cluster_name was not initialized".to_string()),
    }
  }

  pub fn cluster_name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.cluster_name = value
      .try_into()
      .map_err(|_| "conversion to `String` for cluster_name failed".to_string());
    self
  }

  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/nodesdn/{cluster_name}`
  pub async fn send(self) -> Result<ResponseValue<types::DeleteDistinguishedNamesResponseContent>, Error> {
    let Self { client, cluster_name } = self;
    let cluster_name = cluster_name.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/nodesdn/{}",
      client.baseurl,
      encode_path(&cluster_name.to_string()),
    );
    let request = client
      .client
      .delete(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_roles`]
///
///[`Client::patch_roles`]: super::OsClient::patch_roles
#[derive(Debug, Clone)]
pub struct PatchRoles<'a> {
  client: &'a super::OsClient,
  body: Result<types::PatchRolesInputPayload, String>,
}

impl<'a> PatchRoles<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchRolesInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchRolesInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to `/_plugins/_security/api/roles`
  pub async fn send(self) -> Result<ResponseValue<types::PatchRolesResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/roles", client.baseurl,);
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_roles`]
///
///[`Client::get_roles`]: super::OsClient::get_roles
#[derive(Debug, Clone)]
pub struct GetRoles<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetRoles<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/roles/`
  pub async fn send(self) -> Result<ResponseValue<types::RolesMap>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/roles/", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_role`]
///
///[`Client::get_role`]: super::OsClient::get_role
#[derive(Debug, Clone)]
pub struct GetRole<'a> {
  client: &'a super::OsClient,
  role: Result<String, String>,
}

impl<'a> GetRole<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      role: Err("role was not initialized".to_string()),
    }
  }

  pub fn role<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.role = value
      .try_into()
      .map_err(|_| "conversion to `String` for role failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_plugins/_security/api/roles/{role}`
  pub async fn send(self) -> Result<ResponseValue<types::RolesMap>, Error> {
    let Self { client, role } = self;
    let role = role.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/roles/{}",
      client.baseurl,
      encode_path(&role.to_string()),
    );
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::create_role`]
///
///[`Client::create_role`]: super::OsClient::create_role
#[derive(Debug, Clone)]
pub struct CreateRole<'a> {
  client: &'a super::OsClient,
  role: Result<String, String>,
  body: Result<types::builder::Role, String>,
}

impl<'a> CreateRole<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      role: Err("role was not initialized".to_string()),
      body: Ok(types::builder::Role::default()),
    }
  }

  pub fn role<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.role = value
      .try_into()
      .map_err(|_| "conversion to `String` for role failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Role>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `Role` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::Role) -> types::builder::Role, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to `/_plugins/_security/api/roles/{role}`
  pub async fn send(self) -> Result<ResponseValue<types::CreateRoleResponseContent>, Error> {
    let Self { client, role, body } = self;
    let role = role.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::Role>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/roles/{}",
      client.baseurl,
      encode_path(&role.to_string()),
    );
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::delete_role`]
///
///[`Client::delete_role`]: super::OsClient::delete_role
#[derive(Debug, Clone)]
pub struct DeleteRole<'a> {
  client: &'a super::OsClient,
  role: Result<String, String>,
}

impl<'a> DeleteRole<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      role: Err("role was not initialized".to_string()),
    }
  }

  pub fn role<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.role = value
      .try_into()
      .map_err(|_| "conversion to `String` for role failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_plugins/_security/api/roles/{role}`
  pub async fn send(self) -> Result<ResponseValue<types::DeleteRoleResponseContent>, Error> {
    let Self { client, role } = self;
    let role = role.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/roles/{}",
      client.baseurl,
      encode_path(&role.to_string()),
    );
    let request = client
      .client
      .delete(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_role`]
///
///[`Client::patch_role`]: super::OsClient::patch_role
#[derive(Debug, Clone)]
pub struct PatchRole<'a> {
  client: &'a super::OsClient,
  role: Result<String, String>,
  body: Result<types::PatchRoleInputPayload, String>,
}

impl<'a> PatchRole<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      role: Err("role was not initialized".to_string()),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn role<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.role = value
      .try_into()
      .map_err(|_| "conversion to `String` for role failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchRoleInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchRoleInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to `/_plugins/_security/api/roles/{role}`
  pub async fn send(self) -> Result<ResponseValue<types::PatchRoleResponseContent>, Error> {
    let Self { client, role, body } = self;
    let role = role.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/roles/{}",
      client.baseurl,
      encode_path(&role.to_string()),
    );
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_role_mappings`]
///
///[`Client::get_role_mappings`]: super::OsClient::get_role_mappings
#[derive(Debug, Clone)]
pub struct GetRoleMappings<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetRoleMappings<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/rolesmapping`
  pub async fn send(self) -> Result<ResponseValue<types::RoleMappings>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/rolesmapping", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_role_mappings`]
///
///[`Client::patch_role_mappings`]: super::OsClient::patch_role_mappings
#[derive(Debug, Clone)]
pub struct PatchRoleMappings<'a> {
  client: &'a super::OsClient,
  body: Result<types::PatchRoleMappingsInputPayload, String>,
}

impl<'a> PatchRoleMappings<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchRoleMappingsInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchRoleMappingsInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to `/_plugins/_security/api/rolesmapping`
  pub async fn send(self) -> Result<ResponseValue<types::PatchRoleMappingsResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/rolesmapping", client.baseurl,);
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_role_mapping`]
///
///[`Client::get_role_mapping`]: super::OsClient::get_role_mapping
#[derive(Debug, Clone)]
pub struct GetRoleMapping<'a> {
  client: &'a super::OsClient,
  role: Result<String, String>,
}

impl<'a> GetRoleMapping<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      role: Err("role was not initialized".to_string()),
    }
  }

  pub fn role<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.role = value
      .try_into()
      .map_err(|_| "conversion to `String` for role failed".to_string());
    self
  }

  ///Sends a `GET` request to
  /// `/_plugins/_security/api/rolesmapping/{role}`
  pub async fn send(self) -> Result<ResponseValue<types::RoleMappings>, Error> {
    let Self { client, role } = self;
    let role = role.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/rolesmapping/{}",
      client.baseurl,
      encode_path(&role.to_string()),
    );
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::create_role_mapping`]
///
///[`Client::create_role_mapping`]: super::OsClient::create_role_mapping
#[derive(Debug, Clone)]
pub struct CreateRoleMapping<'a> {
  client: &'a super::OsClient,
  role: Result<String, String>,
  body: Result<types::builder::RoleMapping, String>,
}

impl<'a> CreateRoleMapping<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      role: Err("role was not initialized".to_string()),
      body: Ok(types::builder::RoleMapping::default()),
    }
  }

  pub fn role<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.role = value
      .try_into()
      .map_err(|_| "conversion to `String` for role failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RoleMapping>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `RoleMapping` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::RoleMapping) -> types::builder::RoleMapping, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/rolesmapping/{role}`
  pub async fn send(self) -> Result<ResponseValue<types::CreateRoleMappingResponseContent>, Error> {
    let Self { client, role, body } = self;
    let role = role.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::RoleMapping>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/rolesmapping/{}",
      client.baseurl,
      encode_path(&role.to_string()),
    );
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::delete_role_mapping`]
///
///[`Client::delete_role_mapping`]: super::OsClient::delete_role_mapping
#[derive(Debug, Clone)]
pub struct DeleteRoleMapping<'a> {
  client: &'a super::OsClient,
  role: Result<String, String>,
}

impl<'a> DeleteRoleMapping<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      role: Err("role was not initialized".to_string()),
    }
  }

  pub fn role<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.role = value
      .try_into()
      .map_err(|_| "conversion to `String` for role failed".to_string());
    self
  }

  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/rolesmapping/{role}`
  pub async fn send(self) -> Result<ResponseValue<types::DeleteRoleMappingResponseContent>, Error> {
    let Self { client, role } = self;
    let role = role.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/rolesmapping/{}",
      client.baseurl,
      encode_path(&role.to_string()),
    );
    let request = client
      .client
      .delete(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_role_mapping`]
///
///[`Client::patch_role_mapping`]: super::OsClient::patch_role_mapping
#[derive(Debug, Clone)]
pub struct PatchRoleMapping<'a> {
  client: &'a super::OsClient,
  role: Result<String, String>,
  body: Result<types::PatchRoleMappingInputPayload, String>,
}

impl<'a> PatchRoleMapping<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      role: Err("role was not initialized".to_string()),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn role<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.role = value
      .try_into()
      .map_err(|_| "conversion to `String` for role failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchRoleMappingInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchRoleMappingInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to
  /// `/_plugins/_security/api/rolesmapping/{role}`
  pub async fn send(self) -> Result<ResponseValue<types::PatchRoleMappingResponseContent>, Error> {
    let Self { client, role, body } = self;
    let role = role.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/rolesmapping/{}",
      client.baseurl,
      encode_path(&role.to_string()),
    );
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_configuration`]
///
///[`Client::get_configuration`]: super::OsClient::get_configuration
#[derive(Debug, Clone)]
pub struct GetConfiguration<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetConfiguration<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/securityconfig`
  pub async fn send(self) -> Result<ResponseValue<types::DynamicConfig>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/securityconfig", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_configuration`]
///
///[`Client::patch_configuration`]: super::OsClient::patch_configuration
#[derive(Debug, Clone)]
pub struct PatchConfiguration<'a> {
  client: &'a super::OsClient,
  body: Result<types::PatchConfigurationInputPayload, String>,
}

impl<'a> PatchConfiguration<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchConfigurationInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchConfigurationInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to `/_plugins/_security/api/securityconfig`
  pub async fn send(self) -> Result<ResponseValue<types::PatchConfigurationResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/securityconfig", client.baseurl,);
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::update_configuration`]
///
///[`Client::update_configuration`]: super::OsClient::update_configuration
#[derive(Debug, Clone)]
pub struct UpdateConfiguration<'a> {
  client: &'a super::OsClient,
  body: Result<types::builder::DynamicConfig, String>,
}

impl<'a> UpdateConfiguration<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Ok(types::builder::DynamicConfig::default()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::DynamicConfig>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `DynamicConfig` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::DynamicConfig) -> types::builder::DynamicConfig, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/securityconfig/config`
  pub async fn send(self) -> Result<ResponseValue<types::UpdateConfigurationResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body
      .and_then(std::convert::TryInto::<types::DynamicConfig>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/securityconfig/config", client.baseurl,);
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_certificates`]
///
///[`Client::get_certificates`]: super::OsClient::get_certificates
#[derive(Debug, Clone)]
pub struct GetCertificates<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetCertificates<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/ssl/certs`
  pub async fn send(self) -> Result<ResponseValue<types::GetCertificatesResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/ssl/certs", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::reload_http_certificates`]
///
///[`Client::reload_http_certificates`]: super::OsClient::reload_http_certificates
#[derive(Debug, Clone)]
pub struct ReloadHttpCertificates<'a> {
  client: &'a super::OsClient,
}

impl<'a> ReloadHttpCertificates<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/ssl/http/reloadcerts`
  pub async fn send(self) -> Result<ResponseValue<types::ReloadHttpCertificatesResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/ssl/http/reloadcerts", client.baseurl,);
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::reload_transport_certificates`]
///
///[`Client::reload_transport_certificates`]: super::OsClient::reload_transport_certificates
#[derive(Debug, Clone)]
pub struct ReloadTransportCertificates<'a> {
  client: &'a super::OsClient,
}

impl<'a> ReloadTransportCertificates<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/ssl/transport/reloadcerts`
  pub async fn send(self) -> Result<ResponseValue<types::ReloadTransportCertificatesResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/ssl/transport/reloadcerts", client.baseurl,);
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_tenants`]
///
///[`Client::get_tenants`]: super::OsClient::get_tenants
#[derive(Debug, Clone)]
pub struct GetTenants<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetTenants<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/api/tenants/`
  pub async fn send(self) -> Result<ResponseValue<types::TenantsMap>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/api/tenants/", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_tenants`]
///
///[`Client::patch_tenants`]: super::OsClient::patch_tenants
#[derive(Debug, Clone)]
pub struct PatchTenants<'a> {
  client: &'a super::OsClient,
  body: Result<types::PatchTenantsInputPayload, String>,
}

impl<'a> PatchTenants<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchTenantsInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchTenantsInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to `/_plugins/_security/api/tenants/`
  pub async fn send(self) -> Result<ResponseValue<types::PatchTenantsResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_plugins/_security/api/tenants/", client.baseurl,);
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::get_tenant`]
///
///[`Client::get_tenant`]: super::OsClient::get_tenant
#[derive(Debug, Clone)]
pub struct GetTenant<'a> {
  client: &'a super::OsClient,
  tenant: Result<String, String>,
}

impl<'a> GetTenant<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      tenant: Err("tenant was not initialized".to_string()),
    }
  }

  pub fn tenant<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.tenant = value
      .try_into()
      .map_err(|_| "conversion to `String` for tenant failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_plugins/_security/api/tenants/{tenant}`
  pub async fn send(self) -> Result<ResponseValue<types::TenantsMap>, Error> {
    let Self { client, tenant } = self;
    let tenant = tenant.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/tenants/{}",
      client.baseurl,
      encode_path(&tenant.to_string()),
    );
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::create_tenant`]
///
///[`Client::create_tenant`]: super::OsClient::create_tenant
#[derive(Debug, Clone)]
pub struct CreateTenant<'a> {
  client: &'a super::OsClient,
  tenant: Result<String, String>,
  body: Result<types::builder::CreateTenantParams, String>,
}

impl<'a> CreateTenant<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      tenant: Err("tenant was not initialized".to_string()),
      body: Ok(types::builder::CreateTenantParams::default()),
    }
  }

  pub fn tenant<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.tenant = value
      .try_into()
      .map_err(|_| "conversion to `String` for tenant failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::CreateTenantParams>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `CreateTenantParams` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::CreateTenantParams) -> types::builder::CreateTenantParams, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to `/_plugins/_security/api/tenants/{tenant}`
  pub async fn send(self) -> Result<ResponseValue<types::CreateTenantResponseContent>, Error> {
    let Self { client, tenant, body } = self;
    let tenant = tenant.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::CreateTenantParams>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/tenants/{}",
      client.baseurl,
      encode_path(&tenant.to_string()),
    );
    let request = client
      .client
      .put(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::delete_tenant`]
///
///[`Client::delete_tenant`]: super::OsClient::delete_tenant
#[derive(Debug, Clone)]
pub struct DeleteTenant<'a> {
  client: &'a super::OsClient,
  tenant: Result<String, String>,
}

impl<'a> DeleteTenant<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      tenant: Err("tenant was not initialized".to_string()),
    }
  }

  pub fn tenant<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.tenant = value
      .try_into()
      .map_err(|_| "conversion to `String` for tenant failed".to_string());
    self
  }

  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/tenants/{tenant}`
  pub async fn send(self) -> Result<ResponseValue<types::DeleteTenantResponseContent>, Error> {
    let Self { client, tenant } = self;
    let tenant = tenant.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/tenants/{}",
      client.baseurl,
      encode_path(&tenant.to_string()),
    );
    let request = client
      .client
      .delete(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::patch_tenant`]
///
///[`Client::patch_tenant`]: super::OsClient::patch_tenant
#[derive(Debug, Clone)]
pub struct PatchTenant<'a> {
  client: &'a super::OsClient,
  tenant: Result<String, String>,
  body: Result<types::PatchTenantInputPayload, String>,
}

impl<'a> PatchTenant<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      tenant: Err("tenant was not initialized".to_string()),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn tenant<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.tenant = value
      .try_into()
      .map_err(|_| "conversion to `String` for tenant failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PatchTenantInputPayload>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PatchTenantInputPayload` for body failed".to_string());
    self
  }

  ///Sends a `PATCH` request to
  /// `/_plugins/_security/api/tenants/{tenant}`
  pub async fn send(self) -> Result<ResponseValue<types::PatchTenantResponseContent>, Error> {
    let Self { client, tenant, body } = self;
    let tenant = tenant.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_plugins/_security/api/tenants/{}",
      client.baseurl,
      encode_path(&tenant.to_string()),
    );
    let request = client
      .client
      .patch(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::security_health`]
///
///[`Client::security_health`]: super::OsClient::security_health
#[derive(Debug, Clone)]
pub struct SecurityHealth<'a> {
  client: &'a super::OsClient,
}

impl<'a> SecurityHealth<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_plugins/_security/health`
  pub async fn send(self) -> Result<ResponseValue<types::SecurityHealthResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}/_plugins/_security/health", client.baseurl,);
    let request = client
      .client
      .get(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
