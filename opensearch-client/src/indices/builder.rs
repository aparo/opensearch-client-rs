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
///Builder for [`Client::Indices::get_alias_with_name`]
///
///[`Client::Indices::get_alias_with_name`]: super::OsClient::Indices::get_alias_with_name
#[derive(Debug, Clone)]
pub struct IndicesGetAlias<'a> {
  client: &'a super::OsClient,
  name: Result<Option<OpenSearchNameValue>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
}
impl<'a> IndicesGetAlias<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Ok(None),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Option<OpenSearchNameValue>>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
  pub async fn send(self) -> Result<ResponseValue<HashMap<String, types::AliasDefinition>>, Error> {
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
    let url = match name {
      Some(ref name) => {
        format!("{}_alias/{}", client.baseurl, encode_path(&name.to_string()),)
      }
      None => format!("{}_alias", client.baseurl,),
    };
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
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Indices::exists_alias`]
///
///[`Client::Indices::exists_alias`]: super::OsClient::Indices::exists_alias
#[derive(Debug, Clone)]
pub struct IndicesExistsAlias<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
    let url = format!("{}_alias/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::update_aliases`]
///
///[`Client::Indices::update_aliases`]: super::OsClient::Indices::update_aliases
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
    let url = format!("{}_aliases", client.baseurl,);
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
///Builder for [`Client::Indices::analyze_get`]
///
///[`Client::Indices::analyze_get`]: super::OsClient::Indices::analyze_get
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
    let url = format!("{}_analyze", client.baseurl,);
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
///Builder for [`Client::Indices::analyze_post`]
///
///[`Client::Indices::analyze_post`]: super::OsClient::Indices::analyze_post
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
    let url = format!("{}_analyze", client.baseurl,);
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
///Builder for [`Client::Indices::clear_cache`]
///
///[`Client::Indices::clear_cache`]: super::OsClient::Indices::clear_cache
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
    let url = format!("{}_cache/clear", client.baseurl,);
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
///Builder for [`Client::Indices::get_data_stream`]
///
///[`Client::Indices::get_data_stream`]: super::OsClient::Indices::get_data_stream
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
    let url = format!("{}_data_stream", client.baseurl,);
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
///Builder for [`Client::Indices::data_streams_stats`]
///
///[`Client::Indices::data_streams_stats`]: super::OsClient::Indices::data_streams_stats
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
    let url = format!("{}_data_stream/_stats", client.baseurl,);
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
///Builder for [`Client::Indices::get_data_stream_with_name`]
///
///[`Client::Indices::get_data_stream_with_name`]: super::OsClient::Indices::get_data_stream_with_name
#[derive(Debug, Clone)]
pub struct IndicesGetDataStreamWithName<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_data_stream/{name}`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesGetDataStreamWithNameResponseContent>, Error> {
    let Self { client, name } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let url = format!("{}_data_stream/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::create_data_stream`]
///
///[`Client::Indices::create_data_stream`]: super::OsClient::Indices::create_data_stream
#[derive(Debug, Clone)]
pub struct IndicesCreateDataStream<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
    let url = format!("{}_data_stream/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::delete_data_stream`]
///
///[`Client::Indices::delete_data_stream`]: super::OsClient::Indices::delete_data_stream
#[derive(Debug, Clone)]
pub struct IndicesDeleteDataStream<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_data_stream/{name}`
  pub async fn send(self) -> Result<ResponseValue<types::IndicesDeleteDataStreamResponseContent>, Error> {
    let Self { client, name } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let url = format!("{}_data_stream/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::data_streams_stats_with_name`]
