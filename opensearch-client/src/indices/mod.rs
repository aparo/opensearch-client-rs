use crate::{Error, OsClient};
mod builder;
mod types;
pub struct Indices<'a> {
  os_client: &'a OsClient,
}

impl<'a> Indices<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  /// Returns a list of indices in the OpenSearch cluster.
  ///
  /// # Example
  ///
  /// ```
  /// # use opensearch_client::OpenSearchClient;
  /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
  /// # let client = OpenSearchClient::new("http://localhost:9200")?;
  /// let indices = client.indices().list_indices().await?;
  /// # Ok(())
  /// # }
  /// ```
  pub async fn list_indices(&self) -> Result<Vec<String>, Error> {
    let response = self.get_alias().send().await?;
    let alias_result = response.into_inner();
    // let values: Vec<String> = cat_result
    //   .split('\n')
    //   .filter(|x| !x.is_empty())
    //   .map(|x| {
    //     let mut iterator = x.split_ascii_whitespace();
    //     iterator.nth(2).unwrap().to_owned()
    //   })
    //   .collect();
    // Ok(values)
    Ok(vec![])
  }

  ///Returns an alias.
  ///
  ///Sends a `GET` request to `/_alias`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  ///```ignore
  /// let response = client.indices().get_alias()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_alias(&self) -> builder::IndicesGetAlias {
    builder::IndicesGetAlias::new(self.os_client)
  }

  ///Returns an alias.
  ///
  ///Sends a `GET` request to `/_alias/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of alias names.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  ///```ignore
  /// let response = client.indices().get_alias_with_name()
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_alias_with_name(&self) -> builder::IndicesGetAliasWithName {
    builder::IndicesGetAliasWithName::new(self.os_client)
  }

  ///Returns information about whether a particular alias exists.
  ///
  ///Sends a `HEAD` request to `/_alias/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of alias names.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  ///```ignore
  /// let response = client.indices().exists_alias()
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn exists_alias(&self) -> builder::IndicesExistsAlias {
    builder::IndicesExistsAlias::new(self.os_client)
  }

  ///Updates index aliases.
  ///
  ///Sends a `POST` request to `/_aliases`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.indices().update_aliases()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update_aliases(&self) -> builder::IndicesUpdateAliases {
    builder::IndicesUpdateAliases::new(self.os_client)
  }

  ///Performs the analysis process on a text and return the tokens breakdown
  /// of the text.
  ///
  ///Sends a `GET` request to `/_analyze`
  ///
  ///Arguments:
  /// - `index`: The name of the index to scope the operation.
  ///```ignore
  /// let response = client.indices().analyze_get()
  ///    .index(index)
  ///    .send()
  ///    .await;
  /// ```
  pub fn analyze_get(&self) -> builder::IndicesAnalyzeGet {
    builder::IndicesAnalyzeGet::new(self.os_client)
  }

  ///Performs the analysis process on a text and return the tokens breakdown
  /// of the text.
  ///
  ///Sends a `POST` request to `/_analyze`
  ///
  ///Arguments:
  /// - `index`: The name of the index to scope the operation.
  /// - `body`
  ///```ignore
  /// let response = client.indices().analyze_post()
  ///    .index(index)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn analyze_post(&self) -> builder::IndicesAnalyzePost {
    builder::IndicesAnalyzePost::new(self.os_client)
  }

  ///Clears all or specific caches for one or more indices.
  ///
  ///Sends a `POST` request to `/_cache/clear`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fielddata`: Clear field data.
  /// - `fields`: Comma-separated list of fields to clear when using the
  ///   `fielddata` parameter (default: all).
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `query`: Clear query caches.
  /// - `request`: Clear request cache.
  ///```ignore
  /// let response = client.indices().clear_cache()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fielddata(fielddata)
  ///    .fields(fields)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .index(index)
  ///    .query(query)
  ///    .request(request)
  ///    .send()
  ///    .await;
  /// ```
  pub fn clear_cache(&self) -> builder::IndicesClearCache {
    builder::IndicesClearCache::new(self.os_client)
  }

  ///Returns data streams.
  ///
  ///Sends a `GET` request to `/_data_stream`
  ///
  ///```ignore
  /// let response = client.indices().get_data_stream()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_data_stream(&self) -> builder::IndicesGetDataStream {
    builder::IndicesGetDataStream::new(self.os_client)
  }

  ///Provides statistics on operations happening in a data stream.
  ///
  ///Sends a `GET` request to `/_data_stream/_stats`
  ///
  ///```ignore
  /// let response = client.indices().data_streams_stats()
  ///    .send()
  ///    .await;
  /// ```
  pub fn data_streams_stats(&self) -> builder::IndicesDataStreamsStats {
    builder::IndicesDataStreamsStats::new(self.os_client)
  }

  ///Returns data streams.
  ///
  ///Sends a `GET` request to `/_data_stream/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of data streams; use `_all` or empty string
  ///   to perform the operation on all data streams.
  ///```ignore
  /// let response = client.indices().get_data_stream_with_name()
  ///    .name(name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_data_stream_with_name(&self) -> builder::IndicesGetDataStreamWithName {
    builder::IndicesGetDataStreamWithName::new(self.os_client)
  }

  ///Creates or updates a data stream.
  ///
  ///Sends a `PUT` request to `/_data_stream/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the data stream.
  /// - `body`
  ///```ignore
  /// let response = client.indices().create_data_stream()
  ///    .name(name)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_data_stream(&self) -> builder::IndicesCreateDataStream {
    builder::IndicesCreateDataStream::new(self.os_client)
  }

  ///Deletes a data stream.
  ///
  ///Sends a `DELETE` request to `/_data_stream/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of data streams; use `_all` or empty string
  ///   to perform the operation on all data streams.
  ///```ignore
  /// let response = client.indices().delete_data_stream()
  ///    .name(name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_data_stream(&self) -> builder::IndicesDeleteDataStream {
    builder::IndicesDeleteDataStream::new(self.os_client)
  }

  ///Provides statistics on operations happening in a data stream.
  ///
  ///Sends a `GET` request to `/_data_stream/{name}/_stats`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of data streams; use `_all` or empty string
  ///   to perform the operation on all data streams.
  ///```ignore
  /// let response = client.indices().data_streams_stats_with_name()
  ///    .name(name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn data_streams_stats_with_name(&self) -> builder::IndicesDataStreamsStatsWithName {
    builder::IndicesDataStreamsStatsWithName::new(self.os_client)
  }

  ///Performs the flush operation on one or more indices.
  ///
  ///Sends a `GET` request to `/_flush`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `force`: Whether a flush should be forced even if it is not necessarily
  ///   needed ie. if no changes will be committed to the index. This is useful
  ///   if transaction log IDs should be incremented even if no uncommitted
  ///   changes are present. (This setting can be considered as internal).
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `wait_if_ongoing`: If set to true the flush operation will block until
  ///   the flush can be executed if another flush operation is already
  ///   executing. If set to false the flush will be skipped iff if another
  ///   flush operation is already running.
  ///```ignore
  /// let response = client.indices().flush_get()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .force(force)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .wait_if_ongoing(wait_if_ongoing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn flush_get(&self) -> builder::IndicesFlushGet {
    builder::IndicesFlushGet::new(self.os_client)
  }

  ///Performs the flush operation on one or more indices.
  ///
  ///Sends a `POST` request to `/_flush`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `force`: Whether a flush should be forced even if it is not necessarily
  ///   needed ie. if no changes will be committed to the index. This is useful
  ///   if transaction log IDs should be incremented even if no uncommitted
  ///   changes are present. (This setting can be considered as internal).
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `wait_if_ongoing`: If set to true the flush operation will block until
  ///   the flush can be executed if another flush operation is already
  ///   executing. If set to false the flush will be skipped iff if another
  ///   flush operation is already running.
  ///```ignore
  /// let response = client.indices().flush_post()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .force(force)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .wait_if_ongoing(wait_if_ongoing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn flush_post(&self) -> builder::IndicesFlushPost {
    builder::IndicesFlushPost::new(self.os_client)
  }

  ///Performs the force merge operation on one or more indices.
  ///
  ///Sends a `POST` request to `/_forcemerge`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `flush`: Specify whether the index should be flushed after performing
  ///   the operation.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `max_num_segments`: The number of segments the index should be merged
  ///   into (default: dynamic).
  /// - `only_expunge_deletes`: Specify whether the operation should only
  ///   expunge deleted documents.
  ///```ignore
  /// let response = client.indices().forcemerge()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flush(flush)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .max_num_segments(max_num_segments)
  ///    .only_expunge_deletes(only_expunge_deletes)
  ///    .send()
  ///    .await;
  /// ```
  pub fn forcemerge(&self) -> builder::IndicesForcemerge {
    builder::IndicesForcemerge::new(self.os_client)
  }

  ///Returns an index template.
  ///
  ///Sends a `GET` request to `/_index_template`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `flat_settings`: Return settings in flat format.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_index_template()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_index_template(&self) -> builder::IndicesGetIndexTemplate {
    builder::IndicesGetIndexTemplate::new(self.os_client)
  }

  ///Simulate resolving the given template name or body.
  ///
  ///Sends a `POST` request to `/_index_template/_simulate`
  ///
  ///Arguments:
  /// - `cause`: User defined reason for dry-run creating the new template for
  ///   simulation purposes.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `create`: Whether the index template we optionally defined in the body
  ///   should only be dry-run added if new or can also replace an existing one.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `body`
  ///```ignore
  /// let response = client.indices().simulate_template()
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn simulate_template(&self) -> builder::IndicesSimulateTemplate {
    builder::IndicesSimulateTemplate::new(self.os_client)
  }

  ///Simulate resolving the given template name or body.
  ///
  ///Sends a `POST` request to `/_index_template/_simulate/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cause`: User defined reason for dry-run creating the new template for
  ///   simulation purposes.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `create`: Whether the index template we optionally defined in the body
  ///   should only be dry-run added if new or can also replace an existing one.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `body`
  ///```ignore
  /// let response = client.indices().simulate_template_with_name()
  ///    .name(name)
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn simulate_template_with_name(&self) -> builder::IndicesSimulateTemplateWithName {
    builder::IndicesSimulateTemplateWithName::new(self.os_client)
  }

  ///Simulate matching the given index name against the index templates in
  /// the system.
  ///
  ///Sends a `POST` request to `/_index_template/_simulate_index/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the index (it must be a concrete index name).
  /// - `cause`: User defined reason for dry-run creating the new template for
  ///   simulation purposes.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `create`: Whether the index template we optionally defined in the body
  ///   should only be dry-run added if new or can also replace an existing one.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `body`
  ///```ignore
  /// let response = client.indices().simulate_index_template()
  ///    .name(name)
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn simulate_index_template(&self) -> builder::IndicesSimulateIndexTemplate {
    builder::IndicesSimulateIndexTemplate::new(self.os_client)
  }

  ///Returns an index template.
  ///
  ///Sends a `GET` request to `/_index_template/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated names of the index templates.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `flat_settings`: Return settings in flat format.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_index_template_with_name()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_index_template_with_name(&self) -> builder::IndicesGetIndexTemplateWithName {
    builder::IndicesGetIndexTemplateWithName::new(self.os_client)
  }

  ///Creates or updates an index template.
  ///
  ///Sends a `PUT` request to `/_index_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cause`: User defined reason for creating/updating the index template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `create`: Whether the index template should only be added if new or can
  ///   also replace an existing one.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_index_template_put()
  ///    .name(name)
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_index_template_put(&self) -> builder::IndicesPutIndexTemplatePut {
    builder::IndicesPutIndexTemplatePut::new(self.os_client)
  }

  ///Creates or updates an index template.
  ///
  ///Sends a `POST` request to `/_index_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cause`: User defined reason for creating/updating the index template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `create`: Whether the index template should only be added if new or can
  ///   also replace an existing one.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_index_template_post()
  ///    .name(name)
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_index_template_post(&self) -> builder::IndicesPutIndexTemplatePost {
    builder::IndicesPutIndexTemplatePost::new(self.os_client)
  }

  ///Deletes an index template.
  ///
  ///Sends a `DELETE` request to `/_index_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.indices().delete_index_template()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_index_template(&self) -> builder::IndicesDeleteIndexTemplate {
    builder::IndicesDeleteIndexTemplate::new(self.os_client)
  }

  ///Returns information about whether a particular index template exists.
  ///
  ///Sends a `HEAD` request to `/_index_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `flat_settings`: Return settings in flat format.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().exists_index_template()
  ///    .name(name)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn exists_index_template(&self) -> builder::IndicesExistsIndexTemplate {
    builder::IndicesExistsIndexTemplate::new(self.os_client)
  }

  ///Returns mappings for one or more indices.
  ///
  ///Sends a `GET` request to `/_mapping`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_mapping()
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_mapping(&self) -> builder::IndicesGetMapping {
    builder::IndicesGetMapping::new(self.os_client)
  }

  ///Returns mapping for one or more fields.
  ///
  ///Sends a `GET` request to `/_mapping/field/{fields}`
  ///
  ///Arguments:
  /// - `fields`: Comma-separated list of fields.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `include_defaults`: Whether the default mapping values should be
  ///   returned as well.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  ///```ignore
  /// let response = client.indices().get_field_mapping()
  ///    .fields(fields)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_defaults(include_defaults)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_field_mapping(&self) -> builder::IndicesGetFieldMapping {
    builder::IndicesGetFieldMapping::new(self.os_client)
  }

  ///Returns information about ongoing index shard recoveries.
  ///
  ///Sends a `GET` request to `/_recovery`
  ///
  ///Arguments:
  /// - `active_only`: Display only those recoveries that are currently
  ///   on-going.
  /// - `detailed`: Whether to display detailed information about shard
  ///   recovery.
  ///```ignore
  /// let response = client.indices().recovery()
  ///    .active_only(active_only)
  ///    .detailed(detailed)
  ///    .send()
  ///    .await;
  /// ```
  pub fn recovery(&self) -> builder::IndicesRecovery {
    builder::IndicesRecovery::new(self.os_client)
  }

  ///Performs the refresh operation in one or more indices.
  ///
  ///Sends a `GET` request to `/_refresh`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  ///```ignore
  /// let response = client.indices().refresh_get()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn refresh_get(&self) -> builder::IndicesRefreshGet {
    builder::IndicesRefreshGet::new(self.os_client)
  }

  ///Performs the refresh operation in one or more indices.
  ///
  ///Sends a `POST` request to `/_refresh`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  ///```ignore
  /// let response = client.indices().refresh_post()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn refresh_post(&self) -> builder::IndicesRefreshPost {
    builder::IndicesRefreshPost::new(self.os_client)
  }

  ///Returns information about any matching indices, aliases, and data
  /// streams.
  ///
  ///Sends a `GET` request to `/_resolve/index/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of names or wildcard expressions.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  ///```ignore
  /// let response = client.indices().resolve_index()
  ///    .name(name)
  ///    .expand_wildcards(expand_wildcards)
  ///    .send()
  ///    .await;
  /// ```
  pub fn resolve_index(&self) -> builder::IndicesResolveIndex {
    builder::IndicesResolveIndex::new(self.os_client)
  }

  ///Provides low-level information about segments in a Lucene index.
  ///
  ///Sends a `GET` request to `/_segments`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `verbose`: Includes detailed memory usage by Lucene.
  ///```ignore
  /// let response = client.indices().segments()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .verbose(verbose)
  ///    .send()
  ///    .await;
  /// ```
  pub fn segments(&self) -> builder::IndicesSegments {
    builder::IndicesSegments::new(self.os_client)
  }

  ///Returns settings for one or more indices.
  ///
  ///Sends a `GET` request to `/_settings`
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
  /// - `include_defaults`: Whether to return all default setting for each of
  ///   the indices.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_settings()
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_defaults(include_defaults)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_settings(&self) -> builder::IndicesGetSettings {
    builder::IndicesGetSettings::new(self.os_client)
  }

  ///Updates the index settings.
  ///
  ///Sends a `PUT` request to `/_settings`
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
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `preserve_existing`: Whether to update existing settings. If set to
  ///   `true` existing settings on an index remain unchanged.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_settings()
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .preserve_existing(preserve_existing)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_settings(&self) -> builder::IndicesPutSettings {
    builder::IndicesPutSettings::new(self.os_client)
  }

  ///Returns settings for one or more indices.
  ///
  ///Sends a `GET` request to `/_settings/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of settings.
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
  /// - `include_defaults`: Whether to return all default setting for each of
  ///   the indices.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_settings_with_name()
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_defaults(include_defaults)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_settings_with_name(&self) -> builder::IndicesGetSettingsWithName {
    builder::IndicesGetSettingsWithName::new(self.os_client)
  }

  ///Provides store information for shard copies of indices.
  ///
  ///Sends a `GET` request to `/_shard_stores`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `status`: Comma-separated list of statuses used to filter on shards to
  ///   get store information for.
  ///```ignore
  /// let response = client.indices().shard_stores()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .status(status)
  ///    .send()
  ///    .await;
  /// ```
  pub fn shard_stores(&self) -> builder::IndicesShardStores {
    builder::IndicesShardStores::new(self.os_client)
  }

  ///Provides statistics on operations happening in an index.
  ///
  ///Sends a `GET` request to `/_stats`
  ///
  ///Arguments:
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `forbid_closed_indices`: If set to false stats will also collected from
  ///   closed indices if explicitly specified or if expand_wildcards expands to
  ///   closed indices.
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `include_unloaded_segments`: If set to true segment stats will include
  ///   stats for segments that are not currently loaded into memory.
  /// - `level`: Return stats aggregated at cluster, index or shard level.
  ///```ignore
  /// let response = client.indices().stats()
  ///    .completion_fields(completion_fields)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .forbid_closed_indices(forbid_closed_indices)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .include_unloaded_segments(include_unloaded_segments)
  ///    .level(level)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats(&self) -> builder::IndicesStats {
    builder::IndicesStats::new(self.os_client)
  }

  ///Provides statistics on operations happening in an index.
  ///
  ///Sends a `GET` request to `/_stats/{metric}`
  ///
  ///Arguments:
  /// - `metric`: Limit the information returned the specific metrics.
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `forbid_closed_indices`: If set to false stats will also collected from
  ///   closed indices if explicitly specified or if expand_wildcards expands to
  ///   closed indices.
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `include_unloaded_segments`: If set to true segment stats will include
  ///   stats for segments that are not currently loaded into memory.
  /// - `level`: Return stats aggregated at cluster, index or shard level.
  ///```ignore
  /// let response = client.indices().stats_with_metric()
  ///    .metric(metric)
  ///    .completion_fields(completion_fields)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .forbid_closed_indices(forbid_closed_indices)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .include_unloaded_segments(include_unloaded_segments)
  ///    .level(level)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats_with_metric(&self) -> builder::IndicesStatsWithMetric {
    builder::IndicesStatsWithMetric::new(self.os_client)
  }

  ///Returns an index template.
  ///
  ///Sends a `GET` request to `/_template`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `flat_settings`: Return settings in flat format.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_template()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_template(&self) -> builder::IndicesGetTemplate {
    builder::IndicesGetTemplate::new(self.os_client)
  }

  ///Returns an index template.
  ///
  ///Sends a `GET` request to `/_template/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated names of the index templates.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `flat_settings`: Return settings in flat format.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_template_with_name()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_template_with_name(&self) -> builder::IndicesGetTemplateWithName {
    builder::IndicesGetTemplateWithName::new(self.os_client)
  }

  ///Creates or updates an index template.
  ///
  ///Sends a `PUT` request to `/_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `create`: Whether the index template should only be added if new or can
  ///   also replace an existing one.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `order`: The order for this template when merging multiple matching ones
  ///   (higher numbers are merged later, overriding the lower numbers).
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_template_put()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .order(order)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_template_put(&self) -> builder::IndicesPutTemplatePut {
    builder::IndicesPutTemplatePut::new(self.os_client)
  }

  ///Creates or updates an index template.
  ///
  ///Sends a `POST` request to `/_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `create`: Whether the index template should only be added if new or can
  ///   also replace an existing one.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `order`: The order for this template when merging multiple matching ones
  ///   (higher numbers are merged later, overriding the lower numbers).
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_template_post()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .order(order)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_template_post(&self) -> builder::IndicesPutTemplatePost {
    builder::IndicesPutTemplatePost::new(self.os_client)
  }

  ///Deletes an index template.
  ///
  ///Sends a `DELETE` request to `/_template/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the template.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.indices().delete_template()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_template(&self) -> builder::IndicesDeleteTemplate {
    builder::IndicesDeleteTemplate::new(self.os_client)
  }

  ///Returns information about whether a particular index template exists.
  ///
  ///Sends a `HEAD` request to `/_template/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated names of the index templates.
  /// - `flat_settings`: Return settings in flat format.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().exists_template()
  ///    .name(name)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn exists_template(&self) -> builder::IndicesExistsTemplate {
    builder::IndicesExistsTemplate::new(self.os_client)
  }

  ///The _upgrade API is no longer useful and will be removed.
  ///
  ///Sends a `GET` request to `/_upgrade`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  ///```ignore
  /// let response = client.indices().get_upgrade()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_upgrade(&self) -> builder::IndicesGetUpgrade {
    builder::IndicesGetUpgrade::new(self.os_client)
  }

  ///The _upgrade API is no longer useful and will be removed.
  ///
  ///Sends a `POST` request to `/_upgrade`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `only_ancient_segments`: If true, only ancient (an older Lucene major
  ///   release) segments will be upgraded.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  ///```ignore
  /// let response = client.indices().upgrade()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .only_ancient_segments(only_ancient_segments)
  ///    .wait_for_completion(wait_for_completion)
  ///    .send()
  ///    .await;
  /// ```
  pub fn upgrade(&self) -> builder::IndicesUpgrade {
    builder::IndicesUpgrade::new(self.os_client)
  }

  ///Allows a user to validate a potentially expensive query without
  /// executing it.
  ///
  ///Sends a `GET` request to `/_validate/query`
  ///
  ///Arguments:
  /// - `all_shards`: Execute validation on all shards instead of one random
  ///   shard per index.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Return detailed information about the error.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `rewrite`: Provide a more detailed explanation showing the actual Lucene
  ///   query that will be executed.
  ///```ignore
  /// let response = client.indices().validate_query_get()
  ///    .all_shards(all_shards)
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .q(q)
  ///    .rewrite(rewrite)
  ///    .send()
  ///    .await;
  /// ```
  pub fn validate_query_get(&self) -> builder::IndicesValidateQueryGet {
    builder::IndicesValidateQueryGet::new(self.os_client)
  }

  ///Allows a user to validate a potentially expensive query without
  /// executing it.
  ///
  ///Sends a `POST` request to `/_validate/query`
  ///
  ///Arguments:
  /// - `all_shards`: Execute validation on all shards instead of one random
  ///   shard per index.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Return detailed information about the error.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `rewrite`: Provide a more detailed explanation showing the actual Lucene
  ///   query that will be executed.
  /// - `body`
  ///```ignore
  /// let response = client.indices().validate_query_post()
  ///    .all_shards(all_shards)
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .q(q)
  ///    .rewrite(rewrite)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn validate_query_post(&self) -> builder::IndicesValidateQueryPost {
    builder::IndicesValidateQueryPost::new(self.os_client)
  }

  ///Updates an alias to point to a new index when the existing index
  ///is considered to be too large or too old.
  ///
  ///Sends a `POST` request to `/{alias}/_rollover`
  ///
  ///Arguments:
  /// - `alias`: The name of the alias to rollover.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `dry_run`: If set to true the rollover action will only be validated but
  ///   not actually performed even if a condition matches.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Set the number of active shards to wait for on
  ///   the newly created rollover index before the operation returns.
  /// - `body`
  ///```ignore
  /// let response = client.indices().rollover()
  ///    .alias(alias)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .dry_run(dry_run)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn rollover(&self) -> builder::IndicesRollover {
    builder::IndicesRollover::new(self.os_client)
  }

  ///Updates an alias to point to a new index when the existing index
  ///is considered to be too large or too old.
  ///
  ///Sends a `POST` request to `/{alias}/_rollover/{new_index}`
  ///
  ///Arguments:
  /// - `alias`: The name of the alias to rollover.
  /// - `new_index`: The name of the rollover index.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `dry_run`: If set to true the rollover action will only be validated but
  ///   not actually performed even if a condition matches.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Set the number of active shards to wait for on
  ///   the newly created rollover index before the operation returns.
  /// - `body`
  ///```ignore
  /// let response = client.indices().rollover_with_new_index()
  ///    .alias(alias)
  ///    .new_index(new_index)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .dry_run(dry_run)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn rollover_with_new_index(&self) -> builder::IndicesRolloverWithNewIndex {
    builder::IndicesRolloverWithNewIndex::new(self.os_client)
  }

  ///Returns information about one or more indices.
  ///
  ///Sends a `GET` request to `/{index}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices.
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
  /// - `include_defaults`: Whether to return all default setting for each of
  ///   the indices.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_defaults(include_defaults)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get(&self) -> builder::IndicesGet {
    builder::IndicesGet::new(self.os_client)
  }

  ///Creates an index with optional settings and mappings.
  ///
  ///Sends a `PUT` request to `/{index}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Set the number of active shards to wait for
  ///   before the operation returns.
  /// - `body`
  ///```ignore
  /// let response = client.indices().create()
  ///    .index(index)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create(&self) -> builder::IndicesCreate {
    builder::IndicesCreate::new(self.os_client)
  }

  ///Deletes an index.
  ///
  ///Sends a `DELETE` request to `/{index}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to delete; use `_all` or `*`
  ///   string to delete all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.indices().delete()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete(&self) -> builder::IndicesDelete {
    builder::IndicesDelete::new(self.os_client)
  }

  ///Returns information about whether a particular index exists.
  ///
  ///Sends a `HEAD` request to `/{index}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `flat_settings`: Return settings in flat format.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `include_defaults`: Whether to return all default setting for each of
  ///   the indices.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  ///```ignore
  /// let response = client.indices().exists()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_defaults(include_defaults)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn exists(&self) -> builder::IndicesExists {
    builder::IndicesExists::new(self.os_client)
  }

  ///Returns an alias.
  ///
  ///Sends a `GET` request to `/{index}/_alias`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to filter aliases.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  ///```ignore
  /// let response = client.indices().get_alias_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_alias_with_index(&self) -> builder::IndicesGetAliasWithIndex {
    builder::IndicesGetAliasWithIndex::new(self.os_client)
  }

  ///Returns an alias.
  ///
  ///Sends a `GET` request to `/{index}/_alias/{name}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to filter aliases.
  /// - `name`: Comma-separated list of alias names.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  ///```ignore
  /// let response = client.indices().get_alias_with_index_name()
  ///    .index(index)
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_alias_with_index_name(&self) -> builder::IndicesGetAliasWithIndexName {
    builder::IndicesGetAliasWithIndexName::new(self.os_client)
  }

  ///Creates or updates an alias.
  ///
  ///Sends a `PUT` request to `/{index}/_alias/{name}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `name`: The name of the alias to be created or updated.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_alias_put()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_alias_put(&self) -> builder::IndicesPutAliasPut {
    builder::IndicesPutAliasPut::new(self.os_client)
  }

  ///Creates or updates an alias.
  ///
  ///Sends a `POST` request to `/{index}/_alias/{name}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `name`: The name of the alias to be created or updated.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_alias_post()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_alias_post(&self) -> builder::IndicesPutAliasPost {
    builder::IndicesPutAliasPost::new(self.os_client)
  }

  ///Deletes an alias.
  ///
  ///Sends a `DELETE` request to `/{index}/_alias/{name}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `name`: Comma-separated list of aliases to delete (supports wildcards);
  ///   use `_all` to delete all aliases for the specified indices.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.indices().delete_alias()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_alias(&self) -> builder::IndicesDeleteAlias {
    builder::IndicesDeleteAlias::new(self.os_client)
  }

  ///Returns information about whether a particular alias exists.
  ///
  ///Sends a `HEAD` request to `/{index}/_alias/{name}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to filter aliases.
  /// - `name`: Comma-separated list of alias names.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  ///```ignore
  /// let response = client.indices().exists_alias_with_index()
  ///    .index(index)
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn exists_alias_with_index(&self) -> builder::IndicesExistsAliasWithIndex {
    builder::IndicesExistsAliasWithIndex::new(self.os_client)
  }

  ///Creates or updates an alias.
  ///
  ///Sends a `PUT` request to `/{index}/_aliases/{name}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `name`: The name of the alias to be created or updated.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_alias_put_plural()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_alias_put_plural(&self) -> builder::IndicesPutAliasPutPlural {
    builder::IndicesPutAliasPutPlural::new(self.os_client)
  }

  ///Creates or updates an alias.
  ///
  ///Sends a `POST` request to `/{index}/_aliases/{name}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `name`: The name of the alias to be created or updated.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_alias_post_plural()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_alias_post_plural(&self) -> builder::IndicesPutAliasPostPlural {
    builder::IndicesPutAliasPostPlural::new(self.os_client)
  }

  ///Deletes an alias.
  ///
  ///Sends a `DELETE` request to `/{index}/_aliases/{name}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `name`: Comma-separated list of aliases to delete (supports wildcards);
  ///   use `_all` to delete all aliases for the specified indices.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.indices().delete_alias_plural()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_alias_plural(&self) -> builder::IndicesDeleteAliasPlural {
    builder::IndicesDeleteAliasPlural::new(self.os_client)
  }

  ///Performs the analysis process on a text and return the tokens breakdown
  /// of the text.
  ///
  ///Sends a `GET` request to `/{index}/_analyze`
  ///
  ///Arguments:
  /// - `index`: The name of the index to scope the operation.
  ///```ignore
  /// let response = client.indices().analyze_get_with_index()
  ///    .index(index)
  ///    .send()
  ///    .await;
  /// ```
  pub fn analyze_get_with_index(&self) -> builder::IndicesAnalyzeGetWithIndex {
    builder::IndicesAnalyzeGetWithIndex::new(self.os_client)
  }

  ///Performs the analysis process on a text and return the tokens breakdown
  /// of the text.
  ///
  ///Sends a `POST` request to `/{index}/_analyze`
  ///
  ///Arguments:
  /// - `index`: The name of the index to scope the operation.
  /// - `body`
  ///```ignore
  /// let response = client.indices().analyze_post_with_index()
  ///    .index(index)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn analyze_post_with_index(&self) -> builder::IndicesAnalyzePostWithIndex {
    builder::IndicesAnalyzePostWithIndex::new(self.os_client)
  }

  ///Adds a block to an index.
  ///
  ///Sends a `PUT` request to `/{index}/_block/{block}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to add a block to.
  /// - `block`: The block to add (one of read, write, read_only or metadata).
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.indices().add_block()
  ///    .index(index)
  ///    .block(block)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn add_block(&self) -> builder::IndicesAddBlock {
    builder::IndicesAddBlock::new(self.os_client)
  }

  ///Clears all or specific caches for one or more indices.
  ///
  ///Sends a `POST` request to `/{index}/_cache/clear`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fielddata`: Clear field data.
  /// - `fields`: Comma-separated list of fields to clear when using the
  ///   `fielddata` parameter (default: all).
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `query`: Clear query caches.
  /// - `request`: Clear request cache.
  ///```ignore
  /// let response = client.indices().clear_cache_with_index()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fielddata(fielddata)
  ///    .fields(fields)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .index(index)
  ///    .query(query)
  ///    .request(request)
  ///    .send()
  ///    .await;
  /// ```
  pub fn clear_cache_with_index(&self) -> builder::IndicesClearCacheWithIndex {
    builder::IndicesClearCacheWithIndex::new(self.os_client)
  }

  ///Clones an index.
  ///
  ///Sends a `PUT` request to `/{index}/_clone/{target}`
  ///
  ///Arguments:
  /// - `index`: The name of the source index to clone.
  /// - `target`: The name of the target index.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Set the number of active shards to wait for on
  ///   the cloned index before the operation returns.
  /// - `body`
  ///```ignore
  /// let response = client.indices().clone_put()
  ///    .index(index)
  ///    .target(target)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn clone_put(&self) -> builder::IndicesClonePut {
    builder::IndicesClonePut::new(self.os_client)
  }

  ///Clones an index.
  ///
  ///Sends a `POST` request to `/{index}/_clone/{target}`
  ///
  ///Arguments:
  /// - `index`: The name of the source index to clone.
  /// - `target`: The name of the target index.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Set the number of active shards to wait for on
  ///   the cloned index before the operation returns.
  /// - `body`
  ///```ignore
  /// let response = client.indices().clone_post()
  ///    .index(index)
  ///    .target(target)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn clone_post(&self) -> builder::IndicesClonePost {
    builder::IndicesClonePost::new(self.os_client)
  }

  ///Closes an index.
  ///
  ///Sends a `POST` request to `/{index}/_close`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to close.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Sets the number of active shards to wait for
  ///   before the operation returns.
  ///```ignore
  /// let response = client.indices().close()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .send()
  ///    .await;
  /// ```
  pub fn close(&self) -> builder::IndicesClose {
    builder::IndicesClose::new(self.os_client)
  }

  ///Performs the flush operation on one or more indices.
  ///
  ///Sends a `GET` request to `/{index}/_flush`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `force`: Whether a flush should be forced even if it is not necessarily
  ///   needed ie. if no changes will be committed to the index. This is useful
  ///   if transaction log IDs should be incremented even if no uncommitted
  ///   changes are present. (This setting can be considered as internal).
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `wait_if_ongoing`: If set to true the flush operation will block until
  ///   the flush can be executed if another flush operation is already
  ///   executing. If set to false the flush will be skipped iff if another
  ///   flush operation is already running.
  ///```ignore
  /// let response = client.indices().flush_get_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .force(force)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .wait_if_ongoing(wait_if_ongoing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn flush_get_with_index(&self) -> builder::IndicesFlushGetWithIndex {
    builder::IndicesFlushGetWithIndex::new(self.os_client)
  }

  ///Performs the flush operation on one or more indices.
  ///
  ///Sends a `POST` request to `/{index}/_flush`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `force`: Whether a flush should be forced even if it is not necessarily
  ///   needed ie. if no changes will be committed to the index. This is useful
  ///   if transaction log IDs should be incremented even if no uncommitted
  ///   changes are present. (This setting can be considered as internal).
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `wait_if_ongoing`: If set to true the flush operation will block until
  ///   the flush can be executed if another flush operation is already
  ///   executing. If set to false the flush will be skipped iff if another
  ///   flush operation is already running.
  ///```ignore
  /// let response = client.indices().flush_post_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .force(force)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .wait_if_ongoing(wait_if_ongoing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn flush_post_with_index(&self) -> builder::IndicesFlushPostWithIndex {
    builder::IndicesFlushPostWithIndex::new(self.os_client)
  }

  ///Performs the force merge operation on one or more indices.
  ///
  ///Sends a `POST` request to `/{index}/_forcemerge`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `flush`: Specify whether the index should be flushed after performing
  ///   the operation.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `max_num_segments`: The number of segments the index should be merged
  ///   into (default: dynamic).
  /// - `only_expunge_deletes`: Specify whether the operation should only
  ///   expunge deleted documents.
  ///```ignore
  /// let response = client.indices().forcemerge_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flush(flush)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .max_num_segments(max_num_segments)
  ///    .only_expunge_deletes(only_expunge_deletes)
  ///    .send()
  ///    .await;
  /// ```
  pub fn forcemerge_with_index(&self) -> builder::IndicesForcemergeWithIndex {
    builder::IndicesForcemergeWithIndex::new(self.os_client)
  }

  ///Returns mappings for one or more indices.
  ///
  ///Sends a `GET` request to `/{index}/_mapping`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_mapping_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_mapping_with_index(&self) -> builder::IndicesGetMappingWithIndex {
    builder::IndicesGetMappingWithIndex::new(self.os_client)
  }

  ///Updates the index mappings.
  ///
  ///Sends a `PUT` request to `/{index}/_mapping`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `write_index_only`: When true, applies mappings only to the write index
  ///   of an alias or data stream.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_mapping_put()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .write_index_only(write_index_only)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_mapping_put(&self) -> builder::IndicesPutMappingPut {
    builder::IndicesPutMappingPut::new(self.os_client)
  }

  ///Updates the index mappings.
  ///
  ///Sends a `POST` request to `/{index}/_mapping`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `write_index_only`: When true, applies mappings only to the write index
  ///   of an alias or data stream.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_mapping_post()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .write_index_only(write_index_only)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_mapping_post(&self) -> builder::IndicesPutMappingPost {
    builder::IndicesPutMappingPost::new(self.os_client)
  }

  ///Returns mapping for one or more fields.
  ///
  ///Sends a `GET` request to `/{index}/_mapping/field/{fields}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices.
  /// - `fields`: Comma-separated list of fields.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `include_defaults`: Whether the default mapping values should be
  ///   returned as well.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  ///```ignore
  /// let response = client.indices().get_field_mapping_with_index()
  ///    .index(index)
  ///    .fields(fields)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_defaults(include_defaults)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_field_mapping_with_index(&self) -> builder::IndicesGetFieldMappingWithIndex {
    builder::IndicesGetFieldMappingWithIndex::new(self.os_client)
  }

  ///Opens an index.
  ///
  ///Sends a `POST` request to `/{index}/_open`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to open.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Sets the number of active shards to wait for
  ///   before the operation returns.
  ///```ignore
  /// let response = client.indices().open()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .send()
  ///    .await;
  /// ```
  pub fn open(&self) -> builder::IndicesOpen {
    builder::IndicesOpen::new(self.os_client)
  }

  ///Returns information about ongoing index shard recoveries.
  ///
  ///Sends a `GET` request to `/{index}/_recovery`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `active_only`: Display only those recoveries that are currently
  ///   on-going.
  /// - `detailed`: Whether to display detailed information about shard
  ///   recovery.
  ///```ignore
  /// let response = client.indices().recovery_with_index()
  ///    .index(index)
  ///    .active_only(active_only)
  ///    .detailed(detailed)
  ///    .send()
  ///    .await;
  /// ```
  pub fn recovery_with_index(&self) -> builder::IndicesRecoveryWithIndex {
    builder::IndicesRecoveryWithIndex::new(self.os_client)
  }

  ///Performs the refresh operation in one or more indices.
  ///
  ///Sends a `GET` request to `/{index}/_refresh`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  ///```ignore
  /// let response = client.indices().refresh_get_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn refresh_get_with_index(&self) -> builder::IndicesRefreshGetWithIndex {
    builder::IndicesRefreshGetWithIndex::new(self.os_client)
  }

  ///Performs the refresh operation in one or more indices.
  ///
  ///Sends a `POST` request to `/{index}/_refresh`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  ///```ignore
  /// let response = client.indices().refresh_post_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn refresh_post_with_index(&self) -> builder::IndicesRefreshPostWithIndex {
    builder::IndicesRefreshPostWithIndex::new(self.os_client)
  }

  ///Provides low-level information about segments in a Lucene index.
  ///
  ///Sends a `GET` request to `/{index}/_segments`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `verbose`: Includes detailed memory usage by Lucene.
  ///```ignore
  /// let response = client.indices().segments_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .verbose(verbose)
  ///    .send()
  ///    .await;
  /// ```
  pub fn segments_with_index(&self) -> builder::IndicesSegmentsWithIndex {
    builder::IndicesSegmentsWithIndex::new(self.os_client)
  }

  ///Returns settings for one or more indices.
  ///
  ///Sends a `GET` request to `/{index}/_settings`
  ///
  ///Arguments:
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
  /// - `include_defaults`: Whether to return all default setting for each of
  ///   the indices.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_settings_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_defaults(include_defaults)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_settings_with_index(&self) -> builder::IndicesGetSettingsWithIndex {
    builder::IndicesGetSettingsWithIndex::new(self.os_client)
  }

  ///Updates the index settings.
  ///
  ///Sends a `PUT` request to `/{index}/_settings`
  ///
  ///Arguments:
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
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `preserve_existing`: Whether to update existing settings. If set to
  ///   `true` existing settings on an index remain unchanged.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.indices().put_settings_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .preserve_existing(preserve_existing)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_settings_with_index(&self) -> builder::IndicesPutSettingsWithIndex {
    builder::IndicesPutSettingsWithIndex::new(self.os_client)
  }

  ///Returns settings for one or more indices.
  ///
  ///Sends a `GET` request to `/{index}/_settings/{name}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `name`: Comma-separated list of settings.
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
  /// - `include_defaults`: Whether to return all default setting for each of
  ///   the indices.
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.indices().get_settings_with_index_name()
  ///    .index(index)
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flat_settings(flat_settings)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_defaults(include_defaults)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_settings_with_index_name(&self) -> builder::IndicesGetSettingsWithIndexName {
    builder::IndicesGetSettingsWithIndexName::new(self.os_client)
  }

  ///Provides store information for shard copies of indices.
  ///
  ///Sends a `GET` request to `/{index}/_shard_stores`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `status`: Comma-separated list of statuses used to filter on shards to
  ///   get store information for.
  ///```ignore
  /// let response = client.indices().shard_stores_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .status(status)
  ///    .send()
  ///    .await;
  /// ```
  pub fn shard_stores_with_index(&self) -> builder::IndicesShardStoresWithIndex {
    builder::IndicesShardStoresWithIndex::new(self.os_client)
  }

  ///Allow to shrink an existing index into a new index with fewer primary
  /// shards.
  ///
  ///Sends a `PUT` request to `/{index}/_shrink/{target}`
  ///
  ///Arguments:
  /// - `index`: The name of the source index to shrink.
  /// - `target`: The name of the target index.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `copy_settings`: whether or not to copy settings from the source index.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Set the number of active shards to wait for on
  ///   the shrunken index before the operation returns.
  /// - `body`
  ///```ignore
  /// let response = client.indices().shrink_put()
  ///    .index(index)
  ///    .target(target)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .copy_settings(copy_settings)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn shrink_put(&self) -> builder::IndicesShrinkPut {
    builder::IndicesShrinkPut::new(self.os_client)
  }

  ///Allow to shrink an existing index into a new index with fewer primary
  /// shards.
  ///
  ///Sends a `POST` request to `/{index}/_shrink/{target}`
  ///
  ///Arguments:
  /// - `index`: The name of the source index to shrink.
  /// - `target`: The name of the target index.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `copy_settings`: whether or not to copy settings from the source index.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Set the number of active shards to wait for on
  ///   the shrunken index before the operation returns.
  /// - `body`
  ///```ignore
  /// let response = client.indices().shrink_post()
  ///    .index(index)
  ///    .target(target)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .copy_settings(copy_settings)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn shrink_post(&self) -> builder::IndicesShrinkPost {
    builder::IndicesShrinkPost::new(self.os_client)
  }

  ///Allows you to split an existing index into a new index with more primary
  /// shards.
  ///
  ///Sends a `PUT` request to `/{index}/_split/{target}`
  ///
  ///Arguments:
  /// - `index`: The name of the source index to split.
  /// - `target`: The name of the target index.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `copy_settings`: whether or not to copy settings from the source index.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Set the number of active shards to wait for on
  ///   the shrunken index before the operation returns.
  /// - `body`
  ///```ignore
  /// let response = client.indices().split_put()
  ///    .index(index)
  ///    .target(target)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .copy_settings(copy_settings)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn split_put(&self) -> builder::IndicesSplitPut {
    builder::IndicesSplitPut::new(self.os_client)
  }

  ///Allows you to split an existing index into a new index with more primary
  /// shards.
  ///
  ///Sends a `POST` request to `/{index}/_split/{target}`
  ///
  ///Arguments:
  /// - `index`: The name of the source index to split.
  /// - `target`: The name of the target index.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `copy_settings`: whether or not to copy settings from the source index.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Set the number of active shards to wait for on
  ///   the shrunken index before the operation returns.
  /// - `body`
  ///```ignore
  /// let response = client.indices().split_post()
  ///    .index(index)
  ///    .target(target)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .copy_settings(copy_settings)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn split_post(&self) -> builder::IndicesSplitPost {
    builder::IndicesSplitPost::new(self.os_client)
  }

  ///Provides statistics on operations happening in an index.
  ///
  ///Sends a `GET` request to `/{index}/_stats`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `forbid_closed_indices`: If set to false stats will also collected from
  ///   closed indices if explicitly specified or if expand_wildcards expands to
  ///   closed indices.
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `include_unloaded_segments`: If set to true segment stats will include
  ///   stats for segments that are not currently loaded into memory.
  /// - `level`: Return stats aggregated at cluster, index or shard level.
  ///```ignore
  /// let response = client.indices().stats_with_index()
  ///    .index(index)
  ///    .completion_fields(completion_fields)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .forbid_closed_indices(forbid_closed_indices)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .include_unloaded_segments(include_unloaded_segments)
  ///    .level(level)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats_with_index(&self) -> builder::IndicesStatsWithIndex {
    builder::IndicesStatsWithIndex::new(self.os_client)
  }

  ///Provides statistics on operations happening in an index.
  ///
  ///Sends a `GET` request to `/{index}/_stats/{metric}`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `metric`: Limit the information returned the specific metrics.
  /// - `completion_fields`: Comma-separated list of fields for `fielddata` and
  ///   `suggest` index metric (supports wildcards).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fielddata_fields`: Comma-separated list of fields for `fielddata` index
  ///   metric (supports wildcards).
  /// - `fields`: Comma-separated list of fields for `fielddata` and
  ///   `completion` index metric (supports wildcards).
  /// - `forbid_closed_indices`: If set to false stats will also collected from
  ///   closed indices if explicitly specified or if expand_wildcards expands to
  ///   closed indices.
  /// - `groups`: Comma-separated list of search groups for `search` index
  ///   metric.
  /// - `include_segment_file_sizes`: Whether to report the aggregated disk
  ///   usage of each one of the Lucene index files (only applies if segment
  ///   stats are requested).
  /// - `include_unloaded_segments`: If set to true segment stats will include
  ///   stats for segments that are not currently loaded into memory.
  /// - `level`: Return stats aggregated at cluster, index or shard level.
  ///```ignore
  /// let response = client.indices().stats_with_index_metric()
  ///    .index(index)
  ///    .metric(metric)
  ///    .completion_fields(completion_fields)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fielddata_fields(fielddata_fields)
  ///    .fields(fields)
  ///    .forbid_closed_indices(forbid_closed_indices)
  ///    .groups(groups)
  ///    .include_segment_file_sizes(include_segment_file_sizes)
  ///    .include_unloaded_segments(include_unloaded_segments)
  ///    .level(level)
  ///    .send()
  ///    .await;
  /// ```
  pub fn stats_with_index_metric(&self) -> builder::IndicesStatsWithIndexMetric {
    builder::IndicesStatsWithIndexMetric::new(self.os_client)
  }

  ///The _upgrade API is no longer useful and will be removed.
  ///
  ///Sends a `GET` request to `/{index}/_upgrade`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  ///```ignore
  /// let response = client.indices().get_upgrade_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_upgrade_with_index(&self) -> builder::IndicesGetUpgradeWithIndex {
    builder::IndicesGetUpgradeWithIndex::new(self.os_client)
  }

  ///The _upgrade API is no longer useful and will be removed.
  ///
  ///Sends a `POST` request to `/{index}/_upgrade`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `only_ancient_segments`: If true, only ancient (an older Lucene major
  ///   release) segments will be upgraded.
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  ///```ignore
  /// let response = client.indices().upgrade_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .only_ancient_segments(only_ancient_segments)
  ///    .wait_for_completion(wait_for_completion)
  ///    .send()
  ///    .await;
  /// ```
  pub fn upgrade_with_index(&self) -> builder::IndicesUpgradeWithIndex {
    builder::IndicesUpgradeWithIndex::new(self.os_client)
  }

  ///Allows a user to validate a potentially expensive query without
  /// executing it.
  ///
  ///Sends a `GET` request to `/{index}/_validate/query`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `all_shards`: Execute validation on all shards instead of one random
  ///   shard per index.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Return detailed information about the error.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `rewrite`: Provide a more detailed explanation showing the actual Lucene
  ///   query that will be executed.
  ///```ignore
  /// let response = client.indices().validate_query_get_with_index()
  ///    .index(index)
  ///    .all_shards(all_shards)
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .q(q)
  ///    .rewrite(rewrite)
  ///    .send()
  ///    .await;
  /// ```
  pub fn validate_query_get_with_index(&self) -> builder::IndicesValidateQueryGetWithIndex {
    builder::IndicesValidateQueryGetWithIndex::new(self.os_client)
  }

  ///Allows a user to validate a potentially expensive query without
  /// executing it.
  ///
  ///Sends a `POST` request to `/{index}/_validate/query`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `all_shards`: Execute validation on all shards instead of one random
  ///   shard per index.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Return detailed information about the error.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `rewrite`: Provide a more detailed explanation showing the actual Lucene
  ///   query that will be executed.
  /// - `body`
  ///```ignore
  /// let response = client.indices().validate_query_post_with_index()
  ///    .index(index)
  ///    .all_shards(all_shards)
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .q(q)
  ///    .rewrite(rewrite)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn validate_query_post_with_index(&self) -> builder::IndicesValidateQueryPostWithIndex {
    builder::IndicesValidateQueryPostWithIndex::new(self.os_client)
  }
}
