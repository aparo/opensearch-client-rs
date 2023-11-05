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

///Builder for [`Client::indices_get_alias`]
///
///[`Client::indices_get_alias`]: super::OsClient::indices_get_alias
#[derive(Debug, Clone)]
pub struct IndicesGetAlias<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesGetAlias<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `GET` request to `/_alias`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_alias", client.baseurl,);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
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
///Builder for [`Client::indices_get_alias_with_name`]
///
///[`Client::indices_get_alias_with_name`]: super::OsClient::indices_get_alias_with_name
#[derive(Debug, Clone)]
pub struct IndicesGetAliasWithName<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesGetAliasWithNameName, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesGetAliasWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetAliasWithNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetAliasWithNameName` for name failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `GET` request to `/_alias/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_alias/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
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
///Builder for [`Client::indices_exists_alias`]
///
///[`Client::indices_exists_alias`]: super::OsClient::indices_exists_alias
#[derive(Debug, Clone)]
pub struct IndicesExistsAlias<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesExistsAliasName, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesExistsAlias<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesExistsAliasName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesExistsAliasName` for name failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `HEAD` request to `/_alias/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_alias/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    let request = client.client.head(url).query(&query).build()?;
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
///Builder for [`Client::indices_update_aliases`]
///
///[`Client::indices_update_aliases`]: super::OsClient::indices_update_aliases
#[derive(Debug, Clone)]
pub struct IndicesUpdateAliases<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::builder::IndicesUpdateAliasesBodyParams, String>,
}
impl<'a> IndicesUpdateAliases<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Ok(types::builder::IndicesUpdateAliasesBodyParams::default()),
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

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesUpdateAliasesBodyParams>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `IndicesUpdateAliasesBodyParams` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(
      types::builder::IndicesUpdateAliasesBodyParams,
    ) -> types::builder::IndicesUpdateAliasesBodyParams, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `POST` request to `/_aliases`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesUpdateAliasesResponseContent>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      body,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::IndicesUpdateAliasesBodyParams>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!("{}/_aliases", client.baseurl,);
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
///Builder for [`Client::indices_analyze_get`]
///
///[`Client::indices_analyze_get`]: super::OsClient::indices_analyze_get
#[derive(Debug, Clone)]
pub struct IndicesAnalyzeGet<'a> {
  client: &'a super::OsClient,
  index: Result<Option<String>, String>,
}
impl<'a> IndicesAnalyzeGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.index = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for index failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_analyze`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, index } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_analyze", client.baseurl,);
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &index {
      query.push(("index", v.to_string()));
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
///Builder for [`Client::indices_analyze_post`]
///
///[`Client::indices_analyze_post`]: super::OsClient::indices_analyze_post
#[derive(Debug, Clone)]
pub struct IndicesAnalyzePost<'a> {
  client: &'a super::OsClient,
  index: Result<Option<String>, String>,
  body: Result<types::IndicesAnalyzeBodyParams, String>,
}
impl<'a> IndicesAnalyzePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.index = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for index failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesAnalyzeBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesAnalyzeBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_analyze`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, index, body } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_analyze", client.baseurl,);
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &index {
      query.push(("index", v.to_string()));
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
///Builder for [`Client::indices_clear_cache`]
///
///[`Client::indices_clear_cache`]: super::OsClient::indices_clear_cache
#[derive(Debug, Clone)]
pub struct IndicesClearCache<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  fielddata: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  index: Result<Option<Vec<String>>, String>,
  query: Result<Option<bool>, String>,
  request: Result<Option<bool>, String>,
}
impl<'a> IndicesClearCache<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      fielddata: Ok(None),
      fields: Ok(None),
      ignore_unavailable: Ok(None),
      index: Ok(None),
      query: Ok(None),
      request: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn fielddata<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.fielddata = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for fielddata failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  pub fn query<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.query = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for query failed".to_string());
    self
  }

  pub fn request<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.request = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for request failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_cache/clear`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      fielddata,
      fields,
      ignore_unavailable,
      index,
      query,
      request,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fielddata = fielddata.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let index = index.map_err(Error::InvalidRequest)?;
    let query_opt = query.map_err(Error::InvalidRequest)?;
    let request = request.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cache/clear", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fielddata {
      query.push(("fielddata", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &index {
      query.push(("index", v.join(",")));
    }
    if let Some(v) = &query_opt {
      query.push(("query", v.to_string()));
    }
    if let Some(v) = &request {
      query.push(("request", v.to_string()));
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
///Builder for [`Client::indices_get_data_stream`]
///
///[`Client::indices_get_data_stream`]: super::OsClient::indices_get_data_stream
#[derive(Debug, Clone)]
pub struct IndicesGetDataStream<'a> {
  client: &'a super::OsClient,
}
impl<'a> IndicesGetDataStream<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_data_stream`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesGetDataStreamResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}/_data_stream", client.baseurl,);
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
///Builder for [`Client::indices_data_streams_stats`]
///
///[`Client::indices_data_streams_stats`]: super::OsClient::indices_data_streams_stats
#[derive(Debug, Clone)]
pub struct IndicesDataStreamsStats<'a> {
  client: &'a super::OsClient,
}
impl<'a> IndicesDataStreamsStats<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_data_stream/_stats`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}/_data_stream/_stats", client.baseurl,);
    let request = client.client.get(url).build()?;
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
///Builder for [`Client::indices_get_data_stream_with_name`]
///
///[`Client::indices_get_data_stream_with_name`]: super::OsClient::indices_get_data_stream_with_name
#[derive(Debug, Clone)]
pub struct IndicesGetDataStreamWithName<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesGetDataStreamWithNameName, String>,
}
impl<'a> IndicesGetDataStreamWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetDataStreamWithNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetDataStreamWithNameName` for name failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_data_stream/{name}`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesGetDataStreamWithNameResponseContent>, Error> {
    let Self { client, name } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_data_stream/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::indices_create_data_stream`]
///
///[`Client::indices_create_data_stream`]: super::OsClient::indices_create_data_stream
#[derive(Debug, Clone)]
pub struct IndicesCreateDataStream<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesCreateDataStreamName, String>,
  body: Result<types::IndicesCreateDataStreamBodyParams, String>,
}
impl<'a> IndicesCreateDataStream<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesCreateDataStreamName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesCreateDataStreamName` for name failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesCreateDataStreamBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesCreateDataStreamBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_data_stream/{name}`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesCreateDataStreamResponseContent>, Error> {
    let Self { client, name, body } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_data_stream/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::indices_delete_data_stream`]
