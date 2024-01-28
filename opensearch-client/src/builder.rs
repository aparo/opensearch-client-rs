use reqwest::Body;
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "search")]
pub use crate::builder_search::*;
use crate::types::bulk::BulkResponse;
use super::types;
#[allow(unused_imports)]
use super::{
  encode_path, encode_path_option_vec_string, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
  ReqwestResponse, ResponseValue,
};
///Builder for [`Client::info`]
///
///[`Client::info`]: super::OsClient::info
#[derive(Debug, Clone)]
pub struct Info<'a> {
  client: &'a super::OsClient,
}

impl<'a> Info<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/`
  pub async fn send(self) -> Result<ResponseValue<types::InfoResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}", client.baseurl,);
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

///Builder for [`Client::ping`]
///
///[`Client::ping`]: super::OsClient::ping
#[derive(Debug, Clone)]
pub struct Ping<'a> {
  client: &'a super::OsClient,
}

impl<'a> Ping<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `HEAD` request to `/`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}", client.baseurl,);
    let request = client.client.head(url).build()?;
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

///Builder for [`Client::bulk_post`]
///
///[`Client::bulk_post`]: super::OsClient::bulk_post
#[derive(Debug, Clone)]
pub struct BulkPost<'a> {
  client: &'a super::OsClient,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  pipeline: Result<Option<String>, String>,
  refresh: Result<Option<types::RefreshEnum>, String>,
  require_alias: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  type_: Result<Option<String>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<String, String>,
}

impl<'a> BulkPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      source_excludes: Ok(None),
      source_includes: Ok(None),
      pipeline: Ok(None),
      refresh: Ok(None),
      require_alias: Ok(None),
      routing: Ok(None),
      timeout: Ok(None),
      type_: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn pipeline<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.pipeline = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for pipeline failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RefreshEnum>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `RefreshEnum` for refresh failed".to_string());
    self
  }

  pub fn require_alias<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.require_alias = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for require_alias failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn type_<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.type_ = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for type_ failed".to_string());
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
    V: std::convert::TryInto<String>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `BulkBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_bulk`
  pub async fn send(self) -> Result<ResponseValue<BulkResponse>, Error> {
    let Self {
      client,
      source_excludes,
      source_includes,
      pipeline,
      refresh,
      require_alias,
      routing,
      timeout,
      type_,
      wait_for_active_shards,
      body,
    } = self;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let pipeline = pipeline.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let require_alias = require_alias.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let type_ = type_.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = client.baseurl.join("_bulk").unwrap();
    let mut query = Vec::with_capacity(10usize);
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &pipeline {
      query.push(("pipeline", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &require_alias {
      query.push(("require_alias", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &type_ {
      query.push(("type", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
    }
    let request = client
      .client
      .post(url)
      .body(Body::from(body))
      .header("Content-Type", "application/x-ndjson")
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

///Builder for [`Client::dangling_indices_list_dangling_indices`]
///
///[`Client::dangling_indices_list_dangling_indices`]: super::OsClient::dangling_indices_list_dangling_indices
#[derive(Debug, Clone)]
pub struct DanglingIndicesListDanglingIndices<'a> {
  client: &'a super::OsClient,
}

impl<'a> DanglingIndicesListDanglingIndices<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_dangling`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}_dangling", client.baseurl,);
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

///Builder for [`Client::dangling_indices_import_dangling_index`]
///
///[`Client::dangling_indices_import_dangling_index`]: super::OsClient::dangling_indices_import_dangling_index
#[derive(Debug, Clone)]
pub struct DanglingIndicesImportDanglingIndex<'a> {
  client: &'a super::OsClient,
  index_uuid: Result<types::OpenSearchNameValue, String>,
  accept_data_loss: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<types::Timeout>, String>,
  master_timeout: Result<Option<types::Timeout>, String>,
  timeout: Result<Option<types::Timeout>, String>,
}

impl<'a> DanglingIndicesImportDanglingIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index_uuid: Err("index_uuid was not initialized".to_string()),
      accept_data_loss: Ok(None),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn index_uuid<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index_uuid = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index_uuid failed".to_string());
    self
  }

  pub fn accept_data_loss<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.accept_data_loss = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for accept_data_loss failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_dangling/{index_uuid}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index_uuid,
      accept_data_loss,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let index_uuid = index_uuid.map_err(Error::InvalidRequest)?;
    let accept_data_loss = accept_data_loss.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_dangling/{}", client.baseurl, encode_path(&index_uuid.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &accept_data_loss {
      query.push(("accept_data_loss", v.to_string()));
    }
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

///Builder for [`Client::dangling_indices_delete_dangling_index`]
///
///[`Client::dangling_indices_delete_dangling_index`]: super::OsClient::dangling_indices_delete_dangling_index
#[derive(Debug, Clone)]
pub struct DanglingIndicesDeleteDanglingIndex<'a> {
  client: &'a super::OsClient,
  index_uuid: Result<types::OpenSearchNameValue, String>,
  accept_data_loss: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<types::Timeout>, String>,
  master_timeout: Result<Option<types::Timeout>, String>,
  timeout: Result<Option<types::Timeout>, String>,
}

impl<'a> DanglingIndicesDeleteDanglingIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index_uuid: Err("index_uuid was not initialized".to_string()),
      accept_data_loss: Ok(None),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn index_uuid<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index_uuid = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index_uuid failed".to_string());
    self
  }

  pub fn accept_data_loss<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.accept_data_loss = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for accept_data_loss failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_dangling/{index_uuid}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index_uuid,
      accept_data_loss,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let index_uuid = index_uuid.map_err(Error::InvalidRequest)?;
    let accept_data_loss = accept_data_loss.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_dangling/{}", client.baseurl, encode_path(&index_uuid.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &accept_data_loss {
      query.push(("accept_data_loss", v.to_string()));
    }
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

///Builder for [`Client::delete_by_query_rethrottle`]
///
///[`Client::delete_by_query_rethrottle`]: super::OsClient::delete_by_query_rethrottle
#[derive(Debug, Clone)]
pub struct DeleteByQueryRethrottle<'a> {
  client: &'a super::OsClient,
  task_id: Result<types::OpenSearchId, String>,
  requests_per_second: Result<i32, String>,
}

impl<'a> DeleteByQueryRethrottle<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      task_id: Err("task_id was not initialized".to_string()),
      requests_per_second: Err("requests_per_second was not initialized".to_string()),
    }
  }

  pub fn task_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.task_id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for task_id failed".to_string());
    self
  }

  pub fn requests_per_second<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.requests_per_second = value
      .try_into()
      .map_err(|_| "conversion to `i32` for requests_per_second failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_delete_by_query/{task_id}/_rethrottle`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      task_id,
      requests_per_second,
    } = self;
    let task_id = task_id.map_err(Error::InvalidRequest)?;
    let requests_per_second = requests_per_second.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_delete_by_query/{}/_rethrottle",
      client.baseurl,
      encode_path(&task_id.to_string()),
    );
    let mut query = Vec::with_capacity(1usize);
    query.push(("requests_per_second", requests_per_second.to_string()));
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

///Builder for [`Client::field_caps_get`]
///
///[`Client::field_caps_get`]: super::OsClient::field_caps_get
#[derive(Debug, Clone)]
pub struct FieldCapsGet<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_unmapped: Result<Option<bool>, String>,
}

impl<'a> FieldCapsGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      fields: Ok(None),
      ignore_unavailable: Ok(None),
      include_unmapped: Ok(None),
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
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

  pub fn include_unmapped<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_unmapped = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_unmapped failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_field_caps`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      fields,
      ignore_unavailable,
      include_unmapped,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_unmapped = include_unmapped.map_err(Error::InvalidRequest)?;
    let url = format!("{}_field_caps", client.baseurl,);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_unmapped {
      query.push(("include_unmapped", v.to_string()));
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

///Builder for [`Client::field_caps_post`]
///
///[`Client::field_caps_post`]: super::OsClient::field_caps_post
#[derive(Debug, Clone)]
pub struct FieldCapsPost<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_unmapped: Result<Option<bool>, String>,
  body: Result<types::FieldCapsBodyParams, String>,
}

impl<'a> FieldCapsPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      fields: Ok(None),
      ignore_unavailable: Ok(None),
      include_unmapped: Ok(None),
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

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
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

  pub fn include_unmapped<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_unmapped = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_unmapped failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::FieldCapsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `FieldCapsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_field_caps`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      fields,
      ignore_unavailable,
      include_unmapped,
      body,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_unmapped = include_unmapped.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_field_caps", client.baseurl,);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_unmapped {
      query.push(("include_unmapped", v.to_string()));
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

///Builder for [`Client::mget_get`]
///
///[`Client::mget_get`]: super::OsClient::mget_get
#[derive(Debug, Clone)]
pub struct MgetGet<'a> {
  client: &'a super::OsClient,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  refresh: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  stored_fields: Result<Option<Vec<String>>, String>,
}

impl<'a> MgetGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      stored_fields: Ok(None),
    }
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn stored_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stored_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stored_fields failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_mget`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      source,
      source_excludes,
      source_includes,
      preference,
      realtime,
      refresh,
      routing,
      stored_fields,
    } = self;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let stored_fields = stored_fields.map_err(Error::InvalidRequest)?;
    let url = format!("{}_mget", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &stored_fields {
      query.push(("stored_fields", v.join(",")));
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

///Builder for [`Client::mget_post`]
///
///[`Client::mget_post`]: super::OsClient::mget_post
#[derive(Debug, Clone)]
pub struct MgetPost<'a> {
  client: &'a super::OsClient,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  refresh: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  stored_fields: Result<Option<Vec<String>>, String>,
  body: Result<types::MgetBodyParams, String>,
}

impl<'a> MgetPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      stored_fields: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn stored_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stored_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stored_fields failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::MgetBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `MgetBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_mget`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      source,
      source_excludes,
      source_includes,
      preference,
      realtime,
      refresh,
      routing,
      stored_fields,
      body,
    } = self;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let stored_fields = stored_fields.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_mget", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &stored_fields {
      query.push(("stored_fields", v.join(",")));
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

///Builder for [`Client::msearch_get`]
///
///[`Client::msearch_get`]: super::OsClient::msearch_get
#[derive(Debug, Clone)]
pub struct MsearchGet<'a> {
  client: &'a super::OsClient,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  max_concurrent_searches: Result<Option<i32>, String>,
  max_concurrent_shard_requests: Result<Option<i32>, String>,
  pre_filter_shard_size: Result<Option<i32>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  search_type: Result<Option<types::SearchTypeMulti>, String>,
  typed_keys: Result<Option<bool>, String>,
}

