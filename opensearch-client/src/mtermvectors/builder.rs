use crate::types::*;
use super::types;
#[allow(unused_imports)]
use crate::{
  encode_path, encode_path_option_vec_string, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
  ReqwestResponse, ResponseValue,
};

///Builder for [`Client::mtermvectors_get`]
///
///[`Client::mtermvectors_get`]: super::OsClient::mtermvectors_get
#[derive(Debug, Clone)]
pub struct MtermvectorsGet<'a> {
  client: &'a super::OsClient,
  field_statistics: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ids: Result<Option<Vec<String>>, String>,
  offsets: Result<Option<bool>, String>,
  payloads: Result<Option<bool>, String>,
  positions: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  term_statistics: Result<Option<bool>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<VersionType>, String>,
}
impl<'a> MtermvectorsGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      field_statistics: Ok(None),
      fields: Ok(None),
      ids: Ok(None),
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

  pub fn ids<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.ids = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for ids failed".to_string());
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
    V: std::convert::TryInto<VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_mtermvectors`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      field_statistics,
      fields,
      ids,
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
    let field_statistics = field_statistics.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let ids = ids.map_err(Error::InvalidRequest)?;
    let offsets = offsets.map_err(Error::InvalidRequest)?;
    let payloads = payloads.map_err(Error::InvalidRequest)?;
    let positions = positions.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let term_statistics = term_statistics.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_mtermvectors", client.baseurl,);
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &field_statistics {
      query.push(("field_statistics", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ids {
      query.push(("ids", v.join(",")));
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
///Builder for [`Client::mtermvectors_post`]
///
///[`Client::mtermvectors_post`]: super::OsClient::mtermvectors_post
#[derive(Debug, Clone)]
pub struct MtermvectorsPost<'a> {
  client: &'a super::OsClient,
  field_statistics: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ids: Result<Option<Vec<String>>, String>,
  offsets: Result<Option<bool>, String>,
  payloads: Result<Option<bool>, String>,
  positions: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  term_statistics: Result<Option<bool>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<VersionType>, String>,
  body: Result<types::MtermvectorsBodyParams, String>,
}
impl<'a> MtermvectorsPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      field_statistics: Ok(None),
      fields: Ok(None),
      ids: Ok(None),
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

  pub fn ids<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.ids = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for ids failed".to_string());
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
    V: std::convert::TryInto<VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::MtermvectorsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `MtermvectorsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_mtermvectors`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      field_statistics,
      fields,
      ids,
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
    let field_statistics = field_statistics.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let ids = ids.map_err(Error::InvalidRequest)?;
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
    let url = format!("{}/_mtermvectors", client.baseurl,);
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &field_statistics {
      query.push(("field_statistics", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ids {
      query.push(("ids", v.join(",")));
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
///Builder for [`Client::mtermvectors_get_with_index`]
///
///[`Client::mtermvectors_get_with_index`]: super::OsClient::mtermvectors_get_with_index
#[derive(Debug, Clone)]
pub struct MtermvectorsGetWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::MtermvectorsGetWithIndexIndex, String>,
  field_statistics: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ids: Result<Option<Vec<String>>, String>,
  offsets: Result<Option<bool>, String>,
  payloads: Result<Option<bool>, String>,
  positions: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  term_statistics: Result<Option<bool>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<VersionType>, String>,
}
impl<'a> MtermvectorsGetWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      field_statistics: Ok(None),
      fields: Ok(None),
      ids: Ok(None),
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
    V: std::convert::TryInto<types::MtermvectorsGetWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `MtermvectorsGetWithIndexIndex` for index failed".to_string());
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

  pub fn ids<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.ids = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for ids failed".to_string());
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
    V: std::convert::TryInto<VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  ///Sends a `GET` request to `/{index}/_mtermvectors`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      field_statistics,
      fields,
      ids,
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
    let ids = ids.map_err(Error::InvalidRequest)?;
    let offsets = offsets.map_err(Error::InvalidRequest)?;
    let payloads = payloads.map_err(Error::InvalidRequest)?;
    let positions = positions.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let realtime = realtime.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let term_statistics = term_statistics.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let version_type = version_type.map_err(Error::InvalidRequest)?;
    let url = format!("{}/{}/_mtermvectors", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &field_statistics {
      query.push(("field_statistics", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ids {
      query.push(("ids", v.join(",")));
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
///Builder for [`Client::mtermvectors_post_with_index`]
///
///[`Client::mtermvectors_post_with_index`]: super::OsClient::mtermvectors_post_with_index
#[derive(Debug, Clone)]
pub struct MtermvectorsPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::MtermvectorsPostWithIndexIndex, String>,
  field_statistics: Result<Option<bool>, String>,
  fields: Result<Option<Vec<String>>, String>,
  ids: Result<Option<Vec<String>>, String>,
  offsets: Result<Option<bool>, String>,
  payloads: Result<Option<bool>, String>,
  positions: Result<Option<bool>, String>,
  preference: Result<Option<String>, String>,
  realtime: Result<Option<bool>, String>,
  routing: Result<Option<String>, String>,
  term_statistics: Result<Option<bool>, String>,
  version: Result<Option<i32>, String>,
  version_type: Result<Option<VersionType>, String>,
  body: Result<types::MtermvectorsBodyParams, String>,
}
impl<'a> MtermvectorsPostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      field_statistics: Ok(None),
      fields: Ok(None),
      ids: Ok(None),
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
    V: std::convert::TryInto<types::MtermvectorsPostWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `MtermvectorsPostWithIndexIndex` for index failed".to_string());
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

  pub fn ids<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.ids = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for ids failed".to_string());
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
    V: std::convert::TryInto<VersionType>, {
    self.version_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `VersionType` for version_type failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::MtermvectorsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `MtermvectorsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_mtermvectors`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      field_statistics,
      fields,
      ids,
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
    let ids = ids.map_err(Error::InvalidRequest)?;
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
    let url = format!("{}/{}/_mtermvectors", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(12usize);
    if let Some(v) = &field_statistics {
      query.push(("field_statistics", v.to_string()));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &ids {
      query.push(("ids", v.join(",")));
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
