#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

///Cancel the task with specified task id (node_id:task_number).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TasksCancelWithTaskIdTaskId(String);
impl std::ops::Deref for TasksCancelWithTaskIdTaskId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<TasksCancelWithTaskIdTaskId> for String {
  fn from(value: TasksCancelWithTaskIdTaskId) -> Self {
    value.0
  }
}
impl From<&TasksCancelWithTaskIdTaskId> for TasksCancelWithTaskIdTaskId {
  fn from(value: &TasksCancelWithTaskIdTaskId) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for TasksCancelWithTaskIdTaskId {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err(
        "doesn't match pattern \"^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$\"",
      );
    }
    Ok(Self(value.to_string()))
  }
}
impl std::convert::TryFrom<&str> for TasksCancelWithTaskIdTaskId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for TasksCancelWithTaskIdTaskId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for TasksCancelWithTaskIdTaskId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for TasksCancelWithTaskIdTaskId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
///Return the task with specified id (node_id:task_number).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TasksGetTaskId(String);
impl std::ops::Deref for TasksGetTaskId {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<TasksGetTaskId> for String {
  fn from(value: TasksGetTaskId) -> Self {
    value.0
  }
}
impl From<&TasksGetTaskId> for TasksGetTaskId {
  fn from(value: &TasksGetTaskId) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for TasksGetTaskId {
  type Err = &'static str;

  fn from_str(value: &str) -> Result<Self, &'static str> {
    if regress::Regex::new("^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err(
        "doesn't match pattern \"^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$\"",
      );
    }
    Ok(Self(value.to_string()))
  }
}
impl std::convert::TryFrom<&str> for TasksGetTaskId {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for TasksGetTaskId {
  type Error = &'static str;

  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for TasksGetTaskId {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for TasksGetTaskId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>, {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
  }
}
