use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::types::Timeout;
use super::types;
#[allow(unused_imports)]
use crate::{
  encode_path, encode_path_option_vec_string, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
  ReqwestResponse, ResponseValue,
};

///Builder for [`Client::Remote::store_restore`]
///
///[`Client::Remote::store_restore`]: super::OsClient::Remote::store_restore
#[derive(Debug, Clone)]
pub struct RemoteStoreRestore<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  wait_for_completion: Result<Option<bool>, String>,
  body: Result<types::builder::RemoteStoreRestoreBodyParams, String>,
}
impl<'a> RemoteStoreRestore<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      wait_for_completion: Ok(None),
      body: Ok(types::builder::RemoteStoreRestoreBodyParams::default()),
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
    V: std::convert::TryInto<types::RemoteStoreRestoreBodyParams>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `RemoteStoreRestoreBodyParams` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::RemoteStoreRestoreBodyParams) -> types::builder::RemoteStoreRestoreBodyParams,
  {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `POST` request to `/_remotestore/_restore`
  pub async fn send(self) -> Result<ResponseValue<types::RemoteStoreRestoreResponseContent>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      wait_for_completion,
      body,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::RemoteStoreRestoreBodyParams>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!("{}/_remotestore/_restore", client.baseurl,);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_completion {
      query.push(("wait_for_completion", v.to_string()));
    }
    let request = client
      .client
      .post(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .query(&query)
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

pub mod builder {}
