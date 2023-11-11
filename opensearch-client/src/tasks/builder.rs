use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::types::{GroupBy, OpenSearchId, Timeout};
use super::types;
#[allow(unused_imports)]
use crate::{
  encode_path, encode_path_option_vec_string, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
  ReqwestResponse, ResponseValue,
};

///Builder for [`Client::tasks_list`]
///
///[`Client::tasks_list`]: super::OsClient::tasks_list
#[derive(Debug, Clone)]
pub struct TasksList<'a> {
  client: &'a super::OsClient,
  actions: Result<Option<Vec<String>>, String>,
  detailed: Result<Option<bool>, String>,
  group_by: Result<Option<GroupBy>, String>,
  nodes: Result<Option<Vec<String>>, String>,
  parent_task_id: Result<Option<String>, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_completion: Result<Option<bool>, String>,
}
impl<'a> TasksList<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      actions: Ok(None),
      detailed: Ok(None),
      group_by: Ok(None),
      nodes: Ok(None),
      parent_task_id: Ok(None),
      timeout: Ok(None),
      wait_for_completion: Ok(None),
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

  pub fn group_by<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<GroupBy>, {
    self.group_by = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `GroupBy` for group_by failed".to_string());
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

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
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

  ///Sends a `GET` request to `/_tasks`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      actions,
      detailed,
      group_by,
      nodes,
      parent_task_id,
      timeout,
      wait_for_completion,
    } = self;
    let actions = actions.map_err(Error::InvalidRequest)?;
    let detailed = detailed.map_err(Error::InvalidRequest)?;
    let group_by = group_by.map_err(Error::InvalidRequest)?;
    let nodes = nodes.map_err(Error::InvalidRequest)?;
    let parent_task_id = parent_task_id.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let url = format!("{}_tasks", client.baseurl,);
    let mut query = Vec::with_capacity(7usize);
    if let Some(v) = &actions {
      query.push(("actions", v.join(",")));
    }
    if let Some(v) = &detailed {
      query.push(("detailed", v.to_string()));
    }
    if let Some(v) = &group_by {
      query.push(("group_by", v.to_string()));
    }
    if let Some(v) = &nodes {
      query.push(("nodes", v.join(",")));
    }
    if let Some(v) = &parent_task_id {
      query.push(("parent_task_id", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_completion {
      query.push(("wait_for_completion", v.to_string()));
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
///Builder for [`Client::tasks_cancel`]
///
///[`Client::tasks_cancel`]: super::OsClient::tasks_cancel
#[derive(Debug, Clone)]
pub struct TasksCancel<'a> {
  client: &'a super::OsClient,
  actions: Result<Option<Vec<String>>, String>,
  nodes: Result<Option<Vec<String>>, String>,
  parent_task_id: Result<Option<String>, String>,
  wait_for_completion: Result<Option<bool>, String>,
}
impl<'a> TasksCancel<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      actions: Ok(None),
      nodes: Ok(None),
      parent_task_id: Ok(None),
      wait_for_completion: Ok(None),
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

  pub fn wait_for_completion<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_completion = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_completion failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_tasks/_cancel`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      actions,
      nodes,
      parent_task_id,
      wait_for_completion,
    } = self;
    let actions = actions.map_err(Error::InvalidRequest)?;
    let nodes = nodes.map_err(Error::InvalidRequest)?;
    let parent_task_id = parent_task_id.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let url = format!("{}_tasks/_cancel", client.baseurl,);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &actions {
      query.push(("actions", v.join(",")));
    }
    if let Some(v) = &nodes {
      query.push(("nodes", v.join(",")));
    }
    if let Some(v) = &parent_task_id {
      query.push(("parent_task_id", v.to_string()));
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
///Builder for [`Client::tasks_get`]
///
///[`Client::tasks_get`]: super::OsClient::tasks_get
#[derive(Debug, Clone)]
pub struct TasksGet<'a> {
  client: &'a super::OsClient,
  task_id: Result<OpenSearchId, String>,
  timeout: Result<Option<Timeout>, String>,
  wait_for_completion: Result<Option<bool>, String>,
}
impl<'a> TasksGet<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      task_id: Err("task_id was not initialized".to_string()),
      timeout: Ok(None),
      wait_for_completion: Ok(None),
    }
  }

  pub fn task_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchId>, {
    self.task_id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for task_id failed".to_string());
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

  pub fn wait_for_completion<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_completion = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_completion failed".to_string());
    self
  }

  ///Sends a `GET` request to `/_tasks/{task_id}`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      task_id,
      timeout,
      wait_for_completion,
    } = self;
    let task_id = task_id.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let url = format!("{}_tasks/{}", client.baseurl, encode_path(&task_id.to_string()),);
    let mut query = Vec::with_capacity(2usize);
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &wait_for_completion {
      query.push(("wait_for_completion", v.to_string()));
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
///Builder for [`Client::tasks_cancel_with_task_id`]
///
///[`Client::tasks_cancel_with_task_id`]: super::OsClient::tasks_cancel_with_task_id
#[derive(Debug, Clone)]
pub struct TasksCancelWithTaskId<'a> {
  client: &'a super::OsClient,
  task_id: Result<OpenSearchId, String>,
  actions: Result<Option<Vec<String>>, String>,
  nodes: Result<Option<Vec<String>>, String>,
  parent_task_id: Result<Option<String>, String>,
  wait_for_completion: Result<Option<bool>, String>,
}
impl<'a> TasksCancelWithTaskId<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      task_id: Err("task_id was not initialized".to_string()),
      actions: Ok(None),
      nodes: Ok(None),
      parent_task_id: Ok(None),
      wait_for_completion: Ok(None),
    }
  }

  pub fn task_id<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<OpenSearchId>, {
    self.task_id = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchId` for task_id failed".to_string());
    self
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

  pub fn wait_for_completion<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.wait_for_completion = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for wait_for_completion failed".to_string());
    self
  }

  ///Sends a `POST` request to `/_tasks/{task_id}/_cancel`
  pub async fn send(self) -> Result<ResponseValue<()>, Error> {
    let Self {
      client,
      task_id,
      actions,
      nodes,
      parent_task_id,
      wait_for_completion,
    } = self;
    let task_id = task_id.map_err(Error::InvalidRequest)?;
    let actions = actions.map_err(Error::InvalidRequest)?;
    let nodes = nodes.map_err(Error::InvalidRequest)?;
    let parent_task_id = parent_task_id.map_err(Error::InvalidRequest)?;
    let wait_for_completion = wait_for_completion.map_err(Error::InvalidRequest)?;
    let url = format!("{}_tasks/{}/_cancel", client.baseurl, encode_path(&task_id.to_string()),);
    let mut query = Vec::with_capacity(4usize);
    if let Some(v) = &actions {
      query.push(("actions", v.join(",")));
    }
    if let Some(v) = &nodes {
      query.push(("nodes", v.join(",")));
    }
    if let Some(v) = &parent_task_id {
      query.push(("parent_task_id", v.to_string()));
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