///
///[`Client::indices_delete_data_stream`]: super::OsClient::indices_delete_data_stream
#[derive(Debug, Clone)]
pub struct IndicesDeleteDataStream<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesDeleteDataStreamName, String>,
}
impl<'a> IndicesDeleteDataStream<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesDeleteDataStreamName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesDeleteDataStreamName` for name failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_data_stream/{name}`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesDeleteDataStreamResponseContent>, Error> {
    let Self { client, name } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_data_stream/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::indices_data_streams_stats_with_name`]
///
///[`Client::indices_data_streams_stats_with_name`]: super::OsClient::indices_data_streams_stats_with_name
#[derive(Debug, Clone)]
pub struct IndicesDataStreamsStatsWithName<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesDataStreamsStatsWithNameName, String>,
}
impl<'a> IndicesDataStreamsStatsWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesDataStreamsStatsWithNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesDataStreamsStatsWithNameName` for name failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_data_stream/{name}/_stats`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, name } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_data_stream/{}/_stats",
      client.baseurl,
      encode_path(&name.to_string()),
    );
    let request = client.client.get(url).build()?;
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
///Builder for [`Client::indices_flush_get`]
///
///[`Client::indices_flush_get`]: super::OsClient::indices_flush_get
#[derive(Debug, Clone)]
pub struct IndicesFlushGet<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  force: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  wait_if_ongoing: Result<Option<bool>, String>,
}
impl<'a> IndicesFlushGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      force: Ok(None),
      ignore_unavailable: Ok(None),
      wait_if_ongoing: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn force<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.force = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for force failed".to_string());
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

  pub fn wait_if_ongoing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_if_ongoing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_if_ongoing failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_flush`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      force,
      ignore_unavailable,
      wait_if_ongoing,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let force = force.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let wait_if_ongoing = wait_if_ongoing.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_flush", client.baseurl,);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &force {
      query.push(("force", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &wait_if_ongoing {
      query.push(("wait_if_ongoing", v.to_string()));
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
///Builder for [`Client::indices_flush_post`]
///
///[`Client::indices_flush_post`]: super::OsClient::indices_flush_post
#[derive(Debug, Clone)]
pub struct IndicesFlushPost<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  force: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  wait_if_ongoing: Result<Option<bool>, String>,
}
impl<'a> IndicesFlushPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      force: Ok(None),
      ignore_unavailable: Ok(None),
      wait_if_ongoing: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn force<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.force = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for force failed".to_string());
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

  pub fn wait_if_ongoing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_if_ongoing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_if_ongoing failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_flush`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      force,
      ignore_unavailable,
      wait_if_ongoing,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let force = force.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let wait_if_ongoing = wait_if_ongoing.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_flush", client.baseurl,);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &force {
      query.push(("force", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &wait_if_ongoing {
      query.push(("wait_if_ongoing", v.to_string()));
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
///Builder for [`Client::indices_forcemerge`]
///
///[`Client::indices_forcemerge`]: super::OsClient::indices_forcemerge
#[derive(Debug, Clone)]
pub struct IndicesForcemerge<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flush: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  max_num_segments: Result<Option<i32>, String>,
  only_expunge_deletes: Result<Option<bool>, String>,
}
impl<'a> IndicesForcemerge<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      flush: Ok(None),
      ignore_unavailable: Ok(None),
      max_num_segments: Ok(None),
      only_expunge_deletes: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flush<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flush = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flush failed".to_string());
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

  pub fn max_num_segments<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_num_segments = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_num_segments failed".to_string());
    self
  }

  pub fn only_expunge_deletes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.only_expunge_deletes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for only_expunge_deletes failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_forcemerge`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      flush,
      ignore_unavailable,
      max_num_segments,
      only_expunge_deletes,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flush = flush.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let max_num_segments = max_num_segments.map_err(Error::InvalidRequest)?;
    let only_expunge_deletes = only_expunge_deletes.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_forcemerge", client.baseurl,);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flush {
      query.push(("flush", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &max_num_segments {
      query.push(("max_num_segments", v.to_string()));
    }
    if let Some(v) = &only_expunge_deletes {
      query.push(("only_expunge_deletes", v.to_string()));
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
///Builder for [`Client::indices_get_index_template`]
///
///[`Client::indices_get_index_template`]: super::OsClient::indices_get_index_template
#[derive(Debug, Clone)]
pub struct IndicesGetIndexTemplate<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  flat_settings: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetIndexTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      flat_settings: Ok(None),
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  ///Sends a `GET` request to `/_index_template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      flat_settings,
      local,
      master_timeout,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_index_template", client.baseurl,);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
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
///Builder for [`Client::indices_simulate_template`]
///
///[`Client::indices_simulate_template`]: super::OsClient::indices_simulate_template
#[derive(Debug, Clone)]
pub struct IndicesSimulateTemplate<'a> {
  client: &'a super::OsClient,
  cause: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesSimulateTemplateBodyParams, String>,
}
impl<'a> IndicesSimulateTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cause: Ok(None),
      cluster_manager_timeout: Ok(None),
      create: Ok(None),
      master_timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn cause<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.cause = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for cause failed".to_string());
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

  pub fn create<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.create = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for create failed".to_string());
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
    V: std::convert::TryInto<types::IndicesSimulateTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSimulateTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_index_template/_simulate`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      cause,
      cluster_manager_timeout,
      create,
      master_timeout,
      body,
    } = self;
    let cause = cause.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let create = create.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_index_template/_simulate", client.baseurl,);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cause {
      query.push(("cause", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &create {
      query.push(("create", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
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
///Builder for [`Client::indices_simulate_template_with_name`]
///
///[`Client::indices_simulate_template_with_name`]: super::OsClient::indices_simulate_template_with_name
#[derive(Debug, Clone)]
pub struct IndicesSimulateTemplateWithName<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesSimulateTemplateWithNameName, String>,
  cause: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesSimulateTemplateBodyParams, String>,
}
impl<'a> IndicesSimulateTemplateWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cause: Ok(None),
      cluster_manager_timeout: Ok(None),
      create: Ok(None),
      master_timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesSimulateTemplateWithNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSimulateTemplateWithNameName` for name failed".to_string());
    self
  }

  pub fn cause<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.cause = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for cause failed".to_string());
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

  pub fn create<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.create = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for create failed".to_string());
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
    V: std::convert::TryInto<types::IndicesSimulateTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSimulateTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_index_template/_simulate/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cause,
      cluster_manager_timeout,
      create,
      master_timeout,
      body,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cause = cause.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let create = create.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_index_template/_simulate/{}",
      client.baseurl,
      encode_path(&name.to_string()),
    );
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cause {
      query.push(("cause", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &create {
      query.push(("create", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
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
///Builder for [`Client::indices_simulate_index_template`]
///
///[`Client::indices_simulate_index_template`]: super::OsClient::indices_simulate_index_template
#[derive(Debug, Clone)]
pub struct IndicesSimulateIndexTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesSimulateIndexTemplateName, String>,
  cause: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesSimulateIndexTemplateBodyParams, String>,
}
impl<'a> IndicesSimulateIndexTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cause: Ok(None),
      cluster_manager_timeout: Ok(None),
      create: Ok(None),
      master_timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesSimulateIndexTemplateName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSimulateIndexTemplateName` for name failed".to_string());
    self
  }

  pub fn cause<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.cause = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for cause failed".to_string());
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

  pub fn create<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.create = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for create failed".to_string());
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
    V: std::convert::TryInto<types::IndicesSimulateIndexTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSimulateIndexTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_index_template/_simulate_index/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cause,
      cluster_manager_timeout,
      create,
      master_timeout,
      body,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cause = cause.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let create = create.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_index_template/_simulate_index/{}",
      client.baseurl,
      encode_path(&name.to_string()),
    );
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cause {
      query.push(("cause", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &create {
      query.push(("create", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
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
///Builder for [`Client::indices_get_index_template_with_name`]
///
///[`Client::indices_get_index_template_with_name`]: super::OsClient::indices_get_index_template_with_name
#[derive(Debug, Clone)]
pub struct IndicesGetIndexTemplateWithName<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesGetIndexTemplateWithNameName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  flat_settings: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetIndexTemplateWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      flat_settings: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetIndexTemplateWithNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetIndexTemplateWithNameName` for name failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  ///Sends a `GET` request to `/_index_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cluster_manager_timeout,
      flat_settings,
      local,
      master_timeout,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_index_template/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
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
///Builder for [`Client::indices_put_index_template_put`]
///
///[`Client::indices_put_index_template_put`]: super::OsClient::indices_put_index_template_put
#[derive(Debug, Clone)]
pub struct IndicesPutIndexTemplatePut<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesPutIndexTemplatePutName, String>,
  cause: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesPutIndexTemplateBodyParams, String>,
}
impl<'a> IndicesPutIndexTemplatePut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cause: Ok(None),
      cluster_manager_timeout: Ok(None),
      create: Ok(None),
      master_timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutIndexTemplatePutName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutIndexTemplatePutName` for name failed".to_string());
    self
  }

  pub fn cause<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.cause = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for cause failed".to_string());
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

  pub fn create<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.create = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for create failed".to_string());
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
    V: std::convert::TryInto<types::IndicesPutIndexTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutIndexTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_index_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cause,
      cluster_manager_timeout,
      create,
      master_timeout,
      body,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cause = cause.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let create = create.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_index_template/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cause {
      query.push(("cause", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &create {
      query.push(("create", v.to_string()));
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
///Builder for [`Client::indices_put_index_template_post`]
///
///[`Client::indices_put_index_template_post`]: super::OsClient::indices_put_index_template_post
#[derive(Debug, Clone)]
pub struct IndicesPutIndexTemplatePost<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesPutIndexTemplatePostName, String>,
  cause: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesPutIndexTemplateBodyParams, String>,
}
impl<'a> IndicesPutIndexTemplatePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cause: Ok(None),
      cluster_manager_timeout: Ok(None),
      create: Ok(None),
      master_timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutIndexTemplatePostName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutIndexTemplatePostName` for name failed".to_string());
    self
  }

  pub fn cause<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.cause = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for cause failed".to_string());
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

  pub fn create<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.create = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for create failed".to_string());
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
    V: std::convert::TryInto<types::IndicesPutIndexTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutIndexTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_index_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cause,
      cluster_manager_timeout,
      create,
      master_timeout,
      body,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cause = cause.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let create = create.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_index_template/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cause {
      query.push(("cause", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &create {
      query.push(("create", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
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
///Builder for [`Client::indices_delete_index_template`]
///
///[`Client::indices_delete_index_template`]: super::OsClient::indices_delete_index_template
#[derive(Debug, Clone)]
pub struct IndicesDeleteIndexTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesDeleteIndexTemplateName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesDeleteIndexTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesDeleteIndexTemplateName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesDeleteIndexTemplateName` for name failed".to_string());
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

  ///Sends a `DELETE` request to `/_index_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_index_template/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::indices_exists_index_template`]
///
///[`Client::indices_exists_index_template`]: super::OsClient::indices_exists_index_template
#[derive(Debug, Clone)]
pub struct IndicesExistsIndexTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesExistsIndexTemplateName, String>,
  flat_settings: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesExistsIndexTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      flat_settings: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesExistsIndexTemplateName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesExistsIndexTemplateName` for name failed".to_string());
    self
  }

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  ///Sends a `HEAD` request to `/_index_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      flat_settings,
      local,
      master_timeout,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_index_template/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    let request = client.client.head(url).query(&query).build()?;
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
///Builder for [`Client::indices_get_mapping`]
///
///[`Client::indices_get_mapping`]: super::OsClient::indices_get_mapping
#[derive(Debug, Clone)]
pub struct IndicesGetMapping<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetMapping<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `GET` request to `/_mapping`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      ignore_unavailable,
      local,
      master_timeout,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_mapping", client.baseurl,);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_get_field_mapping`]
///
///[`Client::indices_get_field_mapping`]: super::OsClient::indices_get_field_mapping
#[derive(Debug, Clone)]
pub struct IndicesGetFieldMapping<'a> {
  client: &'a super::OsClient,
  fields: Result<types::IndicesGetFieldMappingFields, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_defaults: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesGetFieldMapping<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      fields: Err("fields was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      include_defaults: Ok(None),
      local: Ok(None),
    }
  }

  pub fn fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetFieldMappingFields>, {
    self.fields = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetFieldMappingFields` for fields failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn include_defaults<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_defaults = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_defaults failed".to_string());
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

  ///Sends a `GET` request to `/_mapping/field/{fields}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      fields,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      include_defaults,
      local,
    } = self;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_defaults = include_defaults.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_mapping/field/{}", client.baseurl, encode_path(&fields.to_string()),);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_defaults {
      query.push(("include_defaults", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
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
///Builder for [`Client::indices_recovery`]
///
///[`Client::indices_recovery`]: super::OsClient::indices_recovery
#[derive(Debug, Clone)]
pub struct IndicesRecovery<'a> {
  client: &'a super::OsClient,
  active_only: Result<Option<bool>, String>,
  detailed: Result<Option<bool>, String>,
}
impl<'a> IndicesRecovery<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      active_only: Ok(None),
      detailed: Ok(None),
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

  pub fn detailed<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.detailed = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for detailed failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_recovery`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      active_only,
      detailed,
    } = self;
    let active_only = active_only.map_err(Error::InvalidRequest)?;
    let detailed = detailed.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_recovery", client.baseurl,);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &active_only {
      query.push(("active_only", v.to_string()));
    }
    if let Some(v) = &detailed {
      query.push(("detailed", v.to_string()));
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
///Builder for [`Client::indices_refresh_get`]
///
///[`Client::indices_refresh_get`]: super::OsClient::indices_refresh_get
#[derive(Debug, Clone)]
pub struct IndicesRefreshGet<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
}
impl<'a> IndicesRefreshGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_refresh`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_refresh", client.baseurl,);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_refresh_post`]
///
///[`Client::indices_refresh_post`]: super::OsClient::indices_refresh_post
#[derive(Debug, Clone)]
pub struct IndicesRefreshPost<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
}
impl<'a> IndicesRefreshPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_refresh`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_refresh", client.baseurl,);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_resolve_index`]
///
///[`Client::indices_resolve_index`]: super::OsClient::indices_resolve_index
#[derive(Debug, Clone)]
pub struct IndicesResolveIndex<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesResolveIndexName, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
}
impl<'a> IndicesResolveIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      expand_wildcards: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesResolveIndexName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesResolveIndexName` for name failed".to_string());
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

  ///Sends a `GET` request to `/_resolve/index/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      expand_wildcards,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_resolve/index/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
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
///Builder for [`Client::indices_segments`]
///
///[`Client::indices_segments`]: super::OsClient::indices_segments
#[derive(Debug, Clone)]
pub struct IndicesSegments<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  verbose: Result<Option<bool>, String>,
}
impl<'a> IndicesSegments<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      verbose: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `GET` request to `/_segments`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      verbose,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let verbose = verbose.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_segments", client.baseurl,);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_get_settings`]
///
///[`Client::indices_get_settings`]: super::OsClient::indices_get_settings
#[derive(Debug, Clone)]
pub struct IndicesGetSettings<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_defaults: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetSettings<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      include_defaults: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  pub fn include_defaults<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_defaults = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_defaults failed".to_string());
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

  ///Sends a `GET` request to `/_settings`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      include_defaults,
      local,
      master_timeout,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_defaults = include_defaults.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_settings", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_defaults {
      query.push(("include_defaults", v.to_string()));
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
///Builder for [`Client::indices_put_settings`]
///
///[`Client::indices_put_settings`]: super::OsClient::indices_put_settings
#[derive(Debug, Clone)]
pub struct IndicesPutSettings<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  preserve_existing: Result<Option<bool>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesPutSettingsBodyParams, String>,
}
impl<'a> IndicesPutSettings<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      preserve_existing: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  pub fn preserve_existing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.preserve_existing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for preserve_existing failed".to_string());
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

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutSettingsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutSettingsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_settings`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      master_timeout,
      preserve_existing,
      timeout,
      body,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let preserve_existing = preserve_existing.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_settings", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &preserve_existing {
      query.push(("preserve_existing", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::indices_get_settings_with_name`]
///
///[`Client::indices_get_settings_with_name`]: super::OsClient::indices_get_settings_with_name
#[derive(Debug, Clone)]
pub struct IndicesGetSettingsWithName<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesGetSettingsWithNameName, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_defaults: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetSettingsWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      include_defaults: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetSettingsWithNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetSettingsWithNameName` for name failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  pub fn include_defaults<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_defaults = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_defaults failed".to_string());
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

  ///Sends a `GET` request to `/_settings/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      include_defaults,
      local,
      master_timeout,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_defaults = include_defaults.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_settings/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_defaults {
      query.push(("include_defaults", v.to_string()));
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
///Builder for [`Client::indices_shard_stores`]
///
///[`Client::indices_shard_stores`]: super::OsClient::indices_shard_stores
#[derive(Debug, Clone)]
pub struct IndicesShardStores<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  status: Result<Option<Vec<StatusMember>>, String>,
}
impl<'a> IndicesShardStores<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      status: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn status<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<StatusMember>>, {
    self.status = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < StatusMember >` for status failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_shard_stores`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      status,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let status = status.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_shard_stores", client.baseurl,);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    // if let Some(v) = &status {
    //   query.push(("status", v.to_string()));
    // }
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
///Builder for [`Client::indices_stats`]
///
///[`Client::indices_stats`]: super::OsClient::indices_stats
#[derive(Debug, Clone)]
pub struct IndicesStats<'a> {
  client: &'a super::OsClient,
  completion_fields: Result<Option<Vec<String>>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  forbid_closed_indices: Result<Option<bool>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  include_unloaded_segments: Result<Option<bool>, String>,
  level: Result<Option<types::IndiciesStatLevel>, String>,
}
impl<'a> IndicesStats<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      completion_fields: Ok(None),
      expand_wildcards: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      forbid_closed_indices: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      include_unloaded_segments: Ok(None),
      level: Ok(None),
    }
  }

  pub fn completion_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.completion_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for completion_fields failed".to_string());
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

  pub fn fielddata_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.fielddata_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for fielddata_fields failed".to_string());
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

  pub fn forbid_closed_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.forbid_closed_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for forbid_closed_indices failed".to_string());
    self
  }

  pub fn groups<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.groups = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for groups failed".to_string());
    self
  }

  pub fn include_segment_file_sizes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_segment_file_sizes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_segment_file_sizes failed".to_string());
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndiciesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IndiciesStatLevel` for level failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_stats`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      completion_fields,
      expand_wildcards,
      fielddata_fields,
      fields,
      forbid_closed_indices,
      groups,
      include_segment_file_sizes,
      include_unloaded_segments,
      level,
    } = self;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let forbid_closed_indices = forbid_closed_indices.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let include_unloaded_segments = include_unloaded_segments.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_stats", client.baseurl,);
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &forbid_closed_indices {
      query.push(("forbid_closed_indices", v.to_string()));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &include_unloaded_segments {
      query.push(("include_unloaded_segments", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
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
///Builder for [`Client::indices_stats_with_metric`]
///
///[`Client::indices_stats_with_metric`]: super::OsClient::indices_stats_with_metric
#[derive(Debug, Clone)]
pub struct IndicesStatsWithMetric<'a> {
  client: &'a super::OsClient,
  metric: Result<types::IndicesStatsWithMetricMetric, String>,
  completion_fields: Result<Option<Vec<String>>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  forbid_closed_indices: Result<Option<bool>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  include_unloaded_segments: Result<Option<bool>, String>,
  level: Result<Option<types::IndiciesStatLevel>, String>,
}
impl<'a> IndicesStatsWithMetric<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      metric: Err("metric was not initialized".to_string()),
      completion_fields: Ok(None),
      expand_wildcards: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      forbid_closed_indices: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      include_unloaded_segments: Ok(None),
      level: Ok(None),
    }
  }

  pub fn metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesStatsWithMetricMetric>, {
    self.metric = value
      .try_into()
      .map_err(|_| "conversion to `IndicesStatsWithMetricMetric` for metric failed".to_string());
    self
  }

  pub fn completion_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.completion_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for completion_fields failed".to_string());
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

  pub fn fielddata_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.fielddata_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for fielddata_fields failed".to_string());
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

  pub fn forbid_closed_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.forbid_closed_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for forbid_closed_indices failed".to_string());
    self
  }

  pub fn groups<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.groups = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for groups failed".to_string());
    self
  }

  pub fn include_segment_file_sizes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_segment_file_sizes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_segment_file_sizes failed".to_string());
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndiciesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IndiciesStatLevel` for level failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_stats/{metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      metric,
      completion_fields,
      expand_wildcards,
      fielddata_fields,
      fields,
      forbid_closed_indices,
      groups,
      include_segment_file_sizes,
      include_unloaded_segments,
      level,
    } = self;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let forbid_closed_indices = forbid_closed_indices.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let include_unloaded_segments = include_unloaded_segments.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_stats/{}", client.baseurl, encode_path(&metric.to_string()),);
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &forbid_closed_indices {
      query.push(("forbid_closed_indices", v.to_string()));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &include_unloaded_segments {
      query.push(("include_unloaded_segments", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
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
///Builder for [`Client::indices_get_template`]
///
///[`Client::indices_get_template`]: super::OsClient::indices_get_template
#[derive(Debug, Clone)]
pub struct IndicesGetTemplate<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  flat_settings: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      flat_settings: Ok(None),
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  ///Sends a `GET` request to `/_template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      flat_settings,
      local,
      master_timeout,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_template", client.baseurl,);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
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
///Builder for [`Client::indices_get_template_with_name`]
///
///[`Client::indices_get_template_with_name`]: super::OsClient::indices_get_template_with_name
#[derive(Debug, Clone)]
pub struct IndicesGetTemplateWithName<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesGetTemplateWithNameName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  flat_settings: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetTemplateWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      flat_settings: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetTemplateWithNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetTemplateWithNameName` for name failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  ///Sends a `GET` request to `/_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cluster_manager_timeout,
      flat_settings,
      local,
      master_timeout,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_template/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
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
///Builder for [`Client::indices_put_template_put`]
///
///[`Client::indices_put_template_put`]: super::OsClient::indices_put_template_put
#[derive(Debug, Clone)]
pub struct IndicesPutTemplatePut<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesPutTemplatePutName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  order: Result<Option<i32>, String>,
  body: Result<types::IndicesPutTemplateBodyParams, String>,
}
impl<'a> IndicesPutTemplatePut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      create: Ok(None),
      master_timeout: Ok(None),
      order: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutTemplatePutName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutTemplatePutName` for name failed".to_string());
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

  pub fn create<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.create = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for create failed".to_string());
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

  pub fn order<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.order = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for order failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cluster_manager_timeout,
      create,
      master_timeout,
      order,
      body,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let create = create.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let order = order.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_template/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &create {
      query.push(("create", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &order {
      query.push(("order", v.to_string()));
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
///Builder for [`Client::indices_put_template_post`]
///
///[`Client::indices_put_template_post`]: super::OsClient::indices_put_template_post
#[derive(Debug, Clone)]
pub struct IndicesPutTemplatePost<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesPutTemplatePostName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  order: Result<Option<i32>, String>,
  body: Result<types::IndicesPutTemplateBodyParams, String>,
}
impl<'a> IndicesPutTemplatePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      create: Ok(None),
      master_timeout: Ok(None),
      order: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutTemplatePostName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutTemplatePostName` for name failed".to_string());
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

  pub fn create<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.create = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for create failed".to_string());
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

  pub fn order<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.order = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for order failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cluster_manager_timeout,
      create,
      master_timeout,
      order,
      body,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let create = create.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let order = order.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_template/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &create {
      query.push(("create", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &order {
      query.push(("order", v.to_string()));
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
///Builder for [`Client::indices_delete_template`]
///
///[`Client::indices_delete_template`]: super::OsClient::indices_delete_template
#[derive(Debug, Clone)]
pub struct IndicesDeleteTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesDeleteTemplateName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesDeleteTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesDeleteTemplateName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesDeleteTemplateName` for name failed".to_string());
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

  ///Sends a `DELETE` request to `/_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_template/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::indices_exists_template`]
///
///[`Client::indices_exists_template`]: super::OsClient::indices_exists_template
#[derive(Debug, Clone)]
pub struct IndicesExistsTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<types::IndicesExistsTemplateName, String>,
  flat_settings: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesExistsTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      flat_settings: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesExistsTemplateName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesExistsTemplateName` for name failed".to_string());
    self
  }

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  ///Sends a `HEAD` request to `/_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      flat_settings,
      local,
      master_timeout,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_template/{}", client.baseurl, encode_path(&name.to_string()),);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    let request = client.client.head(url).query(&query).build()?;
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
///Builder for [`Client::indices_get_upgrade`]
///
///[`Client::indices_get_upgrade`]: super::OsClient::indices_get_upgrade
#[derive(Debug, Clone)]
pub struct IndicesGetUpgrade<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
}
impl<'a> IndicesGetUpgrade<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_upgrade`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_upgrade", client.baseurl,);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_upgrade`]
///
///[`Client::indices_upgrade`]: super::OsClient::indices_upgrade
#[derive(Debug, Clone)]
pub struct IndicesUpgrade<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  only_ancient_segments: Result<Option<bool>, String>,
  wait_for_completion: Result<Option<bool>, String>,
}
impl<'a> IndicesUpgrade<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      only_ancient_segments: Ok(None),
      wait_for_completion: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn only_ancient_segments<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.only_ancient_segments = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for only_ancient_segments failed".to_string());
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

  ///Sends a `POST` request to `/_upgrade`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      only_ancient_segments,
      wait_for_completion,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let only_ancient_segments = only_ancient_segments.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_upgrade", client.baseurl,);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &only_ancient_segments {
      query.push(("only_ancient_segments", v.to_string()));
    }
    if let Some(v) = &wait_for_completion {
      query.push(("wait_for_completion", v.to_string()));
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
///Builder for [`Client::indices_validate_query_get`]
///
///[`Client::indices_validate_query_get`]: super::OsClient::indices_validate_query_get
#[derive(Debug, Clone)]
pub struct IndicesValidateQueryGet<'a> {
  client: &'a super::OsClient,
  all_shards: Result<Option<bool>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  default_operator: Result<Option<DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  explain: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  lenient: Result<Option<bool>, String>,
  q: Result<Option<String>, String>,
  rewrite: Result<Option<bool>, String>,
}
impl<'a> IndicesValidateQueryGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      all_shards: Ok(None),
      allow_no_indices: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      expand_wildcards: Ok(None),
      explain: Ok(None),
      ignore_unavailable: Ok(None),
      lenient: Ok(None),
      q: Ok(None),
      rewrite: Ok(None),
    }
  }

  pub fn all_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.all_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for all_shards failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
    self
  }

  pub fn analyze_wildcard<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.analyze_wildcard = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for analyze_wildcard failed".to_string());
    self
  }

  pub fn analyzer<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.analyzer = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for analyzer failed".to_string());
    self
  }

  pub fn default_operator<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<DefaultOperator>, {
    self.default_operator = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `DefaultOperator` for default_operator failed".to_string());
    self
  }

  pub fn df<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.df = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for df failed".to_string());
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

  pub fn explain<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.explain = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for explain failed".to_string());
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

  pub fn lenient<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.lenient = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for lenient failed".to_string());
    self
  }

  pub fn q<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.q = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for q failed".to_string());
    self
  }

  pub fn rewrite<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rewrite = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rewrite failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_validate/query`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      all_shards,
      allow_no_indices,
      analyze_wildcard,
      analyzer,
      default_operator,
      df,
      expand_wildcards,
      explain,
      ignore_unavailable,
      lenient,
      q,
      rewrite,
    } = self;
    let all_shards = all_shards.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let rewrite = rewrite.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_validate/query", client.baseurl,);
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &all_shards {
      query.push(("all_shards", v.to_string()));
    }
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &analyze_wildcard {
      query.push(("analyze_wildcard", v.to_string()));
    }
    if let Some(v) = &analyzer {
      query.push(("analyzer", v.to_string()));
    }
    if let Some(v) = &default_operator {
      query.push(("default_operator", v.to_string()));
    }
    if let Some(v) = &df {
      query.push(("df", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &rewrite {
      query.push(("rewrite", v.to_string()));
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
///Builder for [`Client::indices_validate_query_post`]
///
///[`Client::indices_validate_query_post`]: super::OsClient::indices_validate_query_post
#[derive(Debug, Clone)]
pub struct IndicesValidateQueryPost<'a> {
  client: &'a super::OsClient,
  all_shards: Result<Option<bool>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  default_operator: Result<Option<DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  explain: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  lenient: Result<Option<bool>, String>,
  q: Result<Option<String>, String>,
  rewrite: Result<Option<bool>, String>,
  body: Result<types::IndicesValidateQueryBodyParams, String>,
}
impl<'a> IndicesValidateQueryPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      all_shards: Ok(None),
      allow_no_indices: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      expand_wildcards: Ok(None),
      explain: Ok(None),
      ignore_unavailable: Ok(None),
      lenient: Ok(None),
      q: Ok(None),
      rewrite: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn all_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.all_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for all_shards failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
    self
  }

  pub fn analyze_wildcard<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.analyze_wildcard = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for analyze_wildcard failed".to_string());
    self
  }

  pub fn analyzer<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.analyzer = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for analyzer failed".to_string());
    self
  }

  pub fn default_operator<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<DefaultOperator>, {
    self.default_operator = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `DefaultOperator` for default_operator failed".to_string());
    self
  }

  pub fn df<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.df = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for df failed".to_string());
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

  pub fn explain<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.explain = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for explain failed".to_string());
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

  pub fn lenient<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.lenient = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for lenient failed".to_string());
    self
  }

  pub fn q<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.q = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for q failed".to_string());
    self
  }

  pub fn rewrite<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rewrite = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rewrite failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesValidateQueryBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesValidateQueryBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_validate/query`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      all_shards,
      allow_no_indices,
      analyze_wildcard,
      analyzer,
      default_operator,
      df,
      expand_wildcards,
      explain,
      ignore_unavailable,
      lenient,
      q,
      rewrite,
      body,
    } = self;
    let all_shards = all_shards.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let rewrite = rewrite.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_validate/query", client.baseurl,);
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &all_shards {
      query.push(("all_shards", v.to_string()));
    }
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &analyze_wildcard {
      query.push(("analyze_wildcard", v.to_string()));
    }
    if let Some(v) = &analyzer {
      query.push(("analyzer", v.to_string()));
    }
    if let Some(v) = &default_operator {
      query.push(("default_operator", v.to_string()));
    }
    if let Some(v) = &df {
      query.push(("df", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &rewrite {
      query.push(("rewrite", v.to_string()));
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
///Builder for [`Client::indices_rollover`]
///
///[`Client::indices_rollover`]: super::OsClient::indices_rollover
#[derive(Debug, Clone)]
pub struct IndicesRollover<'a> {
  client: &'a super::OsClient,
  alias: Result<types::IndicesRolloverAlias, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  dry_run: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::IndicesRolloverBodyParams, String>,
}
impl<'a> IndicesRollover<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      alias: Err("alias was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      dry_run: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn alias<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesRolloverAlias>, {
    self.alias = value
      .try_into()
      .map_err(|_| "conversion to `IndicesRolloverAlias` for alias failed".to_string());
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

  pub fn dry_run<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.dry_run = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for dry_run failed".to_string());
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

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesRolloverBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesRolloverBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{alias}/_rollover`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      alias,
      cluster_manager_timeout,
      dry_run,
      master_timeout,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let alias = alias.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let dry_run = dry_run.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_rollover", client.baseurl, encode_path(&alias.to_string()),);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &dry_run {
      query.push(("dry_run", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_rollover_with_new_index`]
///
///[`Client::indices_rollover_with_new_index`]: super::OsClient::indices_rollover_with_new_index
#[derive(Debug, Clone)]
pub struct IndicesRolloverWithNewIndex<'a> {
  client: &'a super::OsClient,
  alias: Result<types::IndicesRolloverWithNewIndexAlias, String>,
  new_index: Result<types::IndicesRolloverWithNewIndexNewIndex, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  dry_run: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::IndicesRolloverBodyParams, String>,
}
impl<'a> IndicesRolloverWithNewIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      alias: Err("alias was not initialized".to_string()),
      new_index: Err("new_index was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      dry_run: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn alias<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesRolloverWithNewIndexAlias>, {
    self.alias = value
      .try_into()
      .map_err(|_| "conversion to `IndicesRolloverWithNewIndexAlias` for alias failed".to_string());
    self
  }

  pub fn new_index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesRolloverWithNewIndexNewIndex>, {
    self.new_index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesRolloverWithNewIndexNewIndex` for new_index failed".to_string());
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

  pub fn dry_run<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.dry_run = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for dry_run failed".to_string());
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

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesRolloverBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesRolloverBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{alias}/_rollover/{new_index}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      alias,
      new_index,
      cluster_manager_timeout,
      dry_run,
      master_timeout,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let alias = alias.map_err(Error::InvalidRequest)?;
    let new_index = new_index.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let dry_run = dry_run.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_rollover/{}",
      client.baseurl,
      encode_path(&alias.to_string()),
      encode_path(&new_index.to_string()),
    );
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &dry_run {
      query.push(("dry_run", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_get`]
///
///[`Client::indices_get`]: super::OsClient::indices_get
#[derive(Debug, Clone)]
pub struct IndicesGet<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesGetIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_defaults: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      include_defaults: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  pub fn include_defaults<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_defaults = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_defaults failed".to_string());
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

  ///Sends a `GET` request to `/{index}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      include_defaults,
      local,
      master_timeout,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_defaults = include_defaults.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_defaults {
      query.push(("include_defaults", v.to_string()));
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
///Builder for [`Client::indices_create`]
///
///[`Client::indices_create`]: super::OsClient::indices_create
#[derive(Debug, Clone)]
pub struct IndicesCreate<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesCreateIndex, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::builder::IndicesCreateBodyParams, String>,
}
impl<'a> IndicesCreate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Ok(types::builder::IndicesCreateBodyParams::default()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesCreateIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesCreateIndex` for index failed".to_string());
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

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesCreateBodyParams>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `IndicesCreateBodyParams` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::IndicesCreateBodyParams) -> types::builder::IndicesCreateBodyParams, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to `/{index}`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesCreateResponseContent>, Error> {
    let Self {
      client,
      index,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::IndicesCreateBodyParams>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}", client.baseurl, encode_path(&index.to_string()),);
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
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
    }
    let request = client
      .client
      .put(url)
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
///Builder for [`Client::indices_delete`]
///
///[`Client::indices_delete`]: super::OsClient::indices_delete
#[derive(Debug, Clone)]
pub struct IndicesDelete<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesDeleteIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<types::IndicesTimeout>, String>,
}
impl<'a> IndicesDelete<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesDeleteIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesDeleteIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesTimeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IndicesTimeout` for timeout failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/{index}`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesDeleteResponseContent>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      master_timeout,
      timeout,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    let request = client
      .client
      .delete(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
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
///Builder for [`Client::indices_exists`]
///
///[`Client::indices_exists`]: super::OsClient::indices_exists
#[derive(Debug, Clone)]
pub struct IndicesExists<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesExistsIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_defaults: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesExists<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      include_defaults: Ok(None),
      local: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesExistsIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesExistsIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  pub fn include_defaults<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_defaults = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_defaults failed".to_string());
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

  ///Sends a `HEAD` request to `/{index}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      include_defaults,
      local,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_defaults = include_defaults.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_defaults {
      query.push(("include_defaults", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    let request = client.client.head(url).query(&query).build()?;
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
///Builder for [`Client::indices_get_alias_with_index`]
///
///[`Client::indices_get_alias_with_index`]: super::OsClient::indices_get_alias_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetAliasWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesGetAliasWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesGetAliasWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetAliasWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetAliasWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `GET` request to `/{index}/_alias`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_alias", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
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
///Builder for [`Client::indices_get_alias_with_index_name`]
///
///[`Client::indices_get_alias_with_index_name`]: super::OsClient::indices_get_alias_with_index_name
#[derive(Debug, Clone)]
pub struct IndicesGetAliasWithIndexName<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesGetAliasWithIndexNameIndex, String>,
  name: Result<types::IndicesGetAliasWithIndexNameName, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesGetAliasWithIndexName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      name: Err("name was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetAliasWithIndexNameIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetAliasWithIndexNameIndex` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetAliasWithIndexNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetAliasWithIndexNameName` for name failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `GET` request to `/{index}/_alias/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      name,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let name = name.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_alias/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&name.to_string()),
    );
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
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
///Builder for [`Client::indices_put_alias_put`]
///
///[`Client::indices_put_alias_put`]: super::OsClient::indices_put_alias_put
#[derive(Debug, Clone)]
pub struct IndicesPutAliasPut<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesPutAliasPutIndex, String>,
  name: Result<types::IndicesPutAliasPutName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesPutAliasBodyParams, String>,
}
impl<'a> IndicesPutAliasPut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasPutIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasPutIndex` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasPutName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasPutName` for name failed".to_string());
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

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_alias/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      name,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_alias/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&name.to_string()),
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
///Builder for [`Client::indices_put_alias_post`]
///
///[`Client::indices_put_alias_post`]: super::OsClient::indices_put_alias_post
#[derive(Debug, Clone)]
pub struct IndicesPutAliasPost<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesPutAliasPostIndex, String>,
  name: Result<types::IndicesPutAliasPostName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesPutAliasBodyParams, String>,
}
impl<'a> IndicesPutAliasPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasPostIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasPostIndex` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasPostName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasPostName` for name failed".to_string());
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

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_alias/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      name,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_alias/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&name.to_string()),
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
///Builder for [`Client::indices_delete_alias`]
///
///[`Client::indices_delete_alias`]: super::OsClient::indices_delete_alias
#[derive(Debug, Clone)]
pub struct IndicesDeleteAlias<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesDeleteAliasIndex, String>,
  name: Result<types::IndicesDeleteAliasName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesDeleteAlias<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesDeleteAliasIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesDeleteAliasIndex` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesDeleteAliasName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesDeleteAliasName` for name failed".to_string());
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

  ///Sends a `DELETE` request to `/{index}/_alias/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      name,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_alias/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&name.to_string()),
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
///Builder for [`Client::indices_exists_alias_with_index`]
///
///[`Client::indices_exists_alias_with_index`]: super::OsClient::indices_exists_alias_with_index
#[derive(Debug, Clone)]
pub struct IndicesExistsAliasWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesExistsAliasWithIndexIndex, String>,
  name: Result<types::IndicesExistsAliasWithIndexName, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesExistsAliasWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      name: Err("name was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesExistsAliasWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesExistsAliasWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesExistsAliasWithIndexName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesExistsAliasWithIndexName` for name failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `HEAD` request to `/{index}/_alias/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      name,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let name = name.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_alias/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&name.to_string()),
    );
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    let request = client.client.head(url).query(&query).build()?;
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
///Builder for [`Client::indices_put_alias_put_plural`]
///
///[`Client::indices_put_alias_put_plural`]: super::OsClient::indices_put_alias_put_plural
#[derive(Debug, Clone)]
pub struct IndicesPutAliasPutPlural<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesPutAliasPutPluralIndex, String>,
  name: Result<types::IndicesPutAliasPutPluralName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesPutAliasBodyParams, String>,
}
impl<'a> IndicesPutAliasPutPlural<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasPutPluralIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasPutPluralIndex` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasPutPluralName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasPutPluralName` for name failed".to_string());
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

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_aliases/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      name,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_aliases/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&name.to_string()),
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
///Builder for [`Client::indices_put_alias_post_plural`]
///
///[`Client::indices_put_alias_post_plural`]: super::OsClient::indices_put_alias_post_plural
#[derive(Debug, Clone)]
pub struct IndicesPutAliasPostPlural<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesPutAliasPostPluralIndex, String>,
  name: Result<types::IndicesPutAliasPostPluralName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesPutAliasBodyParams, String>,
}
impl<'a> IndicesPutAliasPostPlural<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasPostPluralIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasPostPluralIndex` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasPostPluralName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasPostPluralName` for name failed".to_string());
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

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutAliasBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutAliasBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_aliases/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      name,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_aliases/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&name.to_string()),
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
///Builder for [`Client::indices_delete_alias_plural`]
///
///[`Client::indices_delete_alias_plural`]: super::OsClient::indices_delete_alias_plural
#[derive(Debug, Clone)]
pub struct IndicesDeleteAliasPlural<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesDeleteAliasPluralIndex, String>,
  name: Result<types::IndicesDeleteAliasPluralName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesDeleteAliasPlural<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesDeleteAliasPluralIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesDeleteAliasPluralIndex` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesDeleteAliasPluralName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesDeleteAliasPluralName` for name failed".to_string());
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

  ///Sends a `DELETE` request to `/{index}/_aliases/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      name,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_aliases/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&name.to_string()),
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
///Builder for [`Client::indices_analyze_get_with_index`]
///
///[`Client::indices_analyze_get_with_index`]: super::OsClient::indices_analyze_get_with_index
#[derive(Debug, Clone)]
pub struct IndicesAnalyzeGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<Option<String>, String>,
}
impl<'a> IndicesAnalyzeGetWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.index = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for index failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_analyze`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, index } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_analyze",
      client.baseurl,
      encode_path(&index.clone().unwrap_or(String::from("_all"))),
    );
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &index {
      query.push(("index", v.to_string()));
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
///Builder for [`Client::indices_analyze_post_with_index`]
///
///[`Client::indices_analyze_post_with_index`]: super::OsClient::indices_analyze_post_with_index
#[derive(Debug, Clone)]
pub struct IndicesAnalyzePostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<Option<String>, String>,
  body: Result<types::IndicesAnalyzeBodyParams, String>,
}
impl<'a> IndicesAnalyzePostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.index = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for index failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesAnalyzeBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesAnalyzeBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_analyze`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, index, body } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_analyze", client.baseurl);
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &index {
      query.push(("index", v.to_string()));
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
///Builder for [`Client::indices_add_block`]
///
///[`Client::indices_add_block`]: super::OsClient::indices_add_block
#[derive(Debug, Clone)]
pub struct IndicesAddBlock<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesAddBlockIndex, String>,
  block: Result<types::IndicesAddBlockBlock, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesAddBlock<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      block: Err("block was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesAddBlockIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesAddBlockIndex` for index failed".to_string());
    self
  }

  pub fn block<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesAddBlockBlock>, {
    self.block = value
      .try_into()
      .map_err(|_| "conversion to `IndicesAddBlockBlock` for block failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_block/{block}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      block,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      ignore_unavailable,
      master_timeout,
      timeout,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let block = block.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_block/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&block.to_string()),
    );
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    let request = client.client.put(url).query(&query).build()?;
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
///Builder for [`Client::indices_clear_cache_with_index`]
///
///[`Client::indices_clear_cache_with_index`]: super::OsClient::indices_clear_cache_with_index
#[derive(Debug, Clone)]
pub struct IndicesClearCacheWithIndex<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  fielddata: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  index: Result<Option<Vec<String>>, String>,
  query: Result<Option<bool>, String>,
  request: Result<Option<bool>, String>,
}
impl<'a> IndicesClearCacheWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      fielddata: Ok(None),
      fields: Ok(None),
      ignore_unavailable: Ok(None),
      index: Ok(None),
      query: Ok(None),
      request: Ok(None),
    }
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn fielddata<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.fielddata = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for fielddata failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  pub fn query<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.query = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for query failed".to_string());
    self
  }

  pub fn request<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.request = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for request failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_cache/clear`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      fielddata,
      fields,
      ignore_unavailable,
      index,
      query,
      request,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fielddata = fielddata.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let index = index.map_err(Error::InvalidRequest)?;
    let query_opt = query.map_err(Error::InvalidRequest)?;
    let request = request.map_err(Error::InvalidRequest)?;
    let url = match &index {
      Some(idx) => {
        format!(
          "{}/{}/_cache/clear",
          client.baseurl,
          encode_path(&index.clone().unwrap().join(",")),
        )
      }
      None => format!("{}/_cache/clear", client.baseurl),
    };
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fielddata {
      query.push(("fielddata", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &index {
      query.push(("index", v.join(",")));
    }
    if let Some(v) = &query_opt {
      query.push(("query", v.to_string()));
    }
    if let Some(v) = &request {
      query.push(("request", v.to_string()));
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
///Builder for [`Client::indices_clone_put`]
///
///[`Client::indices_clone_put`]: super::OsClient::indices_clone_put
#[derive(Debug, Clone)]
pub struct IndicesClonePut<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesClonePutIndex, String>,
  target: Result<types::IndicesClonePutTarget, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::IndicesCloneBodyParams, String>,
}
impl<'a> IndicesClonePut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      target: Err("target was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesClonePutIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesClonePutIndex` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesClonePutTarget>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `IndicesClonePutTarget` for target failed".to_string());
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

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesCloneBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesCloneBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_clone/{target}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      target,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let target = target.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_clone/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&target.to_string()),
    );
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
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_clone_post`]
///
///[`Client::indices_clone_post`]: super::OsClient::indices_clone_post
#[derive(Debug, Clone)]
pub struct IndicesClonePost<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesClonePostIndex, String>,
  target: Result<types::IndicesClonePostTarget, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::IndicesCloneBodyParams, String>,
}
impl<'a> IndicesClonePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      target: Err("target was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesClonePostIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesClonePostIndex` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesClonePostTarget>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `IndicesClonePostTarget` for target failed".to_string());
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

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesCloneBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesCloneBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_clone/{target}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      target,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let target = target.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_clone/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&target.to_string()),
    );
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
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_close`]
///
///[`Client::indices_close`]: super::OsClient::indices_close
#[derive(Debug, Clone)]
pub struct IndicesClose<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesCloseIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
}
impl<'a> IndicesClose<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesCloseIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesCloseIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_close`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      ignore_unavailable,
      master_timeout,
      timeout,
      wait_for_active_shards,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_close", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_flush_get_with_index`]
