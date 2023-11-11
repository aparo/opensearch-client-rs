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

///Builder for [`Client::Cat::help`]
///
///[`Client::Cat::help`]: super::OsClient::Cat::help
#[derive(Debug, Clone)]
pub struct CatHelp<'a> {
  client: &'a super::OsClient,
  help: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
}
impl<'a> CatHelp<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      help: Ok(None),
      s: Ok(None),
    }
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self { client, help, s } = self;
    let help = help.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat", client.baseurl,);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::aliases`]
///
///[`Client::Cat::aliases`]: super::OsClient::Cat::aliases
#[derive(Debug, Clone)]
pub struct CatAliases<'a> {
  client: &'a super::OsClient,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatAliases<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      expand_wildcards: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/aliases`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      expand_wildcards,
      format,
      h,
      help,
      local,
      s,
      v,
    } = self;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/aliases", client.baseurl,);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::aliases_with_name`]
///
///[`Client::Cat::aliases_with_name`]: super::OsClient::Cat::aliases_with_name
#[derive(Debug, Clone)]
pub struct CatAliasesWithName<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatAliasesWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      expand_wildcards: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
    self
  }

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/aliases/{name}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      name,
      expand_wildcards,
      format,
      h,
      help,
      local,
      s,
      v,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/aliases/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::allocation`]
///
///[`Client::Cat::allocation`]: super::OsClient::Cat::allocation
#[derive(Debug, Clone)]
pub struct CatAllocation<'a> {
  client: &'a super::OsClient,
  bytes: Result<Option<Bytes>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatAllocation<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      bytes: Ok(None),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/allocation`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      bytes,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      v,
    } = self;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/allocation", client.baseurl,);
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::allocation_with_node_id`]
///
///[`Client::Cat::allocation_with_node_id`]: super::OsClient::Cat::allocation_with_node_id
#[derive(Debug, Clone)]
pub struct CatAllocationWithNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  bytes: Result<Option<Bytes>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatAllocationWithNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      bytes: Ok(None),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn node_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchId>, {
    self.node_id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for node_id failed".to_string());
    self
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/allocation/{node_id}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      node_id,
      bytes,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      v,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_cat/allocation/{}",
      client.baseurl,
      encode_path(&node_id.to_string()),
    );
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::cluster_manager`]
///
///[`Client::Cat::cluster_manager`]: super::OsClient::Cat::cluster_manager
#[derive(Debug, Clone)]
pub struct CatClusterManager<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatClusterManager<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/cluster_manager`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      v,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/cluster_manager", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::count`]
///
///[`Client::Cat::count`]: super::OsClient::Cat::count
#[derive(Debug, Clone)]
pub struct CatCount<'a> {
  client: &'a super::OsClient,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatCount<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/count`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      format,
      h,
      help,
      s,
      v,
    } = self;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/count", client.baseurl,);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::count_with_index`]
///
///[`Client::Cat::count_with_index`]: super::OsClient::Cat::count_with_index
#[derive(Debug, Clone)]
pub struct CatCountWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatCountWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/count/{index}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      index,
      format,
      h,
      help,
      s,
      v,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/count/{}", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::fielddata`]
///
///[`Client::Cat::fielddata`]: super::OsClient::Cat::fielddata
#[derive(Debug, Clone)]
pub struct CatFielddata<'a> {
  client: &'a super::OsClient,
  bytes: Result<Option<Bytes>, String>,
  fields: Result<Option<Vec<String>>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatFielddata<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      bytes: Ok(None),
      fields: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
    self
  }

  pub fn fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for fields failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/fielddata`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      bytes,
      fields,
      format,
      h,
      help,
      s,
      v,
    } = self;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/fielddata", client.baseurl,);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::fielddata_with_fields`]