///
///[`Client::Indices::data_streams_stats_with_name`]: super::OsClient::Indices::data_streams_stats_with_name
#[derive(Debug, Clone)]
pub struct IndicesDataStreamsStatsWithName<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_data_stream/{name}/_stats`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, name } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_data_stream/{}/_stats",
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
///Builder for [`Client::Indices::flush_get`]
///
///[`Client::Indices::flush_get`]: super::OsClient::Indices::flush_get
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
    let url = format!("{}_flush", client.baseurl,);
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
///Builder for [`Client::Indices::flush_post`]
///
///[`Client::Indices::flush_post`]: super::OsClient::Indices::flush_post
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
    let url = format!("{}_flush", client.baseurl,);
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
///Builder for [`Client::Indices::forcemerge`]
///
///[`Client::Indices::forcemerge`]: super::OsClient::Indices::forcemerge
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
    let url = format!("{}_forcemerge", client.baseurl,);
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
///Builder for [`Client::Indices::simulate_template`]
///
///[`Client::Indices::simulate_template`]: super::OsClient::Indices::simulate_template
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
    let url = format!("{}_index_template/_simulate", client.baseurl,);
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
///Builder for [`Client::Indices::simulate_template_with_name`]
///
///[`Client::Indices::simulate_template_with_name`]: super::OsClient::Indices::simulate_template_with_name
#[derive(Debug, Clone)]
pub struct IndicesSimulateTemplateWithName<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
      "{}_index_template/_simulate/{}",
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
///Builder for [`Client::Indices::simulate_index_template`]
///
///[`Client::Indices::simulate_index_template`]: super::OsClient::Indices::simulate_index_template
#[derive(Debug, Clone)]
pub struct IndicesSimulateIndexTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
      "{}_index_template/_simulate_index/{}",
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
///Builder for [`Client::Indices::get_index_template_with_name`]
///
///[`Client::Indices::get_index_template_with_name`]: super::OsClient::Indices::get_index_template_with_name
#[derive(Debug, Clone)]
pub struct IndicesGetIndexTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  flat_settings: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetIndexTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Ok(None),
      cluster_manager_timeout: Ok(None),
      flat_settings: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Option<String>>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `String` for name failed".to_string());
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
  pub async fn send(self) -> Result<ResponseValue<HashMap<String, serde_json::Value>>, Error> {
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
    let url = match name {
      Some(ref name) => {
        format!("{}_index_template/{}", client.baseurl, encode_path(&name.to_string()),)
      }
      None => format!("{}_index_template", client.baseurl,),
    };
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
      200u16 => {
        let json: ResponseValue<HashMap<String, Value>> = ResponseValue::from_response(response).await?;
        decode_index_templates(json)
      }
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

fn decode_index_templates(
  data: ResponseValue<HashMap<String, Value>>,
) -> Result<ResponseValue<HashMap<String, serde_json::Value>>, Error> {
  if !data.contains_key("index_templates") {
    return Err(Error::InvalidResponse("Missing value: index_templates".to_string()));
  }
  let mut result: HashMap<String, Value> = HashMap::new();
  for value in data["index_templates"].as_array().unwrap().iter() {
    let k = value["name"].as_str().unwrap().to_string();
    let v = value["index_template"].clone();
    result.insert(k.clone(), v.clone());
  }
  Ok(ResponseValue::new(result, data.status(), data.headers().clone()))
}
///Builder for [`Client::Indices::put_index_template`]
///
///[`Client::Indices::put_index_template`]: super::OsClient::Indices::put_index_template
#[derive(Debug, Clone)]
pub struct IndicesPutIndexTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
  cause: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  body: Result<types::IndicesPutIndexTemplateBodyParams, String>,
}
impl<'a> IndicesPutIndexTemplate<'a> {
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
    let url = format!("{}_index_template/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::delete_index_template`]