///
///[`Client::indices_flush_get_with_index`]: super::OsClient::indices_flush_get_with_index
#[derive(Debug, Clone)]
pub struct IndicesFlushGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesFlushGetWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  force: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  wait_if_ongoing: Result<Option<bool>, String>,
}
impl<'a> IndicesFlushGetWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      force: Ok(None),
      ignore_unavailable: Ok(None),
      wait_if_ongoing: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesFlushGetWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesFlushGetWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn force<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.force = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for force failed".to_string());
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

  pub fn wait_if_ongoing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_if_ongoing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_if_ongoing failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_flush`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      force,
      ignore_unavailable,
      wait_if_ongoing,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let force = force.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let wait_if_ongoing = wait_if_ongoing.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_flush", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &force {
      query.push(("force", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &wait_if_ongoing {
      query.push(("wait_if_ongoing", v.to_string()));
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
///Builder for [`Client::indices_flush_post_with_index`]
///
///[`Client::indices_flush_post_with_index`]: super::OsClient::indices_flush_post_with_index
#[derive(Debug, Clone)]
pub struct IndicesFlushPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesFlushPostWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  force: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  wait_if_ongoing: Result<Option<bool>, String>,
}
impl<'a> IndicesFlushPostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      force: Ok(None),
      ignore_unavailable: Ok(None),
      wait_if_ongoing: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesFlushPostWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesFlushPostWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn force<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.force = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for force failed".to_string());
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

  pub fn wait_if_ongoing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_if_ongoing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_if_ongoing failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_flush`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      force,
      ignore_unavailable,
      wait_if_ongoing,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let force = force.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let wait_if_ongoing = wait_if_ongoing.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_flush", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &force {
      query.push(("force", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &wait_if_ongoing {
      query.push(("wait_if_ongoing", v.to_string()));
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
///Builder for [`Client::indices_forcemerge_with_index`]
///
///[`Client::indices_forcemerge_with_index`]: super::OsClient::indices_forcemerge_with_index
#[derive(Debug, Clone)]
pub struct IndicesForcemergeWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesForcemergeWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flush: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  max_num_segments: Result<Option<i32>, String>,
  only_expunge_deletes: Result<Option<bool>, String>,
}
impl<'a> IndicesForcemergeWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      flush: Ok(None),
      ignore_unavailable: Ok(None),
      max_num_segments: Ok(None),
      only_expunge_deletes: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesForcemergeWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesForcemergeWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flush<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flush = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flush failed".to_string());
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

  pub fn max_num_segments<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_num_segments = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_num_segments failed".to_string());
    self
  }

  pub fn only_expunge_deletes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.only_expunge_deletes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for only_expunge_deletes failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_forcemerge`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      flush,
      ignore_unavailable,
      max_num_segments,
      only_expunge_deletes,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flush = flush.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let max_num_segments = max_num_segments.map_err(Error::InvalidRequest)?;
    let only_expunge_deletes = only_expunge_deletes.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_forcemerge", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flush {
      query.push(("flush", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &max_num_segments {
      query.push(("max_num_segments", v.to_string()));
    }
    if let Some(v) = &only_expunge_deletes {
      query.push(("only_expunge_deletes", v.to_string()));
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
///Builder for [`Client::indices_get_mapping_with_index`]
///
///[`Client::indices_get_mapping_with_index`]: super::OsClient::indices_get_mapping_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetMappingWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesGetMappingWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetMappingWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetMappingWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetMappingWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `GET` request to `/{index}/_mapping`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      ignore_unavailable,
      local,
      master_timeout,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_mapping", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_put_mapping_put`]