impl<'a> MsearchGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      ccs_minimize_roundtrips: Ok(None),
      max_concurrent_searches: Ok(None),
      max_concurrent_shard_requests: Ok(None),
      pre_filter_shard_size: Ok(None),
      rest_total_hits_as_int: Ok(None),
      search_type: Ok(None),
      typed_keys: Ok(None),
    }
  }

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn max_concurrent_searches<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_searches = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_searches failed".to_string());
    self
  }

  pub fn max_concurrent_shard_requests<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_shard_requests = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_shard_requests failed".to_string());
    self
  }

  pub fn pre_filter_shard_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.pre_filter_shard_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for pre_filter_shard_size failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTypeMulti>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTypeMulti` for search_type failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_msearch`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      ccs_minimize_roundtrips,
      max_concurrent_searches,
      max_concurrent_shard_requests,
      pre_filter_shard_size,
      rest_total_hits_as_int,
      search_type,
      typed_keys,
    } = self;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let max_concurrent_searches = max_concurrent_searches.map_err(Error::InvalidRequest)?;
    let max_concurrent_shard_requests = max_concurrent_shard_requests.map_err(Error::InvalidRequest)?;
    let pre_filter_shard_size = pre_filter_shard_size.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let url = format!("{}_msearch", client.baseurl,);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &max_concurrent_searches {
      query.push(("max_concurrent_searches", v.to_string()));
    }
    if let Some(v) = &max_concurrent_shard_requests {
      query.push(("max_concurrent_shard_requests", v.to_string()));
    }
    if let Some(v) = &pre_filter_shard_size {
      query.push(("pre_filter_shard_size", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
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

///Builder for [`Client::msearch_post`]
///
///[`Client::msearch_post`]: super::OsClient::msearch_post
#[derive(Debug, Clone)]
pub struct MsearchPost<'a> {
  client: &'a super::OsClient,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  max_concurrent_searches: Result<Option<i32>, String>,
  max_concurrent_shard_requests: Result<Option<i32>, String>,
  pre_filter_shard_size: Result<Option<i32>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  search_type: Result<Option<types::SearchTypeMulti>, String>,
  typed_keys: Result<Option<bool>, String>,
  body: Result<types::MsearchBodyParams, String>,
}

impl<'a> MsearchPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      ccs_minimize_roundtrips: Ok(None),
      max_concurrent_searches: Ok(None),
      max_concurrent_shard_requests: Ok(None),
      pre_filter_shard_size: Ok(None),
      rest_total_hits_as_int: Ok(None),
      search_type: Ok(None),
      typed_keys: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn max_concurrent_searches<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_searches = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_searches failed".to_string());
    self
  }

  pub fn max_concurrent_shard_requests<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_shard_requests = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_shard_requests failed".to_string());
    self
  }

  pub fn pre_filter_shard_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.pre_filter_shard_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for pre_filter_shard_size failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTypeMulti>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTypeMulti` for search_type failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::MsearchBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `MsearchBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_msearch`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      ccs_minimize_roundtrips,
      max_concurrent_searches,
      max_concurrent_shard_requests,
      pre_filter_shard_size,
      rest_total_hits_as_int,
      search_type,
      typed_keys,
      body,
    } = self;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let max_concurrent_searches = max_concurrent_searches.map_err(Error::InvalidRequest)?;
    let max_concurrent_shard_requests = max_concurrent_shard_requests.map_err(Error::InvalidRequest)?;
    let pre_filter_shard_size = pre_filter_shard_size.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_msearch", client.baseurl,);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &max_concurrent_searches {
      query.push(("max_concurrent_searches", v.to_string()));
    }
    if let Some(v) = &max_concurrent_shard_requests {
      query.push(("max_concurrent_shard_requests", v.to_string()));
    }
    if let Some(v) = &pre_filter_shard_size {
      query.push(("pre_filter_shard_size", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
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

///Builder for [`Client::msearch_template_get`]
///
///[`Client::msearch_template_get`]: super::OsClient::msearch_template_get
#[derive(Debug, Clone)]
pub struct MsearchTemplateGet<'a> {
  client: &'a super::OsClient,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  max_concurrent_searches: Result<Option<i32>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  search_type: Result<Option<types::SearchTypeMulti>, String>,
  typed_keys: Result<Option<bool>, String>,
}

impl<'a> MsearchTemplateGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      ccs_minimize_roundtrips: Ok(None),
      max_concurrent_searches: Ok(None),
      rest_total_hits_as_int: Ok(None),
      search_type: Ok(None),
      typed_keys: Ok(None),
    }
  }

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn max_concurrent_searches<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_searches = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_searches failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTypeMulti>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTypeMulti` for search_type failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_msearch/template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      ccs_minimize_roundtrips,
      max_concurrent_searches,
      rest_total_hits_as_int,
      search_type,
      typed_keys,
    } = self;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let max_concurrent_searches = max_concurrent_searches.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let url = format!("{}_msearch/template", client.baseurl,);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &max_concurrent_searches {
      query.push(("max_concurrent_searches", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
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

///Builder for [`Client::msearch_template_post`]
///
///[`Client::msearch_template_post`]: super::OsClient::msearch_template_post
#[derive(Debug, Clone)]
pub struct MsearchTemplatePost<'a> {
  client: &'a super::OsClient,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  max_concurrent_searches: Result<Option<i32>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  search_type: Result<Option<types::SearchTypeMulti>, String>,
  typed_keys: Result<Option<bool>, String>,
  body: Result<types::MsearchTemplateBodyParams, String>,
}

impl<'a> MsearchTemplatePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      ccs_minimize_roundtrips: Ok(None),
      max_concurrent_searches: Ok(None),
      rest_total_hits_as_int: Ok(None),
      search_type: Ok(None),
      typed_keys: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn max_concurrent_searches<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_searches = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_searches failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTypeMulti>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTypeMulti` for search_type failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::MsearchTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `MsearchTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_msearch/template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      ccs_minimize_roundtrips,
      max_concurrent_searches,
      rest_total_hits_as_int,
      search_type,
      typed_keys,
      body,
    } = self;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let max_concurrent_searches = max_concurrent_searches.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_msearch/template", client.baseurl,);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &max_concurrent_searches {
      query.push(("max_concurrent_searches", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
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

///Builder for [`Client::rank_eval_get`]
///
///[`Client::rank_eval_get`]: super::OsClient::rank_eval_get
#[derive(Debug, Clone)]
pub struct RankEvalGet<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  search_type: Result<Option<types::SearchType>, String>,
}

impl<'a> RankEvalGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      search_type: Ok(None),
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchType>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchType` for search_type failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_rank_eval`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      search_type,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let url = format!("{}_rank_eval", client.baseurl,);
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
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
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

///Builder for [`Client::rank_eval_post`]
///
///[`Client::rank_eval_post`]: super::OsClient::rank_eval_post
#[derive(Debug, Clone)]
pub struct RankEvalPost<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  search_type: Result<Option<types::SearchType>, String>,
  body: Result<types::RankEvalBodyParams, String>,
}

impl<'a> RankEvalPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      search_type: Ok(None),
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

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchType>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchType` for search_type failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RankEvalBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `RankEvalBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_rank_eval`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      search_type,
      body,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_rank_eval", client.baseurl,);
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
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
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

///Builder for [`Client::reindex`]
///
///[`Client::reindex`]: super::OsClient::reindex
#[derive(Debug, Clone)]
pub struct Reindex<'a> {
  client: &'a super::OsClient,
  max_docs: Result<Option<i32>, String>,
  refresh: Result<Option<bool>, String>,
  requests_per_second: Result<Option<i32>, String>,
  scroll: Result<Option<types::ReindexScroll>, String>,
  slices: Result<Option<String>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  wait_for_completion: Result<Option<bool>, String>,
  body: Result<types::ReindexBodyParams, String>,
}

impl<'a> Reindex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      max_docs: Ok(None),
      refresh: Ok(None),
      requests_per_second: Ok(None),
      scroll: Ok(None),
      slices: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      wait_for_completion: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn max_docs<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_docs = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_docs failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn requests_per_second<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.requests_per_second = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for requests_per_second failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ReindexScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ReindexScroll` for scroll failed".to_string());
    self
  }

  pub fn slices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.slices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for slices failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
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
    V: std::convert::TryInto<types::ReindexBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ReindexBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_reindex`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      max_docs,
      refresh,
      requests_per_second,
      scroll,
      slices,
      timeout,
      wait_for_active_shards,
      wait_for_completion,
      body,
    } = self;
    let max_docs = max_docs.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let requests_per_second = requests_per_second.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let slices = slices.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_reindex", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &max_docs {
      query.push(("max_docs", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &requests_per_second {
      query.push(("requests_per_second", v.to_string()));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &slices {
      query.push(("slices", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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

///Builder for [`Client::reindex_rethrottle`]
///
///[`Client::reindex_rethrottle`]: super::OsClient::reindex_rethrottle
#[derive(Debug, Clone)]
pub struct ReindexRethrottle<'a> {
  client: &'a super::OsClient,
  task_id: Result<types::OpenSearchId, String>,
  requests_per_second: Result<i32, String>,
}

impl<'a> ReindexRethrottle<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      task_id: Err("task_id was not initialized".to_string()),
      requests_per_second: Err("requests_per_second was not initialized".to_string()),
    }
  }

  pub fn task_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.task_id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for task_id failed".to_string());
    self
  }

  pub fn requests_per_second<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.requests_per_second = value
      .try_into()
      .map_err(|_| "conversion to `i32` for requests_per_second failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_reindex/{task_id}/_rethrottle`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      task_id,
      requests_per_second,
    } = self;
    let task_id = task_id.map_err(Error::InvalidRequest)?;
    let requests_per_second = requests_per_second.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_reindex/{}/_rethrottle",
      client.baseurl,
      encode_path(&task_id.to_string()),
    );
    let mut query = Vec::with_capacity(1usize);
    query.push(("requests_per_second", requests_per_second.to_string()));
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

///Builder for [`Client::render_search_template_get`]
///
///[`Client::render_search_template_get`]: super::OsClient::render_search_template_get
#[derive(Debug, Clone)]
pub struct RenderSearchTemplateGet<'a> {
  client: &'a super::OsClient,
}

impl<'a> RenderSearchTemplateGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_render/template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}_render/template", client.baseurl,);
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

///Builder for [`Client::render_search_template_post`]
///
///[`Client::render_search_template_post`]: super::OsClient::render_search_template_post
#[derive(Debug, Clone)]
pub struct RenderSearchTemplatePost<'a> {
  client: &'a super::OsClient,
  body: Result<types::RenderSearchTemplateBodyParams, String>,
}

impl<'a> RenderSearchTemplatePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RenderSearchTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `RenderSearchTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_render/template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_render/template", client.baseurl,);
    let request = client.client.post(url).json(&body).build()?;
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

///Builder for [`Client::render_search_template_get_with_id`]
///
///[`Client::render_search_template_get_with_id`]: super::OsClient::render_search_template_get_with_id
#[derive(Debug, Clone)]
pub struct RenderSearchTemplateGetWithId<'a> {
  client: &'a super::OsClient,
  id: Result<types::OpenSearchId, String>,
}

