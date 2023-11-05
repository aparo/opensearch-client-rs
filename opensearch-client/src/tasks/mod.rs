use crate::OsClient;
mod builder;
mod types;
pub struct Tasks<'a> {
  os_client: &'a OsClient,
}

impl<'a> Tasks<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Returns a list of tasks.
  ///
  ///Sends a `GET` request to `/_tasks`
  ///
  ///Arguments:
  /// - `actions`: Comma-separated list of actions that should be returned.
  ///   Leave empty to return all.
  /// - `detailed`: Return detailed task information.
  /// - `group_by`: Group tasks by nodes or parent/child relationships.
  /// - `nodes`: Comma-separated list of node IDs or names to limit the returned
  ///   information; use `_local` to return information from the node you're
  ///   connecting to, leave empty to get information from all nodes.
  /// - `parent_task_id`: Return tasks with specified parent task id
  ///   (node_id:task_number). Set to -1 to return all.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  ///```ignore
  /// let response = client.tasks_list()
  ///    .actions(actions)
  ///    .detailed(detailed)
  ///    .group_by(group_by)
  ///    .nodes(nodes)
  ///    .parent_task_id(parent_task_id)
  ///    .timeout(timeout)
  ///    .wait_for_completion(wait_for_completion)
  ///    .send()
  ///    .await;
  /// ```
  pub fn tasks_list(&self) -> builder::TasksList {
    builder::TasksList::new(self.os_client)
  }

  ///Cancels a task, if it can be cancelled through an API.
  ///
  ///Sends a `POST` request to `/_tasks/_cancel`
  ///
  ///Arguments:
  /// - `actions`: Comma-separated list of actions that should be cancelled.
  ///   Leave empty to cancel all.
  /// - `nodes`: Comma-separated list of node IDs or names to limit the returned
  ///   information; use `_local` to return information from the node you're
  ///   connecting to, leave empty to get information from all nodes.
  /// - `parent_task_id`: Cancel tasks with specified parent task id
  ///   (node_id:task_number). Set to -1 to cancel all.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  ///```ignore
  /// let response = client.tasks_cancel()
  ///    .actions(actions)
  ///    .nodes(nodes)
  ///    .parent_task_id(parent_task_id)
  ///    .wait_for_completion(wait_for_completion)
  ///    .send()
  ///    .await;
  /// ```
  pub fn tasks_cancel(&self) -> builder::TasksCancel {
    builder::TasksCancel::new(self.os_client)
  }

  ///Returns information about a task.
  ///
  ///Sends a `GET` request to `/_tasks/{task_id}`
  ///
  ///Arguments:
  /// - `task_id`: Return the task with specified id (node_id:task_number).
  /// - `timeout`: Operation timeout.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  ///```ignore
  /// let response = client.tasks_get()
  ///    .task_id(task_id)
  ///    .timeout(timeout)
  ///    .wait_for_completion(wait_for_completion)
  ///    .send()
  ///    .await;
  /// ```
  pub fn tasks_get(&self) -> builder::TasksGet {
    builder::TasksGet::new(self.os_client)
  }

  ///Cancels a task, if it can be cancelled through an API.
  ///
  ///Sends a `POST` request to `/_tasks/{task_id}/_cancel`
  ///
  ///Arguments:
  /// - `task_id`: Cancel the task with specified task id (node_id:task_number).
  /// - `actions`: Comma-separated list of actions that should be cancelled.
  ///   Leave empty to cancel all.
  /// - `nodes`: Comma-separated list of node IDs or names to limit the returned
  ///   information; use `_local` to return information from the node you're
  ///   connecting to, leave empty to get information from all nodes.
  /// - `parent_task_id`: Cancel tasks with specified parent task id
  ///   (node_id:task_number). Set to -1 to cancel all.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  ///```ignore
  /// let response = client.tasks_cancel_with_task_id()
  ///    .task_id(task_id)
  ///    .actions(actions)
  ///    .nodes(nodes)
  ///    .parent_task_id(parent_task_id)
  ///    .wait_for_completion(wait_for_completion)
  ///    .send()
  ///    .await;
  /// ```
  pub fn tasks_cancel_with_task_id(&self) -> builder::TasksCancelWithTaskId {
    builder::TasksCancelWithTaskId::new(self.os_client)
  }
}