///
///[`Client::indices_put_mapping_put`]: super::OsClient::indices_put_mapping_put
#[derive(Debug, Clone)]
pub struct IndicesPutMappingPut<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesPutMappingPutIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  write_index_only: Result<Option<bool>, String>,
  body: Result<types::IndicesPutMappingBodyParams, String>,
}
impl<'a> IndicesPutMappingPut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      write_index_only: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutMappingPutIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutMappingPutIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn write_index_only<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.write_index_only = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for write_index_only failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutMappingBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutMappingBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_mapping`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesPutMappingPutResponseContent>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      ignore_unavailable,
      master_timeout,
      timeout,
      write_index_only,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let write_index_only = write_index_only.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_mapping", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &write_index_only {
      query.push(("write_index_only", v.to_string()));
    }
    let request = client
      .client
      .put(url)
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
///Builder for [`Client::indices_put_mapping_post`]
///
///[`Client::indices_put_mapping_post`]: super::OsClient::indices_put_mapping_post
#[derive(Debug, Clone)]
pub struct IndicesPutMappingPost<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesPutMappingPostIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  write_index_only: Result<Option<bool>, String>,
  body: Result<types::IndicesPutMappingBodyParams, String>,
}
impl<'a> IndicesPutMappingPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      write_index_only: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutMappingPostIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutMappingPostIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn write_index_only<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.write_index_only = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for write_index_only failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutMappingBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutMappingBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_mapping`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesPutMappingPostResponseContent>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      ignore_unavailable,
      master_timeout,
      timeout,
      write_index_only,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let write_index_only = write_index_only.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_mapping", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &write_index_only {
      query.push(("write_index_only", v.to_string()));
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
///Builder for [`Client::indices_get_field_mapping_with_index`]
///
///[`Client::indices_get_field_mapping_with_index`]: super::OsClient::indices_get_field_mapping_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetFieldMappingWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesGetFieldMappingWithIndexIndex, String>,
  fields: Result<types::IndicesGetFieldMappingWithIndexFields, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_defaults: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesGetFieldMappingWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      fields: Err("fields was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      include_defaults: Ok(None),
      local: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetFieldMappingWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetFieldMappingWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetFieldMappingWithIndexFields>, {
    self.fields = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetFieldMappingWithIndexFields` for fields failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn include_defaults<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_defaults = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_defaults failed".to_string());
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

  ///Sends a `GET` request to `/{index}/_mapping/field/{fields}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      fields,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      include_defaults,
      local,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_defaults = include_defaults.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_mapping/field/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&fields.to_string()),
    );
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_defaults {
      query.push(("include_defaults", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
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
///Builder for [`Client::indices_open`]
///
///[`Client::indices_open`]: super::OsClient::indices_open
#[derive(Debug, Clone)]
pub struct IndicesOpen<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesOpenIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
}
impl<'a> IndicesOpen<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesOpenIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesOpenIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_open`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      master_timeout,
      timeout,
      wait_for_active_shards,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_open", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_recovery_with_index`]
///
///[`Client::indices_recovery_with_index`]: super::OsClient::indices_recovery_with_index
#[derive(Debug, Clone)]
pub struct IndicesRecoveryWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesRecoveryWithIndexIndex, String>,
  active_only: Result<Option<bool>, String>,
  detailed: Result<Option<bool>, String>,
}
impl<'a> IndicesRecoveryWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      active_only: Ok(None),
      detailed: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesRecoveryWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesRecoveryWithIndexIndex` for index failed".to_string());
    self
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

  pub fn detailed<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.detailed = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for detailed failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_recovery`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      active_only,
      detailed,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let active_only = active_only.map_err(Error::InvalidRequest)?;
    let detailed = detailed.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_recovery", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &active_only {
      query.push(("active_only", v.to_string()));
    }
    if let Some(v) = &detailed {
      query.push(("detailed", v.to_string()));
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
///Builder for [`Client::indices_refresh_get_with_index`]
///
///[`Client::indices_refresh_get_with_index`]: super::OsClient::indices_refresh_get_with_index
#[derive(Debug, Clone)]
pub struct IndicesRefreshGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesRefreshGetWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
}
impl<'a> IndicesRefreshGetWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesRefreshGetWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesRefreshGetWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_refresh`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_refresh", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_refresh_post_with_index`]
///
///[`Client::indices_refresh_post_with_index`]: super::OsClient::indices_refresh_post_with_index
#[derive(Debug, Clone)]
pub struct IndicesRefreshPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesRefreshPostWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
}
impl<'a> IndicesRefreshPostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesRefreshPostWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesRefreshPostWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_refresh`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_refresh", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_segments_with_index`]
///
///[`Client::indices_segments_with_index`]: super::OsClient::indices_segments_with_index
#[derive(Debug, Clone)]
pub struct IndicesSegmentsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesSegmentsWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  verbose: Result<Option<bool>, String>,
}
impl<'a> IndicesSegmentsWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      verbose: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesSegmentsWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSegmentsWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
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

  ///Sends a `GET` request to `/{index}/_segments`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      verbose,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let verbose = verbose.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_segments", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_get_settings_with_index`]
