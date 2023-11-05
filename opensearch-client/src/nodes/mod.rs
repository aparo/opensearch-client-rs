use crate::OsClient;
mod builder;
mod types;
pub struct Nodes<'a> {
  os_client: &'a OsClient,
}

impl<'a> Nodes<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Returns information about hot threads on each node in the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/nodes/hot_threads`
  ///
  ///Arguments:
  /// - `ignore_idle_threads`: Don't show threads that are in known-idle places,
  ///   such as waiting on a socket select or pulling from an empty task queue.
  /// - `interval`: The interval for the second sampling of threads.
  /// - `snapshots`: Number of samples of thread stacktrace.
  /// - `threads`: Specify the number of threads to provide information for.
  /// - `timeout`: Operation timeout.
  /// - `type_`: The type to sample.
  ///```ignore
  /// let response = client.nodes().hot_threads_deprecated_dash()
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn hot_threads_deprecated_dash(&self) -> builder::NodesHotThreadsDeprecatedDash {
    builder::NodesHotThreadsDeprecatedDash::new(self.os_client)
  }

  ///Returns information about hot threads on each node in the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/nodes/hotthreads`
  ///
  ///Arguments:
  /// - `ignore_idle_threads`: Don't show threads that are in known-idle places,
  ///   such as waiting on a socket select or pulling from an empty task queue.
  /// - `interval`: The interval for the second sampling of threads.
  /// - `snapshots`: Number of samples of thread stacktrace.
  /// - `threads`: Specify the number of threads to provide information for.
  /// - `timeout`: Operation timeout.
  /// - `type_`: The type to sample.
  ///```ignore
  /// let response = client.nodes().hot_threads_deprecated_cluster()
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn hot_threads_deprecated_cluster(&self) -> builder::NodesHotThreadsDeprecatedCluster {
    builder::NodesHotThreadsDeprecatedCluster::new(self.os_client)
  }

  ///Returns information about hot threads on each node in the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/nodes/{node_id}/hot_threads`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `ignore_idle_threads`: Don't show threads that are in known-idle places,
  ///   such as waiting on a socket select or pulling from an empty task queue.
  /// - `interval`: The interval for the second sampling of threads.
  /// - `snapshots`: Number of samples of thread stacktrace.
  /// - `threads`: Specify the number of threads to provide information for.
  /// - `timeout`: Operation timeout.
  /// - `type_`: The type to sample.
  ///```ignore
  /// let response = client.nodes().hot_threads_with_node_id_deprecated_dash()
  ///    .node_id(node_id)
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn hot_threads_with_node_id_deprecated_dash(&self) -> builder::NodesHotThreadsWithNodeIdDeprecatedDash {
    builder::NodesHotThreadsWithNodeIdDeprecatedDash::new(self.os_client)
  }

  ///Returns information about hot threads on each node in the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/nodes/{node_id}/hotthreads`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `ignore_idle_threads`: Don't show threads that are in known-idle places,
  ///   such as waiting on a socket select or pulling from an empty task queue.
  /// - `interval`: The interval for the second sampling of threads.
  /// - `snapshots`: Number of samples of thread stacktrace.
  /// - `threads`: Specify the number of threads to provide information for.
  /// - `timeout`: Operation timeout.
  /// - `type_`: The type to sample.
  ///```ignore
  /// let response = client.nodes().hot_threads_with_node_id_deprecated_cluster()
  ///    .node_id(node_id)
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn hot_threads_with_node_id_deprecated_cluster(&self) -> builder::NodesHotThreadsWithNodeIdDeprecatedCluster {
    builder::NodesHotThreadsWithNodeIdDeprecatedCluster::new(self.os_client)
  }

  ///Returns information about nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes`
  ///
  ///Arguments:
  /// - `flat_settings`: Return settings in flat format.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes().info()
  ///    .flat_settings(flat_settings)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn info(&self) -> builder::NodesInfo {
    builder::NodesInfo::new(self.os_client)
  }

  ///Returns information about hot threads on each node in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/hot_threads`
  ///
  ///Arguments:
  /// - `ignore_idle_threads`: Don't show threads that are in known-idle places,
  ///   such as waiting on a socket select or pulling from an empty task queue.
  /// - `interval`: The interval for the second sampling of threads.
  /// - `snapshots`: Number of samples of thread stacktrace.
  /// - `threads`: Specify the number of threads to provide information for.
  /// - `timeout`: Operation timeout.
  /// - `type_`: The type to sample.
  ///```ignore
  /// let response = client.nodes().hot_threads()
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn hot_threads(&self) -> builder::NodesHotThreads {
    builder::NodesHotThreads::new(self.os_client)
  }

  ///Returns information about hot threads on each node in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/hotthreads`
  ///
  ///Arguments:
  /// - `ignore_idle_threads`: Don't show threads that are in known-idle places,
  ///   such as waiting on a socket select or pulling from an empty task queue.
  /// - `interval`: The interval for the second sampling of threads.
  /// - `snapshots`: Number of samples of thread stacktrace.
  /// - `threads`: Specify the number of threads to provide information for.
  /// - `timeout`: Operation timeout.
  /// - `type_`: The type to sample.
  ///```ignore
  /// let response = client.nodes().hot_threads_deprecated()
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn hot_threads_deprecated(&self) -> builder::NodesHotThreadsDeprecated {
    builder::NodesHotThreadsDeprecated::new(self.os_client)
  }

  ///Reloads secure settings.
  ///
  ///Sends a `POST` request to `/_nodes/reload_secure_settings`
  ///
  ///Arguments:
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.nodes().reload_secure_settings()
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn reload_secure_settings(&self) -> builder::NodesReloadSecureSettings {
    builder::NodesReloadSecureSettings::new(self.os_client)
  }

  ///Returns statistical information about nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/stats`
  ///
  ///Arguments:
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `level`: Return indices stats aggregated at index, node or shard level.
  /// - `timeout`: Operation timeout.
  /// - `types`: Comma-separated list of document types for the `indexing` index
  ///   metric.
  ///```ignore
  /// let response = client.nodes().stats()
  ///    .completion_fields(completion_fields)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .level(level)
  ///    .timeout(timeout)
  ///    .types(types)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats(&self) -> builder::NodesStats {
    builder::NodesStats::new(self.os_client)
  }

  ///Returns statistical information about nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/stats/{metric}`
  ///
  ///Arguments:
  /// - `metric`: Limit the information returned to the specified metrics.
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `level`: Return indices stats aggregated at index, node or shard level.
  /// - `timeout`: Operation timeout.
  /// - `types`: Comma-separated list of document types for the `indexing` index
  ///   metric.
  ///```ignore
  /// let response = client.nodes().stats_with_metric()
  ///    .metric(metric)
  ///    .completion_fields(completion_fields)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .level(level)
  ///    .timeout(timeout)
  ///    .types(types)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats_with_metric(&self) -> builder::NodesStatsWithMetric {
    builder::NodesStatsWithMetric::new(self.os_client)
  }

  ///Returns statistical information about nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/stats/{metric}/{index_metric}`
  ///
  ///Arguments:
  /// - `metric`: Limit the information returned to the specified metrics.
  /// - `index_metric`: Limit the information returned for `indices` metric to
  ///   the specific index metrics. Isn't used if `indices` (or `all`) metric
  ///   isn't specified.
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `level`: Return indices stats aggregated at index, node or shard level.
  /// - `timeout`: Operation timeout.
  /// - `types`: Comma-separated list of document types for the `indexing` index
  ///   metric.
  ///```ignore
  /// let response = client.nodes().stats_with_index_metric_metric()
  ///    .metric(metric)
  ///    .index_metric(index_metric)
  ///    .completion_fields(completion_fields)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .level(level)
  ///    .timeout(timeout)
  ///    .types(types)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats_with_index_metric_metric(&self) -> builder::NodesStatsWithIndexMetricMetric {
    builder::NodesStatsWithIndexMetricMetric::new(self.os_client)
  }

  ///Returns low-level information about REST actions usage on nodes.
  ///
  ///Sends a `GET` request to `/_nodes/usage`
  ///
  ///Arguments:
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes().usage()
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn usage(&self) -> builder::NodesUsage {
    builder::NodesUsage::new(self.os_client)
  }

  ///Returns low-level information about REST actions usage on nodes.
  ///
  ///Sends a `GET` request to `/_nodes/usage/{metric}`
  ///
  ///Arguments:
  /// - `metric`: Limit the information returned to the specified metrics.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes().usage_with_metric()
  ///    .metric(metric)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn usage_with_metric(&self) -> builder::NodesUsageWithMetric {
    builder::NodesUsageWithMetric::new(self.os_client)
  }

  ///Returns information about nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/{node_id}`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `flat_settings`: Return settings in flat format.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes().info_with_node_id()
  ///    .node_id(node_id)
  ///    .flat_settings(flat_settings)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn info_with_node_id(&self) -> builder::NodesInfoWithNodeId {
    builder::NodesInfoWithNodeId::new(self.os_client)
  }

  ///Returns information about hot threads on each node in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/{node_id}/hot_threads`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `ignore_idle_threads`: Don't show threads that are in known-idle places,
  ///   such as waiting on a socket select or pulling from an empty task queue.
  /// - `interval`: The interval for the second sampling of threads.
  /// - `snapshots`: Number of samples of thread stacktrace.
  /// - `threads`: Specify the number of threads to provide information for.
  /// - `timeout`: Operation timeout.
  /// - `type_`: The type to sample.
  ///```ignore
  /// let response = client.nodes().hot_threads_with_node_id()
  ///    .node_id(node_id)
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn hot_threads_with_node_id(&self) -> builder::NodesHotThreadsWithNodeId {
    builder::NodesHotThreadsWithNodeId::new(self.os_client)
  }

  ///Returns information about hot threads on each node in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/{node_id}/hotthreads`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `ignore_idle_threads`: Don't show threads that are in known-idle places,
  ///   such as waiting on a socket select or pulling from an empty task queue.
  /// - `interval`: The interval for the second sampling of threads.
  /// - `snapshots`: Number of samples of thread stacktrace.
  /// - `threads`: Specify the number of threads to provide information for.
  /// - `timeout`: Operation timeout.
  /// - `type_`: The type to sample.
  ///```ignore
  /// let response = client.nodes().hot_threads_with_node_id_deprecated()
  ///    .node_id(node_id)
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn hot_threads_with_node_id_deprecated(&self) -> builder::NodesHotThreadsWithNodeIdDeprecated {
    builder::NodesHotThreadsWithNodeIdDeprecated::new(self.os_client)
  }

  ///Reloads secure settings.
  ///
  ///Sends a `POST` request to `/_nodes/{node_id}/reload_secure_settings`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs to span the reload/reinit
  ///   call. Should stay empty because reloading usually involves all cluster
  ///   nodes.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.nodes().reload_secure_settings_with_node_id()
  ///    .node_id(node_id)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn reload_secure_settings_with_node_id(&self) -> builder::NodesReloadSecureSettingsWithNodeId {
    builder::NodesReloadSecureSettingsWithNodeId::new(self.os_client)
  }

  ///Returns statistical information about nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/{node_id}/stats`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `level`: Return indices stats aggregated at index, node or shard level.
  /// - `timeout`: Operation timeout.
  /// - `types`: Comma-separated list of document types for the `indexing` index
  ///   metric.
  ///```ignore
  /// let response = client.nodes().stats_with_node_id()
  ///    .node_id(node_id)
  ///    .completion_fields(completion_fields)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .level(level)
  ///    .timeout(timeout)
  ///    .types(types)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats_with_node_id(&self) -> builder::NodesStatsWithNodeId {
    builder::NodesStatsWithNodeId::new(self.os_client)
  }

  ///Returns statistical information about nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/{node_id}/stats/{metric}`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `metric`: Limit the information returned to the specified metrics.
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `level`: Return indices stats aggregated at index, node or shard level.
  /// - `timeout`: Operation timeout.
  /// - `types`: Comma-separated list of document types for the `indexing` index
  ///   metric.
  ///```ignore
  /// let response = client.nodes().stats_with_metric_node_id()
  ///    .node_id(node_id)
  ///    .metric(metric)
  ///    .completion_fields(completion_fields)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .level(level)
  ///    .timeout(timeout)
  ///    .types(types)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats_with_metric_node_id(&self) -> builder::NodesStatsWithMetricNodeId {
    builder::NodesStatsWithMetricNodeId::new(self.os_client)
  }

  ///Returns statistical information about nodes in the cluster.
  ///
  ///Sends a `GET` request to
  /// `/_nodes/{node_id}/stats/{metric}/{index_metric}`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `metric`: Limit the information returned to the specified metrics.
  /// - `index_metric`: Limit the information returned for `indices` metric to
  ///   the specific index metrics. Isn't used if `indices` (or `all`) metric
  ///   isn't specified.
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `level`: Return indices stats aggregated at index, node or shard level.
  /// - `timeout`: Operation timeout.
  /// - `types`: Comma-separated list of document types for the `indexing` index
  ///   metric.
  ///```ignore
  /// let response = client.nodes().stats_with_index_metric_metric_node_id()
  ///    .node_id(node_id)
  ///    .metric(metric)
  ///    .index_metric(index_metric)
  ///    .completion_fields(completion_fields)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .level(level)
  ///    .timeout(timeout)
  ///    .types(types)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats_with_index_metric_metric_node_id(&self) -> builder::NodesStatsWithIndexMetricMetricNodeId {
    builder::NodesStatsWithIndexMetricMetricNodeId::new(self.os_client)
  }

  ///Returns low-level information about REST actions usage on nodes.
  ///
  ///Sends a `GET` request to `/_nodes/{node_id}/usage`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes().usage_with_node_id()
  ///    .node_id(node_id)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn usage_with_node_id(&self) -> builder::NodesUsageWithNodeId {
    builder::NodesUsageWithNodeId::new(self.os_client)
  }

  ///Returns low-level information about REST actions usage on nodes.
  ///
  ///Sends a `GET` request to `/_nodes/{node_id}/usage/{metric}`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `metric`: Limit the information returned to the specified metrics.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes().usage_with_metric_node_id()
  ///    .node_id(node_id)
  ///    .metric(metric)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn usage_with_metric_node_id(&self) -> builder::NodesUsageWithMetricNodeId {
    builder::NodesUsageWithMetricNodeId::new(self.os_client)
  }

  ///Returns information about nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes/{node_id}/{metric}`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `metric`: Comma-separated list of metrics you wish returned. Leave empty
  ///   to return all.
  /// - `flat_settings`: Return settings in flat format.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes().info_with_metric_node_id()
  ///    .node_id(node_id)
  ///    .metric(metric)
  ///    .flat_settings(flat_settings)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn info_with_metric_node_id(&self) -> builder::NodesInfoWithMetricNodeId {
    builder::NodesInfoWithMetricNodeId::new(self.os_client)
  }
}