///
///[`Client::Cat::fielddata_with_fields`]: super::OsClient::Cat::fielddata_with_fields
#[derive(Debug, Clone)]
pub struct CatFielddataWithFields<'a> {
  client: &'a super::OsClient,
  bytes: Result<Option<Bytes>, String>,
  fields: Result<Option<Vec<String>>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatFielddataWithFields<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      bytes: Ok(None),
      fields: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
    self
  }

  pub fn fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for fields failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/fielddata/{fields}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      bytes,
      fields,
      format,
      h,
      help,
      s,
      v,
    } = self;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_cat/fielddata/{}",
      client.baseurl,
      encode_path_option_vec_string(&fields),
    );
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::health`]
///
///[`Client::Cat::health`]: super::OsClient::Cat::health
#[derive(Debug, Clone)]
pub struct CatHealth<'a> {
  client: &'a super::OsClient,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  ts: Result<Option<bool>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatHealth<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      s: Ok(None),
      time: Ok(None),
      ts: Ok(None),
      v: Ok(None),
    }
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn ts<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ts = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ts failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/health`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      format,
      h,
      help,
      s,
      time,
      ts,
      v,
    } = self;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let ts = ts.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/health", client.baseurl,);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &ts {
      query.push(("ts", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::indices`]
///
///[`Client::Cat::indices`]: super::OsClient::Cat::indices
#[derive(Debug, Clone)]
pub struct CatIndices<'a> {
  client: &'a super::OsClient,
  bytes: Result<Option<Bytes>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  health: Result<Option<Health>, String>,
  help: Result<Option<bool>, String>,
  include_unloaded_segments: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  pri: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatIndices<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      bytes: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      format: Ok(None),
      h: Ok(None),
      health: Ok(None),
      help: Ok(None),
      include_unloaded_segments: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      pri: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
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

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn health<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Health>, {
    self.health = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Health` for health failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn include_unloaded_segments<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_unloaded_segments = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_unloaded_segments failed".to_string());
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

  pub fn pri<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.pri = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for pri failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/indices`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      bytes,
      cluster_manager_timeout,
      expand_wildcards,
      format,
      h,
      health,
      help,
      include_unloaded_segments,
      local,
      master_timeout,
      pri,
      s,
      time,
      v,
    } = self;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let health = health.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let include_unloaded_segments = include_unloaded_segments.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let pri = pri.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = client.baseurl.join("/_cat/indices")?;
    let mut query = Vec::with_capacity(14usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &health {
      query.push(("health", v.to_string()));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &include_unloaded_segments {
      query.push(("include_unloaded_segments", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &pri {
      query.push(("pri", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::indices_with_index`]
///
///[`Client::Cat::indices_with_index`]: super::OsClient::Cat::indices_with_index
#[derive(Debug, Clone)]
pub struct CatIndicesWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  bytes: Result<Option<Bytes>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  health: Result<Option<Health>, String>,
  help: Result<Option<bool>, String>,
  include_unloaded_segments: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  pri: Result<Option<bool>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatIndicesWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      bytes: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      format: Ok(None),
      h: Ok(None),
      health: Ok(None),
      help: Ok(None),
      include_unloaded_segments: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      pri: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
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

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn health<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Health>, {
    self.health = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Health` for health failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn include_unloaded_segments<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_unloaded_segments = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_unloaded_segments failed".to_string());
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

  pub fn pri<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.pri = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for pri failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/indices/{index}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      index,
      bytes,
      cluster_manager_timeout,
      expand_wildcards,
      format,
      h,
      health,
      help,
      include_unloaded_segments,
      local,
      master_timeout,
      pri,
      s,
      time,
      v,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let health = health.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let include_unloaded_segments = include_unloaded_segments.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let pri = pri.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/indices/{}", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(14usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &health {
      query.push(("health", v.to_string()));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &include_unloaded_segments {
      query.push(("include_unloaded_segments", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &pri {
      query.push(("pri", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::master`]
///
///[`Client::Cat::master`]: super::OsClient::Cat::master
#[derive(Debug, Clone)]
pub struct CatMaster<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatMaster<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/master`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      v,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/master", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::nodeattrs`]
///
///[`Client::Cat::nodeattrs`]: super::OsClient::Cat::nodeattrs
#[derive(Debug, Clone)]
pub struct CatNodeattrs<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatNodeattrs<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/nodeattrs`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      v,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/nodeattrs", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::nodes`]
///
///[`Client::Cat::nodes`]: super::OsClient::Cat::nodes
#[derive(Debug, Clone)]
pub struct CatNodes<'a> {
  client: &'a super::OsClient,
  bytes: Result<Option<Bytes>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  full_id: Result<Option<bool>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatNodes<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      bytes: Ok(None),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      full_id: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn full_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.full_id = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for full_id failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/nodes`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      bytes,
      cluster_manager_timeout,
      format,
      full_id,
      h,
      help,
      local,
      master_timeout,
      s,
      time,
      v,
    } = self;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let full_id = full_id.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/nodes", client.baseurl,);
    let mut query = Vec::with_capacity(11usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &full_id {
      query.push(("full_id", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::pending_tasks`]