impl<'a> RenderSearchTemplateGetWithId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_render/template/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, id } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let url = format!("{}_render/template/{}", client.baseurl, encode_path(&id.to_string()),);
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

///Builder for [`Client::render_search_template_post_with_id`]
///
///[`Client::render_search_template_post_with_id`]: super::OsClient::render_search_template_post_with_id
#[derive(Debug, Clone)]
pub struct RenderSearchTemplatePostWithId<'a> {
  client: &'a super::OsClient,
  id: Result<types::OpenSearchId, String>,
  body: Result<types::RenderSearchTemplateBodyParams, String>,
}

impl<'a> RenderSearchTemplatePostWithId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RenderSearchTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `RenderSearchTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_render/template/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, id, body } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_render/template/{}", client.baseurl, encode_path(&id.to_string()),);
    let request = client.client.post(url).json(&body).build()?;
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

///Builder for [`Client::get_script_context`]
///
///[`Client::get_script_context`]: super::OsClient::get_script_context
#[derive(Debug, Clone)]
pub struct GetScriptContext<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetScriptContext<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_script_context`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}_script_context", client.baseurl,);
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

///Builder for [`Client::get_script_languages`]
///
///[`Client::get_script_languages`]: super::OsClient::get_script_languages
#[derive(Debug, Clone)]
pub struct GetScriptLanguages<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetScriptLanguages<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_script_language`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}_script_language", client.baseurl,);
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

///Builder for [`Client::scripts_painless_execute_get`]
///
///[`Client::scripts_painless_execute_get`]: super::OsClient::scripts_painless_execute_get
#[derive(Debug, Clone)]
pub struct ScriptsPainlessExecuteGet<'a> {
  client: &'a super::OsClient,
}

impl<'a> ScriptsPainlessExecuteGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_scripts/painless/_execute`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}_scripts/painless/_execute", client.baseurl,);
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

///Builder for [`Client::scripts_painless_execute_post`]
///
///[`Client::scripts_painless_execute_post`]: super::OsClient::scripts_painless_execute_post
#[derive(Debug, Clone)]
pub struct ScriptsPainlessExecutePost<'a> {
  client: &'a super::OsClient,
  body: Result<types::ScriptsPainlessExecuteBodyParams, String>,
}

impl<'a> ScriptsPainlessExecutePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ScriptsPainlessExecuteBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ScriptsPainlessExecuteBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_scripts/painless/_execute`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_scripts/painless/_execute", client.baseurl,);
    let request = client.client.post(url).json(&body).build()?;
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

///Builder for [`Client::get_script`]
///
///[`Client::get_script`]: super::OsClient::get_script
#[derive(Debug, Clone)]
pub struct GetScript<'a> {
  client: &'a super::OsClient,
  id: Result<types::OpenSearchId, String>,
  cluster_manager_timeout: Result<Option<types::Timeout>, String>,
  master_timeout: Result<Option<types::Timeout>, String>,
}

impl<'a> GetScript<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_scripts/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      id,
      cluster_manager_timeout,
      master_timeout,
    } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_scripts/{}", client.baseurl, encode_path(&id.to_string()),);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
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

///Builder for [`Client::put_script_put`]
///
///[`Client::put_script_put`]: super::OsClient::put_script_put
#[derive(Debug, Clone)]
pub struct PutScriptPut<'a> {
  client: &'a super::OsClient,
  id: Result<types::OpenSearchId, String>,
  cluster_manager_timeout: Result<Option<types::Timeout>, String>,
  master_timeout: Result<Option<types::Timeout>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  body: Result<types::PutScriptBodyParams, String>,
}

impl<'a> PutScriptPut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PutScriptBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PutScriptBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_scripts/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      id,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      body,
    } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_scripts/{}", client.baseurl, encode_path(&id.to_string()),);
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

///Builder for [`Client::put_script_post`]
///
///[`Client::put_script_post`]: super::OsClient::put_script_post
#[derive(Debug, Clone)]
pub struct PutScriptPost<'a> {
  client: &'a super::OsClient,
  id: Result<types::OpenSearchId, String>,
  cluster_manager_timeout: Result<Option<types::Timeout>, String>,
  master_timeout: Result<Option<types::Timeout>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  body: Result<types::PutScriptBodyParams, String>,
}

impl<'a> PutScriptPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PutScriptBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PutScriptBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_scripts/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      id,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      body,
    } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_scripts/{}", client.baseurl, encode_path(&id.to_string()),);
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

///Builder for [`Client::delete_script`]
///
///[`Client::delete_script`]: super::OsClient::delete_script
#[derive(Debug, Clone)]
pub struct DeleteScript<'a> {
  client: &'a super::OsClient,
  id: Result<types::OpenSearchId, String>,
  cluster_manager_timeout: Result<Option<types::Timeout>, String>,
  master_timeout: Result<Option<types::Timeout>, String>,
  timeout: Result<Option<types::Timeout>, String>,
}

impl<'a> DeleteScript<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_scripts/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      id,
      cluster_manager_timeout,
      master_timeout,
      timeout,
    } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_scripts/{}", client.baseurl, encode_path(&id.to_string()),);
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

///Builder for [`Client::put_script_put_with_context`]
///
///[`Client::put_script_put_with_context`]: super::OsClient::put_script_put_with_context
#[derive(Debug, Clone)]
pub struct PutScriptPutWithContext<'a> {
  client: &'a super::OsClient,
  id: Result<types::OpenSearchId, String>,
  context: Result<types::OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<types::Timeout>, String>,
  master_timeout: Result<Option<types::Timeout>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  body: Result<types::PutScriptBodyParams, String>,
}

impl<'a> PutScriptPutWithContext<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
      context: Err("context was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn context<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.context = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for context failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PutScriptBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PutScriptBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_scripts/{id}/{context}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      id,
      context,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      body,
    } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let context = context.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_scripts/{}/{}",
      client.baseurl,
      encode_path(&id.to_string()),
      encode_path(&context.to_string()),
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

///Builder for [`Client::put_script_post_with_context`]
///
///[`Client::put_script_post_with_context`]: super::OsClient::put_script_post_with_context
#[derive(Debug, Clone)]
pub struct PutScriptPostWithContext<'a> {
  client: &'a super::OsClient,
  id: Result<types::OpenSearchId, String>,
  context: Result<types::OpenSearchNameValue, String>,
  cluster_manager_timeout: Result<Option<types::Timeout>, String>,
  master_timeout: Result<Option<types::Timeout>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  body: Result<types::PutScriptBodyParams, String>,
}

impl<'a> PutScriptPostWithContext<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
      context: Err("context was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn context<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.context = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for context failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.cluster_manager_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for cluster_manager_timeout failed".to_string());
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::PutScriptBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `PutScriptBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_scripts/{id}/{context}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      id,
      context,
      cluster_manager_timeout,
      master_timeout,
      timeout,
      body,
    } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let context = context.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_scripts/{}/{}",
      client.baseurl,
      encode_path(&id.to_string()),
      encode_path(&context.to_string()),
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

///Builder for [`Client::delete_pit`]
///
///[`Client::delete_pit`]: super::OsClient::delete_pit
#[derive(Debug, Clone)]
pub struct DeletePit<'a> {
  client: &'a super::OsClient,
  body: Result<types::builder::DeletePitBodyParams, String>,
}

impl<'a> DeletePit<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Ok(types::builder::DeletePitBodyParams::default()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::DeletePitBodyParams>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `DeletePitBodyParams` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::DeletePitBodyParams) -> types::builder::DeletePitBodyParams, {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `DELETE` request to `/_search/point_in_time`
  pub async fn send(self) -> Result<ResponseValue<types::DeletePitResponseContent>, Error> {
    let Self { client, body } = self;
    let body = body
      .and_then(std::convert::TryInto::<types::DeletePitBodyParams>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!("{}_search/point_in_time", client.baseurl,);
    let request = client
      .client
      .delete(url)
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

///Builder for [`Client::get_all_pits`]
///
///[`Client::get_all_pits`]: super::OsClient::get_all_pits
#[derive(Debug, Clone)]
pub struct GetAllPits<'a> {
  client: &'a super::OsClient,
}

impl<'a> GetAllPits<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_search/point_in_time/_all`
  pub async fn send(self) -> Result<ResponseValue<types::GetAllPitsResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}_search/point_in_time/_all", client.baseurl,);
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

///Builder for [`Client::delete_all_pits`]
///
///[`Client::delete_all_pits`]: super::OsClient::delete_all_pits
#[derive(Debug, Clone)]
pub struct DeleteAllPits<'a> {
  client: &'a super::OsClient,
}

impl<'a> DeleteAllPits<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `DELETE` request to `/_search/point_in_time/_all`
  pub async fn send(self) -> Result<ResponseValue<types::DeleteAllPitsResponseContent>, Error> {
    let Self { client } = self;
    let url = format!("{}_search/point_in_time/_all", client.baseurl,);
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

///Builder for [`Client::scroll_get`]
///
///[`Client::scroll_get`]: super::OsClient::scroll_get
#[derive(Debug, Clone)]
pub struct ScrollGet<'a> {
  client: &'a super::OsClient,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  scroll: Result<Option<types::ScrollGetScroll>, String>,
  scroll_id: Result<Option<String>, String>,
}

impl<'a> ScrollGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      rest_total_hits_as_int: Ok(None),
      scroll: Ok(None),
      scroll_id: Ok(None),
    }
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ScrollGetScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ScrollGetScroll` for scroll failed".to_string());
    self
  }

  pub fn scroll_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.scroll_id = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for scroll_id failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_search/scroll`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      rest_total_hits_as_int,
      scroll,
      scroll_id,
    } = self;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let scroll_id = scroll_id.map_err(Error::InvalidRequest)?;
    let url = format!("{}_search/scroll", client.baseurl,);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &scroll_id {
      query.push(("scroll_id", v.to_string()));
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

///Builder for [`Client::scroll_post`]
///
///[`Client::scroll_post`]: super::OsClient::scroll_post
#[derive(Debug, Clone)]
pub struct ScrollPost<'a> {
  client: &'a super::OsClient,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  scroll: Result<Option<types::ScrollPostScroll>, String>,
  scroll_id: Result<Option<String>, String>,
  body: Result<types::ScrollBodyParams, String>,
}

impl<'a> ScrollPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      rest_total_hits_as_int: Ok(None),
      scroll: Ok(None),
      scroll_id: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ScrollPostScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ScrollPostScroll` for scroll failed".to_string());
    self
  }

  pub fn scroll_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.scroll_id = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for scroll_id failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ScrollBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ScrollBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_search/scroll`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      rest_total_hits_as_int,
      scroll,
      scroll_id,
      body,
    } = self;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let scroll_id = scroll_id.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_search/scroll", client.baseurl,);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &scroll_id {
      query.push(("scroll_id", v.to_string()));
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

