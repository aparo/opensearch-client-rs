use crate::OsClient;
mod builder;
mod types;
pub struct Cat<'a> {
  os_client: &'a OsClient,
}

impl<'a> Cat<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Returns help for the Cat APIs.
  ///
  ///Sends a `GET` request to `/_cat`
  ///
  ///Arguments:
  /// - `help`: Return help information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  ///```ignore
  /// let response = client.cat().help()
  ///    .help(help)
  ///    .s(s)
  ///    .send()
  ///    .await;
  /// ```
  pub fn help(&self) -> builder::CatHelp {
    builder::CatHelp::new(self.os_client)
  }

  ///Shows information about currently configured aliases to indices
  /// including filter and routing infos.
  ///
  ///Sends a `GET` request to `/_cat/aliases`
  ///
  ///Arguments:
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().aliases()
  ///    .expand_wildcards(expand_wildcards)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn aliases(&self) -> builder::CatAliases {
    builder::CatAliases::new(self.os_client)
  }

  ///Shows information about currently configured aliases to indices
  /// including filter and routing infos.
  ///
  ///Sends a `GET` request to `/_cat/aliases/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of alias names.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().aliases_with_name()
  ///    .name(name)
  ///    .expand_wildcards(expand_wildcards)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn aliases_with_name(&self) -> builder::CatAliasesWithName {
    builder::CatAliasesWithName::new(self.os_client)
  }

  ///Provides a snapshot of how many shards are allocated to each data node
  /// and how much disk space they are using.
  ///
  ///Sends a `GET` request to `/_cat/allocation`
  ///
  ///Arguments:
  /// - `bytes`: The unit in which to display byte values.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().allocation()
  ///    .bytes(bytes)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn allocation(&self) -> builder::CatAllocation {
    builder::CatAllocation::new(self.os_client)
  }

  ///Provides a snapshot of how many shards are allocated to each data node
  /// and how much disk space they are using.
  ///
  ///Sends a `GET` request to `/_cat/allocation/{node_id}`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information.
  /// - `bytes`: The unit in which to display byte values.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().allocation_with_node_id()
  ///    .node_id(node_id)
  ///    .bytes(bytes)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn allocation_with_node_id(&self) -> builder::CatAllocationWithNodeId {
    builder::CatAllocationWithNodeId::new(self.os_client)
  }

  ///Returns information about the cluster-manager node.
  ///
  ///Sends a `GET` request to `/_cat/cluster_manager`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().cluster_manager()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_manager(&self) -> builder::CatClusterManager {
    builder::CatClusterManager::new(self.os_client)
  }

  ///Provides quick access to the document count of the entire cluster, or
  /// individual indices.
  ///
  ///Sends a `GET` request to `/_cat/count`
  ///
  ///Arguments:
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().count()
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn count(&self) -> builder::CatCount {
    builder::CatCount::new(self.os_client)
  }

  ///Provides quick access to the document count of the entire cluster, or
  /// individual indices.
  ///
  ///Sends a `GET` request to `/_cat/count/{index}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to limit the returned
  ///   information.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().count_with_index()
  ///    .index(index)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn count_with_index(&self) -> builder::CatCountWithIndex {
    builder::CatCountWithIndex::new(self.os_client)
  }

  ///Shows how much heap memory is currently being used by fielddata on every
  /// data node in the cluster.
  ///
  ///Sends a `GET` request to `/_cat/fielddata`
  ///
  ///Arguments:
  /// - `bytes`: The unit in which to display byte values.
  /// - `fields`: Comma-separated list of fields to return in the output.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().fielddata()
  ///    .bytes(bytes)
  ///    .fields(fields)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn fielddata(&self) -> builder::CatFielddata {
    builder::CatFielddata::new(self.os_client)
  }

  ///Shows how much heap memory is currently being used by fielddata on every
  /// data node in the cluster.
  ///
  ///Sends a `GET` request to `/_cat/fielddata/{fields}`
  ///
  ///Arguments:
  /// - `bytes`: The unit in which to display byte values.
  /// - `fields`: Comma-separated list of fields to return in the output.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().fielddata_with_fields()
  ///    .bytes(bytes)
  ///    .fields(fields)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn fielddata_with_fields(&self) -> builder::CatFielddataWithFields {
    builder::CatFielddataWithFields::new(self.os_client)
  }

  ///Returns a concise representation of the cluster health.
  ///
  ///Sends a `GET` request to `/_cat/health`
  ///
  ///Arguments:
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `ts`: Set to false to disable timestamping.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().health()
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .s(s)
  ///    .time(time)
  ///    .ts(ts)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn health(&self) -> builder::CatHealth {
    builder::CatHealth::new(self.os_client)
  }

  ///Returns information about indices: number of primaries and replicas,
  /// document counts, disk size, ...
  ///
  ///Sends a `GET` request to `/_cat/indices`
  ///
  ///Arguments:
  /// - `bytes`: The unit in which to display byte values.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `health`: Health status ('green', 'yellow', or 'red') to filter only
  ///   indices matching the specified health status.
  /// - `help`: Return help information.
  /// - `include_unloaded_segments`: If set to true segment stats will include
  ///   stats for segments that are not currently loaded into memory.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `pri`: Set to true to return stats only for primary shards.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().indices()
  ///    .bytes(bytes)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .format(format)
  ///    .h(h)
  ///    .health(health)
  ///    .help(help)
  ///    .include_unloaded_segments(include_unloaded_segments)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .pri(pri)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices(&self) -> builder::CatIndices {
    builder::CatIndices::new(self.os_client)
  }

  ///Returns information about indices: number of primaries and replicas,
  /// document counts, disk size, ...
  ///
  ///Sends a `GET` request to `/_cat/indices/{index}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to limit the returned
  ///   information.
  /// - `bytes`: The unit in which to display byte values.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `health`: Health status ('green', 'yellow', or 'red') to filter only
  ///   indices matching the specified health status.
  /// - `help`: Return help information.
  /// - `include_unloaded_segments`: If set to true segment stats will include
  ///   stats for segments that are not currently loaded into memory.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `pri`: Set to true to return stats only for primary shards.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().indices_with_index()
  ///    .index(index)
  ///    .bytes(bytes)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .format(format)
  ///    .h(h)
  ///    .health(health)
  ///    .help(help)
  ///    .include_unloaded_segments(include_unloaded_segments)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .pri(pri)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_with_index(&self) -> builder::CatIndicesWithIndex {
    builder::CatIndicesWithIndex::new(self.os_client)
  }

  ///Returns information about the cluster-manager node.
  ///
  ///Sends a `GET` request to `/_cat/master`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().master()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn master(&self) -> builder::CatMaster {
    builder::CatMaster::new(self.os_client)
  }

  ///Returns information about custom node attributes.
  ///
  ///Sends a `GET` request to `/_cat/nodeattrs`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().nodeattrs()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodeattrs(&self) -> builder::CatNodeattrs {
    builder::CatNodeattrs::new(self.os_client)
  }

  ///Returns basic statistics about performance of cluster nodes.
  ///
  ///Sends a `GET` request to `/_cat/nodes`
  ///
  ///Arguments:
  /// - `bytes`: The unit in which to display byte values.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `full_id`: Return the full node ID instead of the shortened version.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().nodes()
  ///    .bytes(bytes)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .full_id(full_id)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes(&self) -> builder::CatNodes {
    builder::CatNodes::new(self.os_client)
  }

  ///Returns a concise representation of the cluster pending tasks.
  ///
  ///Sends a `GET` request to `/_cat/pending_tasks`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().pending_tasks()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn pending_tasks(&self) -> builder::CatPendingTasks {
    builder::CatPendingTasks::new(self.os_client)
  }

  ///List segments for one or several PITs.
  ///
  ///Sends a `GET` request to `/_cat/pit_segments`
  ///
  ///```ignore
  /// let response = client.cat().pit_segments()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn pit_segments(&self) -> builder::CatPitSegments {
    builder::CatPitSegments::new(self.os_client)
  }

  ///Lists all active point-in-time segments.
  ///
  ///Sends a `GET` request to `/_cat/pit_segments/_all`
  ///
  ///```ignore
  /// let response = client.cat().all_pit_segments()
  ///    .send()
  ///    .await;
  /// ```
  pub fn all_pit_segments(&self) -> builder::CatAllPitSegments {
    builder::CatAllPitSegments::new(self.os_client)
  }

  ///Returns information about installed plugins across nodes node.
  ///
  ///Sends a `GET` request to `/_cat/plugins`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().plugins()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn plugins(&self) -> builder::CatPlugins {
    builder::CatPlugins::new(self.os_client)
  }

  ///Returns information about index shard recoveries, both on-going
  /// completed.
  ///
  ///Sends a `GET` request to `/_cat/recovery`
  ///
  ///Arguments:
  /// - `active_only`: If `true`, the response only includes ongoing shard
  ///   recoveries.
  /// - `bytes`: The unit in which to display byte values.
  /// - `detailed`: If `true`, the response includes detailed information about
  ///   shard recoveries.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `index`: Comma-separated list or wildcard expression of index names to
  ///   limit the returned information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().recovery()
  ///    .active_only(active_only)
  ///    .bytes(bytes)
  ///    .detailed(detailed)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .index(index)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn recovery(&self) -> builder::CatRecovery {
    builder::CatRecovery::new(self.os_client)
  }

  ///Returns information about index shard recoveries, both on-going
  /// completed.
  ///
  ///Sends a `GET` request to `/_cat/recovery/{index}`
  ///
  ///Arguments:
  /// - `active_only`: If `true`, the response only includes ongoing shard
  ///   recoveries.
  /// - `bytes`: The unit in which to display byte values.
  /// - `detailed`: If `true`, the response includes detailed information about
  ///   shard recoveries.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `index`: Comma-separated list or wildcard expression of index names to
  ///   limit the returned information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().recovery_with_index()
  ///    .active_only(active_only)
  ///    .bytes(bytes)
  ///    .detailed(detailed)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .index(index)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn recovery_with_index(&self) -> builder::CatRecoveryWithIndex {
    builder::CatRecoveryWithIndex::new(self.os_client)
  }

  ///Returns information about snapshot repositories registered in the
  /// cluster.
  ///
  ///Sends a `GET` request to `/_cat/repositories`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().repositories()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn repositories(&self) -> builder::CatRepositories {
    builder::CatRepositories::new(self.os_client)
  }

  ///Returns information about both on-going and latest completed Segment
  /// Replication events.
  ///
  ///Sends a `GET` request to `/_cat/segment_replication`
  ///
  ///Arguments:
  /// - `active_only`: If `true`, the response only includes ongoing segment
  ///   replication events.
  /// - `bytes`: The unit in which to display byte values.
  /// - `completed_only`: If `true`, the response only includes latest completed
  ///   segment replication events.
  /// - `detailed`: If `true`, the response includes detailed information about
  ///   segment replications.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `index`: Comma-separated list or wildcard expression of index names to
  ///   limit the returned information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `shards`: Comma-separated list of shards to display.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().segment_replication()
  ///    .active_only(active_only)
  ///    .bytes(bytes)
  ///    .completed_only(completed_only)
  ///    .detailed(detailed)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .index(index)
  ///    .s(s)
  ///    .shards(shards)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn segment_replication(&self) -> builder::CatSegmentReplication {
    builder::CatSegmentReplication::new(self.os_client)
  }

  ///Returns information about both on-going and latest completed Segment
  /// Replication events.
  ///
  ///Sends a `GET` request to `/_cat/segment_replication/{index}`
  ///
  ///Arguments:
  /// - `active_only`: If `true`, the response only includes ongoing segment
  ///   replication events.
  /// - `bytes`: The unit in which to display byte values.
  /// - `completed_only`: If `true`, the response only includes latest completed
  ///   segment replication events.
  /// - `detailed`: If `true`, the response includes detailed information about
  ///   segment replications.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `index`: Comma-separated list or wildcard expression of index names to
  ///   limit the returned information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `shards`: Comma-separated list of shards to display.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().segment_replication_with_index()
  ///    .active_only(active_only)
  ///    .bytes(bytes)
  ///    .completed_only(completed_only)
  ///    .detailed(detailed)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .index(index)
  ///    .s(s)
  ///    .shards(shards)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn segment_replication_with_index(&self) -> builder::CatSegmentReplicationWithIndex {
    builder::CatSegmentReplicationWithIndex::new(self.os_client)
  }

  ///Provides low-level information about the segments in the shards of an
  /// index.
  ///
  ///Sends a `GET` request to `/_cat/segments`
  ///
  ///Arguments:
  /// - `bytes`: The unit in which to display byte values.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().segments()
  ///    .bytes(bytes)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn segments(&self) -> builder::CatSegments {
    builder::CatSegments::new(self.os_client)
  }

  ///Provides low-level information about the segments in the shards of an
  /// index.
  ///
  ///Sends a `GET` request to `/_cat/segments/{index}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to limit the returned
  ///   information.
  /// - `bytes`: The unit in which to display byte values.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().segments_with_index()
  ///    .index(index)
  ///    .bytes(bytes)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn segments_with_index(&self) -> builder::CatSegmentsWithIndex {
    builder::CatSegmentsWithIndex::new(self.os_client)
  }

  ///Provides a detailed view of shard allocation on nodes.
  ///
  ///Sends a `GET` request to `/_cat/shards`
  ///
  ///Arguments:
  /// - `bytes`: The unit in which to display byte values.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().shards()
  ///    .bytes(bytes)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn shards(&self) -> builder::CatShards {
    builder::CatShards::new(self.os_client)
  }

  ///Provides a detailed view of shard allocation on nodes.
  ///
  ///Sends a `GET` request to `/_cat/shards/{index}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to limit the returned
  ///   information.
  /// - `bytes`: The unit in which to display byte values.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().shards_with_index()
  ///    .index(index)
  ///    .bytes(bytes)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn shards_with_index(&self) -> builder::CatShardsWithIndex {
    builder::CatShardsWithIndex::new(self.os_client)
  }

  ///Returns all snapshots in a specific repository.
  ///
  ///Sends a `GET` request to `/_cat/snapshots`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().snapshots()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshots(&self) -> builder::CatSnapshots {
    builder::CatSnapshots::new(self.os_client)
  }

  ///Returns all snapshots in a specific repository.
  ///
  ///Sends a `GET` request to `/_cat/snapshots/{repository}`
  ///
  ///Arguments:
  /// - `repository`: Comma-separated list of repository names.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().snapshots_with_repository()
  ///    .repository(repository)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn snapshots_with_repository(&self) -> builder::CatSnapshotsWithRepository {
    builder::CatSnapshotsWithRepository::new(self.os_client)
  }

  ///Returns information about the tasks currently executing on one or more
  /// nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_cat/tasks`
  ///
  ///Arguments:
  /// - `actions`: Comma-separated list of actions that should be returned.
  ///   Leave empty to return all.
  /// - `detailed`: Return detailed task information.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `nodes`: Comma-separated list of node IDs or names to limit the returned
  ///   information; use `_local` to return information from the node you're
  ///   connecting to, leave empty to get information from all nodes.
  /// - `parent_task_id`: Return tasks with specified parent task id
  ///   (node_id:task_number). Set to -1 to return all.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `time`: The unit in which to display time values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().tasks()
  ///    .actions(actions)
  ///    .detailed(detailed)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .nodes(nodes)
  ///    .parent_task_id(parent_task_id)
  ///    .s(s)
  ///    .time(time)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn tasks(&self) -> builder::CatTasks {
    builder::CatTasks::new(self.os_client)
  }

  ///Returns information about existing templates.
  ///
  ///Sends a `GET` request to `/_cat/templates`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().templates()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn templates(&self) -> builder::CatTemplates {
    builder::CatTemplates::new(self.os_client)
  }

  ///Returns information about existing templates.
  ///
  ///Sends a `GET` request to `/_cat/templates/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().templates_with_name()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn templates_with_name(&self) -> builder::CatTemplatesWithName {
    builder::CatTemplatesWithName::new(self.os_client)
  }

  ///Returns cluster-wide thread pool statistics per node.
  ///By default the active, queue and rejected statistics are returned for
  /// all thread pools.
  ///
  ///Sends a `GET` request to `/_cat/thread_pool`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `size`: The multiplier in which to display values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().thread_pool()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .size(size)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn thread_pool(&self) -> builder::CatThreadPool {
    builder::CatThreadPool::new(self.os_client)
  }

  ///Returns cluster-wide thread pool statistics per node.
  ///By default the active, queue and rejected statistics are returned for
  /// all thread pools.
  ///
  ///Sends a `GET` request to `/_cat/thread_pool/{thread_pool_patterns}`
  ///
  ///Arguments:
  /// - `thread_pool_patterns`: Comma-separated list of regular-expressions to
  ///   filter the thread pools in the output.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `format`: A short version of the Accept header, e.g. json, yaml.
  /// - `h`: Comma-separated list of column names to display.
  /// - `help`: Return help information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  /// - `size`: The multiplier in which to display values.
  /// - `v`: Verbose mode. Display column headers.
  ///```ignore
  /// let response = client.cat().thread_pool_with_thread_pool_patterns()
  ///    .thread_pool_patterns(thread_pool_patterns)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .s(s)
  ///    .size(size)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn thread_pool_with_thread_pool_patterns(&self) -> builder::CatThreadPoolWithThreadPoolPatterns {
    builder::CatThreadPoolWithThreadPoolPatterns::new(self.os_client)
  }
}