///
///[`Client::indices_get_settings_with_index`]: super::OsClient::indices_get_settings_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetSettingsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesGetSettingsWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_defaults: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetSettingsWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      include_defaults: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetSettingsWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetSettingsWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  pub fn include_defaults<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_defaults = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_defaults failed".to_string());
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

  ///Sends a `GET` request to `/{index}/_settings`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      include_defaults,
      local,
      master_timeout,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_defaults = include_defaults.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_settings", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_defaults {
      query.push(("include_defaults", v.to_string()));
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
///Builder for [`Client::indices_put_settings_with_index`]
///
///[`Client::indices_put_settings_with_index`]: super::OsClient::indices_put_settings_with_index
#[derive(Debug, Clone)]
pub struct IndicesPutSettingsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesPutSettingsWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  preserve_existing: Result<Option<bool>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesPutSettingsBodyParams, String>,
}
impl<'a> IndicesPutSettingsWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      master_timeout: Ok(None),
      preserve_existing: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutSettingsWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutSettingsWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  pub fn preserve_existing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.preserve_existing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for preserve_existing failed".to_string());
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

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesPutSettingsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesPutSettingsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_settings`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      master_timeout,
      preserve_existing,
      timeout,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let preserve_existing = preserve_existing.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_settings", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &preserve_existing {
      query.push(("preserve_existing", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::indices_get_settings_with_index_name`]
