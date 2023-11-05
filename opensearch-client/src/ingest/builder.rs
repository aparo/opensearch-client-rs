use std::collections::HashMap;

use reqwest::Body;
use serde::{de::DeserializeOwned, Serialize};
use opensearch_dsl::Search;

use crate::types::bulk::BulkResponse;
use super::types;
#[allow(unused_imports)]
use crate::{
  encode_path, encode_path_option_vec_string, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
  ReqwestResponse, ResponseValue,
};
///Builder for [`Client::get_pipeline`]
///
///[`Client::get_pipeline`]: super::OsClient::get_pipeline
#[derive(Debug, Clone)]
pub struct IngestGetPipeline<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<types::IngestGetPipelineClusterManagerTimeout>, String>,
  master_timeout: Result<Option<types::IngestGetPipelineMasterTimeout>, String>,
  name: Result<Option<String>, String>,
}

impl<'a> IngestGetPipeline<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      master_timeout: Ok(None),
      name: Ok(None),
    }
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestGetPipelineClusterManagerTimeout>, {
    self.cluster_manager_timeout = value.try_into().map(Some).map_err(|_| {
      "conversion to `IngestGetPipelineClusterManagerTimeout` for cluster_manager_timeout failed".to_string()
    });
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestGetPipelineMasterTimeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IngestGetPipelineMasterTimeout` for master_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_ingest/pipeline`
  pub async fn send(self) -> Result<ResponseValue<HashMap<String, serde_json::Value>>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      master_timeout,
      name,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let url = match name.map_err(Error::InvalidRequest)? {
      Some(name) => format!("{}/_ingest/pipeline/{}", client.baseurl, encode_path(&name.to_string()),),
      None => format!("{}/_ingest/pipeline", client.baseurl,),
    };
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
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

///Builder for [`Client::ingest_simulate_get`]
///
///[`Client::ingest_simulate_get`]: super::OsClient::ingest_simulate_get
#[derive(Debug, Clone)]
pub struct IngestSimulateGet<'a> {
  client: &'a super::OsClient,
  verbose: Result<Option<bool>, String>,
}

impl<'a> IngestSimulateGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      verbose: Ok(None),
    }
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

  ///Sends a `GET` request to `/_ingest/pipeline/_simulate`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, verbose } = self;
    let verbose = verbose.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_ingest/pipeline/_simulate", client.baseurl,);
    let mut query = Vec::with_capacity(1usize);
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

///Builder for [`Client::ingest_simulate_post`]
///
///[`Client::ingest_simulate_post`]: super::OsClient::ingest_simulate_post
#[derive(Debug, Clone)]
pub struct IngestSimulatePost<'a> {
  client: &'a super::OsClient,
  verbose: Result<Option<bool>, String>,
  body: Result<types::IngestSimulateBodyParams, String>,
}