///
///[`Client::Cat::pending_tasks`]: super::OsClient::Cat::pending_tasks
#[derive(Debug, Clone)]
pub struct CatPendingTasks<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatPendingTasks<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/pending_tasks`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      time,
      v,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/pending_tasks", client.baseurl,);
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::pit_segments`]
///
///[`Client::Cat::pit_segments`]: super::OsClient::Cat::pit_segments
#[derive(Debug, Clone)]
pub struct CatPitSegments<'a> {
  client: &'a super::OsClient,
  body: Result<types::builder::CatPitSegmentsBodyParams, String>,
}
impl<'a> CatPitSegments<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Ok(types::builder::CatPitSegmentsBodyParams::default()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::CatPitSegmentsBodyParams>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `CatPitSegmentsBodyParams` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::CatPitSegmentsBodyParams) -> types::builder::CatPitSegmentsBodyParams, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `GET` request to `/_cat/pit_segments`
  pub async fn send(self) -> Result<ResponseValue<types::CatPitSegmentsResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body
      .and_then(std::convert::TryInto::<types::CatPitSegmentsBodyParams>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/pit_segments", client.baseurl,);
    let request = client
      .client
      .get(url)
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
///Builder for [`Client::Cat::all_pit_segments`]
///
///[`Client::Cat::all_pit_segments`]: super::OsClient::Cat::all_pit_segments
#[derive(Debug, Clone)]
pub struct CatAllPitSegments<'a> {
  client: &'a super::OsClient,
}
impl<'a> CatAllPitSegments<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_cat/pit_segments/_all`
  pub async fn send(self) -> Result<ResponseValue<types::CatAllPitSegmentsResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}_cat/pit_segments/_all", client.baseurl,);
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
///Builder for [`Client::Cat::plugins`]
///
///[`Client::Cat::plugins`]: super::OsClient::Cat::plugins
#[derive(Debug, Clone)]
pub struct CatPlugins<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatPlugins<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/plugins`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      v,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/plugins", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::recovery`]
///
///[`Client::Cat::recovery`]: super::OsClient::Cat::recovery
#[derive(Debug, Clone)]
pub struct CatRecovery<'a> {
  client: &'a super::OsClient,
  active_only: Result<Option<bool>, String>,
  bytes: Result<Option<Bytes>, String>,
  detailed: Result<Option<bool>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  index: Result<Option<Vec<String>>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatRecovery<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      active_only: Ok(None),
      bytes: Ok(None),
      detailed: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      index: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn active_only<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.active_only = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for active_only failed".to_string());
    self
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
    self
  }

  pub fn detailed<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.detailed = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for detailed failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.index = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for index failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/recovery`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      active_only,
      bytes,
      detailed,
      format,
      h,
      help,
      index,
      s,
      time,
      v,
    } = self;
    let active_only = active_only.map_err(Error::InvalidRequest)?;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let detailed = detailed.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let index = index.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/recovery", client.baseurl,);
    let mut query = Vec::with_capacity(10usize);
    if let Some(v) = &active_only {
      query.push(("active_only", v.to_string()));
    }
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &detailed {
      query.push(("detailed", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &index {
      query.push(("index", v.join(",")));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::recovery_with_index`]