///Builder for [`Client::clear_scroll`]
///
///[`Client::clear_scroll`]: super::OsClient::clear_scroll
#[derive(Debug, Clone)]
pub struct ClearScroll<'a> {
  client: &'a super::OsClient,
  body: Result<types::ClearScrollBodyParams, String>,
}

impl<'a> ClearScroll<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClearScrollBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ClearScrollBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_search/scroll`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, body } = self;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_search/scroll", client.baseurl,);
    let request = client.client.delete(url).json(&body).build()?;
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

///Builder for [`Client::scroll_get_with_scroll_id`]
///
///[`Client::scroll_get_with_scroll_id`]: super::OsClient::scroll_get_with_scroll_id
#[derive(Debug, Clone)]
pub struct ScrollGetWithScrollId<'a> {
  client: &'a super::OsClient,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  scroll: Result<Option<types::ScrollGetWithScrollIdScroll>, String>,
  scroll_id: Result<Option<String>, String>,
}

impl<'a> ScrollGetWithScrollId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      rest_total_hits_as_int: Ok(None),
      scroll: Ok(None),
      scroll_id: Ok(None),
    }
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ScrollGetWithScrollIdScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ScrollGetWithScrollIdScroll` for scroll failed".to_string());
    self
  }

  pub fn scroll_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.scroll_id = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for scroll_id failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_search/scroll/{scroll_id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      rest_total_hits_as_int,
      scroll,
      scroll_id,
    } = self;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let scroll_id = scroll_id.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_search/scroll/{}",
      client.baseurl,
      encode_path(&scroll_id.clone().unwrap_or(String::from("")).to_string()),
    );
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &scroll_id {
      query.push(("scroll_id", v.to_string()));
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

///Builder for [`Client::scroll_post_with_scroll_id`]
///
///[`Client::scroll_post_with_scroll_id`]: super::OsClient::scroll_post_with_scroll_id
#[derive(Debug, Clone)]
pub struct ScrollPostWithScrollId<'a> {
  client: &'a super::OsClient,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  scroll: Result<Option<types::ScrollPostWithScrollIdScroll>, String>,
  scroll_id: Result<Option<String>, String>,
  body: Result<types::ScrollBodyParams, String>,
}

impl<'a> ScrollPostWithScrollId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      rest_total_hits_as_int: Ok(None),
      scroll: Ok(None),
      scroll_id: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ScrollPostWithScrollIdScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ScrollPostWithScrollIdScroll` for scroll failed".to_string());
    self
  }

  pub fn scroll_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.scroll_id = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for scroll_id failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ScrollBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ScrollBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_search/scroll/{scroll_id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      rest_total_hits_as_int,
      scroll,
      scroll_id,
      body,
    } = self;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let scroll_id = scroll_id.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_search/scroll/{}",
      client.baseurl,
      encode_path(&scroll_id.clone().unwrap_or(String::from(""))),
    );
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &scroll_id {
      query.push(("scroll_id", v.to_string()));
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

///Builder for [`Client::clear_scroll_with_scroll_id`]
///
///[`Client::clear_scroll_with_scroll_id`]: super::OsClient::clear_scroll_with_scroll_id
#[derive(Debug, Clone)]
pub struct ClearScrollWithScrollId<'a> {
  client: &'a super::OsClient,
  scroll_id: Result<types::OpenSearchId, String>,
  body: Result<types::ClearScrollBodyParams, String>,
}

impl<'a> ClearScrollWithScrollId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      scroll_id: Err("scroll_id was not initialized".to_string()),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn scroll_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.scroll_id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for scroll_id failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClearScrollBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ClearScrollBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_search/scroll/{scroll_id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      scroll_id,
      body,
    } = self;
    let scroll_id = scroll_id.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_search/scroll/{}",
      client.baseurl,
      encode_path(&scroll_id.to_string()),
    );
    let request = client.client.delete(url).json(&body).build()?;
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

///Builder for [`Client::search_template_get`]
///
///[`Client::search_template_get`]: super::OsClient::search_template_get
#[derive(Debug, Clone)]
pub struct SearchTemplateGet<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  explain: Result<Option<bool>, String>,
  ignore_throttled: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  profile: Result<Option<bool>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  routing: Result<Option<Vec<String>>, String>,
  scroll: Result<Option<types::SearchTemplateGetScroll>, String>,
  search_type: Result<Option<types::SearchTypeMulti>, String>,
  typed_keys: Result<Option<bool>, String>,
}

impl<'a> SearchTemplateGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      ccs_minimize_roundtrips: Ok(None),
      expand_wildcards: Ok(None),
      explain: Ok(None),
      ignore_throttled: Ok(None),
      ignore_unavailable: Ok(None),
      preference: Ok(None),
      profile: Ok(None),
      rest_total_hits_as_int: Ok(None),
      routing: Ok(None),
      scroll: Ok(None),
      search_type: Ok(None),
      typed_keys: Ok(None),
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

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn ignore_throttled<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_throttled = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_throttled failed".to_string());
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

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn profile<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.profile = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for profile failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for routing failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTemplateGetScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTemplateGetScroll` for scroll failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTypeMulti>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTypeMulti` for search_type failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_search/template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      ccs_minimize_roundtrips,
      expand_wildcards,
      explain,
      ignore_throttled,
      ignore_unavailable,
      preference,
      profile,
      rest_total_hits_as_int,
      routing,
      scroll,
      search_type,
      typed_keys,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let ignore_throttled = ignore_throttled.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let profile = profile.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let url = format!("{}_search/template", client.baseurl,);
    let mut query = Vec::with_capacity(13usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &ignore_throttled {
      query.push(("ignore_throttled", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &profile {
      query.push(("profile", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.join(",")));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
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

///Builder for [`Client::search_template_post`]
///
///[`Client::search_template_post`]: super::OsClient::search_template_post
#[derive(Debug, Clone)]
pub struct SearchTemplatePost<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  explain: Result<Option<bool>, String>,
  ignore_throttled: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  profile: Result<Option<bool>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  routing: Result<Option<Vec<String>>, String>,
  scroll: Result<Option<types::SearchTemplatePostScroll>, String>,
  search_type: Result<Option<types::SearchTypeMulti>, String>,
  typed_keys: Result<Option<bool>, String>,
  body: Result<types::SearchTemplateBodyParams, String>,
}

impl<'a> SearchTemplatePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      ccs_minimize_roundtrips: Ok(None),
      expand_wildcards: Ok(None),
      explain: Ok(None),
      ignore_throttled: Ok(None),
      ignore_unavailable: Ok(None),
      preference: Ok(None),
      profile: Ok(None),
      rest_total_hits_as_int: Ok(None),
      routing: Ok(None),
      scroll: Ok(None),
      search_type: Ok(None),
      typed_keys: Ok(None),
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

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn ignore_throttled<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_throttled = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_throttled failed".to_string());
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

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn profile<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.profile = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for profile failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for routing failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTemplatePostScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTemplatePostScroll` for scroll failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTypeMulti>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTypeMulti` for search_type failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `SearchTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_search/template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      ccs_minimize_roundtrips,
      expand_wildcards,
      explain,
      ignore_throttled,
      ignore_unavailable,
      preference,
      profile,
      rest_total_hits_as_int,
      routing,
      scroll,
      search_type,
      typed_keys,
      body,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let ignore_throttled = ignore_throttled.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let profile = profile.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_search/template", client.baseurl,);
    let mut query = Vec::with_capacity(13usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &ignore_throttled {
      query.push(("ignore_throttled", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &profile {
      query.push(("profile", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.join(",")));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
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

///Builder for [`Client::search_shards_get`]
///
///[`Client::search_shards_get`]: super::OsClient::search_shards_get
#[derive(Debug, Clone)]
pub struct SearchShardsGet<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  routing: Result<Option<String>, String>,
}

impl<'a> SearchShardsGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
      preference: Ok(None),
      routing: Ok(None),
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_search_shards`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
      preference,
      routing,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let url = format!("{}_search_shards", client.baseurl,);
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
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
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

///Builder for [`Client::search_shards_post`]
///
///[`Client::search_shards_post`]: super::OsClient::search_shards_post
#[derive(Debug, Clone)]
pub struct SearchShardsPost<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  routing: Result<Option<String>, String>,
}

impl<'a> SearchShardsPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
      preference: Ok(None),
      routing: Ok(None),
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_search_shards`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
      preference,
      routing,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let url = format!("{}_search_shards", client.baseurl,);
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
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
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

///Builder for [`Client::update_by_query_rethrottle`]
///
///[`Client::update_by_query_rethrottle`]: super::OsClient::update_by_query_rethrottle
#[derive(Debug, Clone)]
pub struct UpdateByQueryRethrottle<'a> {
  client: &'a super::OsClient,
  task_id: Result<types::OpenSearchId, String>,
  requests_per_second: Result<i32, String>,
}

impl<'a> UpdateByQueryRethrottle<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      task_id: Err("task_id was not initialized".to_string()),
      requests_per_second: Err("requests_per_second was not initialized".to_string()),
    }
  }

  pub fn task_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.task_id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for task_id failed".to_string());
    self
  }

  pub fn requests_per_second<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.requests_per_second = value
      .try_into()
      .map_err(|_| "conversion to `i32` for requests_per_second failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_update_by_query/{task_id}/_rethrottle`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      task_id,
      requests_per_second,
    } = self;
    let task_id = task_id.map_err(Error::InvalidRequest)?;
    let requests_per_second = requests_per_second.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_update_by_query/{}/_rethrottle",
      client.baseurl,
      encode_path(&task_id.to_string()),
    );
    let mut query = Vec::with_capacity(1usize);
    query.push(("requests_per_second", requests_per_second.to_string()));
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

///Builder for [`Client::create_put`]
///
///[`Client::create_put`]: super::OsClient::create_put
#[derive(Debug, Clone)]
pub struct CreatePut<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  pipeline: Result<Option<String>, String>,
  refresh: Result<Option<types::RefreshEnum>, String>,
  routing: Result<Option<String>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::CreateBodyParams, String>,
}

