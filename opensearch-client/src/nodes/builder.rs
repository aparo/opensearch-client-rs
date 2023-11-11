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

///Builder for [`Client::Nodes::hot_threads_deprecated_dash`]
///
///[`Client::Nodes::hot_threads_deprecated_dash`]: super::OsClient::Nodes::hot_threads_deprecated_dash
#[derive(Debug, Clone)]
pub struct NodesHotThreadsDeprecatedDash<'a> {
  client: &'a super::OsClient,
  ignore_idle_threads: Result<Option<bool>, String>,
  interval: Result<Option<types::NodesHotThreadsDeprecatedDashInterval>, String>,
  snapshots: Result<Option<i32>, String>,
  threads: Result<Option<i32>, String>,
  timeout: Result<Option<Timeout>, String>,
  type_: Result<Option<SampleType>, String>,
}
impl<'a> NodesHotThreadsDeprecatedDash<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      ignore_idle_threads: Ok(None),
      interval: Ok(None),
      snapshots: Ok(None),
      threads: Ok(None),
      timeout: Ok(None),
      type_: Ok(None),
    }
  }

  pub fn ignore_idle_threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_idle_threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_idle_threads failed".to_string());
    self
  }

  pub fn interval<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesHotThreadsDeprecatedDashInterval>, {
    self.interval = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesHotThreadsDeprecatedDashInterval` for interval failed".to_string());
    self
  }

  pub fn snapshots<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.snapshots = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for snapshots failed".to_string());
    self
  }

  pub fn threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for threads failed".to_string());
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

  pub fn type_<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<SampleType>, {
    self.type_ = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SampleType` for type_ failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/nodes/hot_threads`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      ignore_idle_threads,
      interval,
      snapshots,
      threads,
      timeout,
      type_,
    } = self;
    let ignore_idle_threads = ignore_idle_threads.map_err(Error::InvalidRequest)?;
    let interval = interval.map_err(Error::InvalidRequest)?;
    let snapshots = snapshots.map_err(Error::InvalidRequest)?;
    let threads = threads.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let type_ = type_.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cluster/nodes/hot_threads", client.baseurl,);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &ignore_idle_threads {
      query.push(("ignore_idle_threads", v.to_string()));
    }
    if let Some(v) = &interval {
      query.push(("interval", v.to_string()));
    }
    if let Some(v) = &snapshots {
      query.push(("snapshots", v.to_string()));
    }
    if let Some(v) = &threads {
      query.push(("threads", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &type_ {
      query.push(("type", v.to_string()));
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
///Builder for [`Client::Nodes::hot_threads_deprecated_cluster`]
///
///[`Client::Nodes::hot_threads_deprecated_cluster`]: super::OsClient::Nodes::hot_threads_deprecated_cluster
#[derive(Debug, Clone)]
pub struct NodesHotThreadsDeprecatedCluster<'a> {
  client: &'a super::OsClient,
  ignore_idle_threads: Result<Option<bool>, String>,
  interval: Result<Option<types::NodesHotThreadsDeprecatedClusterInterval>, String>,
  snapshots: Result<Option<i32>, String>,
  threads: Result<Option<i32>, String>,
  timeout: Result<Option<Timeout>, String>,
  type_: Result<Option<SampleType>, String>,
}
impl<'a> NodesHotThreadsDeprecatedCluster<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      ignore_idle_threads: Ok(None),
      interval: Ok(None),
      snapshots: Ok(None),
      threads: Ok(None),
      timeout: Ok(None),
      type_: Ok(None),
    }
  }

  pub fn ignore_idle_threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_idle_threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_idle_threads failed".to_string());
    self
  }

  pub fn interval<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesHotThreadsDeprecatedClusterInterval>, {
    self.interval = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesHotThreadsDeprecatedClusterInterval` for interval failed".to_string());
    self
  }

  pub fn snapshots<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.snapshots = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for snapshots failed".to_string());
    self
  }

  pub fn threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for threads failed".to_string());
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

  pub fn type_<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<SampleType>, {
    self.type_ = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SampleType` for type_ failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/nodes/hotthreads`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      ignore_idle_threads,
      interval,
      snapshots,
      threads,
      timeout,
      type_,
    } = self;
    let ignore_idle_threads = ignore_idle_threads.map_err(Error::InvalidRequest)?;
    let interval = interval.map_err(Error::InvalidRequest)?;
    let snapshots = snapshots.map_err(Error::InvalidRequest)?;
    let threads = threads.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let type_ = type_.map_err(Error::InvalidRequest)?;
    let url = format!("{}_cluster/nodes/hotthreads", client.baseurl,);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &ignore_idle_threads {
      query.push(("ignore_idle_threads", v.to_string()));
    }
    if let Some(v) = &interval {
      query.push(("interval", v.to_string()));
    }
    if let Some(v) = &snapshots {
      query.push(("snapshots", v.to_string()));
    }
    if let Some(v) = &threads {
      query.push(("threads", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &type_ {
      query.push(("type", v.to_string()));
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
///Builder for [`Client::Nodes::hot_threads_with_node_id_deprecated_dash`]
///
///[`Client::Nodes::hot_threads_with_node_id_deprecated_dash`]: super::OsClient::Nodes::hot_threads_with_node_id_deprecated_dash
#[derive(Debug, Clone)]
pub struct NodesHotThreadsWithNodeIdDeprecatedDash<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  ignore_idle_threads: Result<Option<bool>, String>,
  interval: Result<Option<types::NodesHotThreadsWithNodeIdDeprecatedDashInterval>, String>,
  snapshots: Result<Option<i32>, String>,
  threads: Result<Option<i32>, String>,
  timeout: Result<Option<Timeout>, String>,
  type_: Result<Option<SampleType>, String>,
}
impl<'a> NodesHotThreadsWithNodeIdDeprecatedDash<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      ignore_idle_threads: Ok(None),
      interval: Ok(None),
      snapshots: Ok(None),
      threads: Ok(None),
      timeout: Ok(None),
      type_: Ok(None),
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

  pub fn ignore_idle_threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_idle_threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_idle_threads failed".to_string());
    self
  }

  pub fn interval<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesHotThreadsWithNodeIdDeprecatedDashInterval>, {
    self.interval = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesHotThreadsWithNodeIdDeprecatedDashInterval` for interval failed".to_string());
    self
  }

  pub fn snapshots<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.snapshots = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for snapshots failed".to_string());
    self
  }

  pub fn threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for threads failed".to_string());
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

  pub fn type_<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<SampleType>, {
    self.type_ = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SampleType` for type_ failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/nodes/{node_id}/hot_threads`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      ignore_idle_threads,
      interval,
      snapshots,
      threads,
      timeout,
      type_,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let ignore_idle_threads = ignore_idle_threads.map_err(Error::InvalidRequest)?;
    let interval = interval.map_err(Error::InvalidRequest)?;
    let snapshots = snapshots.map_err(Error::InvalidRequest)?;
    let threads = threads.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let type_ = type_.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_cluster/nodes/{}/hot_threads",
      client.baseurl,
      encode_path(&node_id.to_string()),
    );
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &ignore_idle_threads {
      query.push(("ignore_idle_threads", v.to_string()));
    }
    if let Some(v) = &interval {
      query.push(("interval", v.to_string()));
    }
    if let Some(v) = &snapshots {
      query.push(("snapshots", v.to_string()));
    }
    if let Some(v) = &threads {
      query.push(("threads", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &type_ {
      query.push(("type", v.to_string()));
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
///Builder for
/// [`Client::Nodes::hot_threads_with_node_id_deprecated_cluster`]
///
///[`Client::Nodes::hot_threads_with_node_id_deprecated_cluster`]: super::OsClient::Nodes::hot_threads_with_node_id_deprecated_cluster
#[derive(Debug, Clone)]
pub struct NodesHotThreadsWithNodeIdDeprecatedCluster<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  ignore_idle_threads: Result<Option<bool>, String>,
  interval: Result<Option<types::NodesHotThreadsWithNodeIdDeprecatedClusterInterval>, String>,
  snapshots: Result<Option<i32>, String>,
  threads: Result<Option<i32>, String>,
  timeout: Result<Option<Timeout>, String>,
  type_: Result<Option<SampleType>, String>,
}
impl<'a> NodesHotThreadsWithNodeIdDeprecatedCluster<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      ignore_idle_threads: Ok(None),
      interval: Ok(None),
      snapshots: Ok(None),
      threads: Ok(None),
      timeout: Ok(None),
      type_: Ok(None),
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

  pub fn ignore_idle_threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_idle_threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_idle_threads failed".to_string());
    self
  }

  pub fn interval<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesHotThreadsWithNodeIdDeprecatedClusterInterval>, {
    self.interval = value.try_into().map(Some).map_err(|_| {
      "conversion to `NodesHotThreadsWithNodeIdDeprecatedClusterInterval` for interval failed".to_string()
    });
    self
  }

  pub fn snapshots<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.snapshots = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for snapshots failed".to_string());
    self
  }

  pub fn threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for threads failed".to_string());
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

  pub fn type_<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<SampleType>, {
    self.type_ = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SampleType` for type_ failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/nodes/{node_id}/hotthreads`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      ignore_idle_threads,
      interval,
      snapshots,
      threads,
      timeout,
      type_,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let ignore_idle_threads = ignore_idle_threads.map_err(Error::InvalidRequest)?;
    let interval = interval.map_err(Error::InvalidRequest)?;
    let snapshots = snapshots.map_err(Error::InvalidRequest)?;
    let threads = threads.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let type_ = type_.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_cluster/nodes/{}/hotthreads",
      client.baseurl,
      encode_path(&node_id.to_string()),
    );
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &ignore_idle_threads {
      query.push(("ignore_idle_threads", v.to_string()));
    }
    if let Some(v) = &interval {
      query.push(("interval", v.to_string()));
    }
    if let Some(v) = &snapshots {
      query.push(("snapshots", v.to_string()));
    }
    if let Some(v) = &threads {
      query.push(("threads", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &type_ {
      query.push(("type", v.to_string()));
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
///Builder for [`Client::Nodes::info`]
///
///[`Client::Nodes::info`]: super::OsClient::Nodes::info
#[derive(Debug, Clone)]
pub struct NodesInfo<'a> {
  client: &'a super::OsClient,
  flat_settings: Result<Option<bool>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> NodesInfo<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      flat_settings: Ok(None),
      timeout: Ok(None),
    }
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      flat_settings,
      timeout,
    } = self;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes", client.baseurl,);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::Nodes::hot_threads`]
///
///[`Client::Nodes::hot_threads`]: super::OsClient::Nodes::hot_threads
#[derive(Debug, Clone)]
pub struct NodesHotThreads<'a> {
  client: &'a super::OsClient,
  ignore_idle_threads: Result<Option<bool>, String>,
  interval: Result<Option<types::NodesHotThreadsInterval>, String>,
  snapshots: Result<Option<i32>, String>,
  threads: Result<Option<i32>, String>,
  timeout: Result<Option<Timeout>, String>,
  type_: Result<Option<SampleType>, String>,
}
impl<'a> NodesHotThreads<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      ignore_idle_threads: Ok(None),
      interval: Ok(None),
      snapshots: Ok(None),
      threads: Ok(None),
      timeout: Ok(None),
      type_: Ok(None),
    }
  }

  pub fn ignore_idle_threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_idle_threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_idle_threads failed".to_string());
    self
  }

  pub fn interval<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesHotThreadsInterval>, {
    self.interval = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesHotThreadsInterval` for interval failed".to_string());
    self
  }

  pub fn snapshots<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.snapshots = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for snapshots failed".to_string());
    self
  }

  pub fn threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for threads failed".to_string());
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

  pub fn type_<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<SampleType>, {
    self.type_ = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SampleType` for type_ failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/hot_threads`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      ignore_idle_threads,
      interval,
      snapshots,
      threads,
      timeout,
      type_,
    } = self;
    let ignore_idle_threads = ignore_idle_threads.map_err(Error::InvalidRequest)?;
    let interval = interval.map_err(Error::InvalidRequest)?;
    let snapshots = snapshots.map_err(Error::InvalidRequest)?;
    let threads = threads.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let type_ = type_.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/hot_threads", client.baseurl,);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &ignore_idle_threads {
      query.push(("ignore_idle_threads", v.to_string()));
    }
    if let Some(v) = &interval {
      query.push(("interval", v.to_string()));
    }
    if let Some(v) = &snapshots {
      query.push(("snapshots", v.to_string()));
    }
    if let Some(v) = &threads {
      query.push(("threads", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &type_ {
      query.push(("type", v.to_string()));
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
///Builder for [`Client::Nodes::hot_threads_deprecated`]
///
///[`Client::Nodes::hot_threads_deprecated`]: super::OsClient::Nodes::hot_threads_deprecated
#[derive(Debug, Clone)]
pub struct NodesHotThreadsDeprecated<'a> {
  client: &'a super::OsClient,
  ignore_idle_threads: Result<Option<bool>, String>,
  interval: Result<Option<types::NodesHotThreadsDeprecatedInterval>, String>,
  snapshots: Result<Option<i32>, String>,
  threads: Result<Option<i32>, String>,
  timeout: Result<Option<Timeout>, String>,
  type_: Result<Option<SampleType>, String>,
}
impl<'a> NodesHotThreadsDeprecated<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      ignore_idle_threads: Ok(None),
      interval: Ok(None),
      snapshots: Ok(None),
      threads: Ok(None),
      timeout: Ok(None),
      type_: Ok(None),
    }
  }

  pub fn ignore_idle_threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_idle_threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_idle_threads failed".to_string());
    self
  }

  pub fn interval<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesHotThreadsDeprecatedInterval>, {
    self.interval = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesHotThreadsDeprecatedInterval` for interval failed".to_string());
    self
  }

  pub fn snapshots<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.snapshots = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for snapshots failed".to_string());
    self
  }

  pub fn threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for threads failed".to_string());
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

  pub fn type_<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<SampleType>, {
    self.type_ = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SampleType` for type_ failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/hotthreads`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      ignore_idle_threads,
      interval,
      snapshots,
      threads,
      timeout,
      type_,
    } = self;
    let ignore_idle_threads = ignore_idle_threads.map_err(Error::InvalidRequest)?;
    let interval = interval.map_err(Error::InvalidRequest)?;
    let snapshots = snapshots.map_err(Error::InvalidRequest)?;
    let threads = threads.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let type_ = type_.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/hotthreads", client.baseurl,);
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &ignore_idle_threads {
      query.push(("ignore_idle_threads", v.to_string()));
    }
    if let Some(v) = &interval {
      query.push(("interval", v.to_string()));
    }
    if let Some(v) = &snapshots {
      query.push(("snapshots", v.to_string()));
    }
    if let Some(v) = &threads {
      query.push(("threads", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &type_ {
      query.push(("type", v.to_string()));
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
///Builder for [`Client::Nodes::reload_secure_settings`]
///
///[`Client::Nodes::reload_secure_settings`]: super::OsClient::Nodes::reload_secure_settings
#[derive(Debug, Clone)]
pub struct NodesReloadSecureSettings<'a> {
  client: &'a super::OsClient,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::NodesReloadSecureSettingsBodyParams, String>,
}
impl<'a> NodesReloadSecureSettings<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
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
    V: std::convert::TryInto<types::NodesReloadSecureSettingsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `NodesReloadSecureSettingsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_nodes/reload_secure_settings`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, timeout, body } = self;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/reload_secure_settings", client.baseurl,);
    let mut query = Vec::with_capacity(1usize);
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
///Builder for [`Client::Nodes::stats`]
///
///[`Client::Nodes::stats`]: super::OsClient::Nodes::stats
#[derive(Debug, Clone)]
pub struct NodesStats<'a> {
  client: &'a super::OsClient,
  completion_fields: Result<Option<Vec<String>>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  level: Result<Option<types::NodesStatLevel>, String>,
  timeout: Result<Option<Timeout>, String>,
  types: Result<Option<Vec<String>>, String>,
}
impl<'a> NodesStats<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      completion_fields: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      level: Ok(None),
      timeout: Ok(None),
      types: Ok(None),
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesStatLevel` for level failed".to_string());
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

  pub fn types<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.types = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for types failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/stats`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      completion_fields,
      fielddata_fields,
      fields,
      groups,
      include_segment_file_sizes,
      level,
      timeout,
      types,
    } = self;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let types = types.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/stats", client.baseurl,);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &types {
      query.push(("types", v.join(",")));
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
///Builder for [`Client::Nodes::stats_with_metric`]
///
///[`Client::Nodes::stats_with_metric`]: super::OsClient::Nodes::stats_with_metric
#[derive(Debug, Clone)]
pub struct NodesStatsWithMetric<'a> {
  client: &'a super::OsClient,
  metric: Result<OpenSearchNameValue, String>,
  completion_fields: Result<Option<Vec<String>>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  level: Result<Option<types::NodesStatLevel>, String>,
  timeout: Result<Option<Timeout>, String>,
  types: Result<Option<Vec<String>>, String>,
}
impl<'a> NodesStatsWithMetric<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      metric: Err("metric was not initialized".to_string()),
      completion_fields: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      level: Ok(None),
      timeout: Ok(None),
      types: Ok(None),
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesStatLevel` for level failed".to_string());
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

  pub fn types<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.types = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for types failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/stats/{metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      metric,
      completion_fields,
      fielddata_fields,
      fields,
      groups,
      include_segment_file_sizes,
      level,
      timeout,
      types,
    } = self;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let types = types.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/stats/{}", client.baseurl, encode_path(&metric.to_string()),);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &types {
      query.push(("types", v.join(",")));
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
///Builder for [`Client::Nodes::stats_with_index_metric_metric`]
///
///[`Client::Nodes::stats_with_index_metric_metric`]: super::OsClient::Nodes::stats_with_index_metric_metric
#[derive(Debug, Clone)]
pub struct NodesStatsWithIndexMetricMetric<'a> {
  client: &'a super::OsClient,
  metric: Result<OpenSearchNameValue, String>,
  index_metric: Result<OpenSearchNameValue, String>,
  completion_fields: Result<Option<Vec<String>>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  level: Result<Option<types::NodesStatLevel>, String>,
  timeout: Result<Option<Timeout>, String>,
  types: Result<Option<Vec<String>>, String>,
}
impl<'a> NodesStatsWithIndexMetricMetric<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      metric: Err("metric was not initialized".to_string()),
      index_metric: Err("index_metric was not initialized".to_string()),
      completion_fields: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      level: Ok(None),
      timeout: Ok(None),
      types: Ok(None),
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

  pub fn index_metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index_metric = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index_metric failed".to_string());
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesStatLevel` for level failed".to_string());
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

  pub fn types<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.types = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for types failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/stats/{metric}/{index_metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      metric,
      index_metric,
      completion_fields,
      fielddata_fields,
      fields,
      groups,
      include_segment_file_sizes,
      level,
      timeout,
      types,
    } = self;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let index_metric = index_metric.map_err(Error::InvalidRequest)?;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let types = types.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_nodes/stats/{}/{}",
      client.baseurl,
      encode_path(&metric.to_string()),
      encode_path(&index_metric.to_string()),
    );
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &types {
      query.push(("types", v.join(",")));
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
///Builder for [`Client::Nodes::usage`]
///
///[`Client::Nodes::usage`]: super::OsClient::Nodes::usage
#[derive(Debug, Clone)]
pub struct NodesUsage<'a> {
  client: &'a super::OsClient,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> NodesUsage<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      timeout: Ok(None),
    }
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

  ///Sends a `GET` request to `/_nodes/usage`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, timeout } = self;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/usage", client.baseurl,);
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::Nodes::usage_with_metric`]
///
///[`Client::Nodes::usage_with_metric`]: super::OsClient::Nodes::usage_with_metric
#[derive(Debug, Clone)]
pub struct NodesUsageWithMetric<'a> {
  client: &'a super::OsClient,
  metric: Result<OpenSearchNameValue, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> NodesUsageWithMetric<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      metric: Err("metric was not initialized".to_string()),
      timeout: Ok(None),
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/usage/{metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      metric,
      timeout,
    } = self;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/usage/{}", client.baseurl, encode_path(&metric.to_string()),);
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::Nodes::info_with_node_id`]
///
///[`Client::Nodes::info_with_node_id`]: super::OsClient::Nodes::info_with_node_id
#[derive(Debug, Clone)]
pub struct NodesInfoWithNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  flat_settings: Result<Option<bool>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> NodesInfoWithNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      flat_settings: Ok(None),
      timeout: Ok(None),
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

  pub fn flat_settings<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.flat_settings = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for flat_settings failed".to_string());
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

  ///Sends a `GET` request to `/_nodes/{node_id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      flat_settings,
      timeout,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/{}", client.baseurl, encode_path(&node_id.to_string()),);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::Nodes::hot_threads_with_node_id`]
