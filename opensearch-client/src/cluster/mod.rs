use crate::OsClient;
mod builder;
mod types;
pub struct Cluster<'a> {
  os_client: &'a OsClient,
}

impl<'a> Cluster<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Provides explanations for shard allocations in the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/allocation/explain`
  ///
  ///Arguments:
  /// - `include_disk_info`: Return information about disk usage and shard
  ///   sizes.
  /// - `include_yes_decisions`: Return 'YES' decisions in explanation.
  ///```ignore
  /// let response = client.cluster_allocation_explain_get()
  ///    .include_disk_info(include_disk_info)
  ///    .include_yes_decisions(include_yes_decisions)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_allocation_explain_get(&self) -> builder::ClusterAllocationExplainGet {
    builder::ClusterAllocationExplainGet::new(self.os_client)
  }

  ///Provides explanations for shard allocations in the cluster.
  ///
  ///Sends a `POST` request to `/_cluster/allocation/explain`
  ///
  ///Arguments:
  /// - `include_disk_info`: Return information about disk usage and shard
  ///   sizes.
  /// - `include_yes_decisions`: Return 'YES' decisions in explanation.
  /// - `body`
  ///```ignore
  /// let response = client.cluster_allocation_explain_post()
  ///    .include_disk_info(include_disk_info)
  ///    .include_yes_decisions(include_yes_decisions)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_allocation_explain_post(&self) -> builder::ClusterAllocationExplainPost {
    builder::ClusterAllocationExplainPost::new(self.os_client)
  }

  ///Delete any existing decommission.
  ///
  ///Sends a `DELETE` request to `/_cluster/decommission/awareness/`
  ///
  ///```ignore
  /// let response = client.cluster_delete_decommission_awareness()
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_delete_decommission_awareness(&self) -> builder::ClusterDeleteDecommissionAwareness {
    builder::ClusterDeleteDecommissionAwareness::new(self.os_client)
  }

  ///Get details and status of decommissioned attribute.
  ///
  ///Sends a `GET` request to
  /// `/_cluster/decommission/awareness/{awareness_attribute_name}/_status`
  ///
  ///Arguments:
  /// - `awareness_attribute_name`: Awareness attribute name.
  ///```ignore
  /// let response = client.cluster_get_decommission_awareness()
  ///    .awareness_attribute_name(awareness_attribute_name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_get_decommission_awareness(&self) -> builder::ClusterGetDecommissionAwareness {
    builder::ClusterGetDecommissionAwareness::new(self.os_client)
  }

  ///Decommissions an awareness attribute.
  ///
  ///Sends a `PUT` request to
  /// `/_cluster/decommission/awareness/{awareness_attribute_name}/
  /// {awareness_attribute_value}`
  ///
  ///Arguments:
  /// - `awareness_attribute_name`: Awareness attribute name.
  /// - `awareness_attribute_value`: Awareness attribute value.
  ///```ignore
  /// let response = client.cluster_put_decommission_awareness()
  ///    .awareness_attribute_name(awareness_attribute_name)
  ///    .awareness_attribute_value(awareness_attribute_value)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_put_decommission_awareness(&self) -> builder::ClusterPutDecommissionAwareness {
    builder::ClusterPutDecommissionAwareness::new(self.os_client)
  }

  ///Returns basic information about the health of the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/health`
  ///
  ///Arguments:
  /// - `awareness_attribute`: The awareness attribute for which the health is
  ///   required.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `ensure_node_commissioned`: Checks whether local node is commissioned or
  ///   not. If set to true on a local call it will throw exception if node is
  ///   decommissioned.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `level`: Specify the level of detail for returned information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Wait until the specified number of shards is
  ///   active.
  /// - `wait_for_events`: Wait until all currently queued events with the given
  ///   priority are processed.
  /// - `wait_for_no_initializing_shards`: Whether to wait until there are no
  ///   initializing shards in the cluster.
  /// - `wait_for_no_relocating_shards`: Whether to wait until there are no
  ///   relocating shards in the cluster.
  /// - `wait_for_nodes`: Wait until the specified number of nodes is available.
  /// - `wait_for_status`: Wait until cluster is in a specific state.
  ///```ignore
  /// let response = client.cluster_health()
  ///    .awareness_attribute(awareness_attribute)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .ensure_node_commissioned(ensure_node_commissioned)
  ///    .expand_wildcards(expand_wildcards)
  ///    .level(level)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .wait_for_events(wait_for_events)
  ///    .wait_for_no_initializing_shards(wait_for_no_initializing_shards)
  ///    .wait_for_no_relocating_shards(wait_for_no_relocating_shards)
  ///    .wait_for_nodes(wait_for_nodes)
  ///    .wait_for_status(wait_for_status)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_health(&self) -> builder::ClusterHealth {
    builder::ClusterHealth::new(self.os_client)
  }

  ///Returns basic information about the health of the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/health/{index}`
  ///
  ///Arguments:
  /// - `index`: Limit the information returned to specific indicies.
  /// - `awareness_attribute`: The awareness attribute for which the health is
  ///   required.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `ensure_node_commissioned`: Checks whether local node is commissioned or
  ///   not. If set to true on a local call it will throw exception if node is
  ///   decommissioned.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `level`: Specify the level of detail for returned information.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Wait until the specified number of shards is
  ///   active.
  /// - `wait_for_events`: Wait until all currently queued events with the given
  ///   priority are processed.
  /// - `wait_for_no_initializing_shards`: Whether to wait until there are no
  ///   initializing shards in the cluster.
  /// - `wait_for_no_relocating_shards`: Whether to wait until there are no
  ///   relocating shards in the cluster.
  /// - `wait_for_nodes`: Wait until the specified number of nodes is available.
  /// - `wait_for_status`: Wait until cluster is in a specific state.
  ///```ignore
  /// let response = client.cluster_health_with_index()
  ///    .index(index)
  ///    .awareness_attribute(awareness_attribute)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .ensure_node_commissioned(ensure_node_commissioned)
  ///    .expand_wildcards(expand_wildcards)
  ///    .level(level)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .wait_for_events(wait_for_events)
  ///    .wait_for_no_initializing_shards(wait_for_no_initializing_shards)
  ///    .wait_for_no_relocating_shards(wait_for_no_relocating_shards)
  ///    .wait_for_nodes(wait_for_nodes)
  ///    .wait_for_status(wait_for_status)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_health_with_index(&self) -> builder::ClusterHealthWithIndex {
    builder::ClusterHealthWithIndex::new(self.os_client)
  }

  ///Returns a list of any cluster-level changes (e.g. create index, update
  /// mapping, allocate or fail shard) which have not yet been executed.
  ///
  ///Sends a `GET` request to `/_cluster/pending_tasks`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.cluster_pending_tasks()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_pending_tasks(&self) -> builder::ClusterPendingTasks {
    builder::ClusterPendingTasks::new(self.os_client)
  }

  ///Allows to manually change the allocation of individual shards in the
  /// cluster.
  ///
  ///Sends a `POST` request to `/_cluster/reroute`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `dry_run`: Simulate the operation only and return the resulting state.
  /// - `explain`: Return an explanation of why the commands can or cannot be
  ///   executed.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `metric`: Limit the information returned to the specified metrics.
  ///   Defaults to all but metadata.
  /// - `retry_failed`: Retries allocation of shards that are blocked due to too
  ///   many subsequent allocation failures.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.cluster_reroute()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .dry_run(dry_run)
  ///    .explain(explain)
  ///    .master_timeout(master_timeout)
  ///    .metric(metric)
  ///    .retry_failed(retry_failed)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_reroute(&self) -> builder::ClusterReroute {
    builder::ClusterReroute::new(self.os_client)
  }

  ///Delete weighted shard routing weights.
  ///
  ///Sends a `DELETE` request to `/_cluster/routing/awareness/weights`
  ///
  ///```ignore
  /// let response = client.cluster_delete_weighted_routing()
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_delete_weighted_routing(&self) -> builder::ClusterDeleteWeightedRouting {
    builder::ClusterDeleteWeightedRouting::new(self.os_client)
  }

  ///Fetches weighted shard routing weights.
  ///
  ///Sends a `GET` request to
  /// `/_cluster/routing/awareness/{attribute}/weights`
  ///
  ///Arguments:
  /// - `attribute`: Awareness attribute name.
  ///```ignore
  /// let response = client.cluster_get_weighted_routing()
  ///    .attribute(attribute)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_get_weighted_routing(&self) -> builder::ClusterGetWeightedRouting {
    builder::ClusterGetWeightedRouting::new(self.os_client)
  }

  ///Updates weighted shard routing weights.
  ///
  ///Sends a `PUT` request to
  /// `/_cluster/routing/awareness/{attribute}/weights`
  ///
  ///Arguments:
  /// - `attribute`: Awareness attribute name.
  ///```ignore
  /// let response = client.cluster_put_weighted_routing()
  ///    .attribute(attribute)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_put_weighted_routing(&self) -> builder::ClusterPutWeightedRouting {
    builder::ClusterPutWeightedRouting::new(self.os_client)
  }

  ///Returns cluster settings.
  ///
  ///Sends a `GET` request to `/_cluster/settings`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `flat_settings`: Return settings in flat format.
  /// - `include_defaults`: Whether to return all default clusters setting.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.cluster_get_settings()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .include_defaults(include_defaults)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_get_settings(&self) -> builder::ClusterGetSettings {
    builder::ClusterGetSettings::new(self.os_client)
  }

  ///Updates the cluster settings.
  ///
  ///Sends a `PUT` request to `/_cluster/settings`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `flat_settings`: Return settings in flat format.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.cluster_put_settings()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_put_settings(&self) -> builder::ClusterPutSettings {
    builder::ClusterPutSettings::new(self.os_client)
  }

  ///Returns a comprehensive information about the state of the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/state`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `flat_settings`: Return settings in flat format.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `wait_for_metadata_version`: Wait for the metadata version to be equal
  ///   or greater than the specified metadata version.
  /// - `wait_for_timeout`: The maximum time to wait for
  ///   wait_for_metadata_version before timing out.
  ///```ignore
  /// let response = client.cluster_state()
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .wait_for_metadata_version(wait_for_metadata_version)
  ///    .wait_for_timeout(wait_for_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_state(&self) -> builder::ClusterState {
    builder::ClusterState::new(self.os_client)
  }

  ///Returns a comprehensive information about the state of the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/state/{metric}`
  ///
  ///Arguments:
  /// - `metric`: Limit the information returned to the specified metrics.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `flat_settings`: Return settings in flat format.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `wait_for_metadata_version`: Wait for the metadata version to be equal
  ///   or greater than the specified metadata version.
  /// - `wait_for_timeout`: The maximum time to wait for
  ///   wait_for_metadata_version before timing out.
  ///```ignore
  /// let response = client.cluster_state_with_metric()
  ///    .metric(metric)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .wait_for_metadata_version(wait_for_metadata_version)
  ///    .wait_for_timeout(wait_for_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_state_with_metric(&self) -> builder::ClusterStateWithMetric {
    builder::ClusterStateWithMetric::new(self.os_client)
  }

  ///Returns a comprehensive information about the state of the cluster.
  ///
  ///Sends a `GET` request to `/_cluster/state/{metric}/{index}`
  ///
  ///Arguments:
  /// - `metric`: Limit the information returned to the specified metrics.
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `flat_settings`: Return settings in flat format.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `wait_for_metadata_version`: Wait for the metadata version to be equal
  ///   or greater than the specified metadata version.
  /// - `wait_for_timeout`: The maximum time to wait for
  ///   wait_for_metadata_version before timing out.
  ///```ignore
  /// let response = client.cluster_state_with_index_metric()
  ///    .metric(metric)
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .wait_for_metadata_version(wait_for_metadata_version)
  ///    .wait_for_timeout(wait_for_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_state_with_index_metric(&self) -> builder::ClusterStateWithIndexMetric {
    builder::ClusterStateWithIndexMetric::new(self.os_client)
  }

  ///Returns high-level overview of cluster statistics.
  ///
  ///Sends a `GET` request to `/_cluster/stats`
  ///
  ///Arguments:
  /// - `flat_settings`: Return settings in flat format.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.cluster_stats()
  ///    .flat_settings(flat_settings)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_stats(&self) -> builder::ClusterStats {
    builder::ClusterStats::new(self.os_client)
  }

  ///Returns high-level overview of cluster statistics.
  ///
  ///Sends a `GET` request to `/_cluster/stats/nodes/{node_id}`
  ///
  ///Arguments:
  /// - `node_id`: Comma-separated list of node IDs or names to limit the
  ///   returned information; use `_local` to return information from the node
  ///   you're connecting to, leave empty to get information from all nodes.
  /// - `flat_settings`: Return settings in flat format.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.cluster_stats_with_node_id()
  ///    .node_id(node_id)
  ///    .flat_settings(flat_settings)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_stats_with_node_id(&self) -> builder::ClusterStatsWithNodeId {
    builder::ClusterStatsWithNodeId::new(self.os_client)
  }

  ///Updates the cluster voting config exclusions by node ids or node names.
  ///
  ///Sends a `POST` request to `/_cluster/voting_config_exclusions`
  ///
  ///Arguments:
  /// - `node_ids`: Comma-separated list of the persistent ids of the nodes to
  ///   exclude from the voting configuration. If specified, you may not also
  ///   specify ?node_names.
  /// - `node_names`: Comma-separated list of the names of the nodes to exclude
  ///   from the voting configuration. If specified, you may not also specify
  ///   ?node_ids.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.cluster_post_voting_config_exclusions()
  ///    .node_ids(node_ids)
  ///    .node_names(node_names)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_post_voting_config_exclusions(&self) -> builder::ClusterPostVotingConfigExclusions {
    builder::ClusterPostVotingConfigExclusions::new(self.os_client)
  }

  ///Clears cluster voting config exclusions.
  ///
  ///Sends a `DELETE` request to `/_cluster/voting_config_exclusions`
  ///
  ///Arguments:
  /// - `wait_for_removal`: Specifies whether to wait for all excluded nodes to
  ///   be removed from the cluster before clearing the voting configuration
  ///   exclusions list.
  ///```ignore
  /// let response = client.cluster_delete_voting_config_exclusions()
  ///    .wait_for_removal(wait_for_removal)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_delete_voting_config_exclusions(&self) -> builder::ClusterDeleteVotingConfigExclusions {
    builder::ClusterDeleteVotingConfigExclusions::new(self.os_client)
  }

  ///Returns one or more component templates.
  ///
  ///Sends a `GET` request to `/_component_template`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.cluster_get_component_template()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_get_component_template(&self) -> builder::ClusterGetComponentTemplate {
    builder::ClusterGetComponentTemplate::new(self.os_client)
  }

  ///Returns one or more component templates.
  ///
  ///Sends a `GET` request to `/_component_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The Comma-separated names of the component templates.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.cluster_get_component_template_with_name()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_get_component_template_with_name(&self) -> builder::ClusterGetComponentTemplateWithName {
    builder::ClusterGetComponentTemplateWithName::new(self.os_client)
  }

  ///Creates or updates a component template.
  ///
  ///Sends a `PUT` request to `/_component_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `create`: Whether the index template should only be added if new or can
  ///   also replace an existing one.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.cluster_put_component_template_put()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_put_component_template_put(&self) -> builder::ClusterPutComponentTemplatePut {
    builder::ClusterPutComponentTemplatePut::new(self.os_client)
  }

  ///Creates or updates a component template.
  ///
  ///Sends a `POST` request to `/_component_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `create`: Whether the index template should only be added if new or can
  ///   also replace an existing one.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.cluster_put_component_template_post()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_put_component_template_post(&self) -> builder::ClusterPutComponentTemplatePost {
    builder::ClusterPutComponentTemplatePost::new(self.os_client)
  }

  ///Deletes a component template.
  ///
  ///Sends a `DELETE` request to `/_component_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.cluster_delete_component_template()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_delete_component_template(&self) -> builder::ClusterDeleteComponentTemplate {
    builder::ClusterDeleteComponentTemplate::new(self.os_client)
  }

  ///Returns information about whether a particular component template exist.
  ///
  ///Sends a `HEAD` request to `/_component_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.cluster_exists_component_template()
  ///    .name(name)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_exists_component_template(&self) -> builder::ClusterExistsComponentTemplate {
    builder::ClusterExistsComponentTemplate::new(self.os_client)
  }

  ///Returns the information about configured remote clusters.
  ///
  ///Sends a `GET` request to `/_remote/info`
  ///
  ///```ignore
  /// let response = client.cluster_remote_info()
  ///    .send()
  ///    .await;
  /// ```
  pub fn cluster_remote_info(&self) -> builder::ClusterRemoteInfo {
    builder::ClusterRemoteInfo::new(self.os_client)
  }
}