///
///[`Client::Indices::delete_index_template`]: super::OsClient::Indices::delete_index_template
#[derive(Debug, Clone)]
pub struct IndicesDeleteIndexTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    let url = format!("{}_index_template/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::exists_index_template`]
///
///[`Client::Indices::exists_index_template`]: super::OsClient::Indices::exists_index_template
#[derive(Debug, Clone)]
pub struct IndicesExistsIndexTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
    let url = format!("{}_index_template/{}", client.baseurl, encode_path(&name.to_string()),);
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

///Builder for [`Client::Indices::get_component_template`]
///
///[`Client::Indices::get_component_template`]: super::OsClient::Indices::get_component_template
#[derive(Debug, Clone)]
pub struct IndicesGetComponentTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesGetComponentTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Ok(None),
      cluster_manager_timeout: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Option<String>>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `String` for name failed".to_string());
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

  ///Sends a `GET` request to `/_component_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<HashMap<String, serde_json::Value>>, Error> {
    let Self {
      client,
      name,
      cluster_manager_timeout,
      local,
      master_timeout,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = match name {
      Some(ref name) => {
        format!(
          "{}_component_template/{}",
          client.baseurl,
          encode_path(&name.to_string()),
        )
      }
      None => format!("{}_component_template", client.baseurl,),
    };
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
      200u16 => {
        let json: ResponseValue<HashMap<String, Value>> = ResponseValue::from_response(response).await?;
        decode_index_components(json)
      }
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

fn decode_index_components(
  data: ResponseValue<HashMap<String, Value>>,
) -> Result<ResponseValue<HashMap<String, serde_json::Value>>, Error> {
  if !data.contains_key("component_templates") {
    return Err(Error::InvalidResponse("Missing value: component_templates".to_string()));
  }
  let mut result: HashMap<String, Value> = HashMap::new();
  for value in data["component_templates"].as_array().unwrap().iter() {
    let k = value["name"].as_str().unwrap().to_string();
    let v = value["component_template"].clone();
    result.insert(k.clone(), v.clone());
  }
  Ok(ResponseValue::new(result, data.status(), data.headers().clone()))
}

///Builder for [`Client::Indices::put_component_template`]
///
///[`Client::Indices::put_component_template`]: super::OsClient::Indices::put_component_template
#[derive(Debug, Clone)]
pub struct IndicesPutComponentTemplate<'a, T: Serialize> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<T, String>,
}
impl<'a, T: Serialize> IndicesPutComponentTemplate<'a, T> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      create: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
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
    V: std::convert::TryInto<T>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `Json` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_component_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      cluster_manager_timeout,
      create,
      master_timeout,
      timeout,
      body,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let create = create.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_component_template/{}",
      client.baseurl,
      encode_path(&name.to_string()),
    );
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
///Builder for [`Client::Indices::delete_component_template`]
///
///[`Client::Indices::delete_component_template`]: super::OsClient::Indices::delete_component_template
#[derive(Debug, Clone)]
pub struct IndicesDeleteComponentTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesDeleteComponentTemplate<'a> {
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

  ///Sends a `DELETE` request to `/_component_template/{name}`
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
    let url = format!(
      "{}_component_template/{}",
      client.baseurl,
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
///Builder for [`Client::Indices::exists_component_template`]
///
///[`Client::Indices::exists_component_template`]: super::OsClient::Indices::exists_component_template
#[derive(Debug, Clone)]
pub struct IndicesExistsComponentTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> IndicesExistsComponentTemplate<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      local: Ok(None),
      master_timeout: Ok(None),
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

  ///Sends a `HEAD` request to `/_component_template/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      local,
      master_timeout,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_component_template/{}",
      client.baseurl,
      encode_path(&name.to_string()),
    );
    let mut query = Vec::with_capacity(2usize);
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

///Builder for [`Client::Indices::get_mapping`]
///
///[`Client::Indices::get_mapping`]: super::OsClient::Indices::get_mapping
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
    let url = format!("{}_mapping", client.baseurl,);
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
///Builder for [`Client::Indices::get_field_mapping`]
///
///[`Client::Indices::get_field_mapping`]: super::OsClient::Indices::get_field_mapping
#[derive(Debug, Clone)]
pub struct IndicesGetFieldMapping<'a> {
  client: &'a super::OsClient,
  fields: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.fields = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for fields failed".to_string());
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
    let url = format!("{}_mapping/field/{}", client.baseurl, encode_path(&fields.to_string()),);
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
///Builder for [`Client::Indices::recovery`]
///
///[`Client::Indices::recovery`]: super::OsClient::Indices::recovery
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
    let url = format!("{}_recovery", client.baseurl,);
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
///Builder for [`Client::Indices::refresh_post`]
///
///[`Client::Indices::refresh_post`]: super::OsClient::Indices::refresh_post
#[derive(Debug, Clone)]
pub struct IndicesRefresh<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
}
impl<'a> IndicesRefresh<'a> {
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
    let url = format!("{}_refresh", client.baseurl,);
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
///Builder for [`Client::Indices::resolve_index`]
///
///[`Client::Indices::resolve_index`]: super::OsClient::Indices::resolve_index
#[derive(Debug, Clone)]
pub struct IndicesResolveIndex<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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