///
///[`Client::Nodes::hot_threads_with_node_id`]: super::OsClient::Nodes::hot_threads_with_node_id
#[derive(Debug, Clone)]
pub struct NodesHotThreadsWithNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  ignore_idle_threads: Result<Option<bool>, String>,
  interval: Result<Option<types::NodesHotThreadsWithNodeIdInterval>, String>,
  snapshots: Result<Option<i32>, String>,
  threads: Result<Option<i32>, String>,
  timeout: Result<Option<Timeout>, String>,
  type_: Result<Option<SampleType>, String>,
}
impl<'a> NodesHotThreadsWithNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      ignore_idle_threads: Ok(None),
      interval: Ok(None),
      snapshots: Ok(None),
      threads: Ok(None),
      timeout: Ok(None),
      type_: Ok(None),
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

  pub fn ignore_idle_threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_idle_threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_idle_threads failed".to_string());
    self
  }

  pub fn interval<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesHotThreadsWithNodeIdInterval>, {
    self.interval = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesHotThreadsWithNodeIdInterval` for interval failed".to_string());
    self
  }

  pub fn snapshots<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.snapshots = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for snapshots failed".to_string());
    self
  }

  pub fn threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for threads failed".to_string());
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

  pub fn type_<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<SampleType>, {
    self.type_ = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SampleType` for type_ failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/{node_id}/hot_threads`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      ignore_idle_threads,
      interval,
      snapshots,
      threads,
      timeout,
      type_,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let ignore_idle_threads = ignore_idle_threads.map_err(Error::InvalidRequest)?;
    let interval = interval.map_err(Error::InvalidRequest)?;
    let snapshots = snapshots.map_err(Error::InvalidRequest)?;
    let threads = threads.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let type_ = type_.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_nodes/{}/hot_threads",
      client.baseurl,
      encode_path(&node_id.to_string()),
    );
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &ignore_idle_threads {
      query.push(("ignore_idle_threads", v.to_string()));
    }
    if let Some(v) = &interval {
      query.push(("interval", v.to_string()));
    }
    if let Some(v) = &snapshots {
      query.push(("snapshots", v.to_string()));
    }
    if let Some(v) = &threads {
      query.push(("threads", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &type_ {
      query.push(("type", v.to_string()));
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
///Builder for [`Client::Nodes::hot_threads_with_node_id_deprecated`]
///
///[`Client::Nodes::hot_threads_with_node_id_deprecated`]: super::OsClient::Nodes::hot_threads_with_node_id_deprecated
#[derive(Debug, Clone)]
pub struct NodesHotThreadsWithNodeIdDeprecated<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  ignore_idle_threads: Result<Option<bool>, String>,
  interval: Result<Option<types::NodesHotThreadsWithNodeIdDeprecatedInterval>, String>,
  snapshots: Result<Option<i32>, String>,
  threads: Result<Option<i32>, String>,
  timeout: Result<Option<Timeout>, String>,
  type_: Result<Option<SampleType>, String>,
}
impl<'a> NodesHotThreadsWithNodeIdDeprecated<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      ignore_idle_threads: Ok(None),
      interval: Ok(None),
      snapshots: Ok(None),
      threads: Ok(None),
      timeout: Ok(None),
      type_: Ok(None),
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

  pub fn ignore_idle_threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_idle_threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_idle_threads failed".to_string());
    self
  }

  pub fn interval<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesHotThreadsWithNodeIdDeprecatedInterval>, {
    self.interval = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesHotThreadsWithNodeIdDeprecatedInterval` for interval failed".to_string());
    self
  }

  pub fn snapshots<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.snapshots = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for snapshots failed".to_string());
    self
  }

  pub fn threads<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.threads = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for threads failed".to_string());
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

  pub fn type_<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<SampleType>, {
    self.type_ = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SampleType` for type_ failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/{node_id}/hotthreads`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      ignore_idle_threads,
      interval,
      snapshots,
      threads,
      timeout,
      type_,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let ignore_idle_threads = ignore_idle_threads.map_err(Error::InvalidRequest)?;
    let interval = interval.map_err(Error::InvalidRequest)?;
    let snapshots = snapshots.map_err(Error::InvalidRequest)?;
    let threads = threads.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let type_ = type_.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_nodes/{}/hotthreads",
      client.baseurl,
      encode_path(&node_id.to_string()),
    );
    let mut query = Vec::with_capacity(6usize);
    if let Some(v) = &ignore_idle_threads {
      query.push(("ignore_idle_threads", v.to_string()));
    }
    if let Some(v) = &interval {
      query.push(("interval", v.to_string()));
    }
    if let Some(v) = &snapshots {
      query.push(("snapshots", v.to_string()));
    }
    if let Some(v) = &threads {
      query.push(("threads", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &type_ {
      query.push(("type", v.to_string()));
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
///Builder for [`Client::Nodes::reload_secure_settings_with_node_id`]
///
///[`Client::Nodes::reload_secure_settings_with_node_id`]: super::OsClient::Nodes::reload_secure_settings_with_node_id
#[derive(Debug, Clone)]
pub struct NodesReloadSecureSettingsWithNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::NodesReloadSecureSettingsBodyParams, String>,
}
impl<'a> NodesReloadSecureSettingsWithNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
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
    V: std::convert::TryInto<types::NodesReloadSecureSettingsBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `NodesReloadSecureSettingsBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_nodes/{node_id}/reload_secure_settings`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      timeout,
      body,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_nodes/{}/reload_secure_settings",
      client.baseurl,
      encode_path(&node_id.to_string()),
    );
    let mut query = Vec::with_capacity(1usize);
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
///Builder for [`Client::Nodes::stats_with_node_id`]
///
///[`Client::Nodes::stats_with_node_id`]: super::OsClient::Nodes::stats_with_node_id
#[derive(Debug, Clone)]
pub struct NodesStatsWithNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  completion_fields: Result<Option<Vec<String>>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  level: Result<Option<types::NodesStatLevel>, String>,
  timeout: Result<Option<Timeout>, String>,
  types: Result<Option<Vec<String>>, String>,
}
impl<'a> NodesStatsWithNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      completion_fields: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      level: Ok(None),
      timeout: Ok(None),
      types: Ok(None),
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

  pub fn completion_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.completion_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for completion_fields failed".to_string());
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesStatLevel` for level failed".to_string());
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

  pub fn types<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.types = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for types failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/{node_id}/stats`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      completion_fields,
      fielddata_fields,
      fields,
      groups,
      include_segment_file_sizes,
      level,
      timeout,
      types,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let types = types.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/{}/stats", client.baseurl, encode_path(&node_id.to_string()),);
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &types {
      query.push(("types", v.join(",")));
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
///Builder for [`Client::Nodes::stats_with_metric_node_id`]
///
///[`Client::Nodes::stats_with_metric_node_id`]: super::OsClient::Nodes::stats_with_metric_node_id
#[derive(Debug, Clone)]
pub struct NodesStatsWithMetricNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  metric: Result<OpenSearchNameValue, String>,
  completion_fields: Result<Option<Vec<String>>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  level: Result<Option<types::NodesStatLevel>, String>,
  timeout: Result<Option<Timeout>, String>,
  types: Result<Option<Vec<String>>, String>,
}
impl<'a> NodesStatsWithMetricNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      metric: Err("metric was not initialized".to_string()),
      completion_fields: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      level: Ok(None),
      timeout: Ok(None),
      types: Ok(None),
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesStatLevel` for level failed".to_string());
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

  pub fn types<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.types = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for types failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/{node_id}/stats/{metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      metric,
      completion_fields,
      fielddata_fields,
      fields,
      groups,
      include_segment_file_sizes,
      level,
      timeout,
      types,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let types = types.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_nodes/{}/stats/{}",
      client.baseurl,
      encode_path(&node_id.to_string()),
      encode_path(&metric.to_string()),
    );
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &types {
      query.push(("types", v.join(",")));
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
///Builder for [`Client::Nodes::stats_with_index_metric_metric_node_id`]
///
///[`Client::Nodes::stats_with_index_metric_metric_node_id`]: super::OsClient::Nodes::stats_with_index_metric_metric_node_id
#[derive(Debug, Clone)]
pub struct NodesStatsWithIndexMetricMetricNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  metric: Result<OpenSearchNameValue, String>,
  index_metric: Result<OpenSearchNameValue, String>,
  completion_fields: Result<Option<Vec<String>>, String>,
  fielddata_fields: Result<Option<Vec<String>>, String>,
  fields: Result<Option<Vec<String>>, String>,
  groups: Result<Option<Vec<String>>, String>,
  include_segment_file_sizes: Result<Option<bool>, String>,
  level: Result<Option<types::NodesStatLevel>, String>,
  timeout: Result<Option<Timeout>, String>,
  types: Result<Option<Vec<String>>, String>,
}
impl<'a> NodesStatsWithIndexMetricMetricNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      metric: Err("metric was not initialized".to_string()),
      index_metric: Err("index_metric was not initialized".to_string()),
      completion_fields: Ok(None),
      fielddata_fields: Ok(None),
      fields: Ok(None),
      groups: Ok(None),
      include_segment_file_sizes: Ok(None),
      level: Ok(None),
      timeout: Ok(None),
      types: Ok(None),
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

  pub fn metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.metric = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for metric failed".to_string());
    self
  }

  pub fn index_metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.index_metric = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index_metric failed".to_string());
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::NodesStatLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `NodesStatLevel` for level failed".to_string());
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

  pub fn types<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.types = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for types failed".to_string());
    self
  }

  ///Sends a `GET` request to
  /// `/_nodes/{node_id}/stats/{metric}/{index_metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      metric,
      index_metric,
      completion_fields,
      fielddata_fields,
      fields,
      groups,
      include_segment_file_sizes,
      level,
      timeout,
      types,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let index_metric = index_metric.map_err(Error::InvalidRequest)?;
    let completion_fields = completion_fields.map_err(Error::InvalidRequest)?;
    let fielddata_fields = fielddata_fields.map_err(Error::InvalidRequest)?;
    let fields = fields.map_err(Error::InvalidRequest)?;
    let groups = groups.map_err(Error::InvalidRequest)?;
    let include_segment_file_sizes = include_segment_file_sizes.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let types = types.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_nodes/{}/stats/{}/{}",
      client.baseurl,
      encode_path(&node_id.to_string()),
      encode_path(&metric.to_string()),
      encode_path(&index_metric.to_string()),
    );
    let mut query = Vec::with_capacity(8usize);
    if let Some(v) = &completion_fields {
      query.push(("completion_fields", v.join(",")));
    }
    if let Some(v) = &fielddata_fields {
      query.push(("fielddata_fields", v.join(",")));
    }
    if let Some(v) = &fields {
      query.push(("fields", v.join(",")));
    }
    if let Some(v) = &groups {
      query.push(("groups", v.join(",")));
    }
    if let Some(v) = &include_segment_file_sizes {
      query.push(("include_segment_file_sizes", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &types {
      query.push(("types", v.join(",")));
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
///Builder for [`Client::Nodes::usage_with_node_id`]
///
///[`Client::Nodes::usage_with_node_id`]: super::OsClient::Nodes::usage_with_node_id
#[derive(Debug, Clone)]
pub struct NodesUsageWithNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> NodesUsageWithNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      timeout: Ok(None),
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/{node_id}/usage`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      timeout,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}_nodes/{}/usage", client.baseurl, encode_path(&node_id.to_string()),);
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::Nodes::usage_with_metric_node_id`]
///
///[`Client::Nodes::usage_with_metric_node_id`]: super::OsClient::Nodes::usage_with_metric_node_id
#[derive(Debug, Clone)]
pub struct NodesUsageWithMetricNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  metric: Result<OpenSearchNameValue, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> NodesUsageWithMetricNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      metric: Err("metric was not initialized".to_string()),
      timeout: Ok(None),
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

  pub fn metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.metric = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for metric failed".to_string());
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

  ///Sends a `GET` request to `/_nodes/{node_id}/usage/{metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      metric,
      timeout,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_nodes/{}/usage/{}",
      client.baseurl,
      encode_path(&node_id.to_string()),
      encode_path(&metric.to_string()),
    );
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::Nodes::info_with_metric_node_id`]
///
///[`Client::Nodes::info_with_metric_node_id`]: super::OsClient::Nodes::info_with_metric_node_id
#[derive(Debug, Clone)]
pub struct NodesInfoWithMetricNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<OpenSearchId, String>,
  metric: Result<OpenSearchNameValue, String>,
  flat_settings: Result<Option<bool>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> NodesInfoWithMetricNodeId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_id: Err("node_id was not initialized".to_string()),
      metric: Err("metric was not initialized".to_string()),
      flat_settings: Ok(None),
      timeout: Ok(None),
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

  pub fn metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchNameValue>, {
    self.metric = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for metric failed".to_string());
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_nodes/{node_id}/{metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_id,
      metric,
      flat_settings,
      timeout,
    } = self;
    let node_id = node_id.map_err(Error::InvalidRequest)?;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}_nodes/{}/{}",
      client.baseurl,
      encode_path(&node_id.to_string()),
      encode_path(&metric.to_string()),
    );
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
