use crate::OsClient;
mod builder;
mod types;
pub struct Remote<'a> {
  os_client: &'a OsClient,
}

impl<'a> Remote<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Restores from remote store.
  ///
  ///Sends a `POST` request to `/_remotestore/_restore`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  /// - `body`
  ///```ignore
  /// let response = client.remote().store_restore()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .wait_for_completion(wait_for_completion)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn store_restore(&self) -> builder::RemoteStoreRestore {
    builder::RemoteStoreRestore::new(self.os_client)
  }
}