  ///Sends a `GET` request to `/_resolve/index/{name}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      name,
      expand_wildcards,
    } = self;
    let name = name.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let url = format!("{}_resolve/index/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::segments`]
///
///[`Client::Indices::segments`]: super::OsClient::Indices::segments
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
    let url = format!("{}_segments", client.baseurl,);
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
///Builder for [`Client::Indices::get_settings`]
///
///[`Client::Indices::get_settings`]: super::OsClient::Indices::get_settings
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
    let url = format!("{}_settings", client.baseurl,);
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
///Builder for [`Client::Indices::put_settings`]
///
///[`Client::Indices::put_settings`]: super::OsClient::Indices::put_settings
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
    let url = format!("{}_settings", client.baseurl,);
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
///Builder for [`Client::Indices::get_settings_with_name`]
///
///[`Client::Indices::get_settings_with_name`]: super::OsClient::Indices::get_settings_with_name
#[derive(Debug, Clone)]
pub struct IndicesGetSettingsWithName<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
    let url = format!("{}_settings/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::shard_stores`]
///
///[`Client::Indices::shard_stores`]: super::OsClient::Indices::shard_stores
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
    let url = format!("{}_shard_stores", client.baseurl,);
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
///Builder for [`Client::Indices::stats`]
///
///[`Client::Indices::stats`]: super::OsClient::Indices::stats
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
    let url = format!("{}_stats", client.baseurl,);
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
///Builder for [`Client::Indices::stats_with_metric`]
///
///[`Client::Indices::stats_with_metric`]: super::OsClient::Indices::stats_with_metric
#[derive(Debug, Clone)]
pub struct IndicesStatsWithMetric<'a> {
  client: &'a super::OsClient,
  metric: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.metric = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for metric failed".to_string());
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
    let url = format!("{}_stats/{}", client.baseurl, encode_path(&metric.to_string()),);
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
///Builder for [`Client::Indices::get_templates`]
///
///[`Client::Indices::get_templates`]: super::OsClient::Indices::get_templates
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
    let url = format!("{}_template", client.baseurl,);
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
///Builder for [`Client::Indices::get_template`]
///
///[`Client::Indices::get_template`]: super::OsClient::Indices::get_template
#[derive(Debug, Clone)]
pub struct IndicesGetTemplateWithName<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    let url = format!("{}_template/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::put_template`]
///
///[`Client::Indices::put_template`]: super::OsClient::Indices::put_template
#[derive(Debug, Clone)]
pub struct IndicesPutTemplate<'a, T: Serialize> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  order: Result<Option<i32>, String>,
  body: Result<T, String>,
}
impl<'a, T: Serialize> IndicesPutTemplate<'a, T> {
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
    V: std::convert::TryInto<T>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `Json` for body failed".to_string());
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
    let url = format!("{}_template/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::delete_template`]
///
///[`Client::Indices::delete_template`]: super::OsClient::Indices::delete_template
#[derive(Debug, Clone)]
pub struct IndicesDeleteTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    let url = format!("{}_template/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::exists_template`]
///
///[`Client::Indices::exists_template`]: super::OsClient::Indices::exists_template
#[derive(Debug, Clone)]
pub struct IndicesExistsTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
    let url = format!("{}_template/{}", client.baseurl, encode_path(&name.to_string()),);
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
///Builder for [`Client::Indices::validate_query`]
///
///[`Client::Indices::validate_query`]: super::OsClient::Indices::validate_query
#[derive(Debug, Clone)]
pub struct IndicesValidateQuery<'a> {
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
impl<'a> IndicesValidateQuery<'a> {
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
    let url = format!("{}_validate/query", client.baseurl,);
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
///Builder for [`Client::Indices::rollover`]
///
///[`Client::Indices::rollover`]: super::OsClient::Indices::rollover
#[derive(Debug, Clone)]
pub struct IndicesRollover<'a> {
  client: &'a super::OsClient,
  alias: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.alias = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for alias failed".to_string());
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
    let url = format!("{}{}/_rollover", client.baseurl, encode_path(&alias.to_string()),);
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
///Builder for [`Client::Indices::rollover_with_new_index`]
///
///[`Client::Indices::rollover_with_new_index`]: super::OsClient::Indices::rollover_with_new_index
#[derive(Debug, Clone)]
pub struct IndicesRolloverWithNewIndex<'a> {
  client: &'a super::OsClient,
  alias: Result<OpenSearchNameValue, String>,
  new_index: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.alias = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for alias failed".to_string());
    self
  }

  pub fn new_index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.new_index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for new_index failed".to_string());
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
      "{}{}/_rollover/{}",
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
///Builder for [`Client::Indices::get`]
///
///[`Client::Indices::get`]: super::OsClient::Indices::get
#[derive(Debug, Clone)]
pub struct IndicesGet<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
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
    let url = format!("{}{}", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::create`]
///
///[`Client::Indices::create`]: super::OsClient::Indices::create
#[derive(Debug, Clone)]
pub struct IndicesCreate<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
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
    let url = format!("{}{}", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::delete`]
///
///[`Client::Indices::delete`]: super::OsClient::Indices::delete
#[derive(Debug, Clone)]
pub struct IndicesDelete<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
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
    let url = format!("{}{}", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::exists`]
///
///[`Client::Indices::exists`]: super::OsClient::Indices::exists
#[derive(Debug, Clone)]
pub struct IndicesExists<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
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
    let url = format!("{}{}", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::get_alias_with_index`]
///
///[`Client::Indices::get_alias_with_index`]: super::OsClient::Indices::get_alias_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetAliasWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
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
    let url = format!("{}{}/_alias", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::get_alias_with_index_name`]
///
///[`Client::Indices::get_alias_with_index_name`]: super::OsClient::Indices::get_alias_with_index_name
#[derive(Debug, Clone)]
pub struct IndicesGetAliasWithIndexName<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
      "{}{}/_alias/{}",
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
///Builder for [`Client::Indices::put_alias_put`]
///
///[`Client::Indices::put_alias_put`]: super::OsClient::Indices::put_alias_put
#[derive(Debug, Clone)]
pub struct IndicesPutAliasPut<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
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
      "{}{}/_alias/{}",
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
///Builder for [`Client::Indices::put_alias_post`]
///
///[`Client::Indices::put_alias_post`]: super::OsClient::Indices::put_alias_post
#[derive(Debug, Clone)]
pub struct IndicesPutAliasPost<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
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
      "{}{}/_alias/{}",
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
///Builder for [`Client::Indices::delete_alias`]
///
///[`Client::Indices::delete_alias`]: super::OsClient::Indices::delete_alias
#[derive(Debug, Clone)]
pub struct IndicesDeleteAlias<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
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
      "{}{}/_alias/{}",
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
///Builder for [`Client::Indices::exists_alias_with_index`]
///
///[`Client::Indices::exists_alias_with_index`]: super::OsClient::Indices::exists_alias_with_index
#[derive(Debug, Clone)]
pub struct IndicesExistsAliasWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
      "{}{}/_alias/{}",
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
///Builder for [`Client::Indices::put_alias_put_plural`]
///
///[`Client::Indices::put_alias_put_plural`]: super::OsClient::Indices::put_alias_put_plural
#[derive(Debug, Clone)]
pub struct IndicesPutAliasPutPlural<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
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
      "{}{}/_aliases/{}",
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
///Builder for [`Client::Indices::put_alias_post_plural`]
///
///[`Client::Indices::put_alias_post_plural`]: super::OsClient::Indices::put_alias_post_plural
#[derive(Debug, Clone)]
pub struct IndicesPutAliasPostPlural<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
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
      "{}{}/_aliases/{}",
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
///Builder for [`Client::Indices::delete_alias_plural`]
///
///[`Client::Indices::delete_alias_plural`]: super::OsClient::Indices::delete_alias_plural
#[derive(Debug, Clone)]
pub struct IndicesDeleteAliasPlural<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
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
      "{}{}/_aliases/{}",
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
///Builder for [`Client::Indices::analyze_get_with_index`]
///
///[`Client::Indices::analyze_get_with_index`]: super::OsClient::Indices::analyze_get_with_index
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
      "{}{}/_analyze",
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
///Builder for [`Client::Indices::analyze_post_with_index`]
///
///[`Client::Indices::analyze_post_with_index`]: super::OsClient::Indices::analyze_post_with_index
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
    let url = format!("{}_analyze", client.baseurl);
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
///Builder for [`Client::Indices::add_block`]
///
///[`Client::Indices::add_block`]: super::OsClient::Indices::add_block
#[derive(Debug, Clone)]
pub struct IndicesAddBlock<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  block: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn block<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.block = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for block failed".to_string());
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
      "{}{}/_block/{}",
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
///Builder for [`Client::Indices::clear_cache_with_index`]
///
///[`Client::Indices::clear_cache_with_index`]: super::OsClient::Indices::clear_cache_with_index
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
          "{}{}/_cache/clear",
          client.baseurl,
          encode_path(&index.clone().unwrap().join(",")),
        )
      }
      None => format!("{}_cache/clear", client.baseurl),
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
///Builder for [`Client::Indices::clone_put`]
///
///[`Client::Indices::clone_put`]: super::OsClient::Indices::clone_put
#[derive(Debug, Clone)]
pub struct IndicesClonePut<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  target: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for target failed".to_string());
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
      "{}{}/_clone/{}",
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
///Builder for [`Client::Indices::clone_post`]
///
///[`Client::Indices::clone_post`]: super::OsClient::Indices::clone_post
#[derive(Debug, Clone)]
pub struct IndicesClonePost<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  target: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for target failed".to_string());
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
      "{}{}/_clone/{}",
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
///Builder for [`Client::Indices::close`]
///
///[`Client::Indices::close`]: super::OsClient::Indices::close
#[derive(Debug, Clone)]
pub struct IndicesClose<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
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
    let url = format!("{}{}/_close", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::flush_get_with_index`]
///
///[`Client::Indices::flush_get_with_index`]: super::OsClient::Indices::flush_get_with_index
#[derive(Debug, Clone)]
pub struct IndicesFlushGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_flush", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::flush_post_with_index`]
///
///[`Client::Indices::flush_post_with_index`]: super::OsClient::Indices::flush_post_with_index
#[derive(Debug, Clone)]
pub struct IndicesFlushPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_flush", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::forcemerge_with_index`]
///
///[`Client::Indices::forcemerge_with_index`]: super::OsClient::Indices::forcemerge_with_index
#[derive(Debug, Clone)]
pub struct IndicesForcemergeWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_forcemerge", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::get_mapping_with_index`]
///
///[`Client::Indices::get_mapping_with_index`]: super::OsClient::Indices::get_mapping_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetMappingWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
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
    let url = format!("{}{}/_mapping", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::put_mapping_put`]
///
///[`Client::Indices::put_mapping_put`]: super::OsClient::Indices::put_mapping_put
#[derive(Debug, Clone)]
pub struct IndicesPutMappingPut<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_mapping", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::put_mapping_post`]
///
///[`Client::Indices::put_mapping_post`]: super::OsClient::Indices::put_mapping_post
#[derive(Debug, Clone)]
pub struct IndicesPutMappingPost<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_mapping", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::get_field_mapping_with_index`]
///
///[`Client::Indices::get_field_mapping_with_index`]: super::OsClient::Indices::get_field_mapping_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetFieldMappingWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  fields: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.fields = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for fields failed".to_string());
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
      "{}{}/_mapping/field/{}",
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
///Builder for [`Client::Indices::open`]
///
///[`Client::Indices::open`]: super::OsClient::Indices::open
#[derive(Debug, Clone)]
pub struct IndicesOpen<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
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
    let url = format!("{}{}/_open", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::recovery_with_index`]
///
///[`Client::Indices::recovery_with_index`]: super::OsClient::Indices::recovery_with_index
#[derive(Debug, Clone)]
pub struct IndicesRecoveryWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_recovery", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::refresh_get_with_index`]
///
///[`Client::Indices::refresh_get_with_index`]: super::OsClient::Indices::refresh_get_with_index
#[derive(Debug, Clone)]
pub struct IndicesRefreshGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_refresh", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::refresh_post_with_index`]
///
///[`Client::Indices::refresh_post_with_index`]: super::OsClient::Indices::refresh_post_with_index
#[derive(Debug, Clone)]
pub struct IndicesRefreshPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_refresh", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::segments_with_index`]
///
///[`Client::Indices::segments_with_index`]: super::OsClient::Indices::segments_with_index
#[derive(Debug, Clone)]
pub struct IndicesSegmentsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_segments", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::get_settings_with_index`]
///
///[`Client::Indices::get_settings_with_index`]: super::OsClient::Indices::get_settings_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetSettingsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_settings", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::put_settings_with_index`]
///
///[`Client::Indices::put_settings_with_index`]: super::OsClient::Indices::put_settings_with_index
#[derive(Debug, Clone)]
pub struct IndicesPutSettingsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_settings", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::get_settings_with_index_name`]
///
///[`Client::Indices::get_settings_with_index_name`]: super::OsClient::Indices::get_settings_with_index_name
#[derive(Debug, Clone)]
pub struct IndicesGetSettingsWithIndexName<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
  name: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for name failed".to_string());
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
      "{}{}/_settings/{}",
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
///Builder for [`Client::Indices::shard_stores_with_index`]
///
///[`Client::Indices::shard_stores_with_index`]: super::OsClient::Indices::shard_stores_with_index
#[derive(Debug, Clone)]
pub struct IndicesShardStoresWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_shard_stores", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::shrink_put`]
///
///[`Client::Indices::shrink_put`]: super::OsClient::Indices::shrink_put
#[derive(Debug, Clone)]
pub struct IndicesShrinkPut<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  target: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for target failed".to_string());
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
      "{}{}/_shrink/{}",
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
///Builder for [`Client::Indices::shrink_post`]
///
///[`Client::Indices::shrink_post`]: super::OsClient::Indices::shrink_post
#[derive(Debug, Clone)]
pub struct IndicesShrinkPost<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  target: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for target failed".to_string());
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
      "{}{}/_shrink/{}",
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
///Builder for [`Client::Indices::split_put`]
///
///[`Client::Indices::split_put`]: super::OsClient::Indices::split_put
#[derive(Debug, Clone)]
pub struct IndicesSplitPut<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  target: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for target failed".to_string());
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
      "{}{}/_split/{}",
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
///Builder for [`Client::Indices::split_post`]
///
///[`Client::Indices::split_post`]: super::OsClient::Indices::split_post
#[derive(Debug, Clone)]
pub struct IndicesSplitPost<'a> {
  client: &'a super::OsClient,
  index: Result<OpenSearchNameValue, String>,
  target: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn target<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.target = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for target failed".to_string());
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
      "{}{}/_split/{}",
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
///Builder for [`Client::Indices::stats_with_index`]
///
///[`Client::Indices::stats_with_index`]: super::OsClient::Indices::stats_with_index
#[derive(Debug, Clone)]
pub struct IndicesStatsWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_stats", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::stats_with_index_metric`]
///
///[`Client::Indices::stats_with_index_metric`]: super::OsClient::Indices::stats_with_index_metric
#[derive(Debug, Clone)]
pub struct IndicesStatsWithIndexMetric<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
  metric: Result<OpenSearchNameValue, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
  }

  pub fn metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.metric = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for metric failed".to_string());
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
      "{}{}/_stats/{}",
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
///Builder for [`Client::Indices::get_upgrade_with_index`]
///
///[`Client::Indices::get_upgrade_with_index`]: super::OsClient::Indices::get_upgrade_with_index
#[derive(Debug, Clone)]
pub struct IndicesGetUpgradeWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_upgrade", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::upgrade_with_index`]
///
///[`Client::Indices::upgrade_with_index`]: super::OsClient::Indices::upgrade_with_index
#[derive(Debug, Clone)]
pub struct IndicesUpgradeWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_upgrade", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::validate_query_get_with_index`]
///
///[`Client::Indices::validate_query_get_with_index`]: super::OsClient::Indices::validate_query_get_with_index
#[derive(Debug, Clone)]
pub struct IndicesValidateQueryGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_validate/query", client.baseurl, encode_path(&index.to_string()),);
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
///Builder for [`Client::Indices::validate_query_post_with_index`]
///
///[`Client::Indices::validate_query_post_with_index`]: super::OsClient::Indices::validate_query_post_with_index
#[derive(Debug, Clone)]
pub struct IndicesValidateQueryPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<IndexNames, String>,
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
    V: std::convert::TryInto<IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
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
    let url = format!("{}{}/_validate/query", client.baseurl, encode_path(&index.to_string()),);
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