impl<'a> CreatePut<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      pipeline: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      timeout: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn pipeline<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.pipeline = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for pipeline failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RefreshEnum>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `RefreshEnum` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
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
    V: std::convert::TryInto<types::CreateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `CreateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_create/{id}`
  pub async fn send(self) -> Result<ResponseValue<types::IndexResponse>, Error> {
    let Self {
      client,
      index,
      id,
      pipeline,
      refresh,
      routing,
      timeout,
      version,
      version_type,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let pipeline = pipeline.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_create/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &pipeline {
      query.push(("pipeline", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
    }
    let request = client.client.put(url).json(&body).query(&query).build()?;
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

///Builder for [`Client::delete_by_query`]
///
///[`Client::delete_by_query`]: super::OsClient::delete_by_query
#[derive(Debug, Clone)]
pub struct DeleteByQuery<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  conflicts: Result<Option<types::Conflicts>, String>,
  default_operator: Result<Option<types::DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  from: Result<Option<i32>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  lenient: Result<Option<bool>, String>,
  max_docs: Result<Option<i32>, String>,
  preference: Result<Option<String>, String>,
  q: Result<Option<String>, String>,
  refresh: Result<Option<bool>, String>,
  request_cache: Result<Option<bool>, String>,
  requests_per_second: Result<Option<i32>, String>,
  routing: Result<Option<Vec<String>>, String>,
  scroll: Result<Option<types::DeleteByQueryScroll>, String>,
  scroll_size: Result<Option<i32>, String>,
  search_timeout: Result<Option<types::Timeout>, String>,
  search_type: Result<Option<types::SearchType>, String>,
  size: Result<Option<i32>, String>,
  slices: Result<Option<String>, String>,
  sort: Result<Option<Vec<String>>, String>,
  stats: Result<Option<Vec<String>>, String>,
  terminate_after: Result<Option<i32>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  version: Result<Option<bool>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  wait_for_completion: Result<Option<bool>, String>,
  body: Result<types::DeleteByQueryBodyParams, String>,
}

impl<'a> DeleteByQuery<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      allow_no_indices: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      conflicts: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      expand_wildcards: Ok(None),
      from: Ok(None),
      ignore_unavailable: Ok(None),
      lenient: Ok(None),
      max_docs: Ok(None),
      preference: Ok(None),
      q: Ok(None),
      refresh: Ok(None),
      request_cache: Ok(None),
      requests_per_second: Ok(None),
      routing: Ok(None),
      scroll: Ok(None),
      scroll_size: Ok(None),
      search_timeout: Ok(None),
      search_type: Ok(None),
      size: Ok(None),
      slices: Ok(None),
      sort: Ok(None),
      stats: Ok(None),
      terminate_after: Ok(None),
      timeout: Ok(None),
      version: Ok(None),
      wait_for_active_shards: Ok(None),
      wait_for_completion: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
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

  pub fn conflicts<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Conflicts>, {
    self.conflicts = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Conflicts` for conflicts failed".to_string());
    self
  }

  pub fn default_operator<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::DefaultOperator>, {
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn from<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.from = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for from failed".to_string());
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

  pub fn max_docs<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_docs = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_docs failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
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

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn request_cache<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.request_cache = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for request_cache failed".to_string());
    self
  }

  pub fn requests_per_second<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.requests_per_second = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for requests_per_second failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for routing failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::DeleteByQueryScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `DeleteByQueryScroll` for scroll failed".to_string());
    self
  }

  pub fn scroll_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.scroll_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for scroll_size failed".to_string());
    self
  }

  pub fn search_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.search_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for search_timeout failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchType>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchType` for search_type failed".to_string());
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

  pub fn slices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.slices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for slices failed".to_string());
    self
  }

  pub fn sort<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.sort = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for sort failed".to_string());
    self
  }

  pub fn stats<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stats = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stats failed".to_string());
    self
  }

  pub fn terminate_after<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.terminate_after = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for terminate_after failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for version failed".to_string());
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
    V: std::convert::TryInto<types::DeleteByQueryBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `DeleteByQueryBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_delete_by_query`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      source,
      source_excludes,
      source_includes,
      allow_no_indices,
      analyze_wildcard,
      analyzer,
      conflicts,
      default_operator,
      df,
      expand_wildcards,
      from,
      ignore_unavailable,
      lenient,
      max_docs,
      preference,
      q,
      refresh,
      request_cache,
      requests_per_second,
      routing,
      scroll,
      scroll_size,
      search_timeout,
      search_type,
      size,
      slices,
      sort,
      stats,
      terminate_after,
      timeout,
      version,
      wait_for_active_shards,
      wait_for_completion,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let conflicts = conflicts.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let from = from.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let max_docs = max_docs.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let request_cache = request_cache.map_err(Error::InvalidRequest)?;
    let requests_per_second = requests_per_second.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let scroll_size = scroll_size.map_err(Error::InvalidRequest)?;
    let search_timeout = search_timeout.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let size = size.map_err(Error::InvalidRequest)?;
    let slices = slices.map_err(Error::InvalidRequest)?;
    let sort = sort.map_err(Error::InvalidRequest)?;
    let stats = stats.map_err(Error::InvalidRequest)?;
    let terminate_after = terminate_after.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_delete_by_query", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(33usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
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
    if let Some(v) = &conflicts {
      query.push(("conflicts", v.to_string()));
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
    if let Some(v) = &from {
      query.push(("from", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &max_docs {
      query.push(("max_docs", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &request_cache {
      query.push(("request_cache", v.to_string()));
    }
    if let Some(v) = &requests_per_second {
      query.push(("requests_per_second", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.join(",")));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &scroll_size {
      query.push(("scroll_size", v.to_string()));
    }
    if let Some(v) = &search_timeout {
      query.push(("search_timeout", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &size {
      query.push(("size", v.to_string()));
    }
    if let Some(v) = &slices {
      query.push(("slices", v.to_string()));
    }
    if let Some(v) = &sort {
      query.push(("sort", v.join(",")));
    }
    if let Some(v) = &stats {
      query.push(("stats", v.join(",")));
    }
    if let Some(v) = &terminate_after {
      query.push(("terminate_after", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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

///Builder for [`Client::get`]
///
///[`Client::get`]: super::OsClient::get
#[derive(Debug, Clone)]
pub struct Get<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  refresh: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  stored_fields: Result<Option<Vec<String>>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
}

impl<'a> Get<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      stored_fields: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn stored_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stored_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stored_fields failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_doc/{id}`
  pub async fn send<T: DeserializeOwned + std::default::Default>(
    self,
  ) -> Result<ResponseValue<types::GetResponseContent<T>>, Error> {
    let Self {
      client,
      index,
      id,
      source,
      source_excludes,
      source_includes,
      preference,
      realtime,
      refresh,
      routing,
      stored_fields,
      version,
      version_type,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let stored_fields = stored_fields.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_doc/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(10usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &stored_fields {
      query.push(("stored_fields", v.join(",")));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
    }
    let request = client
      .client
      .get(url)
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

///Builder for [`Client::index_put_with_id`]
///
///[`Client::index_put_with_id`]: super::OsClient::index_put_with_id
#[derive(Debug, Clone)]
pub struct IndexPutWithId<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  if_primary_term: Result<Option<i32>, String>,
  if_seq_no: Result<Option<i32>, String>,
  op_type: Result<Option<types::OpType>, String>,
  pipeline: Result<Option<String>, String>,
  refresh: Result<Option<types::RefreshEnum>, String>,
  require_alias: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::IndexBodyParams, String>,
}

impl<'a> IndexPutWithId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      if_primary_term: Ok(None),
      if_seq_no: Ok(None),
      op_type: Ok(None),
      pipeline: Ok(None),
      refresh: Ok(None),
      require_alias: Ok(None),
      routing: Ok(None),
      timeout: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn if_primary_term<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.if_primary_term = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for if_primary_term failed".to_string());
    self
  }

  pub fn if_seq_no<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.if_seq_no = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for if_seq_no failed".to_string());
    self
  }

  pub fn op_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpType>, {
    self.op_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `OpType` for op_type failed".to_string());
    self
  }

  pub fn pipeline<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.pipeline = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for pipeline failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RefreshEnum>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `RefreshEnum` for refresh failed".to_string());
    self
  }

  pub fn require_alias<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.require_alias = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for require_alias failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
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
    V: std::convert::TryInto<types::IndexBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IndexBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/{index}/_doc/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      id,
      if_primary_term,
      if_seq_no,
      op_type,
      pipeline,
      refresh,
      require_alias,
      routing,
      timeout,
      version,
      version_type,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let if_primary_term = if_primary_term.map_err(Error::InvalidRequest)?;
    let if_seq_no = if_seq_no.map_err(Error::InvalidRequest)?;
    let op_type = op_type.map_err(Error::InvalidRequest)?;
    let pipeline = pipeline.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let require_alias = require_alias.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_doc/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(11usize);
    if let Some(v) = &if_primary_term {
      query.push(("if_primary_term", v.to_string()));
    }
    if let Some(v) = &if_seq_no {
      query.push(("if_seq_no", v.to_string()));
    }
    if let Some(v) = &op_type {
      query.push(("op_type", v.to_string()));
    }
    if let Some(v) = &pipeline {
      query.push(("pipeline", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &require_alias {
      query.push(("require_alias", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
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

///Builder for [`Client::index_post_with_id`]
///
///[`Client::index_post_with_id`]: super::OsClient::index_post_with_id
#[derive(Debug, Clone)]
pub struct IndexPost<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<Option<String>, String>,
  if_primary_term: Result<Option<i32>, String>,
  if_seq_no: Result<Option<i32>, String>,
  op_type: Result<Option<types::OpType>, String>,
  pipeline: Result<Option<String>, String>,
  refresh: Result<Option<types::RefreshEnum>, String>,
  require_alias: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<serde_json::Value, String>,
}

impl<'a> IndexPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Ok(None),
      if_primary_term: Ok(None),
      if_seq_no: Ok(None),
      op_type: Ok(None),
      pipeline: Ok(None),
      refresh: Ok(None),
      require_alias: Ok(None),
      routing: Ok(None),
      timeout: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.id = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for id failed".to_string());
    self
  }

  pub fn if_primary_term<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.if_primary_term = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for if_primary_term failed".to_string());
    self
  }

  pub fn if_seq_no<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.if_seq_no = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for if_seq_no failed".to_string());
    self
  }

  pub fn op_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpType>, {
    self.op_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `OpType` for op_type failed".to_string());
    self
  }

  pub fn pipeline<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.pipeline = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for pipeline failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RefreshEnum>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `RefreshEnum` for refresh failed".to_string());
    self
  }

  pub fn require_alias<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.require_alias = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for require_alias failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
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

  pub fn body<V: Serialize>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<serde_json::Value>, {
    self.body = serde_json::to_value(value).map_err(|_| "conversion to `serde_json:Value` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_doc/{id}`
  pub async fn send(self) -> Result<ResponseValue<types::IndexResponse>, Error> {
    let Self {
      client,
      index,
      id,
      if_primary_term,
      if_seq_no,
      op_type,
      pipeline,
      refresh,
      require_alias,
      routing,
      timeout,
      version,
      version_type,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let if_primary_term = if_primary_term.map_err(Error::InvalidRequest)?;
    let if_seq_no = if_seq_no.map_err(Error::InvalidRequest)?;
    let op_type = op_type.map_err(Error::InvalidRequest)?;
    let pipeline = pipeline.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let require_alias = require_alias.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = match &id {
      Some(id) => {
        format!(
          "{}{}/_doc/{}",
          client.baseurl,
          encode_path(&index.to_string()),
          encode_path(&id.to_string()),
        )
      }
      None => format!("{}{}/_doc", client.baseurl, encode_path(&index.to_string()),),
    };
    let mut query = Vec::with_capacity(11usize);
    if let Some(v) = &if_primary_term {
      query.push(("if_primary_term", v.to_string()));
    }
    if let Some(v) = &if_seq_no {
      query.push(("if_seq_no", v.to_string()));
    }
    if let Some(v) = &op_type {
      query.push(("op_type", v.to_string()));
    }
    if let Some(v) = &pipeline {
      query.push(("pipeline", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &require_alias {
      query.push(("require_alias", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
    }
    let request = client.client.post(url).json(&body).query(&query).build()?;
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

///Builder for [`Client::delete`]
///
///[`Client::delete`]: super::OsClient::delete
#[derive(Debug, Clone)]
pub struct Delete<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  if_primary_term: Result<Option<i32>, String>,
  if_seq_no: Result<Option<i32>, String>,
  refresh: Result<Option<types::RefreshEnum>, String>,
  routing: Result<Option<String>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
}

impl<'a> Delete<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      if_primary_term: Ok(None),
      if_seq_no: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      timeout: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
      wait_for_active_shards: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn if_primary_term<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.if_primary_term = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for if_primary_term failed".to_string());
    self
  }

  pub fn if_seq_no<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.if_seq_no = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for if_seq_no failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RefreshEnum>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `RefreshEnum` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
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

  ///Sends a `DELETE` request to `/{index}/_doc/{id}`
  pub async fn send(self) -> Result<ResponseValue<types::DocumentDeleteResponse>, Error> {
    let Self {
      client,
      index,
      id,
      if_primary_term,
      if_seq_no,
      refresh,
      routing,
      timeout,
      version,
      version_type,
      wait_for_active_shards,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let if_primary_term = if_primary_term.map_err(Error::InvalidRequest)?;
    let if_seq_no = if_seq_no.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_doc/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &if_primary_term {
      query.push(("if_primary_term", v.to_string()));
    }
    if let Some(v) = &if_seq_no {
      query.push(("if_seq_no", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
    }
    let request = client.client.delete(url).query(&query).build()?;
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

///Builder for [`Client::exists`]
///
///[`Client::exists`]: super::OsClient::exists
#[derive(Debug, Clone)]
pub struct Exists<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  refresh: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  stored_fields: Result<Option<Vec<String>>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
}

impl<'a> Exists<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      stored_fields: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn stored_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stored_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stored_fields failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  ///Sends a `HEAD` request to `/{index}/_doc/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      id,
      source,
      source_excludes,
      source_includes,
      preference,
      realtime,
      refresh,
      routing,
      stored_fields,
      version,
      version_type,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let stored_fields = stored_fields.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_doc/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(10usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &stored_fields {
      query.push(("stored_fields", v.join(",")));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
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

///Builder for [`Client::explain_post`]
///
///[`Client::explain_post`]: super::OsClient::explain_post
#[derive(Debug, Clone)]
pub struct ExplainPost<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  default_operator: Result<Option<types::DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  lenient: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  q: Result<Option<String>, String>,
  routing: Result<Option<String>, String>,
  stored_fields: Result<Option<Vec<String>>, String>,
  body: Result<types::ExplainBodyParams, String>,
}

impl<'a> ExplainPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      lenient: Ok(None),
      preference: Ok(None),
      q: Ok(None),
      routing: Ok(None),
      stored_fields: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
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
    V: std::convert::TryInto<types::DefaultOperator>, {
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

  pub fn lenient<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.lenient = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for lenient failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
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

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn stored_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stored_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stored_fields failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExplainBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ExplainBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_explain/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      id,
      source,
      source_excludes,
      source_includes,
      analyze_wildcard,
      analyzer,
      default_operator,
      df,
      lenient,
      preference,
      q,
      routing,
      stored_fields,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let stored_fields = stored_fields.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_explain/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
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
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &stored_fields {
      query.push(("stored_fields", v.join(",")));
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

///Builder for [`Client::field_caps_get_with_index`]
///
///[`Client::field_caps_get_with_index`]: super::OsClient::field_caps_get_with_index
#[derive(Debug, Clone)]
pub struct FieldCapsGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_unmapped: Result<Option<bool>, String>,
}

impl<'a> FieldCapsGetWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      fields: Ok(None),
      ignore_unavailable: Ok(None),
      include_unmapped: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
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

  pub fn include_unmapped<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_unmapped = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_unmapped failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_field_caps`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      fields,
      ignore_unavailable,
      include_unmapped,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_unmapped = include_unmapped.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_field_caps", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_unmapped {
      query.push(("include_unmapped", v.to_string()));
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

///Builder for [`Client::field_caps_post_with_index`]
///
///[`Client::field_caps_post_with_index`]: super::OsClient::field_caps_post_with_index
#[derive(Debug, Clone)]
pub struct FieldCapsPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  include_unmapped: Result<Option<bool>, String>,
  body: Result<types::FieldCapsBodyParams, String>,
}

impl<'a> FieldCapsPostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      fields: Ok(None),
      ignore_unavailable: Ok(None),
      include_unmapped: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
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

  pub fn include_unmapped<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_unmapped = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_unmapped failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::FieldCapsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `FieldCapsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_field_caps`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      fields,
      ignore_unavailable,
      include_unmapped,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let include_unmapped = include_unmapped.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_field_caps", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &include_unmapped {
      query.push(("include_unmapped", v.to_string()));
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

///Builder for [`Client::mget_get_with_index`]
///
///[`Client::mget_get_with_index`]: super::OsClient::mget_get_with_index
#[derive(Debug, Clone)]
pub struct MgetGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  refresh: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  stored_fields: Result<Option<Vec<String>>, String>,
}

impl<'a> MgetGetWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      stored_fields: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn stored_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stored_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stored_fields failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_mget`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      source,
      source_excludes,
      source_includes,
      preference,
      realtime,
      refresh,
      routing,
      stored_fields,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let stored_fields = stored_fields.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_mget", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &stored_fields {
      query.push(("stored_fields", v.join(",")));
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

///Builder for [`Client::mget_post_with_index`]
///
///[`Client::mget_post_with_index`]: super::OsClient::mget_post_with_index
#[derive(Debug, Clone)]
pub struct MgetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  refresh: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  stored_fields: Result<Option<Vec<String>>, String>,
  body: Result<types::MgetBodyParams, String>,
}

impl<'a> MgetWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      stored_fields: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn stored_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stored_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stored_fields failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::MgetBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `MgetBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_mget`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      source,
      source_excludes,
      source_includes,
      preference,
      realtime,
      refresh,
      routing,
      stored_fields,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let stored_fields = stored_fields.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_mget", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &stored_fields {
      query.push(("stored_fields", v.join(",")));
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

///Builder for [`Client::msearch_post_with_index`]
///
///[`Client::msearch_post_with_index`]: super::OsClient::msearch_post_with_index
#[derive(Debug, Clone)]
pub struct MsearchPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  max_concurrent_searches: Result<Option<i32>, String>,
  max_concurrent_shard_requests: Result<Option<i32>, String>,
  pre_filter_shard_size: Result<Option<i32>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  search_type: Result<Option<types::SearchTypeMulti>, String>,
  typed_keys: Result<Option<bool>, String>,
  body: Result<types::MsearchBodyParams, String>,
}

impl<'a> MsearchPostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      ccs_minimize_roundtrips: Ok(None),
      max_concurrent_searches: Ok(None),
      max_concurrent_shard_requests: Ok(None),
      pre_filter_shard_size: Ok(None),
      rest_total_hits_as_int: Ok(None),
      search_type: Ok(None),
      typed_keys: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn max_concurrent_searches<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_searches = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_searches failed".to_string());
    self
  }

  pub fn max_concurrent_shard_requests<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_shard_requests = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_shard_requests failed".to_string());
    self
  }

  pub fn pre_filter_shard_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.pre_filter_shard_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for pre_filter_shard_size failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTypeMulti>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTypeMulti` for search_type failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::MsearchBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `MsearchBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_msearch`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      ccs_minimize_roundtrips,
      max_concurrent_searches,
      max_concurrent_shard_requests,
      pre_filter_shard_size,
      rest_total_hits_as_int,
      search_type,
      typed_keys,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let max_concurrent_searches = max_concurrent_searches.map_err(Error::InvalidRequest)?;
    let max_concurrent_shard_requests = max_concurrent_shard_requests.map_err(Error::InvalidRequest)?;
    let pre_filter_shard_size = pre_filter_shard_size.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_msearch", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &max_concurrent_searches {
      query.push(("max_concurrent_searches", v.to_string()));
    }
    if let Some(v) = &max_concurrent_shard_requests {
      query.push(("max_concurrent_shard_requests", v.to_string()));
    }
    if let Some(v) = &pre_filter_shard_size {
      query.push(("pre_filter_shard_size", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
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

///Builder for [`Client::msearch_template_post_with_index`]
///
///[`Client::msearch_template_post_with_index`]: super::OsClient::msearch_template_post_with_index
#[derive(Debug, Clone)]
pub struct MsearchTemplatePostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  max_concurrent_searches: Result<Option<i32>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  search_type: Result<Option<types::SearchTypeMulti>, String>,
  typed_keys: Result<Option<bool>, String>,
  body: Result<types::MsearchTemplateBodyParams, String>,
}

impl<'a> MsearchTemplatePostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      ccs_minimize_roundtrips: Ok(None),
      max_concurrent_searches: Ok(None),
      rest_total_hits_as_int: Ok(None),
      search_type: Ok(None),
      typed_keys: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn max_concurrent_searches<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_searches = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_searches failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTypeMulti>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTypeMulti` for search_type failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::MsearchTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `MsearchTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_msearch/template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      ccs_minimize_roundtrips,
      max_concurrent_searches,
      rest_total_hits_as_int,
      search_type,
      typed_keys,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let max_concurrent_searches = max_concurrent_searches.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_msearch/template",
      client.baseurl,
      encode_path(&index.to_string()),
    );
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &max_concurrent_searches {
      query.push(("max_concurrent_searches", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
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

///Builder for [`Client::rank_eval_post_with_index`]
///
///[`Client::rank_eval_post_with_index`]: super::OsClient::rank_eval_post_with_index
#[derive(Debug, Clone)]
pub struct RankEvalPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  search_type: Result<Option<types::SearchType>, String>,
  body: Result<types::RankEvalBodyParams, String>,
}

impl<'a> RankEvalPostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      search_type: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchType>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchType` for search_type failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RankEvalBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `RankEvalBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_rank_eval`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      search_type,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_rank_eval", client.baseurl, encode_path(&index.to_string()),);
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
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
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

///Builder for [`Client::create_pit`]
///
///[`Client::create_pit`]: super::OsClient::create_pit
#[derive(Debug, Clone)]
pub struct CreatePit<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  allow_partial_pit_creation: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  keep_alive: Result<Option<String>, String>,
  preference: Result<Option<String>, String>,
  routing: Result<Option<Vec<String>>, String>,
}

impl<'a> CreatePit<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_partial_pit_creation: Ok(None),
      expand_wildcards: Ok(None),
      keep_alive: Ok(None),
      preference: Ok(None),
      routing: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
  }

  pub fn allow_partial_pit_creation<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_partial_pit_creation = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_partial_pit_creation failed".to_string());
    self
  }

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn keep_alive<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.keep_alive = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for keep_alive failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for routing failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_search/point_in_time`
  pub async fn send(self) -> Result<ResponseValue<types::CreatePitResponseContent>, Error> {
    let Self {
      client,
      index,
      allow_partial_pit_creation,
      expand_wildcards,
      keep_alive,
      preference,
      routing,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_partial_pit_creation = allow_partial_pit_creation.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let keep_alive = keep_alive.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_search/point_in_time",
      client.baseurl,
      encode_path(&index.to_string()),
    );
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &allow_partial_pit_creation {
      query.push(("allow_partial_pit_creation", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &keep_alive {
      query.push(("keep_alive", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.join(",")));
    }
    let request = client
      .client
      .post(url)
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

///Builder for [`Client::search_template_post_with_index`]
///
///[`Client::search_template_post_with_index`]: super::OsClient::search_template_post_with_index
#[derive(Debug, Clone)]
pub struct SearchTemplatePostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  allow_no_indices: Result<Option<bool>, String>,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  explain: Result<Option<bool>, String>,
  ignore_throttled: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  profile: Result<Option<bool>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  routing: Result<Option<Vec<String>>, String>,
  scroll: Result<Option<types::SearchTemplatePostWithIndexScroll>, String>,
  search_type: Result<Option<types::SearchTypeMulti>, String>,
  typed_keys: Result<Option<bool>, String>,
  body: Result<types::SearchTemplateBodyParams, String>,
}

impl<'a> SearchTemplatePostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      ccs_minimize_roundtrips: Ok(None),
      expand_wildcards: Ok(None),
      explain: Ok(None),
      ignore_throttled: Ok(None),
      ignore_unavailable: Ok(None),
      preference: Ok(None),
      profile: Ok(None),
      rest_total_hits_as_int: Ok(None),
      routing: Ok(None),
      scroll: Ok(None),
      search_type: Ok(None),
      typed_keys: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
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

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn ignore_throttled<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_throttled = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_throttled failed".to_string());
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

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn profile<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.profile = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for profile failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for routing failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTemplatePostWithIndexScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTemplatePostWithIndexScroll` for scroll failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTypeMulti>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchTypeMulti` for search_type failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `SearchTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_search/template`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      ccs_minimize_roundtrips,
      expand_wildcards,
      explain,
      ignore_throttled,
      ignore_unavailable,
      preference,
      profile,
      rest_total_hits_as_int,
      routing,
      scroll,
      search_type,
      typed_keys,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let ignore_throttled = ignore_throttled.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let profile = profile.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_search/template", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(13usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &ignore_throttled {
      query.push(("ignore_throttled", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &profile {
      query.push(("profile", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.join(",")));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
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

///Builder for [`Client::search_shards_get_with_index`]
///
///[`Client::search_shards_get_with_index`]: super::OsClient::search_shards_get_with_index
#[derive(Debug, Clone)]
pub struct SearchShardsGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  routing: Result<Option<String>, String>,
}

impl<'a> SearchShardsGetWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
      preference: Ok(None),
      routing: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_search_shards`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
      preference,
      routing,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_search_shards", client.baseurl, encode_path(&index.to_string()),);
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
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
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

///Builder for [`Client::search_shards_post_with_index`]
///
///[`Client::search_shards_post_with_index`]: super::OsClient::search_shards_post_with_index
#[derive(Debug, Clone)]
pub struct SearchShardsPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  allow_no_indices: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  routing: Result<Option<String>, String>,
}

impl<'a> SearchShardsPostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      expand_wildcards: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
      preference: Ok(None),
      routing: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
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

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_search_shards`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      expand_wildcards,
      ignore_unavailable,
      local,
      preference,
      routing,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_search_shards", client.baseurl, encode_path(&index.to_string()),);
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
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
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

///Builder for [`Client::get_source`]
///
///[`Client::get_source`]: super::OsClient::get_source
#[derive(Debug, Clone)]
pub struct GetSource<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  refresh: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
}

impl<'a> GetSource<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_source/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      id,
      source,
      source_excludes,
      source_includes,
      preference,
      realtime,
      refresh,
      routing,
      version,
      version_type,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_source/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
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

///Builder for [`Client::exists_source`]
///
///[`Client::exists_source`]: super::OsClient::exists_source
#[derive(Debug, Clone)]
pub struct ExistsSource<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  refresh: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
}

impl<'a> ExistsSource<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      refresh: Ok(None),
      routing: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  ///Sends a `HEAD` request to `/{index}/_source/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      id,
      source,
      source_excludes,
      source_includes,
      preference,
      realtime,
      refresh,
      routing,
      version,
      version_type,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_source/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(9usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
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

///Builder for [`Client::termvectors_get`]
///
///[`Client::termvectors_get`]: super::OsClient::termvectors_get
#[derive(Debug, Clone)]
pub struct TermvectorsGet<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  field_statistics: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  offsets: Result<Option<bool>, String>,
  payloads: Result<Option<bool>, String>,
  positions: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  term_statistics: Result<Option<bool>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
}

impl<'a> TermvectorsGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      field_statistics: Ok(None),
      fields: Ok(None),
      offsets: Ok(None),
      payloads: Ok(None),
      positions: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      routing: Ok(None),
      term_statistics: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn field_statistics<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.field_statistics = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for field_statistics failed".to_string());
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

  pub fn offsets<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.offsets = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for offsets failed".to_string());
    self
  }

  pub fn payloads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.payloads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for payloads failed".to_string());
    self
  }

  pub fn positions<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.positions = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for positions failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn term_statistics<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.term_statistics = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for term_statistics failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_termvectors`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      field_statistics,
      fields,
      offsets,
      payloads,
      positions,
      preference,
      realtime,
      routing,
      term_statistics,
      version,
      version_type,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let field_statistics = field_statistics.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let offsets = offsets.map_err(Error::InvalidRequest)?;
    let payloads = payloads.map_err(Error::InvalidRequest)?;
    let positions = positions.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let term_statistics = term_statistics.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_termvectors", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(11usize);
    if let Some(v) = &field_statistics {
      query.push(("field_statistics", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &offsets {
      query.push(("offsets", v.to_string()));
    }
    if let Some(v) = &payloads {
      query.push(("payloads", v.to_string()));
    }
    if let Some(v) = &positions {
      query.push(("positions", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &term_statistics {
      query.push(("term_statistics", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
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

///Builder for [`Client::termvectors_post`]
///
///[`Client::termvectors_post`]: super::OsClient::termvectors_post
#[derive(Debug, Clone)]
pub struct TermvectorsPost<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  field_statistics: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  offsets: Result<Option<bool>, String>,
  payloads: Result<Option<bool>, String>,
  positions: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  term_statistics: Result<Option<bool>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
  body: Result<types::TermvectorsBodyParams, String>,
}

impl<'a> TermvectorsPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      field_statistics: Ok(None),
      fields: Ok(None),
      offsets: Ok(None),
      payloads: Ok(None),
      positions: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      routing: Ok(None),
      term_statistics: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn field_statistics<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.field_statistics = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for field_statistics failed".to_string());
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

  pub fn offsets<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.offsets = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for offsets failed".to_string());
    self
  }

  pub fn payloads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.payloads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for payloads failed".to_string());
    self
  }

  pub fn positions<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.positions = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for positions failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn term_statistics<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.term_statistics = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for term_statistics failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::TermvectorsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `TermvectorsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_termvectors`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      field_statistics,
      fields,
      offsets,
      payloads,
      positions,
      preference,
      realtime,
      routing,
      term_statistics,
      version,
      version_type,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let field_statistics = field_statistics.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let offsets = offsets.map_err(Error::InvalidRequest)?;
    let payloads = payloads.map_err(Error::InvalidRequest)?;
    let positions = positions.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let term_statistics = term_statistics.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_termvectors", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(11usize);
    if let Some(v) = &field_statistics {
      query.push(("field_statistics", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &offsets {
      query.push(("offsets", v.to_string()));
    }
    if let Some(v) = &payloads {
      query.push(("payloads", v.to_string()));
    }
    if let Some(v) = &positions {
      query.push(("positions", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &term_statistics {
      query.push(("term_statistics", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
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

///Builder for [`Client::termvectors_get_with_id`]
///
///[`Client::termvectors_get_with_id`]: super::OsClient::termvectors_get_with_id
#[derive(Debug, Clone)]
pub struct TermvectorsGetWithId<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  field_statistics: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  offsets: Result<Option<bool>, String>,
  payloads: Result<Option<bool>, String>,
  positions: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  term_statistics: Result<Option<bool>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
}

impl<'a> TermvectorsGetWithId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      field_statistics: Ok(None),
      fields: Ok(None),
      offsets: Ok(None),
      payloads: Ok(None),
      positions: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      routing: Ok(None),
      term_statistics: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn field_statistics<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.field_statistics = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for field_statistics failed".to_string());
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

  pub fn offsets<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.offsets = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for offsets failed".to_string());
    self
  }

  pub fn payloads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.payloads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for payloads failed".to_string());
    self
  }

  pub fn positions<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.positions = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for positions failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn term_statistics<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.term_statistics = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for term_statistics failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_termvectors/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      id,
      field_statistics,
      fields,
      offsets,
      payloads,
      positions,
      preference,
      realtime,
      routing,
      term_statistics,
      version,
      version_type,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let field_statistics = field_statistics.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let offsets = offsets.map_err(Error::InvalidRequest)?;
    let payloads = payloads.map_err(Error::InvalidRequest)?;
    let positions = positions.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let term_statistics = term_statistics.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_termvectors/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(11usize);
    if let Some(v) = &field_statistics {
      query.push(("field_statistics", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &offsets {
      query.push(("offsets", v.to_string()));
    }
    if let Some(v) = &payloads {
      query.push(("payloads", v.to_string()));
    }
    if let Some(v) = &positions {
      query.push(("positions", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &term_statistics {
      query.push(("term_statistics", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
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

///Builder for [`Client::termvectors_post_with_id`]
///
///[`Client::termvectors_post_with_id`]: super::OsClient::termvectors_post_with_id
#[derive(Debug, Clone)]
pub struct TermvectorsPostWithId<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  field_statistics: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  offsets: Result<Option<bool>, String>,
  payloads: Result<Option<bool>, String>,
  positions: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  term_statistics: Result<Option<bool>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<types::VersionType>, String>,
  body: Result<types::TermvectorsBodyParams, String>,
}

impl<'a> TermvectorsPostWithId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      field_statistics: Ok(None),
      fields: Ok(None),
      offsets: Ok(None),
      payloads: Ok(None),
      positions: Ok(None),
      preference: Ok(None),
      realtime: Ok(None),
      routing: Ok(None),
      term_statistics: Ok(None),
      version: Ok(None),
      version_type: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn field_statistics<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.field_statistics = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for field_statistics failed".to_string());
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

  pub fn offsets<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.offsets = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for offsets failed".to_string());
    self
  }

  pub fn payloads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.payloads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for payloads failed".to_string());
    self
  }

  pub fn positions<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.positions = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for positions failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn realtime<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.realtime = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for realtime failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn term_statistics<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.term_statistics = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for term_statistics failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for version failed".to_string());
    self
  }

  pub fn version_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::TermvectorsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `TermvectorsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_termvectors/{id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      id,
      field_statistics,
      fields,
      offsets,
      payloads,
      positions,
      preference,
      realtime,
      routing,
      term_statistics,
      version,
      version_type,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let field_statistics = field_statistics.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let offsets = offsets.map_err(Error::InvalidRequest)?;
    let payloads = payloads.map_err(Error::InvalidRequest)?;
    let positions = positions.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let term_statistics = term_statistics.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_termvectors/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(11usize);
    if let Some(v) = &field_statistics {
      query.push(("field_statistics", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &offsets {
      query.push(("offsets", v.to_string()));
    }
    if let Some(v) = &payloads {
      query.push(("payloads", v.to_string()));
    }
    if let Some(v) = &positions {
      query.push(("positions", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &realtime {
      query.push(("realtime", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
    }
    if let Some(v) = &term_statistics {
      query.push(("term_statistics", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &version_type {
      query.push(("version_type", v.to_string()));
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

///Builder for [`Client::update`]
///
///[`Client::update`]: super::OsClient::update
#[derive(Debug, Clone)]
pub struct Update<'a> {
  client: &'a super::OsClient,
  index: Result<types::OpenSearchNameValue, String>,
  id: Result<types::OpenSearchId, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  if_primary_term: Result<Option<i32>, String>,
  if_seq_no: Result<Option<i32>, String>,
  lang: Result<Option<String>, String>,
  refresh: Result<Option<types::RefreshEnum>, String>,
  require_alias: Result<Option<bool>, String>,
  retry_on_conflict: Result<Option<i32>, String>,
  routing: Result<Option<String>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  body: Result<types::UpdateBodyParams, String>,
}

impl<'a> Update<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      id: Err("id was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      if_primary_term: Ok(None),
      if_seq_no: Ok(None),
      lang: Ok(None),
      refresh: Ok(None),
      require_alias: Ok(None),
      retry_on_conflict: Ok(None),
      routing: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for id failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn if_primary_term<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.if_primary_term = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for if_primary_term failed".to_string());
    self
  }

  pub fn if_seq_no<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.if_seq_no = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for if_seq_no failed".to_string());
    self
  }

  pub fn lang<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.lang = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for lang failed".to_string());
    self
  }

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::RefreshEnum>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `RefreshEnum` for refresh failed".to_string());
    self
  }

  pub fn require_alias<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.require_alias = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for require_alias failed".to_string());
    self
  }

  pub fn retry_on_conflict<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.retry_on_conflict = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for retry_on_conflict failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for routing failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
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
    V: std::convert::TryInto<types::UpdateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `UpdateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_update/{id}`
  pub async fn send(self) -> Result<ResponseValue<types::IndexResponse>, Error> {
    let Self {
      client,
      index,
      id,
      source,
      source_excludes,
      source_includes,
      if_primary_term,
      if_seq_no,
      lang,
      refresh,
      require_alias,
      retry_on_conflict,
      routing,
      timeout,
      wait_for_active_shards,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let id = id.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let if_primary_term = if_primary_term.map_err(Error::InvalidRequest)?;
    let if_seq_no = if_seq_no.map_err(Error::InvalidRequest)?;
    let lang = lang.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let require_alias = require_alias.map_err(Error::InvalidRequest)?;
    let retry_on_conflict = retry_on_conflict.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}{}/_update/{}",
      client.baseurl,
      encode_path(&index.to_string()),
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &if_primary_term {
      query.push(("if_primary_term", v.to_string()));
    }
    if let Some(v) = &if_seq_no {
      query.push(("if_seq_no", v.to_string()));
    }
    if let Some(v) = &lang {
      query.push(("lang", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &require_alias {
      query.push(("require_alias", v.to_string()));
    }
    if let Some(v) = &retry_on_conflict {
      query.push(("retry_on_conflict", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.to_string()));
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
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::update_by_query`]
///
///[`Client::update_by_query`]: super::OsClient::update_by_query
#[derive(Debug, Clone)]
pub struct UpdateByQuery<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  conflicts: Result<Option<types::Conflicts>, String>,
  default_operator: Result<Option<types::DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  from: Result<Option<i32>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  lenient: Result<Option<bool>, String>,
  max_docs: Result<Option<i32>, String>,
  pipeline: Result<Option<String>, String>,
  preference: Result<Option<String>, String>,
  q: Result<Option<String>, String>,
  refresh: Result<Option<bool>, String>,
  request_cache: Result<Option<bool>, String>,
  requests_per_second: Result<Option<i32>, String>,
  routing: Result<Option<Vec<String>>, String>,
  scroll: Result<Option<types::UpdateByQueryScroll>, String>,
  scroll_size: Result<Option<i32>, String>,
  search_timeout: Result<Option<types::Timeout>, String>,
  search_type: Result<Option<types::SearchType>, String>,
  size: Result<Option<i32>, String>,
  slices: Result<Option<String>, String>,
  sort: Result<Option<Vec<String>>, String>,
  stats: Result<Option<Vec<String>>, String>,
  terminate_after: Result<Option<i32>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  version: Result<Option<bool>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  wait_for_completion: Result<Option<bool>, String>,
  body: Result<types::UpdateByQueryBodyParams, String>,
}

impl<'a> UpdateByQuery<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      allow_no_indices: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      conflicts: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      expand_wildcards: Ok(None),
      from: Ok(None),
      ignore_unavailable: Ok(None),
      lenient: Ok(None),
      max_docs: Ok(None),
      pipeline: Ok(None),
      preference: Ok(None),
      q: Ok(None),
      refresh: Ok(None),
      request_cache: Ok(None),
      requests_per_second: Ok(None),
      routing: Ok(None),
      scroll: Ok(None),
      scroll_size: Ok(None),
      search_timeout: Ok(None),
      search_type: Ok(None),
      size: Ok(None),
      slices: Ok(None),
      sort: Ok(None),
      stats: Ok(None),
      terminate_after: Ok(None),
      timeout: Ok(None),
      version: Ok(None),
      wait_for_active_shards: Ok(None),
      wait_for_completion: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
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

  pub fn conflicts<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Conflicts>, {
    self.conflicts = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Conflicts` for conflicts failed".to_string());
    self
  }

  pub fn default_operator<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::DefaultOperator>, {
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
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn from<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.from = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for from failed".to_string());
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

  pub fn max_docs<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_docs = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_docs failed".to_string());
    self
  }

  pub fn pipeline<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.pipeline = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for pipeline failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
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

  pub fn refresh<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.refresh = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for refresh failed".to_string());
    self
  }

  pub fn request_cache<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.request_cache = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for request_cache failed".to_string());
    self
  }

  pub fn requests_per_second<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.requests_per_second = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for requests_per_second failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for routing failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::UpdateByQueryScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `UpdateByQueryScroll` for scroll failed".to_string());
    self
  }

  pub fn scroll_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.scroll_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for scroll_size failed".to_string());
    self
  }

  pub fn search_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.search_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for search_timeout failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchType>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchType` for search_type failed".to_string());
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

  pub fn slices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.slices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for slices failed".to_string());
    self
  }

  pub fn sort<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.sort = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for sort failed".to_string());
    self
  }

  pub fn stats<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stats = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stats failed".to_string());
    self
  }

  pub fn terminate_after<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.terminate_after = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for terminate_after failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for version failed".to_string());
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
    V: std::convert::TryInto<types::UpdateByQueryBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `UpdateByQueryBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_update_by_query`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      source,
      source_excludes,
      source_includes,
      allow_no_indices,
      analyze_wildcard,
      analyzer,
      conflicts,
      default_operator,
      df,
      expand_wildcards,
      from,
      ignore_unavailable,
      lenient,
      max_docs,
      pipeline,
      preference,
      q,
      refresh,
      request_cache,
      requests_per_second,
      routing,
      scroll,
      scroll_size,
      search_timeout,
      search_type,
      size,
      slices,
      sort,
      stats,
      terminate_after,
      timeout,
      version,
      wait_for_active_shards,
      wait_for_completion,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let conflicts = conflicts.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let from = from.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let max_docs = max_docs.map_err(Error::InvalidRequest)?;
    let pipeline = pipeline.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let refresh = refresh.map_err(Error::InvalidRequest)?;
    let request_cache = request_cache.map_err(Error::InvalidRequest)?;
    let requests_per_second = requests_per_second.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let scroll_size = scroll_size.map_err(Error::InvalidRequest)?;
    let search_timeout = search_timeout.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let size = size.map_err(Error::InvalidRequest)?;
    let slices = slices.map_err(Error::InvalidRequest)?;
    let sort = sort.map_err(Error::InvalidRequest)?;
    let stats = stats.map_err(Error::InvalidRequest)?;
    let terminate_after = terminate_after.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}{}/_update_by_query", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(34usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
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
    if let Some(v) = &conflicts {
      query.push(("conflicts", v.to_string()));
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
    if let Some(v) = &from {
      query.push(("from", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &max_docs {
      query.push(("max_docs", v.to_string()));
    }
    if let Some(v) = &pipeline {
      query.push(("pipeline", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &refresh {
      query.push(("refresh", v.to_string()));
    }
    if let Some(v) = &request_cache {
      query.push(("request_cache", v.to_string()));
    }
    if let Some(v) = &requests_per_second {
      query.push(("requests_per_second", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.join(",")));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &scroll_size {
      query.push(("scroll_size", v.to_string()));
    }
    if let Some(v) = &search_timeout {
      query.push(("search_timeout", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &size {
      query.push(("size", v.to_string()));
    }
    if let Some(v) = &slices {
      query.push(("slices", v.to_string()));
    }
    if let Some(v) = &sort {
      query.push(("sort", v.join(",")));
    }
    if let Some(v) = &stats {
      query.push(("stats", v.join(",")));
    }
    if let Some(v) = &terminate_after {
      query.push(("terminate_after", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    if let Some(v) = &wait_for_active_shards {
      query.push(("wait_for_active_shards", v.to_string()));
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
