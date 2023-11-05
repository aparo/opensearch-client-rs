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

///Builder for [`Client::Cluster::allocation_explain_get`]
///
///[`Client::Cluster::allocation_explain_get`]: super::OsClient::Cluster::allocation_explain_get
#[derive(Debug, Clone)]
pub struct ClusterAllocationExplainGet<'a> {
  client: &'a super::OsClient,
  include_disk_info: Result<Option<bool>, String>,
  include_yes_decisions: Result<Option<bool>, String>,
}
impl<'a> ClusterAllocationExplainGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      include_disk_info: Ok(None),
      include_yes_decisions: Ok(None),
    }
  }

  pub fn include_disk_info<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_disk_info = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_disk_info failed".to_string());
    self
  }

  pub fn include_yes_decisions<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_yes_decisions = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_yes_decisions failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/allocation/explain`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      include_disk_info,
      include_yes_decisions,
    } = self;
    let include_disk_info = include_disk_info.map_err(Error::InvalidRequest)?;
    let include_yes_decisions = include_yes_decisions.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/allocation/explain", client.baseurl,);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &include_disk_info {
      query.push(("include_disk_info", v.to_string()));
    }
    if let Some(v) = &include_yes_decisions {
      query.push(("include_yes_decisions", v.to_string()));
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
///Builder for [`Client::Cluster::allocation_explain_post`]
///
///[`Client::Cluster::allocation_explain_post`]: super::OsClient::Cluster::allocation_explain_post
#[derive(Debug, Clone)]
pub struct ClusterAllocationExplainPost<'a> {
  client: &'a super::OsClient,
  include_disk_info: Result<Option<bool>, String>,
  include_yes_decisions: Result<Option<bool>, String>,
  body: Result<types::ClusterAllocationExplainBodyParams, String>,
}
impl<'a> ClusterAllocationExplainPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      include_disk_info: Ok(None),
      include_yes_decisions: Ok(None),
      body: Err("body was not initialized".to_string()),
    }
  }

  pub fn include_disk_info<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_disk_info = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_disk_info failed".to_string());
    self
  }

  pub fn include_yes_decisions<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_yes_decisions = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_yes_decisions failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterAllocationExplainBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ClusterAllocationExplainBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_cluster/allocation/explain`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      include_disk_info,
      include_yes_decisions,
      body,
    } = self;
    let include_disk_info = include_disk_info.map_err(Error::InvalidRequest)?;
    let include_yes_decisions = include_yes_decisions.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/allocation/explain", client.baseurl,);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &include_disk_info {
      query.push(("include_disk_info", v.to_string()));
    }
    if let Some(v) = &include_yes_decisions {
      query.push(("include_yes_decisions", v.to_string()));
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
///Builder for [`Client::Cluster::delete_decommission_awareness`]
///
///[`Client::Cluster::delete_decommission_awareness`]: super::OsClient::Cluster::delete_decommission_awareness
#[derive(Debug, Clone)]
pub struct ClusterDeleteDecommissionAwareness<'a> {
  client: &'a super::OsClient,
}
impl<'a> ClusterDeleteDecommissionAwareness<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `DELETE` request to `/_cluster/decommission/awareness/`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}/_cluster/decommission/awareness/", client.baseurl,);
    let request = client.client.delete(url).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => Ok(ResponseValue::empty(response)),
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cluster::get_decommission_awareness`]
///
///[`Client::Cluster::get_decommission_awareness`]: super::OsClient::Cluster::get_decommission_awareness
#[derive(Debug, Clone)]
pub struct ClusterGetDecommissionAwareness<'a> {
  client: &'a super::OsClient,
  awareness_attribute_name: Result<types::ClusterGetDecommissionAwarenessAwarenessAttributeName, String>,
}
impl<'a> ClusterGetDecommissionAwareness<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      awareness_attribute_name: Err("awareness_attribute_name was not initialized".to_string()),
    }
  }

  pub fn awareness_attribute_name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterGetDecommissionAwarenessAwarenessAttributeName>, {
    self.awareness_attribute_name = value.try_into().map_err(|_| {
      "conversion to `ClusterGetDecommissionAwarenessAwarenessAttributeName` for awareness_attribute_name failed"
        .to_string()
    });
    self
  }

  ///Sends a `GET` request to
  /// `/_cluster/decommission/awareness/{awareness_attribute_name}/
  /// _status`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      awareness_attribute_name,
    } = self;
    let awareness_attribute_name = awareness_attribute_name.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_cluster/decommission/awareness/{}/_status",
      client.baseurl,
      encode_path(&awareness_attribute_name.to_string()),
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
///Builder for [`Client::Cluster::put_decommission_awareness`]
///
///[`Client::Cluster::put_decommission_awareness`]: super::OsClient::Cluster::put_decommission_awareness
#[derive(Debug, Clone)]
pub struct ClusterPutDecommissionAwareness<'a> {
  client: &'a super::OsClient,
  awareness_attribute_name: Result<types::ClusterPutDecommissionAwarenessAwarenessAttributeName, String>,
  awareness_attribute_value: Result<types::ClusterPutDecommissionAwarenessAwarenessAttributeValue, String>,
}
impl<'a> ClusterPutDecommissionAwareness<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      awareness_attribute_name: Err("awareness_attribute_name was not initialized".to_string()),
      awareness_attribute_value: Err("awareness_attribute_value was not initialized".to_string()),
    }
  }

  pub fn awareness_attribute_name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterPutDecommissionAwarenessAwarenessAttributeName>, {
    self.awareness_attribute_name = value.try_into().map_err(|_| {
      "conversion to `ClusterPutDecommissionAwarenessAwarenessAttributeName` for awareness_attribute_name failed"
        .to_string()
    });
    self
  }

  pub fn awareness_attribute_value<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterPutDecommissionAwarenessAwarenessAttributeValue>, {
    self.awareness_attribute_value = value.try_into().map_err(|_| {
      "conversion to `ClusterPutDecommissionAwarenessAwarenessAttributeValue` for awareness_attribute_value failed"
        .to_string()
    });
    self
  }

  ///Sends a `PUT` request to
  /// `/_cluster/decommission/awareness/{awareness_attribute_name}/
  /// {awareness_attribute_value}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      awareness_attribute_name,
      awareness_attribute_value,
    } = self;
    let awareness_attribute_name = awareness_attribute_name.map_err(Error::InvalidRequest)?;
    let awareness_attribute_value = awareness_attribute_value.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_cluster/decommission/awareness/{}/{}",
      client.baseurl,
      encode_path(&awareness_attribute_name.to_string()),
      encode_path(&awareness_attribute_value.to_string()),
    );
    let request = client.client.put(url).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => Ok(ResponseValue::empty(response)),
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cluster::health`]
///
///[`Client::Cluster::health`]: super::OsClient::Cluster::health
#[derive(Debug, Clone)]
pub struct ClusterHealth<'a> {
  client: &'a super::OsClient,
  awareness_attribute: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  ensure_node_commissioned: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  level: Result<Option<ClusterHealthLevel>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  wait_for_events: Result<Option<WaitForEvents>, String>,
  wait_for_no_initializing_shards: Result<Option<bool>, String>,
  wait_for_no_relocating_shards: Result<Option<bool>, String>,
  wait_for_nodes: Result<Option<String>, String>,
  wait_for_status: Result<Option<WaitForStatus>, String>,
}
impl<'a> ClusterHealth<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      awareness_attribute: Ok(None),
      cluster_manager_timeout: Ok(None),
      ensure_node_commissioned: Ok(None),
      expand_wildcards: Ok(None),
      level: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      wait_for_events: Ok(None),
      wait_for_no_initializing_shards: Ok(None),
      wait_for_no_relocating_shards: Ok(None),
      wait_for_nodes: Ok(None),
      wait_for_status: Ok(None),
    }
  }

  pub fn awareness_attribute<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.awareness_attribute = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for awareness_attribute failed".to_string());
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

  pub fn ensure_node_commissioned<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ensure_node_commissioned = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ensure_node_commissioned failed".to_string());
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<ClusterHealthLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ClusterHealthLevel` for level failed".to_string());
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

  pub fn wait_for_events<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<WaitForEvents>, {
    self.wait_for_events = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `WaitForEvents` for wait_for_events failed".to_string());
    self
  }

  pub fn wait_for_no_initializing_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_no_initializing_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_no_initializing_shards failed".to_string());
    self
  }

  pub fn wait_for_no_relocating_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_no_relocating_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_no_relocating_shards failed".to_string());
    self
  }

  pub fn wait_for_nodes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_nodes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_nodes failed".to_string());
    self
  }

  pub fn wait_for_status<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<WaitForStatus>, {
    self.wait_for_status = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `WaitForStatus` for wait_for_status failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/health`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      awareness_attribute,
      cluster_manager_timeout,
      ensure_node_commissioned,
      expand_wildcards,
      level,
      local,
      master_timeout,
      timeout,
      wait_for_active_shards,
      wait_for_events,
      wait_for_no_initializing_shards,
      wait_for_no_relocating_shards,
      wait_for_nodes,
      wait_for_status,
    } = self;
    let awareness_attribute = awareness_attribute.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let ensure_node_commissioned = ensure_node_commissioned.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let wait_for_events = wait_for_events.map_err(Error::InvalidRequest)?;
    let wait_for_no_initializing_shards = wait_for_no_initializing_shards.map_err(Error::InvalidRequest)?;
    let wait_for_no_relocating_shards = wait_for_no_relocating_shards.map_err(Error::InvalidRequest)?;
    let wait_for_nodes = wait_for_nodes.map_err(Error::InvalidRequest)?;
    let wait_for_status = wait_for_status.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/health", client.baseurl,);
    let mut query = Vec::with_capacity(14usize);
    if let Some(v) = &awareness_attribute {
      query.push(("awareness_attribute", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &ensure_node_commissioned {
      query.push(("ensure_node_commissioned", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
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
    if let Some(v) = &wait_for_events {
      query.push(("wait_for_events", v.to_string()));
    }
    if let Some(v) = &wait_for_no_initializing_shards {
      query.push(("wait_for_no_initializing_shards", v.to_string()));
    }
    if let Some(v) = &wait_for_no_relocating_shards {
      query.push(("wait_for_no_relocating_shards", v.to_string()));
    }
    if let Some(v) = &wait_for_nodes {
      query.push(("wait_for_nodes", v.to_string()));
    }
    if let Some(v) = &wait_for_status {
      query.push(("wait_for_status", v.to_string()));
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
///Builder for [`Client::Cluster::health_with_index`]
///
///[`Client::Cluster::health_with_index`]: super::OsClient::Cluster::health_with_index
#[derive(Debug, Clone)]
pub struct ClusterHealthWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::ClusterHealthWithIndexIndex, String>,
  awareness_attribute: Result<Option<String>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  ensure_node_commissioned: Result<Option<bool>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  level: Result<Option<ClusterHealthLevel>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_active_shards: Result<Option<String>, String>,
  wait_for_events: Result<Option<WaitForEvents>, String>,
  wait_for_no_initializing_shards: Result<Option<bool>, String>,
  wait_for_no_relocating_shards: Result<Option<bool>, String>,
  wait_for_nodes: Result<Option<String>, String>,
  wait_for_status: Result<Option<WaitForStatus>, String>,
}
impl<'a> ClusterHealthWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      awareness_attribute: Ok(None),
      cluster_manager_timeout: Ok(None),
      ensure_node_commissioned: Ok(None),
      expand_wildcards: Ok(None),
      level: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      wait_for_active_shards: Ok(None),
      wait_for_events: Ok(None),
      wait_for_no_initializing_shards: Ok(None),
      wait_for_no_relocating_shards: Ok(None),
      wait_for_nodes: Ok(None),
      wait_for_status: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterHealthWithIndexIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `ClusterHealthWithIndexIndex` for index failed".to_string());
    self
  }

  pub fn awareness_attribute<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.awareness_attribute = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for awareness_attribute failed".to_string());
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

  pub fn ensure_node_commissioned<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ensure_node_commissioned = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ensure_node_commissioned failed".to_string());
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

  pub fn level<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<ClusterHealthLevel>, {
    self.level = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ClusterHealthLevel` for level failed".to_string());
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

  pub fn wait_for_events<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<WaitForEvents>, {
    self.wait_for_events = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `WaitForEvents` for wait_for_events failed".to_string());
    self
  }

  pub fn wait_for_no_initializing_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_no_initializing_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_no_initializing_shards failed".to_string());
    self
  }

  pub fn wait_for_no_relocating_shards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_no_relocating_shards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_no_relocating_shards failed".to_string());
    self
  }

  pub fn wait_for_nodes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.wait_for_nodes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for wait_for_nodes failed".to_string());
    self
  }

  pub fn wait_for_status<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<WaitForStatus>, {
    self.wait_for_status = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `WaitForStatus` for wait_for_status failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/health/{index}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      index,
      awareness_attribute,
      cluster_manager_timeout,
      ensure_node_commissioned,
      expand_wildcards,
      level,
      local,
      master_timeout,
      timeout,
      wait_for_active_shards,
      wait_for_events,
      wait_for_no_initializing_shards,
      wait_for_no_relocating_shards,
      wait_for_nodes,
      wait_for_status,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let awareness_attribute = awareness_attribute.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let ensure_node_commissioned = ensure_node_commissioned.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let level = level.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_active_shards = wait_for_active_shards.map_err(Error::InvalidRequest)?;
    let wait_for_events = wait_for_events.map_err(Error::InvalidRequest)?;
    let wait_for_no_initializing_shards = wait_for_no_initializing_shards.map_err(Error::InvalidRequest)?;
    let wait_for_no_relocating_shards = wait_for_no_relocating_shards.map_err(Error::InvalidRequest)?;
    let wait_for_nodes = wait_for_nodes.map_err(Error::InvalidRequest)?;
    let wait_for_status = wait_for_status.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/health/{}", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(14usize);
    if let Some(v) = &awareness_attribute {
      query.push(("awareness_attribute", v.to_string()));
    }
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &ensure_node_commissioned {
      query.push(("ensure_node_commissioned", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &level {
      query.push(("level", v.to_string()));
    }
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
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
    if let Some(v) = &wait_for_events {
      query.push(("wait_for_events", v.to_string()));
    }
    if let Some(v) = &wait_for_no_initializing_shards {
      query.push(("wait_for_no_initializing_shards", v.to_string()));
    }
    if let Some(v) = &wait_for_no_relocating_shards {
      query.push(("wait_for_no_relocating_shards", v.to_string()));
    }
    if let Some(v) = &wait_for_nodes {
      query.push(("wait_for_nodes", v.to_string()));
    }
    if let Some(v) = &wait_for_status {
      query.push(("wait_for_status", v.to_string()));
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
///Builder for [`Client::Cluster::pending_tasks`]
///
///[`Client::Cluster::pending_tasks`]: super::OsClient::Cluster::pending_tasks
#[derive(Debug, Clone)]
pub struct ClusterPendingTasks<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterPendingTasks<'a> {
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

  ///Sends a `GET` request to `/_cluster/pending_tasks`
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
    let url = format!("{}/_cluster/pending_tasks", client.baseurl,);
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
///Builder for [`Client::Cluster::reroute`]
///
///[`Client::Cluster::reroute`]: super::OsClient::Cluster::reroute
#[derive(Debug, Clone)]
pub struct ClusterReroute<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  dry_run: Result<Option<bool>, String>,
  explain: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  metric: Result<Option<Vec<types::ClusterRerouteMetricMember>>, String>,
  retry_failed: Result<Option<bool>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::ClusterRerouteBodyParams, String>,
}
impl<'a> ClusterReroute<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      dry_run: Ok(None),
      explain: Ok(None),
      master_timeout: Ok(None),
      metric: Ok(None),
      retry_failed: Ok(None),
      timeout: Ok(None),
      body: Err("body was not initialized".to_string()),
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

  pub fn dry_run<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.dry_run = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for dry_run failed".to_string());
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

  pub fn master_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.master_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for master_timeout failed".to_string());
    self
  }

  pub fn metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<types::ClusterRerouteMetricMember>>, {
    self.metric = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < ClusterRerouteMetricMember >` for metric failed".to_string());
    self
  }

  pub fn retry_failed<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.retry_failed = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for retry_failed failed".to_string());
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
    V: std::convert::TryInto<types::ClusterRerouteBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ClusterRerouteBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_cluster/reroute`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      dry_run,
      explain,
      master_timeout,
      metric,
      retry_failed,
      timeout,
      body,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let dry_run = dry_run.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let retry_failed = retry_failed.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/reroute", client.baseurl,);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &dry_run {
      query.push(("dry_run", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    // if let Some(v) = &metric {
    //   query.push(("metric", v.into_iter().map(|p|
    // p.to_string()).collect().join(","))); }
    if let Some(v) = &retry_failed {
      query.push(("retry_failed", v.to_string()));
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
///Builder for [`Client::Cluster::delete_weighted_routing`]
///
///[`Client::Cluster::delete_weighted_routing`]: super::OsClient::Cluster::delete_weighted_routing
#[derive(Debug, Clone)]
pub struct ClusterDeleteWeightedRouting<'a> {
  client: &'a super::OsClient,
}
impl<'a> ClusterDeleteWeightedRouting<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `DELETE` request to `/_cluster/routing/awareness/weights`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}/_cluster/routing/awareness/weights", client.baseurl,);
    let request = client.client.delete(url).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => Ok(ResponseValue::empty(response)),
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cluster::get_weighted_routing`]
///
///[`Client::Cluster::get_weighted_routing`]: super::OsClient::Cluster::get_weighted_routing
#[derive(Debug, Clone)]
pub struct ClusterGetWeightedRouting<'a> {
  client: &'a super::OsClient,
  attribute: Result<types::ClusterGetWeightedRoutingAttribute, String>,
}
impl<'a> ClusterGetWeightedRouting<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      attribute: Err("attribute was not initialized".to_string()),
    }
  }

  pub fn attribute<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterGetWeightedRoutingAttribute>, {
    self.attribute = value
      .try_into()
      .map_err(|_| "conversion to `ClusterGetWeightedRoutingAttribute` for attribute failed".to_string());
    self
  }

  ///Sends a `GET` request to
  /// `/_cluster/routing/awareness/{attribute}/weights`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, attribute } = self;
    let attribute = attribute.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_cluster/routing/awareness/{}/weights",
      client.baseurl,
      encode_path(&attribute.to_string()),
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
///Builder for [`Client::Cluster::put_weighted_routing`]
///
///[`Client::Cluster::put_weighted_routing`]: super::OsClient::Cluster::put_weighted_routing
#[derive(Debug, Clone)]
pub struct ClusterPutWeightedRouting<'a> {
  client: &'a super::OsClient,
  attribute: Result<types::ClusterPutWeightedRoutingAttribute, String>,
}
impl<'a> ClusterPutWeightedRouting<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      attribute: Err("attribute was not initialized".to_string()),
    }
  }

  pub fn attribute<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterPutWeightedRoutingAttribute>, {
    self.attribute = value
      .try_into()
      .map_err(|_| "conversion to `ClusterPutWeightedRoutingAttribute` for attribute failed".to_string());
    self
  }

  ///Sends a `PUT` request to
  /// `/_cluster/routing/awareness/{attribute}/weights`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client, attribute } = self;
    let attribute = attribute.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_cluster/routing/awareness/{}/weights",
      client.baseurl,
      encode_path(&attribute.to_string()),
    );
    let request = client.client.put(url).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
    match response.status().as_u16() {
      200u16 => Ok(ResponseValue::empty(response)),
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}
///Builder for [`Client::Cluster::get_settings`]
///
///[`Client::Cluster::get_settings`]: super::OsClient::Cluster::get_settings
#[derive(Debug, Clone)]
pub struct ClusterGetSettings<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  flat_settings: Result<Option<bool>, String>,
  include_defaults: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterGetSettings<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      flat_settings: Ok(None),
      include_defaults: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
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

  pub fn include_defaults<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.include_defaults = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for include_defaults failed".to_string());
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

  ///Sends a `GET` request to `/_cluster/settings`
  pub async fn send(self) -> Result<ResponseValue<types::ClusterGetSettingsResponseContent>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      flat_settings,
      include_defaults,
      master_timeout,
      timeout,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let include_defaults = include_defaults.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/settings", client.baseurl,);
    let mut query = Vec::with_capacity(5usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &include_defaults {
      query.push(("include_defaults", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::Cluster::put_settings`]
///
///[`Client::Cluster::put_settings`]: super::OsClient::Cluster::put_settings
#[derive(Debug, Clone)]
pub struct ClusterPutSettings<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  flat_settings: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::builder::ClusterPutSettingsBodyParams, String>,
}
impl<'a> ClusterPutSettings<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      cluster_manager_timeout: Ok(None),
      flat_settings: Ok(None),
      master_timeout: Ok(None),
      timeout: Ok(None),
      body: Ok(types::builder::ClusterPutSettingsBodyParams::default()),
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
    V: std::convert::TryInto<types::ClusterPutSettingsBodyParams>, {
    self.body = value
      .try_into()
      .map(From::from)
      .map_err(|_| "conversion to `ClusterPutSettingsBodyParams` for body failed".to_string());
    self
  }

  pub fn body_map<F>(mut self, f: F) -> Self
  where
    F: std::ops::FnOnce(types::builder::ClusterPutSettingsBodyParams) -> types::builder::ClusterPutSettingsBodyParams,
  {
    self.body = self.body.map(f);
    self
  }

  ///Sends a `PUT` request to `/_cluster/settings`
  pub async fn send(self) -> Result<ResponseValue<types::ClusterPutSettingsResponseContent>, Error> {
    let Self {
      client,
      cluster_manager_timeout,
      flat_settings,
      master_timeout,
      timeout,
      body,
    } = self;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let body = body
      .and_then(std::convert::TryInto::<types::ClusterPutSettingsBodyParams>::try_into)
      .map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/settings", client.baseurl,);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &cluster_manager_timeout {
      query.push(("cluster_manager_timeout", v.to_string()));
    }
    if let Some(v) = &flat_settings {
      query.push(("flat_settings", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
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
///Builder for [`Client::Cluster::state`]
///
///[`Client::Cluster::state`]: super::OsClient::Cluster::state
#[derive(Debug, Clone)]
pub struct ClusterState<'a> {
  client: &'a super::OsClient,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  wait_for_metadata_version: Result<Option<i32>, String>,
  wait_for_timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterState<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      wait_for_metadata_version: Ok(None),
      wait_for_timeout: Ok(None),
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

  pub fn wait_for_metadata_version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.wait_for_metadata_version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for wait_for_metadata_version failed".to_string());
    self
  }

  pub fn wait_for_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.wait_for_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for wait_for_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/state`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      local,
      master_timeout,
      wait_for_metadata_version,
      wait_for_timeout,
    } = self;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let wait_for_metadata_version = wait_for_metadata_version.map_err(Error::InvalidRequest)?;
    let wait_for_timeout = wait_for_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/state", client.baseurl,);
    let mut query = Vec::with_capacity(9usize);
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
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_metadata_version {
      query.push(("wait_for_metadata_version", v.to_string()));
    }
    if let Some(v) = &wait_for_timeout {
      query.push(("wait_for_timeout", v.to_string()));
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
///Builder for [`Client::Cluster::state_with_metric`]
///
///[`Client::Cluster::state_with_metric`]: super::OsClient::Cluster::state_with_metric
#[derive(Debug, Clone)]
pub struct ClusterStateWithMetric<'a> {
  client: &'a super::OsClient,
  metric: Result<types::ClusterStateWithMetricMetric, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  wait_for_metadata_version: Result<Option<i32>, String>,
  wait_for_timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterStateWithMetric<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      metric: Err("metric was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      wait_for_metadata_version: Ok(None),
      wait_for_timeout: Ok(None),
    }
  }

  pub fn metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterStateWithMetricMetric>, {
    self.metric = value
      .try_into()
      .map_err(|_| "conversion to `ClusterStateWithMetricMetric` for metric failed".to_string());
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

  pub fn wait_for_metadata_version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.wait_for_metadata_version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for wait_for_metadata_version failed".to_string());
    self
  }

  pub fn wait_for_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.wait_for_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for wait_for_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/state/{metric}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      metric,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      local,
      master_timeout,
      wait_for_metadata_version,
      wait_for_timeout,
    } = self;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let wait_for_metadata_version = wait_for_metadata_version.map_err(Error::InvalidRequest)?;
    let wait_for_timeout = wait_for_timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/state/{}", client.baseurl, encode_path(&metric.to_string()),);
    let mut query = Vec::with_capacity(9usize);
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
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_metadata_version {
      query.push(("wait_for_metadata_version", v.to_string()));
    }
    if let Some(v) = &wait_for_timeout {
      query.push(("wait_for_timeout", v.to_string()));
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
///Builder for [`Client::Cluster::state_with_index_metric`]
///
///[`Client::Cluster::state_with_index_metric`]: super::OsClient::Cluster::state_with_index_metric
#[derive(Debug, Clone)]
pub struct ClusterStateWithIndexMetric<'a> {
  client: &'a super::OsClient,
  metric: Result<types::ClusterStateWithIndexMetricMetric, String>,
  index: Result<types::ClusterStateWithIndexMetricIndex, String>,
  allow_no_indices: Result<Option<bool>, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  expand_wildcards: Result<Option<ExpandWildcards>, String>,
  flat_settings: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  wait_for_metadata_version: Result<Option<i32>, String>,
  wait_for_timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterStateWithIndexMetric<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      metric: Err("metric was not initialized".to_string()),
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      cluster_manager_timeout: Ok(None),
      expand_wildcards: Ok(None),
      flat_settings: Ok(None),
      ignore_unavailable: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
      wait_for_metadata_version: Ok(None),
      wait_for_timeout: Ok(None),
    }
  }

  pub fn metric<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterStateWithIndexMetricMetric>, {
    self.metric = value
      .try_into()
      .map_err(|_| "conversion to `ClusterStateWithIndexMetricMetric` for metric failed".to_string());
    self
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterStateWithIndexMetricIndex>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `ClusterStateWithIndexMetricIndex` for index failed".to_string());
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

  pub fn wait_for_metadata_version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.wait_for_metadata_version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for wait_for_metadata_version failed".to_string());
    self
  }

  pub fn wait_for_timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.wait_for_timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for wait_for_timeout failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_cluster/state/{metric}/{index}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      metric,
      index,
      allow_no_indices,
      cluster_manager_timeout,
      expand_wildcards,
      flat_settings,
      ignore_unavailable,
      local,
      master_timeout,
      wait_for_metadata_version,
      wait_for_timeout,
    } = self;
    let metric = metric.map_err(Error::InvalidRequest)?;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let cluster_manager_timeout = cluster_manager_timeout.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let local = local.map_err(Error::InvalidRequest)?;
    let master_timeout = master_timeout.map_err(Error::InvalidRequest)?;
    let wait_for_metadata_version = wait_for_metadata_version.map_err(Error::InvalidRequest)?;
    let wait_for_timeout = wait_for_timeout.map_err(Error::InvalidRequest)?;
    let url = format!(
      "{}/_cluster/state/{}/{}",
      client.baseurl,
      encode_path(&metric.to_string()),
      encode_path(&index.to_string()),
    );
    let mut query = Vec::with_capacity(9usize);
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
    if let Some(v) = &local {
      query.push(("local", v.to_string()));
    }
    if let Some(v) = &master_timeout {
      query.push(("master_timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_metadata_version {
      query.push(("wait_for_metadata_version", v.to_string()));
    }
    if let Some(v) = &wait_for_timeout {
      query.push(("wait_for_timeout", v.to_string()));
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
///Builder for [`Client::Cluster::stats`]
///
///[`Client::Cluster::stats`]: super::OsClient::Cluster::stats
#[derive(Debug, Clone)]
pub struct ClusterStats<'a> {
  client: &'a super::OsClient,
  flat_settings: Result<Option<bool>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterStats<'a> {
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

  ///Sends a `GET` request to `/_cluster/stats`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      flat_settings,
      timeout,
    } = self;
    let flat_settings = flat_settings.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/stats", client.baseurl,);
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
///Builder for [`Client::Cluster::stats_with_node_id`]
///
///[`Client::Cluster::stats_with_node_id`]: super::OsClient::Cluster::stats_with_node_id
#[derive(Debug, Clone)]
pub struct ClusterStatsWithNodeId<'a> {
  client: &'a super::OsClient,
  node_id: Result<types::ClusterStatsWithNodeIdNodeId, String>,
  flat_settings: Result<Option<bool>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterStatsWithNodeId<'a> {
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
    V: std::convert::TryInto<types::ClusterStatsWithNodeIdNodeId>, {
    self.node_id = value
      .try_into()
      .map_err(|_| "conversion to `ClusterStatsWithNodeIdNodeId` for node_id failed".to_string());
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

  ///Sends a `GET` request to `/_cluster/stats/nodes/{node_id}`
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
    let url = format!(
      "{}/_cluster/stats/nodes/{}",
      client.baseurl,
      encode_path(&node_id.to_string()),
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
///Builder for [`Client::Cluster::post_voting_config_exclusions`]
///
///[`Client::Cluster::post_voting_config_exclusions`]: super::OsClient::Cluster::post_voting_config_exclusions
#[derive(Debug, Clone)]
pub struct ClusterPostVotingConfigExclusions<'a> {
  client: &'a super::OsClient,
  node_ids: Result<Option<String>, String>,
  node_names: Result<Option<String>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterPostVotingConfigExclusions<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      node_ids: Ok(None),
      node_names: Ok(None),
      timeout: Ok(None),
    }
  }

  pub fn node_ids<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.node_ids = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for node_ids failed".to_string());
    self
  }

  pub fn node_names<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.node_names = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for node_names failed".to_string());
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

  ///Sends a `POST` request to `/_cluster/voting_config_exclusions`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      node_ids,
      node_names,
      timeout,
    } = self;
    let node_ids = node_ids.map_err(Error::InvalidRequest)?;
    let node_names = node_names.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/voting_config_exclusions", client.baseurl,);
    let mut query = Vec::with_capacity(3usize);
    if let Some(v) = &node_ids {
      query.push(("node_ids", v.to_string()));
    }
    if let Some(v) = &node_names {
      query.push(("node_names", v.to_string()));
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
///Builder for [`Client::Cluster::delete_voting_config_exclusions`]
///
///[`Client::Cluster::delete_voting_config_exclusions`]: super::OsClient::Cluster::delete_voting_config_exclusions
#[derive(Debug, Clone)]
pub struct ClusterDeleteVotingConfigExclusions<'a> {
  client: &'a super::OsClient,
  wait_for_removal: Result<Option<bool>, String>,
}
impl<'a> ClusterDeleteVotingConfigExclusions<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      wait_for_removal: Ok(None),
    }
  }

  pub fn wait_for_removal<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_removal = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_removal failed".to_string());
    self
  }

  ///Sends a `DELETE` request to `/_cluster/voting_config_exclusions`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      wait_for_removal,
    } = self;
    let wait_for_removal = wait_for_removal.map_err(Error::InvalidRequest)?;
    let url = format!("{}/_cluster/voting_config_exclusions", client.baseurl,);
    let mut query = Vec::with_capacity(1usize);
    if let Some(v) = &wait_for_removal {
      query.push(("wait_for_removal", v.to_string()));
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
///Builder for [`Client::Cluster::get_component_template`]
///
///[`Client::Cluster::get_component_template`]: super::OsClient::Cluster::get_component_template
#[derive(Debug, Clone)]
pub struct ClusterGetComponentTemplate<'a> {
  client: &'a super::OsClient,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterGetComponentTemplate<'a> {
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

  ///Sends a `GET` request to `/_component_template`
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
    let url = format!("{}/_component_template", client.baseurl,);
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
///Builder for [`Client::Cluster::get_component_template_with_name`]
///
///[`Client::Cluster::get_component_template_with_name`]: super::OsClient::Cluster::get_component_template_with_name
#[derive(Debug, Clone)]
pub struct ClusterGetComponentTemplateWithName<'a> {
  client: &'a super::OsClient,
  name: Result<types::ClusterGetComponentTemplateWithNameName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterGetComponentTemplateWithName<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      name: Err("name was not initialized".to_string()),
      cluster_manager_timeout: Ok(None),
      local: Ok(None),
      master_timeout: Ok(None),
    }
  }

  pub fn name<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ClusterGetComponentTemplateWithNameName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `ClusterGetComponentTemplateWithNameName` for name failed".to_string());
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
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
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
    let url = format!(
      "{}/_component_template/{}",
      client.baseurl,
      encode_path(&name.to_string()),
    );
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
///Builder for [`Client::Cluster::put_component_template_put`]
///
///[`Client::Cluster::put_component_template_put`]: super::OsClient::Cluster::put_component_template_put
#[derive(Debug, Clone)]
pub struct ClusterPutComponentTemplatePut<'a> {
  client: &'a super::OsClient,
  name: Result<types::ClusterPutComponentTemplatePutName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::ClusterPutComponentTemplateBodyParams, String>,
}
impl<'a> ClusterPutComponentTemplatePut<'a> {
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
    V: std::convert::TryInto<types::ClusterPutComponentTemplatePutName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `ClusterPutComponentTemplatePutName` for name failed".to_string());
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
    V: std::convert::TryInto<types::ClusterPutComponentTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ClusterPutComponentTemplateBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `PUT` request to `/_component_template/{name}`
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
      "{}/_component_template/{}",
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
///Builder for [`Client::Cluster::put_component_template_post`]
///
///[`Client::Cluster::put_component_template_post`]: super::OsClient::Cluster::put_component_template_post
#[derive(Debug, Clone)]
pub struct ClusterPutComponentTemplatePost<'a> {
  client: &'a super::OsClient,
  name: Result<types::ClusterPutComponentTemplatePostName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  create: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
  body: Result<types::ClusterPutComponentTemplateBodyParams, String>,
}
impl<'a> ClusterPutComponentTemplatePost<'a> {
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
    V: std::convert::TryInto<types::ClusterPutComponentTemplatePostName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `ClusterPutComponentTemplatePostName` for name failed".to_string());
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
    V: std::convert::TryInto<types::ClusterPutComponentTemplateBodyParams>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `ClusterPutComponentTemplateBodyParams` for body failed".to_string());
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
      "{}/_component_template/{}",
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
///Builder for [`Client::Cluster::delete_component_template`]
///
///[`Client::Cluster::delete_component_template`]: super::OsClient::Cluster::delete_component_template
#[derive(Debug, Clone)]
pub struct ClusterDeleteComponentTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<types::ClusterDeleteComponentTemplateName, String>,
  cluster_manager_timeout: Result<Option<Timeout>, String>,
  master_timeout: Result<Option<Timeout>, String>,
  timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterDeleteComponentTemplate<'a> {
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
    V: std::convert::TryInto<types::ClusterDeleteComponentTemplateName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `ClusterDeleteComponentTemplateName` for name failed".to_string());
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
      "{}/_component_template/{}",
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
///Builder for [`Client::Cluster::exists_component_template`]
///
///[`Client::Cluster::exists_component_template`]: super::OsClient::Cluster::exists_component_template
#[derive(Debug, Clone)]
pub struct ClusterExistsComponentTemplate<'a> {
  client: &'a super::OsClient,
  name: Result<types::ClusterExistsComponentTemplateName, String>,
  local: Result<Option<bool>, String>,
  master_timeout: Result<Option<Timeout>, String>,
}
impl<'a> ClusterExistsComponentTemplate<'a> {
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
    V: std::convert::TryInto<types::ClusterExistsComponentTemplateName>, {
    self.name = value
      .try_into()
      .map_err(|_| "conversion to `ClusterExistsComponentTemplateName` for name failed".to_string());
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
      "{}/_component_template/{}",
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
///Builder for [`Client::Cluster::remote_info`]
///
///[`Client::Cluster::remote_info`]: super::OsClient::Cluster::remote_info
#[derive(Debug, Clone)]
pub struct ClusterRemoteInfo<'a> {
  client: &'a super::OsClient,
}
impl<'a> ClusterRemoteInfo<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self { client }
  }

  ///Sends a `GET` request to `/_remote/info`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self { client } = self;
    let url = format!("{}/_remote/info", client.baseurl,);
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
