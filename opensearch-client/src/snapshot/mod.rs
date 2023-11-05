use crate::OsClient;
mod builder;
mod types;
pub struct Snapshot<'a> {
  os_client: &'a OsClient,
}

impl<'a> Snapshot<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Returns information about a repository.
  ///
  ///Sends a `GET` request to `/_snapshot`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.snapshot_get_repository()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_get_repository(&self) -> builder::SnapshotGetRepository {
    builder::SnapshotGetRepository::new(self.os_client)
  }

  ///Returns information about the status of a snapshot.
  ///
  ///Sends a `GET` request to `/_snapshot/_status`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `ignore_unavailable`: Whether to ignore unavailable snapshots, defaults
  ///   to false which means a SnapshotMissingException is thrown.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.snapshot_status()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_status(&self) -> builder::SnapshotStatus {
    builder::SnapshotStatus::new(self.os_client)
  }

  ///Returns information about a repository.
  ///
  ///Sends a `GET` request to `/_snapshot/{repository}`
  ///
  ///Arguments:
  /// - `repository`: Comma-separated list of repository names.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.snapshot_get_repository_with_repository()
  ///    .repository(repository)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_get_repository_with_repository(&self) -> builder::SnapshotGetRepositoryWithRepository {
    builder::SnapshotGetRepositoryWithRepository::new(self.os_client)
  }

  ///Creates a repository.
  ///
  ///Sends a `PUT` request to `/_snapshot/{repository}`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `verify`: Whether to verify the repository after creation.
  /// - `body`
  ///```ignore
  /// let response = client.snapshot_create_repository_put()
  ///    .repository(repository)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .verify(verify)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_create_repository_put(&self) -> builder::SnapshotCreateRepositoryPut {
    builder::SnapshotCreateRepositoryPut::new(self.os_client)
  }

  ///Creates a repository.
  ///
  ///Sends a `POST` request to `/_snapshot/{repository}`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `verify`: Whether to verify the repository after creation.
  /// - `body`
  ///```ignore
  /// let response = client.snapshot_create_repository_post()
  ///    .repository(repository)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .verify(verify)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_create_repository_post(&self) -> builder::SnapshotCreateRepositoryPost {
    builder::SnapshotCreateRepositoryPost::new(self.os_client)
  }

  ///Deletes a repository.
  ///
  ///Sends a `DELETE` request to `/_snapshot/{repository}`
  ///
  ///Arguments:
  /// - `repository`: Name of the snapshot repository to unregister. Wildcard
  ///   (`*`) patterns are supported.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.snapshot_delete_repository()
  ///    .repository(repository)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_delete_repository(&self) -> builder::SnapshotDeleteRepository {
    builder::SnapshotDeleteRepository::new(self.os_client)
  }

  ///Removes stale data from repository.
  ///
  ///Sends a `POST` request to `/_snapshot/{repository}/_cleanup`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.snapshot_cleanup_repository()
  ///    .repository(repository)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_cleanup_repository(&self) -> builder::SnapshotCleanupRepository {
    builder::SnapshotCleanupRepository::new(self.os_client)
  }

  ///Returns information about the status of a snapshot.
  ///
  ///Sends a `GET` request to `/_snapshot/{repository}/_status`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `ignore_unavailable`: Whether to ignore unavailable snapshots, defaults
  ///   to false which means a SnapshotMissingException is thrown.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.snapshot_status_with_repository()
  ///    .repository(repository)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_status_with_repository(&self) -> builder::SnapshotStatusWithRepository {
    builder::SnapshotStatusWithRepository::new(self.os_client)
  }

  ///Verifies a repository.
  ///
  ///Sends a `POST` request to `/_snapshot/{repository}/_verify`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.snapshot_verify_repository()
  ///    .repository(repository)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_verify_repository(&self) -> builder::SnapshotVerifyRepository {
    builder::SnapshotVerifyRepository::new(self.os_client)
  }

  ///Returns information about a snapshot.
  ///
  ///Sends a `GET` request to `/_snapshot/{repository}/{snapshot}`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `snapshot`: Comma-separated list of snapshot names.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `ignore_unavailable`: Whether to ignore unavailable snapshots, defaults
  ///   to false which means a SnapshotMissingException is thrown.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `verbose`: Whether to show verbose snapshot info or only show the basic
  ///   info found in the repository index blob.
  ///```ignore
  /// let response = client.snapshot_get()
  ///    .repository(repository)
  ///    .snapshot(snapshot)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .verbose(verbose)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_get(&self) -> builder::SnapshotGet {
    builder::SnapshotGet::new(self.os_client)
  }

  ///Creates a snapshot in a repository.
  ///
  ///Sends a `PUT` request to `/_snapshot/{repository}/{snapshot}`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `snapshot`: Snapshot name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  /// - `body`
  ///```ignore
  /// let response = client.snapshot_create_put()
  ///    .repository(repository)
  ///    .snapshot(snapshot)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .wait_for_completion(wait_for_completion)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_create_put(&self) -> builder::SnapshotCreatePut {
    builder::SnapshotCreatePut::new(self.os_client)
  }

  ///Creates a snapshot in a repository.
  ///
  ///Sends a `POST` request to `/_snapshot/{repository}/{snapshot}`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `snapshot`: Snapshot name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  /// - `body`
  ///```ignore
  /// let response = client.snapshot_create_post()
  ///    .repository(repository)
  ///    .snapshot(snapshot)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .wait_for_completion(wait_for_completion)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_create_post(&self) -> builder::SnapshotCreatePost {
    builder::SnapshotCreatePost::new(self.os_client)
  }

  ///Deletes a snapshot.
  ///
  ///Sends a `DELETE` request to `/_snapshot/{repository}/{snapshot}`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `snapshot`: Snapshot name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.snapshot_delete()
  ///    .repository(repository)
  ///    .snapshot(snapshot)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_delete(&self) -> builder::SnapshotDelete {
    builder::SnapshotDelete::new(self.os_client)
  }

  ///Clones indices from one snapshot into another snapshot in the same
  /// repository.
  ///
  ///Sends a `PUT` request to
  /// `/_snapshot/{repository}/{snapshot}/_clone/{target_snapshot}`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `snapshot`: Snapshot name.
  /// - `target_snapshot`: The name of the cloned snapshot to create.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `body`
  ///```ignore
  /// let response = client.snapshot_clone()
  ///    .repository(repository)
  ///    .snapshot(snapshot)
  ///    .target_snapshot(target_snapshot)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_clone(&self) -> builder::SnapshotClone {
    builder::SnapshotClone::new(self.os_client)
  }

  ///Restores a snapshot.
  ///
  ///Sends a `POST` request to `/_snapshot/{repository}/{snapshot}/_restore`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `snapshot`: Snapshot name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  /// - `body`
  ///```ignore
  /// let response = client.snapshot_restore()
  ///    .repository(repository)
  ///    .snapshot(snapshot)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .wait_for_completion(wait_for_completion)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_restore(&self) -> builder::SnapshotRestore {
    builder::SnapshotRestore::new(self.os_client)
  }

  ///Returns information about the status of a snapshot.
  ///
  ///Sends a `GET` request to `/_snapshot/{repository}/{snapshot}/_status`
  ///
  ///Arguments:
  /// - `repository`: Repository name.
  /// - `snapshot`: Comma-separated list of snapshot names.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `ignore_unavailable`: Whether to ignore unavailable snapshots, defaults
  ///   to false which means a SnapshotMissingException is thrown.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.snapshot_status_with_repository_snapshot()
  ///    .repository(repository)
  ///    .snapshot(snapshot)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshot_status_with_repository_snapshot(&self) -> builder::SnapshotStatusWithRepositorySnapshot {
    builder::SnapshotStatusWithRepositorySnapshot::new(self.os_client)
  }
}