impl<'a> IngestSimulatePost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      verbose: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
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

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestSimulateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IngestSimulateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_ingest/pipeline/_simulate`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, verbose, body } = self;
    let verbose = verbose.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_ingest/pipeline/_simulate", client.baseurl,);
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &verbose {
      query.push(("verbose", v.to_string()));
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

///Builder for [`Client::ingest_get_pipeline_with_id`]
///
///[`Client::ingest_get_pipeline_with_id`]: super::OsClient::ingest_get_pipeline_with_id
#[derive(Debug, Clone)]
pub struct IngestGetPipelineWithId<'a> {
  client: &'a super::OsClient,
  id: Result<types::IngestGetPipelineWithIdId, String>,
  cluster_manager_timeout: Result<Option<types::IngestGetPipelineWithIdClusterManagerTimeout>, String>,
  master_timeout: Result<Option<types::IngestGetPipelineWithIdMasterTimeout>, String>,
}

impl<'a> IngestGetPipelineWithId<'a> {
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
    V: std::convert::TryInto<types::IngestGetPipelineWithIdId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `IngestGetPipelineWithIdId` for id failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestGetPipelineWithIdClusterManagerTimeout>, {
    self.cluster_manager_timeout = value.try_into().map(Some).map_err(|_| {
      "conversion to `IngestGetPipelineWithIdClusterManagerTimeout` for cluster_manager_timeout failed".to_string()
    });
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestGetPipelineWithIdMasterTimeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IngestGetPipelineWithIdMasterTimeout` for master_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_ingest/pipeline/{id}`
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
    let url = format!("{}/_ingest/pipeline/{}", client.baseurl, encode_path(&id.to_string()),);
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

///Builder for [`Client::ingest_put_pipeline`]
///
///[`Client::ingest_put_pipeline`]: super::OsClient::ingest_put_pipeline
#[derive(Debug, Clone)]
pub struct IngestPutPipeline<'a> {
  client: &'a super::OsClient,
  id: Result<types::IngestPutPipelineId, String>,
  cluster_manager_timeout: Result<Option<types::IngestPutPipelineClusterManagerTimeout>, String>,
  master_timeout: Result<Option<types::IngestPutPipelineMasterTimeout>, String>,
  timeout: Result<Option<types::IngestPutPipelineTimeout>, String>,
  body: Result<types::IngestPutPipelineBodyParams, String>,
}

impl<'a> IngestPutPipeline<'a> {
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
    V: std::convert::TryInto<types::IngestPutPipelineId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `IngestPutPipelineId` for id failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestPutPipelineClusterManagerTimeout>, {
    self.cluster_manager_timeout = value.try_into().map(Some).map_err(|_| {
      "conversion to `IngestPutPipelineClusterManagerTimeout` for cluster_manager_timeout failed".to_string()
    });
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestPutPipelineMasterTimeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IngestPutPipelineMasterTimeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestPutPipelineTimeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IngestPutPipelineTimeout` for timeout failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestPutPipelineBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IngestPutPipelineBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_ingest/pipeline/{id}`
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
    let url = format!("{}/_ingest/pipeline/{}", client.baseurl, encode_path(&id.to_string()),);
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

///Builder for [`Client::ingest_delete_pipeline`]
///
///[`Client::ingest_delete_pipeline`]: super::OsClient::ingest_delete_pipeline
#[derive(Debug, Clone)]
pub struct IngestDeletePipeline<'a> {
  client: &'a super::OsClient,
  id: Result<types::IngestDeletePipelineId, String>,
  cluster_manager_timeout: Result<Option<types::IngestDeletePipelineClusterManagerTimeout>, String>,
  master_timeout: Result<Option<types::IngestDeletePipelineMasterTimeout>, String>,
  timeout: Result<Option<types::IngestDeletePipelineTimeout>, String>,
}

impl<'a> IngestDeletePipeline<'a> {
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
    V: std::convert::TryInto<types::IngestDeletePipelineId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `IngestDeletePipelineId` for id failed".to_string());
    self
  }

  pub fn cluster_manager_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestDeletePipelineClusterManagerTimeout>, {
    self.cluster_manager_timeout = value.try_into().map(Some).map_err(|_| {
      "conversion to `IngestDeletePipelineClusterManagerTimeout` for cluster_manager_timeout failed".to_string()
    });
    self
  }

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestDeletePipelineMasterTimeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IngestDeletePipelineMasterTimeout` for master_timeout failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestDeletePipelineTimeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `IngestDeletePipelineTimeout` for timeout failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_ingest/pipeline/{id}`
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
    let url = format!("{}/_ingest/pipeline/{}", client.baseurl, encode_path(&id.to_string()),);
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

///Builder for [`Client::ingest_simulate_get_with_id`]
///
///[`Client::ingest_simulate_get_with_id`]: super::OsClient::ingest_simulate_get_with_id
#[derive(Debug, Clone)]
pub struct IngestSimulateGetWithId<'a> {
  client: &'a super::OsClient,
  id: Result<types::IngestSimulateGetWithIdId, String>,
  verbose: Result<Option<bool>, String>,
}

impl<'a> IngestSimulateGetWithId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
      verbose: Ok(None),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestSimulateGetWithIdId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `IngestSimulateGetWithIdId` for id failed".to_string());
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

  ///Sends a `GET` request to `/_ingest/pipeline/{id}/_simulate`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, id, verbose } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let verbose = verbose.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_ingest/pipeline/{}/_simulate",
      client.baseurl,
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(1usize);
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

///Builder for [`Client::ingest_simulate_post_with_id`]
///
///[`Client::ingest_simulate_post_with_id`]: super::OsClient::ingest_simulate_post_with_id
#[derive(Debug, Clone)]
pub struct IngestSimulatePostWithId<'a> {
  client: &'a super::OsClient,
  id: Result<types::IngestSimulatePostWithIdId, String>,
  verbose: Result<Option<bool>, String>,
  body: Result<types::IngestSimulateBodyParams, String>,
}

impl<'a> IngestSimulatePostWithId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      id: Err("id was not initialized".to_string()),
      verbose: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestSimulatePostWithIdId>, {
    self.id = value
      .try_into()
      .map_err(|_| "conversion to `IngestSimulatePostWithIdId` for id failed".to_string());
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

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IngestSimulateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `IngestSimulateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_ingest/pipeline/{id}/_simulate`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      id,
      verbose,
      body,
    } = self;
    let id = id.map_err(Error::InvalidRequest)?;
    let verbose = verbose.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_ingest/pipeline/{}/_simulate",
      client.baseurl,
      encode_path(&id.to_string()),
    );
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &verbose {
      query.push(("verbose", v.to_string()));
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

///Builder for [`Client::ingest_processor_grok`]
///
///[`Client::ingest_processor_grok`]: super::OsClient::ingest_processor_grok
#[derive(Debug, Clone)]
pub struct IngestProcessorGrok<'a> {
  client: &'a super::OsClient,
}

impl<'a> IngestProcessorGrok<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_ingest/processor/grok`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}/_ingest/processor/grok", client.baseurl,);
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
