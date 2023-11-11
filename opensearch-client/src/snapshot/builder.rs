use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::types::{OpenSearchId, OpenSearchNameValue, Timeout};
use super::types;
#[allow(unused_imports)]
use crate::{
  encode_path, encode_path_option_vec_string, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
  ReqwestResponse, ResponseValue,
};

///Builder for [`Client::Snapshot::get_repository`]
///
///[`Client::Snapshot::get_repository`]: super::OsClient::Snapshot::get_repository
#[derive(Debug, Clone)]
pub struct SnapshotGetRepository<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> SnapshotGetRepository<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn local<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.local = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for local failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_snapshot`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      local,
      master_timeout,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_snapshot", client.baseurl,);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
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
///Builder for [`Client::Snapshot::status`]
///
///[`Client::Snapshot::status`]: super::OsClient::Snapshot::status
#[derive(Debug, Clone)]
pub struct SnapshotStatus<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> SnapshotStatus<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_snapshot/_status`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      ignore_unavailable,
      master_timeout,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_snapshot/_status", client.baseurl,);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
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
///Builder for [`Client::Snapshot::get_repository_with_repository`]
///
///[`Client::Snapshot::get_repository_with_repository`]: super::OsClient::Snapshot::get_repository_with_repository
#[derive(Debug, Clone)]
pub struct SnapshotGetRepositoryWithRepository<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> SnapshotGetRepositoryWithRepository<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn local<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.local = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for local failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_snapshot/{repository}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      cluster_manager_timeout,
      local,
      master_timeout,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_snapshot/{}", client.baseurl, encode_path(&repository.to_string()),);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
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
///Builder for [`Client::Snapshot::create_repository_put`]
///
///[`Client::Snapshot::create_repository_put`]: super::OsClient::Snapshot::create_repository_put
#[derive(Debug, Clone)]
pub struct SnapshotCreateRepositoryPut<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  verify: Result<Option<bool>, String>,
  body: Result<types::SnapshotCreateRepositoryBodyParams, String>,
}
impl<'a> SnapshotCreateRepositoryPut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      verify: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn verify<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.verify = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for verify failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SnapshotCreateRepositoryBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `SnapshotCreateRepositoryBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_snapshot/{repository}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      verify,
      body,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let verify = verify.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_snapshot/{}", client.baseurl, encode_path(&repository.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &verify {
      query.push(("verify", v.to_string()));
    }
    let request = client.client.put(url).json(&body).query(&query).build()?;
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
///Builder for [`Client::Snapshot::create_repository_post`]
///
///[`Client::Snapshot::create_repository_post`]: super::OsClient::Snapshot::create_repository_post
#[derive(Debug, Clone)]
pub struct SnapshotCreateRepositoryPost<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  verify: Result<Option<bool>, String>,
  body: Result<types::SnapshotCreateRepositoryBodyParams, String>,
}
impl<'a> SnapshotCreateRepositoryPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      verify: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn verify<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.verify = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for verify failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SnapshotCreateRepositoryBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `SnapshotCreateRepositoryBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_snapshot/{repository}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      verify,
      body,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let verify = verify.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_snapshot/{}", client.baseurl, encode_path(&repository.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &verify {
      query.push(("verify", v.to_string()));
    }
    let request = client.client.post(url).json(&body).query(&query).build()?;
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
///Builder for [`Client::Snapshot::delete_repository`]
///
///[`Client::Snapshot::delete_repository`]: super::OsClient::Snapshot::delete_repository
#[derive(Debug, Clone)]
pub struct SnapshotDeleteRepository<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> SnapshotDeleteRepository<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_snapshot/{repository}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_snapshot/{}", client.baseurl, encode_path(&repository.to_string()),);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    let request = client.client.delete(url).query(&query).build()?;
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
///Builder for [`Client::Snapshot::cleanup_repository`]
///
///[`Client::Snapshot::cleanup_repository`]: super::OsClient::Snapshot::cleanup_repository
#[derive(Debug, Clone)]
pub struct SnapshotCleanupRepository<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> SnapshotCleanupRepository<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_snapshot/{repository}/_cleanup`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/_cleanup",
      client.baseurl,
      encode_path(&repository.to_string()),
    );
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    let request = client.client.post(url).query(&query).build()?;
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
///Builder for [`Client::Snapshot::status_with_repository`]
///
///[`Client::Snapshot::status_with_repository`]: super::OsClient::Snapshot::status_with_repository
#[derive(Debug, Clone)]
pub struct SnapshotStatusWithRepository<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> SnapshotStatusWithRepository<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_snapshot/{repository}/_status`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      cluster_manager_timeout,
      ignore_unavailable,
      master_timeout,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/_status",
      client.baseurl,
      encode_path(&repository.to_string()),
    );
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
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
///Builder for [`Client::Snapshot::verify_repository`]
///
///[`Client::Snapshot::verify_repository`]: super::OsClient::Snapshot::verify_repository
#[derive(Debug, Clone)]
pub struct SnapshotVerifyRepository<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> SnapshotVerifyRepository<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_snapshot/{repository}/_verify`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/_verify",
      client.baseurl,
      encode_path(&repository.to_string()),
    );
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    let request = client.client.post(url).query(&query).build()?;
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
///Builder for [`Client::Snapshot::get`]
///
///[`Client::Snapshot::get`]: super::OsClient::Snapshot::get
#[derive(Debug, Clone)]
pub struct SnapshotGet<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  snapshot: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  verbose: Result<Option<bool>, String>,
}
impl<'a> SnapshotGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      snapshot: Err("snapshot was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      verbose: Ok(None),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn snapshot<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.snapshot = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for snapshot failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn verbose<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.verbose = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for verbose failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_snapshot/{repository}/{snapshot}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      snapshot,
      cluster_manager_timeout,
      ignore_unavailable,
      master_timeout,
      verbose,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let snapshot = snapshot.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let verbose = verbose.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/{}",
      client.baseurl,
      encode_path(&repository.to_string()),
      encode_path(&snapshot.to_string()),
    );
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &verbose {
      query.push(("verbose", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
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
///Builder for [`Client::Snapshot::create_put`]
///
///[`Client::Snapshot::create_put`]: super::OsClient::Snapshot::create_put
#[derive(Debug, Clone)]
pub struct SnapshotCreatePut<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  snapshot: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  wait_for_completion: Result<Option<bool>, String>,
  body: Result<types::SnapshotCreateBodyParams, String>,
}
impl<'a> SnapshotCreatePut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      snapshot: Err("snapshot was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      wait_for_completion: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn snapshot<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.snapshot = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for snapshot failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn wait_for_completion<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_completion = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_completion failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SnapshotCreateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `SnapshotCreateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_snapshot/{repository}/{snapshot}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      snapshot,
      cluster_manager_timeout,
      master_timeout,
      wait_for_completion,
      body,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let snapshot = snapshot.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/{}",
      client.baseurl,
      encode_path(&repository.to_string()),
      encode_path(&snapshot.to_string()),
    );
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_completion {
      query.push(("wait_for_completion", v.to_string()));
    }
    let request = client.client.put(url).json(&body).query(&query).build()?;
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
///Builder for [`Client::Snapshot::create_post`]
///
///[`Client::Snapshot::create_post`]: super::OsClient::Snapshot::create_post
#[derive(Debug, Clone)]
pub struct SnapshotCreatePost<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  snapshot: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  wait_for_completion: Result<Option<bool>, String>,
  body: Result<types::SnapshotCreateBodyParams, String>,
}
impl<'a> SnapshotCreatePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      snapshot: Err("snapshot was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      wait_for_completion: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn snapshot<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.snapshot = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for snapshot failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn wait_for_completion<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_completion = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_completion failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SnapshotCreateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `SnapshotCreateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_snapshot/{repository}/{snapshot}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      snapshot,
      cluster_manager_timeout,
      master_timeout,
      wait_for_completion,
      body,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let snapshot = snapshot.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/{}",
      client.baseurl,
      encode_path(&repository.to_string()),
      encode_path(&snapshot.to_string()),
    );
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_completion {
      query.push(("wait_for_completion", v.to_string()));
    }
    let request = client.client.post(url).json(&body).query(&query).build()?;
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
///Builder for [`Client::Snapshot::delete`]
///
///[`Client::Snapshot::delete`]: super::OsClient::Snapshot::delete
#[derive(Debug, Clone)]
pub struct SnapshotDelete<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  snapshot: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> SnapshotDelete<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      snapshot: Err("snapshot was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn snapshot<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.snapshot = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for snapshot failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_snapshot/{repository}/{snapshot}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      snapshot,
      cluster_manager_timeout,
      master_timeout,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let snapshot = snapshot.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/{}",
      client.baseurl,
      encode_path(&repository.to_string()),
      encode_path(&snapshot.to_string()),
    );
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    let request = client.client.delete(url).query(&query).build()?;
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
///Builder for [`Client::Snapshot::clone`]
///
///[`Client::Snapshot::clone`]: super::OsClient::Snapshot::clone
#[derive(Debug, Clone)]
pub struct SnapshotClone<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  snapshot: Result<OpenSearchNameValue, String>,
  target_snapshot: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  body: Result<types::SnapshotCloneBodyParams, String>,
}
impl<'a> SnapshotClone<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      snapshot: Err("snapshot was not initialized".to_string()),
      target_snapshot: Err("target_snapshot was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn snapshot<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.snapshot = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for snapshot failed".to_string());
    self
  }

  pub fn target_snapshot<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.target_snapshot = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for target_snapshot failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SnapshotCloneBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `SnapshotCloneBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to
  /// `/_snapshot/{repository}/{snapshot}/_clone/{target_snapshot}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      snapshot,
      target_snapshot,
      cluster_manager_timeout,
      master_timeout,
      body,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let snapshot = snapshot.map_err(Error::InvalidRequest)?;
    let target_snapshot = target_snapshot.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/{}/_clone/{}",
      client.baseurl,
      encode_path(&repository.to_string()),
      encode_path(&snapshot.to_string()),
      encode_path(&target_snapshot.to_string()),
    );
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    let request = client.client.put(url).json(&body).query(&query).build()?;
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
///Builder for [`Client::Snapshot::restore`]
///
///[`Client::Snapshot::restore`]: super::OsClient::Snapshot::restore
#[derive(Debug, Clone)]
pub struct SnapshotRestore<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  snapshot: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  wait_for_completion: Result<Option<bool>, String>,
  body: Result<types::SnapshotRestoreBodyParams, String>,
}
impl<'a> SnapshotRestore<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      snapshot: Err("snapshot was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      wait_for_completion: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn snapshot<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.snapshot = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for snapshot failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn wait_for_completion<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_completion = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_completion failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SnapshotRestoreBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `SnapshotRestoreBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to
  /// `/_snapshot/{repository}/{snapshot}/_restore`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      snapshot,
      cluster_manager_timeout,
      master_timeout,
      wait_for_completion,
      body,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let snapshot = snapshot.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/{}/_restore",
      client.baseurl,
      encode_path(&repository.to_string()),
      encode_path(&snapshot.to_string()),
    );
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_completion {
      query.push(("wait_for_completion", v.to_string()));
    }
    let request = client.client.post(url).json(&body).query(&query).build()?;
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
///Builder for [`Client::Snapshot::status_with_repository_snapshot`]
///
///[`Client::Snapshot::status_with_repository_snapshot`]: super::OsClient::Snapshot::status_with_repository_snapshot
#[derive(Debug, Clone)]
pub struct SnapshotStatusWithRepositorySnapshot<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  snapshot: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> SnapshotStatusWithRepositorySnapshot<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      snapshot: Err("snapshot was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn repository<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.repository = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for repository failed".to_string());
    self
  }

  pub fn snapshot<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.snapshot = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for snapshot failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to
  /// `/_snapshot/{repository}/{snapshot}/_status`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      repository,
      snapshot,
      cluster_manager_timeout,
      ignore_unavailable,
      master_timeout,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let snapshot = snapshot.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_snapshot/{}/{}/_status",
      client.baseurl,
      encode_path(&repository.to_string()),
      encode_path(&snapshot.to_string()),
    );
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
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