///
///[`Client::Cat::recovery_with_index`]: super::OsClient::Cat::recovery_with_index
#[derive(Debug, Clone)]
pub struct CatRecoveryWithIndex<'a> {
  client: &'a super::OsClient,
  active_only: Result<Option<bool>, String>,
  bytes: Result<Option<Bytes>, String>,
  detailed: Result<Option<bool>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  index: Result<Option<Vec<String>>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatRecoveryWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      active_only: Ok(None),
      bytes: Ok(None),
      detailed: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      index: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn active_only<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.active_only = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for active_only failed".to_string());
    self
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
    self
  }

  pub fn detailed<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.detailed = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for detailed failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.index = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for index failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/recovery/{index}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      active_only,
      bytes,
      detailed,
      format,
      h,
      help,
      index,
      s,
      time,
      v,
    } = self;
    let active_only = active_only.map_err(Error::InvalidRequest)?;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let detailed = detailed.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let index = index.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = match index {
      Some(_) => {
        format!(
          "{}_cat/recovery/{}",
          client.baseurl,
          encode_path(&index.clone().unwrap().join(",")),
        )
      }
      None => format!("{}_cat/recovery", client.baseurl),
    };
    let mut query = Vec::with_capacity(10usize);
    if let Some(v) = &active_only {
      query.push(("active_only", v.to_string()));
    }
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &detailed {
      query.push(("detailed", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &index {
      query.push(("index", v.join(",")));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::repositories`]
///
///[`Client::Cat::repositories`]: super::OsClient::Cat::repositories
#[derive(Debug, Clone)]
pub struct CatRepositories<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatRepositories<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/repositories`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      v,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/repositories", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::segment_replication`]
///
///[`Client::Cat::segment_replication`]: super::OsClient::Cat::segment_replication
#[derive(Debug, Clone)]
pub struct CatSegmentReplication<'a> {
  client: &'a super::OsClient,
  active_only: Result<Option<bool>, String>,
  bytes: Result<Option<Bytes>, String>,
  completed_only: Result<Option<bool>, String>,
  detailed: Result<Option<bool>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  index: Result<Option<Vec<String>>, String>,
  s: Result<Option<Vec<String>>, String>,
  shards: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatSegmentReplication<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      active_only: Ok(None),
      bytes: Ok(None),
      completed_only: Ok(None),
      detailed: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      index: Ok(None),
      s: Ok(None),
      shards: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn active_only<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.active_only = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for active_only failed".to_string());
    self
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
    self
  }

  pub fn completed_only<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.completed_only = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for completed_only failed".to_string());
    self
  }

  pub fn detailed<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.detailed = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for detailed failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.index = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for index failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for shards failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/segment_replication`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      active_only,
      bytes,
      completed_only,
      detailed,
      format,
      h,
      help,
      index,
      s,
      shards,
      time,
      v,
    } = self;
    let active_only = active_only.map_err(Error::InvalidRequest)?;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let completed_only = completed_only.map_err(Error::InvalidRequest)?;
    let detailed = detailed.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let index = index.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let shards = shards.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/segment_replication", client.baseurl,);
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &active_only {
      query.push(("active_only", v.to_string()));
    }
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &completed_only {
      query.push(("completed_only", v.to_string()));
    }
    if let Some(v) = &detailed {
      query.push(("detailed", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &index {
      query.push(("index", v.join(",")));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &shards {
      query.push(("shards", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::segment_replication_with_index`]
///
///[`Client::Cat::segment_replication_with_index`]: super::OsClient::Cat::segment_replication_with_index
#[derive(Debug, Clone)]
pub struct CatSegmentReplicationWithIndex<'a> {
  client: &'a super::OsClient,
  active_only: Result<Option<bool>, String>,
  bytes: Result<Option<Bytes>, String>,
  completed_only: Result<Option<bool>, String>,
  detailed: Result<Option<bool>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  index: Result<Option<Vec<String>>, String>,
  s: Result<Option<Vec<String>>, String>,
  shards: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatSegmentReplicationWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      active_only: Ok(None),
      bytes: Ok(None),
      completed_only: Ok(None),
      detailed: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      index: Ok(None),
      s: Ok(None),
      shards: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn active_only<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.active_only = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for active_only failed".to_string());
    self
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
    self
  }

  pub fn completed_only<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.completed_only = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for completed_only failed".to_string());
    self
  }

  pub fn detailed<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.detailed = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for detailed failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.index = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for index failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for shards failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/segment_replication/{index}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      active_only,
      bytes,
      completed_only,
      detailed,
      format,
      h,
      help,
      index,
      s,
      shards,
      time,
      v,
    } = self;
    let active_only = active_only.map_err(Error::InvalidRequest)?;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let completed_only = completed_only.map_err(Error::InvalidRequest)?;
    let detailed = detailed.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let index = index.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let shards = shards.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = match index {
      Some(_) => {
        format!(
          "{}_cat/segment_replication/{}",
          client.baseurl,
          encode_path(&index.clone().unwrap().join(",")),
        )
      }
      None => format!("{}_cat/segment_replication", client.baseurl),
    };

    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &active_only {
      query.push(("active_only", v.to_string()));
    }
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &completed_only {
      query.push(("completed_only", v.to_string()));
    }
    if let Some(v) = &detailed {
      query.push(("detailed", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &index {
      query.push(("index", v.join(",")));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &shards {
      query.push(("shards", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::segments`]
///
///[`Client::Cat::segments`]: super::OsClient::Cat::segments
#[derive(Debug, Clone)]
pub struct CatSegments<'a> {
  client: &'a super::OsClient,
  bytes: Result<Option<Bytes>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatSegments<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      bytes: Ok(None),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/segments`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      bytes,
      cluster_manager_timeout,
      format,
      h,
      help,
      master_timeout,
      s,
      v,
    } = self;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/segments", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::segments_with_index`]
///
///[`Client::Cat::segments_with_index`]: super::OsClient::Cat::segments_with_index
#[derive(Debug, Clone)]
pub struct CatSegmentsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  bytes: Result<Option<Bytes>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatSegmentsWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      bytes: Ok(None),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/segments/{index}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      index,
      bytes,
      cluster_manager_timeout,
      format,
      h,
      help,
      master_timeout,
      s,
      v,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/segments/{}", client.baseurl, encode_path(&index.to_string()),);

    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::shards`]
///
///[`Client::Cat::shards`]: super::OsClient::Cat::shards
#[derive(Debug, Clone)]
pub struct CatShards<'a> {
  client: &'a super::OsClient,
  bytes: Result<Option<Bytes>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatShards<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      bytes: Ok(None),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/shards`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      bytes,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      time,
      v,
    } = self;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/shards", client.baseurl,);
    let mut query = Vec::with_capacity(10usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::shards_with_index`]
///
///[`Client::Cat::shards_with_index`]: super::OsClient::Cat::shards_with_index
#[derive(Debug, Clone)]
pub struct CatShardsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  bytes: Result<Option<Bytes>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatShardsWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      bytes: Ok(None),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn bytes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Bytes>, {
    self.bytes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Bytes` for bytes failed".to_string());
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/shards/{index}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      index,
      bytes,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      time,
      v,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let bytes = bytes.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/shards/{}", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(10usize);
    if let Some(v) = &bytes {
      query.push(("bytes", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::snapshots`]
///
///[`Client::Cat::snapshots`]: super::OsClient::Cat::snapshots
#[derive(Debug, Clone)]
pub struct CatSnapshots<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatSnapshots<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/snapshots`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      format,
      h,
      help,
      ignore_unavailable,
      master_timeout,
      s,
      time,
      v,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/snapshots", client.baseurl,);
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::snapshots_with_repository`]
///
///[`Client::Cat::snapshots_with_repository`]: super::OsClient::Cat::snapshots_with_repository
#[derive(Debug, Clone)]
pub struct CatSnapshotsWithRepository<'a> {
  client: &'a super::OsClient,
  repository: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatSnapshotsWithRepository<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      repository: Err("repository was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/snapshots/{repository}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      repository,
      cluster_manager_timeout,
      format,
      h,
      help,
      ignore_unavailable,
      master_timeout,
      s,
      time,
      v,
    } = self;
    let repository = repository.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_cat/snapshots/{}",
      client.baseurl,
      encode_path(&repository.to_string()),
    );
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::tasks`]
///
///[`Client::Cat::tasks`]: super::OsClient::Cat::tasks
#[derive(Debug, Clone)]
pub struct CatTasks<'a> {
  client: &'a super::OsClient,
  actions: Result<Option<Vec<String>>, String>,
  detailed: Result<Option<bool>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  nodes: Result<Option<Vec<String>>, String>,
  parent_task_id: Result<Option<String>, String>,
  s: Result<Option<Vec<String>>, String>,
  time: Result<Option<Time>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatTasks<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      actions: Ok(None),
      detailed: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      nodes: Ok(None),
      parent_task_id: Ok(None),
      s: Ok(None),
      time: Ok(None),
      v: Ok(None),
    }
  }

  pub fn actions<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.actions = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for actions failed".to_string());
    self
  }

  pub fn detailed<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.detailed = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for detailed failed".to_string());
    self
  }

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
    self
  }

  pub fn nodes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.nodes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for nodes failed".to_string());
    self
  }

  pub fn parent_task_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.parent_task_id = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for parent_task_id failed".to_string());
    self
  }

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn time<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Time>, {
    self.time = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Time` for time failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/tasks`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      actions,
      detailed,
      format,
      h,
      help,
      nodes,
      parent_task_id,
      s,
      time,
      v,
    } = self;
    let actions = actions.map_err(Error::InvalidRequest)?;
    let detailed = detailed.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let nodes = nodes.map_err(Error::InvalidRequest)?;
    let parent_task_id = parent_task_id.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let time = time.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/tasks", client.baseurl,);
    let mut query = Vec::with_capacity(10usize);
    if let Some(v) = &actions {
      query.push(("actions", v.join(",")));
    }
    if let Some(v) = &detailed {
      query.push(("detailed", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &nodes {
      query.push(("nodes", v.join(",")));
    }
    if let Some(v) = &parent_task_id {
      query.push(("parent_task_id", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &time {
      query.push(("time", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::templates`]
///
///[`Client::Cat::templates`]: super::OsClient::Cat::templates
#[derive(Debug, Clone)]
pub struct CatTemplates<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatTemplates<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/templates`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      v,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/templates", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::templates_with_name`]
///
///[`Client::Cat::templates_with_name`]: super::OsClient::Cat::templates_with_name
#[derive(Debug, Clone)]
pub struct CatTemplatesWithName<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatTemplatesWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      v: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/templates/{name}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      name,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      v,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/templates/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::thread_pool`]
///
///[`Client::Cat::thread_pool`]: super::OsClient::Cat::thread_pool
#[derive(Debug, Clone)]
pub struct CatThreadPool<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  size: Result<Option<i32>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatThreadPool<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      size: Ok(None),
      v: Ok(None),
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for size failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/thread_pool`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      size,
      v,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let size = size.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cat/thread_pool", client.baseurl,);
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &size {
      query.push(("size", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cat::thread_pool_with_thread_pool_patterns`]
///
///[`Client::Cat::thread_pool_with_thread_pool_patterns`]: super::OsClient::Cat::thread_pool_with_thread_pool_patterns
#[derive(Debug, Clone)]
pub struct CatThreadPoolWithThreadPoolPatterns<'a> {
  client: &'a super::OsClient,
  thread_pool_patterns: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  format: Result<Option<String>, String>,
  h: Result<Option<Vec<String>>, String>,
  help: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  s: Result<Option<Vec<String>>, String>,
  size: Result<Option<i32>, String>,
  v: Result<Option<bool>, String>,
}
impl<'a> CatThreadPoolWithThreadPoolPatterns<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      thread_pool_patterns: Err("thread_pool_patterns was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      format: Ok(None),
      h: Ok(None),
      help: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      s: Ok(None),
      size: Ok(None),
      v: Ok(None),
    }
  }

  pub fn thread_pool_patterns<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.thread_pool_patterns = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for thread_pool_patterns failed".to_string());
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

  pub fn format<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.format = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for format failed".to_string());
    self
  }

  pub fn h<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.h = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for h failed".to_string());
    self
  }

  pub fn help<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.help = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for help failed".to_string());
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

  pub fn s<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.s = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for s failed".to_string());
    self
  }

  pub fn size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for size failed".to_string());
    self
  }

  pub fn v<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.v = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for v failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cat/thread_pool/{thread_pool_patterns}`
  pub async fn send(self) -> Result<ResponseValue<String>, Error> {
    let Self {
      client,
      thread_pool_patterns,
      cluster_manager_timeout,
      format,
      h,
      help,
      local,
      master_timeout,
      s,
      size,
      v,
    } = self;
    let thread_pool_patterns = thread_pool_patterns.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let format = format.map_err(Error::InvalidRequest)?;
    let h = h.map_err(Error::InvalidRequest)?;
    let help = help.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let s = s.map_err(Error::InvalidRequest)?;
    let size = size.map_err(Error::InvalidRequest)?;
    let v = v.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_cat/thread_pool/{}",
      client.baseurl,
      encode_path(&thread_pool_patterns.to_string()),
    );
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &format {
      query.push(("format", v.to_string()));
    }
    if let Some(v) = &h {
      query.push(("h", v.join(",")));
    }
    if let Some(v) = &help {
      query.push(("help", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &s {
      query.push(("s", v.join(",")));
    }
    if let Some(v) = &size {
      query.push(("size", v.to_string()));
    }
    if let Some(v) = &v {
      query.push(("v", v.to_string()));
    }
    let request = client.client.get(url).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => ResponseValue::text(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