///
///[`Client::indices_get_settings_with_index_name`]: super::OsClient::indices_get_settings_with_index_name
#[derive(Debug, Clone)]
pub struct IndicesGetSettingsWithIndexName<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesGetSettingsWithIndexNameIndex, String>,
  name: Result<types::IndicesGetSettingsWithIndexNameName, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_defaults: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetSettingsWithIndexName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      name: Err("name was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      include_defaults: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetSettingsWithIndexNameIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetSettingsWithIndexNameIndex` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetSettingsWithIndexNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetSettingsWithIndexNameName` for name failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  pub fn include_defaults<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_defaults = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_defaults failed".to_string());
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

  ///Sends a `GET` request to `/{index}/_settings/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      name,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      include_defaults,
      local,
      master_timeout,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let name = name.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_defaults = include_defaults.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_settings/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&name.to_string()),
    );
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_defaults {
      query.push(("include_defaults", v.to_string()));
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
///Builder for [`Client::indices_shard_stores_with_index`]
///
///[`Client::indices_shard_stores_with_index`]: super::OsClient::indices_shard_stores_with_index
#[derive(Debug, Clone)]
pub struct IndicesShardStoresWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesShardStoresWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  status: Result<Option<Vec<StatusMember>>, String>,
}
impl<'a> IndicesShardStoresWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      status: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesShardStoresWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesShardStoresWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn status<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<StatusMember>>, {
    self.status = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < StatusMember >` for status failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_shard_stores`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      status,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let status = status.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_shard_stores", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    // if let Some(v) = &status {
    //   query.push(("status", v.to_string()));
    // }
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
///Builder for [`Client::indices_shrink_put`]
///
///[`Client::indices_shrink_put`]: super::OsClient::indices_shrink_put
#[derive(Debug, Clone)]
pub struct IndicesShrinkPut<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesShrinkPutIndex, String>,
  target: Result<types::IndicesShrinkPutTarget, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  copy_settings: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::IndicesShrinkBodyParams, String>,
}
impl<'a> IndicesShrinkPut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      target: Err("target was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      copy_settings: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesShrinkPutIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesShrinkPutIndex` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesShrinkPutTarget>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `IndicesShrinkPutTarget` for target failed".to_string());
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

  pub fn copy_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.copy_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for copy_settings failed".to_string());
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

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesShrinkBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesShrinkBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_shrink/{target}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      target,
      cluster_manager_timeout,
      copy_settings,
      master_timeout,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let target = target.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let copy_settings = copy_settings.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_shrink/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&target.to_string()),
    );
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &copy_settings {
      query.push(("copy_settings", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_shrink_post`]
///
///[`Client::indices_shrink_post`]: super::OsClient::indices_shrink_post
#[derive(Debug, Clone)]
pub struct IndicesShrinkPost<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesShrinkPostIndex, String>,
  target: Result<types::IndicesShrinkPostTarget, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  copy_settings: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::IndicesShrinkBodyParams, String>,
}
impl<'a> IndicesShrinkPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      target: Err("target was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      copy_settings: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesShrinkPostIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesShrinkPostIndex` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesShrinkPostTarget>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `IndicesShrinkPostTarget` for target failed".to_string());
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

  pub fn copy_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.copy_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for copy_settings failed".to_string());
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

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesShrinkBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesShrinkBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_shrink/{target}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      target,
      cluster_manager_timeout,
      copy_settings,
      master_timeout,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let target = target.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let copy_settings = copy_settings.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_shrink/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&target.to_string()),
    );
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &copy_settings {
      query.push(("copy_settings", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_split_put`]
///
///[`Client::indices_split_put`]: super::OsClient::indices_split_put
#[derive(Debug, Clone)]
pub struct IndicesSplitPut<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesSplitPutIndex, String>,
  target: Result<types::IndicesSplitPutTarget, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  copy_settings: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::IndicesSplitBodyParams, String>,
}
impl<'a> IndicesSplitPut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      target: Err("target was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      copy_settings: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesSplitPutIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSplitPutIndex` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesSplitPutTarget>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSplitPutTarget` for target failed".to_string());
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

  pub fn copy_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.copy_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for copy_settings failed".to_string());
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

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesSplitBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSplitBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_split/{target}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      target,
      cluster_manager_timeout,
      copy_settings,
      master_timeout,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let target = target.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let copy_settings = copy_settings.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_split/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&target.to_string()),
    );
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &copy_settings {
      query.push(("copy_settings", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_split_post`]
///
///[`Client::indices_split_post`]: super::OsClient::indices_split_post
#[derive(Debug, Clone)]
pub struct IndicesSplitPost<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesSplitPostIndex, String>,
  target: Result<types::IndicesSplitPostTarget, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  copy_settings: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::IndicesSplitBodyParams, String>,
}
impl<'a> IndicesSplitPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      target: Err("target was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      copy_settings: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesSplitPostIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSplitPostIndex` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesSplitPostTarget>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSplitPostTarget` for target failed".to_string());
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

  pub fn copy_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.copy_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for copy_settings failed".to_string());
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

  pub fn wait_for_active_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_active_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_active_shards failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesSplitBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesSplitBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_split/{target}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      target,
      cluster_manager_timeout,
      copy_settings,
      master_timeout,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let target = target.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let copy_settings = copy_settings.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_split/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&target.to_string()),
    );
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &copy_settings {
      query.push(("copy_settings", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
///Builder for [`Client::indices_stats_with_index`]
///
///[`Client::indices_stats_with_index`]: super::OsClient::indices_stats_with_index
#[derive(Debug, Clone)]
pub struct IndicesStatsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesStatsWithIndexIndex, String>,
  completion_fields: Result<Option<Vec<String>>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  forbid_closed_indices: Result<Option<bool>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  include_unloaded_segments: Result<Option<bool>, String>,
  level: Result<Option<types::IndiciesStatLevel>, String>,
}
impl<'a> IndicesStatsWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      completion_fields: Ok(None),
      expand_wildcards: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      forbid_closed_indices: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      include_unloaded_segments: Ok(None),
      level: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesStatsWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesStatsWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn completion_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.completion_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for completion_fields failed".to_string());
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

  pub fn fielddata_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.fielddata_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for fielddata_fields failed".to_string());
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

  pub fn forbid_closed_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.forbid_closed_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for forbid_closed_indices failed".to_string());
    self
  }

  pub fn groups<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.groups = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for groups failed".to_string());
    self
  }

  pub fn include_segment_file_sizes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_segment_file_sizes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_segment_file_sizes failed".to_string());
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndiciesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IndiciesStatLevel` for level failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_stats`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      completion_fields,
      expand_wildcards,
      fielddata_fields,
      fields,
      forbid_closed_indices,
      groups,
      include_segment_file_sizes,
      include_unloaded_segments,
      level,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let forbid_closed_indices = forbid_closed_indices.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let include_unloaded_segments = include_unloaded_segments.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_stats", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &forbid_closed_indices {
      query.push(("forbid_closed_indices", v.to_string()));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &include_unloaded_segments {
      query.push(("include_unloaded_segments", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
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
///Builder for [`Client::indices_stats_with_index_metric`]
///
///[`Client::indices_stats_with_index_metric`]: super::OsClient::indices_stats_with_index_metric
#[derive(Debug, Clone)]
pub struct IndicesStatsWithIndexMetric<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesStatsWithIndexMetricIndex, String>,
  metric: Result<types::IndicesStatsWithIndexMetricMetric, String>,
  completion_fields: Result<Option<Vec<String>>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  forbid_closed_indices: Result<Option<bool>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  include_unloaded_segments: Result<Option<bool>, String>,
  level: Result<Option<types::IndiciesStatLevel>, String>,
}
impl<'a> IndicesStatsWithIndexMetric<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      metric: Err("metric was not initialized".to_string()),
      completion_fields: Ok(None),
      expand_wildcards: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      forbid_closed_indices: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      include_unloaded_segments: Ok(None),
      level: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesStatsWithIndexMetricIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesStatsWithIndexMetricIndex` for index failed".to_string());
    self
  }

  pub fn metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesStatsWithIndexMetricMetric>, {
    self.metric = value
      .try_into()
      .map_err(|_| "conversion to `IndicesStatsWithIndexMetricMetric` for metric failed".to_string());
    self
  }

  pub fn completion_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.completion_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for completion_fields failed".to_string());
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

  pub fn fielddata_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.fielddata_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for fielddata_fields failed".to_string());
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

  pub fn forbid_closed_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.forbid_closed_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for forbid_closed_indices failed".to_string());
    self
  }

  pub fn groups<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.groups = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for groups failed".to_string());
    self
  }

  pub fn include_segment_file_sizes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_segment_file_sizes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_segment_file_sizes failed".to_string());
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndiciesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IndiciesStatLevel` for level failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_stats/{metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      metric,
      completion_fields,
      expand_wildcards,
      fielddata_fields,
      fields,
      forbid_closed_indices,
      groups,
      include_segment_file_sizes,
      include_unloaded_segments,
      level,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let forbid_closed_indices = forbid_closed_indices.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let include_unloaded_segments = include_unloaded_segments.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/{}/_stats/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&metric.to_string()),
    );
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &forbid_closed_indices {
      query.push(("forbid_closed_indices", v.to_string()));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &include_unloaded_segments {
      query.push(("include_unloaded_segments", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
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
///Builder for [`Client::indices_get_upgrade_with_index`]
///
///[`Client::indices_get_upgrade_with_index`]: super::OsClient::indices_get_upgrade_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetUpgradeWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesGetUpgradeWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
}
impl<'a> IndicesGetUpgradeWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesGetUpgradeWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesGetUpgradeWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_upgrade`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_upgrade", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
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
///Builder for [`Client::indices_upgrade_with_index`]
///
///[`Client::indices_upgrade_with_index`]: super::OsClient::indices_upgrade_with_index
#[derive(Debug, Clone)]
pub struct IndicesUpgradeWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesUpgradeWithIndexIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  only_ancient_segments: Result<Option<bool>, String>,
  wait_for_completion: Result<Option<bool>, String>,
}
impl<'a> IndicesUpgradeWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      only_ancient_segments: Ok(None),
      wait_for_completion: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesUpgradeWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesUpgradeWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
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

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn only_ancient_segments<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.only_ancient_segments = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for only_ancient_segments failed".to_string());
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

  ///Sends a `POST` request to `/{index}/_upgrade`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      only_ancient_segments,
      wait_for_completion,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let only_ancient_segments = only_ancient_segments.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_upgrade", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &only_ancient_segments {
      query.push(("only_ancient_segments", v.to_string()));
    }
    if let Some(v) = &wait_for_completion {
      query.push(("wait_for_completion", v.to_string()));
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
///Builder for [`Client::indices_validate_query_get_with_index`]
///
///[`Client::indices_validate_query_get_with_index`]: super::OsClient::indices_validate_query_get_with_index
#[derive(Debug, Clone)]
pub struct IndicesValidateQueryGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesValidateQueryGetWithIndexIndex, String>,
  all_shards: Result<Option<bool>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  default_operator: Result<Option<DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  explain: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  lenient: Result<Option<bool>, String>,
  q: Result<Option<String>, String>,
  rewrite: Result<Option<bool>, String>,
}
impl<'a> IndicesValidateQueryGetWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      all_shards: Ok(None),
      allow_no_indices: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      expand_wildcards: Ok(None),
      explain: Ok(None),
      ignore_unavailable: Ok(None),
      lenient: Ok(None),
      q: Ok(None),
      rewrite: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesValidateQueryGetWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesValidateQueryGetWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn all_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.all_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for all_shards failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
    self
  }

  pub fn analyze_wildcard<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.analyze_wildcard = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for analyze_wildcard failed".to_string());
    self
  }

  pub fn analyzer<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.analyzer = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for analyzer failed".to_string());
    self
  }

  pub fn default_operator<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<DefaultOperator>, {
    self.default_operator = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `DefaultOperator` for default_operator failed".to_string());
    self
  }

  pub fn df<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.df = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for df failed".to_string());
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

  pub fn explain<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.explain = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for explain failed".to_string());
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

  pub fn lenient<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.lenient = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for lenient failed".to_string());
    self
  }

  pub fn q<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.q = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for q failed".to_string());
    self
  }

  pub fn rewrite<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rewrite = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rewrite failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_validate/query`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      all_shards,
      allow_no_indices,
      analyze_wildcard,
      analyzer,
      default_operator,
      df,
      expand_wildcards,
      explain,
      ignore_unavailable,
      lenient,
      q,
      rewrite,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let all_shards = all_shards.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let rewrite = rewrite.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_validate/query", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &all_shards {
      query.push(("all_shards", v.to_string()));
    }
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &analyze_wildcard {
      query.push(("analyze_wildcard", v.to_string()));
    }
    if let Some(v) = &analyzer {
      query.push(("analyzer", v.to_string()));
    }
    if let Some(v) = &default_operator {
      query.push(("default_operator", v.to_string()));
    }
    if let Some(v) = &df {
      query.push(("df", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &rewrite {
      query.push(("rewrite", v.to_string()));
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
///Builder for [`Client::indices_validate_query_post_with_index`]
///
///[`Client::indices_validate_query_post_with_index`]: super::OsClient::indices_validate_query_post_with_index
#[derive(Debug, Clone)]
pub struct IndicesValidateQueryPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndicesValidateQueryPostWithIndexIndex, String>,
  all_shards: Result<Option<bool>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  default_operator: Result<Option<DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  explain: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  lenient: Result<Option<bool>, String>,
  q: Result<Option<String>, String>,
  rewrite: Result<Option<bool>, String>,
  body: Result<types::IndicesValidateQueryBodyParams, String>,
}
impl<'a> IndicesValidateQueryPostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      all_shards: Ok(None),
      allow_no_indices: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      expand_wildcards: Ok(None),
      explain: Ok(None),
      ignore_unavailable: Ok(None),
      lenient: Ok(None),
      q: Ok(None),
      rewrite: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesValidateQueryPostWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndicesValidateQueryPostWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn all_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.all_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for all_shards failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
    self
  }

  pub fn analyze_wildcard<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.analyze_wildcard = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for analyze_wildcard failed".to_string());
    self
  }

  pub fn analyzer<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.analyzer = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for analyzer failed".to_string());
    self
  }

  pub fn default_operator<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<DefaultOperator>, {
    self.default_operator = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `DefaultOperator` for default_operator failed".to_string());
    self
  }

  pub fn df<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.df = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for df failed".to_string());
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

  pub fn explain<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.explain = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for explain failed".to_string());
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

  pub fn lenient<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.lenient = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for lenient failed".to_string());
    self
  }

  pub fn q<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.q = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for q failed".to_string());
    self
  }

  pub fn rewrite<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rewrite = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rewrite failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndicesValidateQueryBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndicesValidateQueryBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_validate/query`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      all_shards,
      allow_no_indices,
      analyze_wildcard,
      analyzer,
      default_operator,
      df,
      expand_wildcards,
      explain,
      ignore_unavailable,
      lenient,
      q,
      rewrite,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let all_shards = all_shards.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let rewrite = rewrite.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_validate/query", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &all_shards {
      query.push(("all_shards", v.to_string()));
    }
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &analyze_wildcard {
      query.push(("analyze_wildcard", v.to_string()));
    }
    if let Some(v) = &analyzer {
      query.push(("analyzer", v.to_string()));
    }
    if let Some(v) = &default_operator {
      query.push(("default_operator", v.to_string()));
    }
    if let Some(v) = &df {
      query.push(("df", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &rewrite {
      query.push(("rewrite", v.to_string()));
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
