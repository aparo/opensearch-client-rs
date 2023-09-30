mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, encode_path_option_vec_string, RequestBuilderExt};
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types;
pub mod builder;
#[derive(Clone, Debug)]
///Client for OpenSearch
///
///Version: 2021-11-23
pub struct Client {
  pub(crate) baseurl: String,
  pub(crate) client: reqwest::Client,
}

impl Client {
  /// Create a new client.
  ///
  /// `baseurl` is the base URL provided to the internal
  /// `reqwest::Client`, and should include a scheme and hostname,
  /// as well as port and a path stem if applicable.
  pub fn new(baseurl: &str) -> Self {
    #[cfg(not(target_arch = "wasm32"))]
    let client = {
      let dur = std::time::Duration::from_secs(15);
      reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
    };
    #[cfg(target_arch = "wasm32")]
    let client = reqwest::ClientBuilder::new();
    Self::new_with_client(baseurl, client.build().unwrap())
  }

  /// Construct a new client with an existing `reqwest::Client`,
  /// allowing more control over its configuration.
  ///
  /// `baseurl` is the base URL provided to the internal
  /// `reqwest::Client`, and should include a scheme and hostname,
  /// as well as port and a path stem if applicable.
  pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
    Self {
      baseurl: baseurl.to_string(),
      client,
    }
  }

  /// Get the base URL to which requests are made.
  pub fn baseurl(&self) -> &String {
    &self.baseurl
  }

  /// Get the internal `reqwest::Client` used to make requests.
  pub fn client(&self) -> &reqwest::Client {
    &self.client
  }

  /// Get the version of this API.
  ///
  /// This string is pulled directly from the source OpenAPI
  /// document and may be in any format the API selects.
  pub fn api_version(&self) -> &'static str {
    "2021-11-23"
  }
}

impl Client {
  ///Returns basic information about the cluster.
  ///
  ///Sends a `GET` request to `/`
  ///
  ///```ignore
  /// let response = client.info()
  ///    .send()
  ///    .await;
  /// ```
  pub fn info(&self) -> builder::Info {
    builder::Info::new(self)
  }

  ///Returns whether the cluster is running.
  ///
  ///Sends a `HEAD` request to `/`
  ///
  ///```ignore
  /// let response = client.ping()
  ///    .send()
  ///    .await;
  /// ```
  pub fn ping(&self) -> builder::Ping {
    builder::Ping::new(self)
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
  /// let response = client.indices_get_alias()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_alias(&self) -> builder::IndicesGetAlias {
    builder::IndicesGetAlias::new(self)
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
  /// let response = client.indices_get_alias_with_name()
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_alias_with_name(&self) -> builder::IndicesGetAliasWithName {
    builder::IndicesGetAliasWithName::new(self)
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
  /// let response = client.indices_exists_alias()
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_exists_alias(&self) -> builder::IndicesExistsAlias {
    builder::IndicesExistsAlias::new(self)
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
  /// let response = client.indices_update_aliases()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_update_aliases(&self) -> builder::IndicesUpdateAliases {
    builder::IndicesUpdateAliases::new(self)
  }

  ///Performs the analysis process on a text and return the tokens breakdown
  /// of the text.
  ///
  ///Sends a `GET` request to `/_analyze`
  ///
  ///Arguments:
  /// - `index`: The name of the index to scope the operation.
  ///```ignore
  /// let response = client.indices_analyze_get()
  ///    .index(index)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_analyze_get(&self) -> builder::IndicesAnalyzeGet {
    builder::IndicesAnalyzeGet::new(self)
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
  /// let response = client.indices_analyze_post()
  ///    .index(index)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_analyze_post(&self) -> builder::IndicesAnalyzePost {
    builder::IndicesAnalyzePost::new(self)
  }

  ///Allows to perform multiple index/update/delete operations in a single
  /// request.
  ///
  ///Sends a `PUT` request to `/_bulk`
  ///
  ///Arguments:
  /// - `source`: True or false to return the _source field or not, or default
  ///   list of fields to return, can be overridden on each sub-request.
  /// - `source_excludes`: Default list of fields to exclude from the returned
  ///   _source field, can be overridden on each sub-request.
  /// - `source_includes`: Default list of fields to extract and return from the
  ///   _source field, can be overridden on each sub-request.
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `require_alias`: Sets require_alias for all incoming documents.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `type_`: Default document type for items which don't provide one.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.bulk_put()
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .pipeline(pipeline)
  ///    .refresh(refresh)
  ///    .require_alias(require_alias)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn bulk_put(&self) -> builder::BulkPut {
    builder::BulkPut::new(self)
  }

  ///Allows to perform multiple index/update/delete operations in a single
  /// request.
  ///
  ///Sends a `POST` request to `/_bulk`
  ///
  ///Arguments:
  /// - `source`: True or false to return the _source field or not, or default
  ///   list of fields to return, can be overridden on each sub-request.
  /// - `source_excludes`: Default list of fields to exclude from the returned
  ///   _source field, can be overridden on each sub-request.
  /// - `source_includes`: Default list of fields to extract and return from the
  ///   _source field, can be overridden on each sub-request.
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `require_alias`: Sets require_alias for all incoming documents.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `type_`: Default document type for items which don't provide one.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.bulk_post()
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .pipeline(pipeline)
  ///    .refresh(refresh)
  ///    .require_alias(require_alias)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn bulk_post(&self) -> builder::BulkPost {
    builder::BulkPost::new(self)
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
  /// let response = client.indices_clear_cache()
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
  pub fn indices_clear_cache(&self) -> builder::IndicesClearCache {
    builder::IndicesClearCache::new(self)
  }

  ///Returns help for the Cat APIs.
  ///
  ///Sends a `GET` request to `/_cat`
  ///
  ///Arguments:
  /// - `help`: Return help information.
  /// - `s`: Comma-separated list of column names or column aliases to sort by.
  ///```ignore
  /// let response = client.cat_help()
  ///    .help(help)
  ///    .s(s)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cat_help(&self) -> builder::CatHelp {
    builder::CatHelp::new(self)
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
  /// let response = client.cat_aliases()
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
  pub fn cat_aliases(&self) -> builder::CatAliases {
    builder::CatAliases::new(self)
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
  /// let response = client.cat_aliases_with_name()
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
  pub fn cat_aliases_with_name(&self) -> builder::CatAliasesWithName {
    builder::CatAliasesWithName::new(self)
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
  /// let response = client.cat_allocation()
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
  pub fn cat_allocation(&self) -> builder::CatAllocation {
    builder::CatAllocation::new(self)
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
  /// let response = client.cat_allocation_with_node_id()
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
  pub fn cat_allocation_with_node_id(&self) -> builder::CatAllocationWithNodeId {
    builder::CatAllocationWithNodeId::new(self)
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
  /// let response = client.cat_cluster_manager()
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
  pub fn cat_cluster_manager(&self) -> builder::CatClusterManager {
    builder::CatClusterManager::new(self)
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
  /// let response = client.cat_count()
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cat_count(&self) -> builder::CatCount {
    builder::CatCount::new(self)
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
  /// let response = client.cat_count_with_index()
  ///    .index(index)
  ///    .format(format)
  ///    .h(h)
  ///    .help(help)
  ///    .s(s)
  ///    .v(v)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cat_count_with_index(&self) -> builder::CatCountWithIndex {
    builder::CatCountWithIndex::new(self)
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
  /// let response = client.cat_fielddata()
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
  pub fn cat_fielddata(&self) -> builder::CatFielddata {
    builder::CatFielddata::new(self)
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
  /// let response = client.cat_fielddata_with_fields()
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
  pub fn cat_fielddata_with_fields(&self) -> builder::CatFielddataWithFields {
    builder::CatFielddataWithFields::new(self)
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
  /// let response = client.cat_health()
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
  pub fn cat_health(&self) -> builder::CatHealth {
    builder::CatHealth::new(self)
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
  /// let response = client.cat_indices()
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
  pub fn cat_indices(&self) -> builder::CatIndices {
    builder::CatIndices::new(self)
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
  /// let response = client.cat_indices_with_index()
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
  pub fn cat_indices_with_index(&self) -> builder::CatIndicesWithIndex {
    builder::CatIndicesWithIndex::new(self)
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
  /// let response = client.cat_master()
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
  pub fn cat_master(&self) -> builder::CatMaster {
    builder::CatMaster::new(self)
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
  /// let response = client.cat_nodeattrs()
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
  pub fn cat_nodeattrs(&self) -> builder::CatNodeattrs {
    builder::CatNodeattrs::new(self)
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
  /// let response = client.cat_nodes()
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
  pub fn cat_nodes(&self) -> builder::CatNodes {
    builder::CatNodes::new(self)
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
  /// let response = client.cat_pending_tasks()
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
  pub fn cat_pending_tasks(&self) -> builder::CatPendingTasks {
    builder::CatPendingTasks::new(self)
  }

  ///List segments for one or several PITs.
  ///
  ///Sends a `GET` request to `/_cat/pit_segments`
  ///
  ///```ignore
  /// let response = client.cat_pit_segments()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn cat_pit_segments(&self) -> builder::CatPitSegments {
    builder::CatPitSegments::new(self)
  }

  ///Lists all active point-in-time segments.
  ///
  ///Sends a `GET` request to `/_cat/pit_segments/_all`
  ///
  ///```ignore
  /// let response = client.cat_all_pit_segments()
  ///    .send()
  ///    .await;
  /// ```
  pub fn cat_all_pit_segments(&self) -> builder::CatAllPitSegments {
    builder::CatAllPitSegments::new(self)
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
  /// let response = client.cat_plugins()
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
  pub fn cat_plugins(&self) -> builder::CatPlugins {
    builder::CatPlugins::new(self)
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
  /// let response = client.cat_recovery()
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
  pub fn cat_recovery(&self) -> builder::CatRecovery {
    builder::CatRecovery::new(self)
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
  /// let response = client.cat_recovery_with_index()
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
  pub fn cat_recovery_with_index(&self) -> builder::CatRecoveryWithIndex {
    builder::CatRecoveryWithIndex::new(self)
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
  /// let response = client.cat_repositories()
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
  pub fn cat_repositories(&self) -> builder::CatRepositories {
    builder::CatRepositories::new(self)
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
  /// let response = client.cat_segment_replication()
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
  pub fn cat_segment_replication(&self) -> builder::CatSegmentReplication {
    builder::CatSegmentReplication::new(self)
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
  /// let response = client.cat_segment_replication_with_index()
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
  pub fn cat_segment_replication_with_index(&self) -> builder::CatSegmentReplicationWithIndex {
    builder::CatSegmentReplicationWithIndex::new(self)
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
  /// let response = client.cat_segments()
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
  pub fn cat_segments(&self) -> builder::CatSegments {
    builder::CatSegments::new(self)
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
  /// let response = client.cat_segments_with_index()
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
  pub fn cat_segments_with_index(&self) -> builder::CatSegmentsWithIndex {
    builder::CatSegmentsWithIndex::new(self)
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
  /// let response = client.cat_shards()
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
  pub fn cat_shards(&self) -> builder::CatShards {
    builder::CatShards::new(self)
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
  /// let response = client.cat_shards_with_index()
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
  pub fn cat_shards_with_index(&self) -> builder::CatShardsWithIndex {
    builder::CatShardsWithIndex::new(self)
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
  /// let response = client.cat_snapshots()
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
  pub fn cat_snapshots(&self) -> builder::CatSnapshots {
    builder::CatSnapshots::new(self)
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
  /// let response = client.cat_snapshots_with_repository()
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
  pub fn cat_snapshots_with_repository(&self) -> builder::CatSnapshotsWithRepository {
    builder::CatSnapshotsWithRepository::new(self)
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
  /// let response = client.cat_tasks()
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
  pub fn cat_tasks(&self) -> builder::CatTasks {
    builder::CatTasks::new(self)
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
  /// let response = client.cat_templates()
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
  pub fn cat_templates(&self) -> builder::CatTemplates {
    builder::CatTemplates::new(self)
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
  /// let response = client.cat_templates_with_name()
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
  pub fn cat_templates_with_name(&self) -> builder::CatTemplatesWithName {
    builder::CatTemplatesWithName::new(self)
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
  /// let response = client.cat_thread_pool()
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
  pub fn cat_thread_pool(&self) -> builder::CatThreadPool {
    builder::CatThreadPool::new(self)
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
  /// let response = client.cat_thread_pool_with_thread_pool_patterns()
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
  pub fn cat_thread_pool_with_thread_pool_patterns(&self) -> builder::CatThreadPoolWithThreadPoolPatterns {
    builder::CatThreadPoolWithThreadPoolPatterns::new(self)
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
    builder::ClusterAllocationExplainGet::new(self)
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
    builder::ClusterAllocationExplainPost::new(self)
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
    builder::ClusterDeleteDecommissionAwareness::new(self)
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
    builder::ClusterGetDecommissionAwareness::new(self)
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
    builder::ClusterPutDecommissionAwareness::new(self)
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
    builder::ClusterHealth::new(self)
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
    builder::ClusterHealthWithIndex::new(self)
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
  /// let response = client.nodes_hot_threads_deprecated_dash()
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_hot_threads_deprecated_dash(&self) -> builder::NodesHotThreadsDeprecatedDash {
    builder::NodesHotThreadsDeprecatedDash::new(self)
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
  /// let response = client.nodes_hot_threads_deprecated_cluster()
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_hot_threads_deprecated_cluster(&self) -> builder::NodesHotThreadsDeprecatedCluster {
    builder::NodesHotThreadsDeprecatedCluster::new(self)
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
  /// let response = client.nodes_hot_threads_with_node_id_deprecated_dash()
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
  pub fn nodes_hot_threads_with_node_id_deprecated_dash(&self) -> builder::NodesHotThreadsWithNodeIdDeprecatedDash {
    builder::NodesHotThreadsWithNodeIdDeprecatedDash::new(self)
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
  /// let response = client.nodes_hot_threads_with_node_id_deprecated_cluster()
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
  pub fn nodes_hot_threads_with_node_id_deprecated_cluster(
    &self,
  ) -> builder::NodesHotThreadsWithNodeIdDeprecatedCluster {
    builder::NodesHotThreadsWithNodeIdDeprecatedCluster::new(self)
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
    builder::ClusterPendingTasks::new(self)
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
    builder::ClusterReroute::new(self)
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
    builder::ClusterDeleteWeightedRouting::new(self)
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
    builder::ClusterGetWeightedRouting::new(self)
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
    builder::ClusterPutWeightedRouting::new(self)
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
    builder::ClusterGetSettings::new(self)
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
    builder::ClusterPutSettings::new(self)
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
    builder::ClusterState::new(self)
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
    builder::ClusterStateWithMetric::new(self)
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
    builder::ClusterStateWithIndexMetric::new(self)
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
    builder::ClusterStats::new(self)
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
    builder::ClusterStatsWithNodeId::new(self)
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
    builder::ClusterPostVotingConfigExclusions::new(self)
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
    builder::ClusterDeleteVotingConfigExclusions::new(self)
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
    builder::ClusterGetComponentTemplate::new(self)
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
    builder::ClusterGetComponentTemplateWithName::new(self)
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
    builder::ClusterPutComponentTemplatePut::new(self)
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
    builder::ClusterPutComponentTemplatePost::new(self)
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
    builder::ClusterDeleteComponentTemplate::new(self)
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
    builder::ClusterExistsComponentTemplate::new(self)
  }

  ///Returns number of documents matching a query.
  ///
  ///Sends a `GET` request to `/_count`
  ///
  ///Arguments:
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
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `min_score`: Include only documents with a specific `_score` value in
  ///   the result.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  ///```ignore
  /// let response = client.count_get()
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .min_score(min_score)
  ///    .preference(preference)
  ///    .q(q)
  ///    .routing(routing)
  ///    .terminate_after(terminate_after)
  ///    .send()
  ///    .await;
  /// ```
  pub fn count_get(&self) -> builder::CountGet {
    builder::CountGet::new(self)
  }

  ///Returns number of documents matching a query.
  ///
  ///Sends a `POST` request to `/_count`
  ///
  ///Arguments:
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
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `min_score`: Include only documents with a specific `_score` value in
  ///   the result.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  /// - `body`
  ///```ignore
  /// let response = client.count_post()
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .min_score(min_score)
  ///    .preference(preference)
  ///    .q(q)
  ///    .routing(routing)
  ///    .terminate_after(terminate_after)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn count_post(&self) -> builder::CountPost {
    builder::CountPost::new(self)
  }

  ///Returns all dangling indices.
  ///
  ///Sends a `GET` request to `/_dangling`
  ///
  ///```ignore
  /// let response = client.dangling_indices_list_dangling_indices()
  ///    .send()
  ///    .await;
  /// ```
  pub fn dangling_indices_list_dangling_indices(&self) -> builder::DanglingIndicesListDanglingIndices {
    builder::DanglingIndicesListDanglingIndices::new(self)
  }

  ///Imports the specified dangling index.
  ///
  ///Sends a `POST` request to `/_dangling/{index_uuid}`
  ///
  ///Arguments:
  /// - `index_uuid`: The UUID of the dangling index.
  /// - `accept_data_loss`: Must be set to true in order to import the dangling
  ///   index.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.dangling_indices_import_dangling_index()
  ///    .index_uuid(index_uuid)
  ///    .accept_data_loss(accept_data_loss)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn dangling_indices_import_dangling_index(&self) -> builder::DanglingIndicesImportDanglingIndex {
    builder::DanglingIndicesImportDanglingIndex::new(self)
  }

  ///Deletes the specified dangling index.
  ///
  ///Sends a `DELETE` request to `/_dangling/{index_uuid}`
  ///
  ///Arguments:
  /// - `index_uuid`: The UUID of the dangling index.
  /// - `accept_data_loss`: Must be set to true in order to delete the dangling
  ///   index.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.dangling_indices_delete_dangling_index()
  ///    .index_uuid(index_uuid)
  ///    .accept_data_loss(accept_data_loss)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn dangling_indices_delete_dangling_index(&self) -> builder::DanglingIndicesDeleteDanglingIndex {
    builder::DanglingIndicesDeleteDanglingIndex::new(self)
  }

  ///Returns data streams.
  ///
  ///Sends a `GET` request to `/_data_stream`
  ///
  ///```ignore
  /// let response = client.indices_get_data_stream()
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_data_stream(&self) -> builder::IndicesGetDataStream {
    builder::IndicesGetDataStream::new(self)
  }

  ///Provides statistics on operations happening in a data stream.
  ///
  ///Sends a `GET` request to `/_data_stream/_stats`
  ///
  ///```ignore
  /// let response = client.indices_data_streams_stats()
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_data_streams_stats(&self) -> builder::IndicesDataStreamsStats {
    builder::IndicesDataStreamsStats::new(self)
  }

  ///Returns data streams.
  ///
  ///Sends a `GET` request to `/_data_stream/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of data streams; use `_all` or empty string
  ///   to perform the operation on all data streams.
  ///```ignore
  /// let response = client.indices_get_data_stream_with_name()
  ///    .name(name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_data_stream_with_name(&self) -> builder::IndicesGetDataStreamWithName {
    builder::IndicesGetDataStreamWithName::new(self)
  }

  ///Creates or updates a data stream.
  ///
  ///Sends a `PUT` request to `/_data_stream/{name}`
  ///
  ///Arguments:
  /// - `name`: The name of the data stream.
  /// - `body`
  ///```ignore
  /// let response = client.indices_create_data_stream()
  ///    .name(name)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_create_data_stream(&self) -> builder::IndicesCreateDataStream {
    builder::IndicesCreateDataStream::new(self)
  }

  ///Deletes a data stream.
  ///
  ///Sends a `DELETE` request to `/_data_stream/{name}`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of data streams; use `_all` or empty string
  ///   to perform the operation on all data streams.
  ///```ignore
  /// let response = client.indices_delete_data_stream()
  ///    .name(name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_delete_data_stream(&self) -> builder::IndicesDeleteDataStream {
    builder::IndicesDeleteDataStream::new(self)
  }

  ///Provides statistics on operations happening in a data stream.
  ///
  ///Sends a `GET` request to `/_data_stream/{name}/_stats`
  ///
  ///Arguments:
  /// - `name`: Comma-separated list of data streams; use `_all` or empty string
  ///   to perform the operation on all data streams.
  ///```ignore
  /// let response = client.indices_data_streams_stats_with_name()
  ///    .name(name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_data_streams_stats_with_name(&self) -> builder::IndicesDataStreamsStatsWithName {
    builder::IndicesDataStreamsStatsWithName::new(self)
  }

  ///Changes the number of requests per second for a particular Delete By
  /// Query operation.
  ///
  ///Sends a `POST` request to `/_delete_by_query/{task_id}/_rethrottle`
  ///
  ///Arguments:
  /// - `task_id`: The task id to rethrottle.
  /// - `requests_per_second`: The throttle for this request in sub-requests per
  ///   second. -1 means no throttle.
  ///```ignore
  /// let response = client.delete_by_query_rethrottle()
  ///    .task_id(task_id)
  ///    .requests_per_second(requests_per_second)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_by_query_rethrottle(&self) -> builder::DeleteByQueryRethrottle {
    builder::DeleteByQueryRethrottle::new(self)
  }

  ///Returns the information about the capabilities of fields among multiple
  /// indices.
  ///
  ///Sends a `GET` request to `/_field_caps`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fields`: Comma-separated list of field names.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `include_unmapped`: Indicates whether unmapped fields should be included
  ///   in the response.
  ///```ignore
  /// let response = client.field_caps_get()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fields(fields)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_unmapped(include_unmapped)
  ///    .send()
  ///    .await;
  /// ```
  pub fn field_caps_get(&self) -> builder::FieldCapsGet {
    builder::FieldCapsGet::new(self)
  }

  ///Returns the information about the capabilities of fields among multiple
  /// indices.
  ///
  ///Sends a `POST` request to `/_field_caps`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fields`: Comma-separated list of field names.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `include_unmapped`: Indicates whether unmapped fields should be included
  ///   in the response.
  /// - `body`
  ///```ignore
  /// let response = client.field_caps_post()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fields(fields)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_unmapped(include_unmapped)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn field_caps_post(&self) -> builder::FieldCapsPost {
    builder::FieldCapsPost::new(self)
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
  /// let response = client.indices_flush_get()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .force(force)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .wait_if_ongoing(wait_if_ongoing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_flush_get(&self) -> builder::IndicesFlushGet {
    builder::IndicesFlushGet::new(self)
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
  /// let response = client.indices_flush_post()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .force(force)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .wait_if_ongoing(wait_if_ongoing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_flush_post(&self) -> builder::IndicesFlushPost {
    builder::IndicesFlushPost::new(self)
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
  /// let response = client.indices_forcemerge()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .flush(flush)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .max_num_segments(max_num_segments)
  ///    .only_expunge_deletes(only_expunge_deletes)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_forcemerge(&self) -> builder::IndicesForcemerge {
    builder::IndicesForcemerge::new(self)
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
  /// let response = client.indices_get_index_template()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_index_template(&self) -> builder::IndicesGetIndexTemplate {
    builder::IndicesGetIndexTemplate::new(self)
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
  /// let response = client.indices_simulate_template()
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_simulate_template(&self) -> builder::IndicesSimulateTemplate {
    builder::IndicesSimulateTemplate::new(self)
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
  /// let response = client.indices_simulate_template_with_name()
  ///    .name(name)
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_simulate_template_with_name(&self) -> builder::IndicesSimulateTemplateWithName {
    builder::IndicesSimulateTemplateWithName::new(self)
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
  /// let response = client.indices_simulate_index_template()
  ///    .name(name)
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_simulate_index_template(&self) -> builder::IndicesSimulateIndexTemplate {
    builder::IndicesSimulateIndexTemplate::new(self)
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
  /// let response = client.indices_get_index_template_with_name()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_index_template_with_name(&self) -> builder::IndicesGetIndexTemplateWithName {
    builder::IndicesGetIndexTemplateWithName::new(self)
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
  /// let response = client.indices_put_index_template_put()
  ///    .name(name)
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_put_index_template_put(&self) -> builder::IndicesPutIndexTemplatePut {
    builder::IndicesPutIndexTemplatePut::new(self)
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
  /// let response = client.indices_put_index_template_post()
  ///    .name(name)
  ///    .cause(cause)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_put_index_template_post(&self) -> builder::IndicesPutIndexTemplatePost {
    builder::IndicesPutIndexTemplatePost::new(self)
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
  /// let response = client.indices_delete_index_template()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_delete_index_template(&self) -> builder::IndicesDeleteIndexTemplate {
    builder::IndicesDeleteIndexTemplate::new(self)
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
  /// let response = client.indices_exists_index_template()
  ///    .name(name)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_exists_index_template(&self) -> builder::IndicesExistsIndexTemplate {
    builder::IndicesExistsIndexTemplate::new(self)
  }

  ///Returns a pipeline.
  ///
  ///Sends a `GET` request to `/_ingest/pipeline`
  ///
  ///Arguments:
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.ingest_get_pipeline()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_get_pipeline(&self) -> builder::IngestGetPipeline {
    builder::IngestGetPipeline::new(self)
  }

  ///Allows to simulate a pipeline with example documents.
  ///
  ///Sends a `GET` request to `/_ingest/pipeline/_simulate`
  ///
  ///Arguments:
  /// - `verbose`: Verbose mode. Display data output for each processor in
  ///   executed pipeline.
  ///```ignore
  /// let response = client.ingest_simulate_get()
  ///    .verbose(verbose)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_simulate_get(&self) -> builder::IngestSimulateGet {
    builder::IngestSimulateGet::new(self)
  }

  ///Allows to simulate a pipeline with example documents.
  ///
  ///Sends a `POST` request to `/_ingest/pipeline/_simulate`
  ///
  ///Arguments:
  /// - `verbose`: Verbose mode. Display data output for each processor in
  ///   executed pipeline.
  /// - `body`
  ///```ignore
  /// let response = client.ingest_simulate_post()
  ///    .verbose(verbose)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_simulate_post(&self) -> builder::IngestSimulatePost {
    builder::IngestSimulatePost::new(self)
  }

  ///Returns a pipeline.
  ///
  ///Sends a `GET` request to `/_ingest/pipeline/{id}`
  ///
  ///Arguments:
  /// - `id`: Comma-separated list of pipeline ids. Wildcards supported.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.ingest_get_pipeline_with_id()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_get_pipeline_with_id(&self) -> builder::IngestGetPipelineWithId {
    builder::IngestGetPipelineWithId::new(self)
  }

  ///Creates or updates a pipeline.
  ///
  ///Sends a `PUT` request to `/_ingest/pipeline/{id}`
  ///
  ///Arguments:
  /// - `id`: Pipeline ID.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.ingest_put_pipeline()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_put_pipeline(&self) -> builder::IngestPutPipeline {
    builder::IngestPutPipeline::new(self)
  }

  ///Deletes a pipeline.
  ///
  ///Sends a `DELETE` request to `/_ingest/pipeline/{id}`
  ///
  ///Arguments:
  /// - `id`: Pipeline ID.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.ingest_delete_pipeline()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_delete_pipeline(&self) -> builder::IngestDeletePipeline {
    builder::IngestDeletePipeline::new(self)
  }

  ///Allows to simulate a pipeline with example documents.
  ///
  ///Sends a `GET` request to `/_ingest/pipeline/{id}/_simulate`
  ///
  ///Arguments:
  /// - `id`: Pipeline ID.
  /// - `verbose`: Verbose mode. Display data output for each processor in
  ///   executed pipeline.
  ///```ignore
  /// let response = client.ingest_simulate_get_with_id()
  ///    .id(id)
  ///    .verbose(verbose)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_simulate_get_with_id(&self) -> builder::IngestSimulateGetWithId {
    builder::IngestSimulateGetWithId::new(self)
  }

  ///Allows to simulate a pipeline with example documents.
  ///
  ///Sends a `POST` request to `/_ingest/pipeline/{id}/_simulate`
  ///
  ///Arguments:
  /// - `id`: Pipeline ID.
  /// - `verbose`: Verbose mode. Display data output for each processor in
  ///   executed pipeline.
  /// - `body`
  ///```ignore
  /// let response = client.ingest_simulate_post_with_id()
  ///    .id(id)
  ///    .verbose(verbose)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_simulate_post_with_id(&self) -> builder::IngestSimulatePostWithId {
    builder::IngestSimulatePostWithId::new(self)
  }

  ///Returns a list of the built-in patterns.
  ///
  ///Sends a `GET` request to `/_ingest/processor/grok`
  ///
  ///```ignore
  /// let response = client.ingest_processor_grok()
  ///    .send()
  ///    .await;
  /// ```
  pub fn ingest_processor_grok(&self) -> builder::IngestProcessorGrok {
    builder::IngestProcessorGrok::new(self)
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
  /// let response = client.indices_get_mapping()
  ///    .allow_no_indices(allow_no_indices)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_mapping(&self) -> builder::IndicesGetMapping {
    builder::IndicesGetMapping::new(self)
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
  /// let response = client.indices_get_field_mapping()
  ///    .fields(fields)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_defaults(include_defaults)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_field_mapping(&self) -> builder::IndicesGetFieldMapping {
    builder::IndicesGetFieldMapping::new(self)
  }

  ///Allows to get multiple documents in one request.
  ///
  ///Sends a `GET` request to `/_mget`
  ///
  ///Arguments:
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specify whether to perform the operation in realtime or
  ///   search mode.
  /// - `refresh`: Refresh the shard containing the document before performing
  ///   the operation.
  /// - `routing`: Routing value.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  ///```ignore
  /// let response = client.mget_get()
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .stored_fields(stored_fields)
  ///    .send()
  ///    .await;
  /// ```
  pub fn mget_get(&self) -> builder::MgetGet {
    builder::MgetGet::new(self)
  }

  ///Allows to get multiple documents in one request.
  ///
  ///Sends a `POST` request to `/_mget`
  ///
  ///Arguments:
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specify whether to perform the operation in realtime or
  ///   search mode.
  /// - `refresh`: Refresh the shard containing the document before performing
  ///   the operation.
  /// - `routing`: Routing value.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  /// - `body`
  ///```ignore
  /// let response = client.mget_post()
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .stored_fields(stored_fields)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn mget_post(&self) -> builder::MgetPost {
    builder::MgetPost::new(self)
  }

  ///Allows to execute several search operations in one request.
  ///
  ///Sends a `GET` request to `/_msearch`
  ///
  ///Arguments:
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `max_concurrent_searches`: Controls the maximum number of concurrent
  ///   searches the multi search api will execute.
  /// - `max_concurrent_shard_requests`: The number of concurrent shard requests
  ///   each sub search executes concurrently per node. This value should be
  ///   used to limit the impact of the search on the cluster in order to limit
  ///   the number of concurrent shard requests.
  /// - `pre_filter_shard_size`: Threshold that enforces a pre-filter round-trip
  ///   to prefilter search shards based on query rewriting if the number of
  ///   shards the search request expands to exceeds the threshold. This filter
  ///   round-trip can limit the number of shards significantly if for instance
  ///   a shard can not match any documents based on its rewrite method ie. if
  ///   date filters are mandatory to match but the shard bounds and the query
  ///   are disjoint.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  ///```ignore
  /// let response = client.msearch_get()
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .max_concurrent_searches(max_concurrent_searches)
  ///    .max_concurrent_shard_requests(max_concurrent_shard_requests)
  ///    .pre_filter_shard_size(pre_filter_shard_size)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .send()
  ///    .await;
  /// ```
  pub fn msearch_get(&self) -> builder::MsearchGet {
    builder::MsearchGet::new(self)
  }

  ///Allows to execute several search operations in one request.
  ///
  ///Sends a `POST` request to `/_msearch`
  ///
  ///Arguments:
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `max_concurrent_searches`: Controls the maximum number of concurrent
  ///   searches the multi search api will execute.
  /// - `max_concurrent_shard_requests`: The number of concurrent shard requests
  ///   each sub search executes concurrently per node. This value should be
  ///   used to limit the impact of the search on the cluster in order to limit
  ///   the number of concurrent shard requests.
  /// - `pre_filter_shard_size`: Threshold that enforces a pre-filter round-trip
  ///   to prefilter search shards based on query rewriting if the number of
  ///   shards the search request expands to exceeds the threshold. This filter
  ///   round-trip can limit the number of shards significantly if for instance
  ///   a shard can not match any documents based on its rewrite method ie. if
  ///   date filters are mandatory to match but the shard bounds and the query
  ///   are disjoint.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `body`
  ///```ignore
  /// let response = client.msearch_post()
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .max_concurrent_searches(max_concurrent_searches)
  ///    .max_concurrent_shard_requests(max_concurrent_shard_requests)
  ///    .pre_filter_shard_size(pre_filter_shard_size)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn msearch_post(&self) -> builder::MsearchPost {
    builder::MsearchPost::new(self)
  }

  ///Allows to execute several search template operations in one request.
  ///
  ///Sends a `GET` request to `/_msearch/template`
  ///
  ///Arguments:
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `max_concurrent_searches`: Controls the maximum number of concurrent
  ///   searches the multi search api will execute.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  ///```ignore
  /// let response = client.msearch_template_get()
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .max_concurrent_searches(max_concurrent_searches)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .send()
  ///    .await;
  /// ```
  pub fn msearch_template_get(&self) -> builder::MsearchTemplateGet {
    builder::MsearchTemplateGet::new(self)
  }

  ///Allows to execute several search template operations in one request.
  ///
  ///Sends a `POST` request to `/_msearch/template`
  ///
  ///Arguments:
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `max_concurrent_searches`: Controls the maximum number of concurrent
  ///   searches the multi search api will execute.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `body`
  ///```ignore
  /// let response = client.msearch_template_post()
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .max_concurrent_searches(max_concurrent_searches)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn msearch_template_post(&self) -> builder::MsearchTemplatePost {
    builder::MsearchTemplatePost::new(self)
  }

  ///Returns multiple termvectors in one request.
  ///
  ///Sends a `GET` request to `/_mtermvectors`
  ///
  ///Arguments:
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  ///   Applies to all returned documents unless otherwise specified in body
  ///   'params' or 'docs'.
  /// - `fields`: Comma-separated list of fields to return. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `ids`: Comma-separated list of documents ids. You must define ids as
  ///   parameter or set 'ids' or 'docs' in the request body.
  /// - `offsets`: Specifies if term offsets should be returned. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `payloads`: Specifies if term payloads should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `positions`: Specifies if term positions should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on. Applies to all returned documents unless otherwise
  ///   specified in body 'params' or 'docs'.
  /// - `realtime`: Specifies if requests are real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.mtermvectors_get()
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .ids(ids)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn mtermvectors_get(&self) -> builder::MtermvectorsGet {
    builder::MtermvectorsGet::new(self)
  }

  ///Returns multiple termvectors in one request.
  ///
  ///Sends a `POST` request to `/_mtermvectors`
  ///
  ///Arguments:
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  ///   Applies to all returned documents unless otherwise specified in body
  ///   'params' or 'docs'.
  /// - `fields`: Comma-separated list of fields to return. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `ids`: Comma-separated list of documents ids. You must define ids as
  ///   parameter or set 'ids' or 'docs' in the request body.
  /// - `offsets`: Specifies if term offsets should be returned. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `payloads`: Specifies if term payloads should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `positions`: Specifies if term positions should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on. Applies to all returned documents unless otherwise
  ///   specified in body 'params' or 'docs'.
  /// - `realtime`: Specifies if requests are real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `body`
  ///```ignore
  /// let response = client.mtermvectors_post()
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .ids(ids)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn mtermvectors_post(&self) -> builder::MtermvectorsPost {
    builder::MtermvectorsPost::new(self)
  }

  ///Returns information about nodes in the cluster.
  ///
  ///Sends a `GET` request to `/_nodes`
  ///
  ///Arguments:
  /// - `flat_settings`: Return settings in flat format.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes_info()
  ///    .flat_settings(flat_settings)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_info(&self) -> builder::NodesInfo {
    builder::NodesInfo::new(self)
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
  /// let response = client.nodes_hot_threads()
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_hot_threads(&self) -> builder::NodesHotThreads {
    builder::NodesHotThreads::new(self)
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
  /// let response = client.nodes_hot_threads_deprecated()
  ///    .ignore_idle_threads(ignore_idle_threads)
  ///    .interval(interval)
  ///    .snapshots(snapshots)
  ///    .threads(threads)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_hot_threads_deprecated(&self) -> builder::NodesHotThreadsDeprecated {
    builder::NodesHotThreadsDeprecated::new(self)
  }

  ///Reloads secure settings.
  ///
  ///Sends a `POST` request to `/_nodes/reload_secure_settings`
  ///
  ///Arguments:
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.nodes_reload_secure_settings()
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_reload_secure_settings(&self) -> builder::NodesReloadSecureSettings {
    builder::NodesReloadSecureSettings::new(self)
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
  /// let response = client.nodes_stats()
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
  pub fn nodes_stats(&self) -> builder::NodesStats {
    builder::NodesStats::new(self)
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
  /// let response = client.nodes_stats_with_metric()
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
  pub fn nodes_stats_with_metric(&self) -> builder::NodesStatsWithMetric {
    builder::NodesStatsWithMetric::new(self)
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
  /// let response = client.nodes_stats_with_index_metric_metric()
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
  pub fn nodes_stats_with_index_metric_metric(&self) -> builder::NodesStatsWithIndexMetricMetric {
    builder::NodesStatsWithIndexMetricMetric::new(self)
  }

  ///Returns low-level information about REST actions usage on nodes.
  ///
  ///Sends a `GET` request to `/_nodes/usage`
  ///
  ///Arguments:
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes_usage()
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_usage(&self) -> builder::NodesUsage {
    builder::NodesUsage::new(self)
  }

  ///Returns low-level information about REST actions usage on nodes.
  ///
  ///Sends a `GET` request to `/_nodes/usage/{metric}`
  ///
  ///Arguments:
  /// - `metric`: Limit the information returned to the specified metrics.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.nodes_usage_with_metric()
  ///    .metric(metric)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_usage_with_metric(&self) -> builder::NodesUsageWithMetric {
    builder::NodesUsageWithMetric::new(self)
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
  /// let response = client.nodes_info_with_node_id()
  ///    .node_id(node_id)
  ///    .flat_settings(flat_settings)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_info_with_node_id(&self) -> builder::NodesInfoWithNodeId {
    builder::NodesInfoWithNodeId::new(self)
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
  /// let response = client.nodes_hot_threads_with_node_id()
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
  pub fn nodes_hot_threads_with_node_id(&self) -> builder::NodesHotThreadsWithNodeId {
    builder::NodesHotThreadsWithNodeId::new(self)
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
  /// let response = client.nodes_hot_threads_with_node_id_deprecated()
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
  pub fn nodes_hot_threads_with_node_id_deprecated(&self) -> builder::NodesHotThreadsWithNodeIdDeprecated {
    builder::NodesHotThreadsWithNodeIdDeprecated::new(self)
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
  /// let response = client.nodes_reload_secure_settings_with_node_id()
  ///    .node_id(node_id)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_reload_secure_settings_with_node_id(&self) -> builder::NodesReloadSecureSettingsWithNodeId {
    builder::NodesReloadSecureSettingsWithNodeId::new(self)
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
  /// let response = client.nodes_stats_with_node_id()
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
  pub fn nodes_stats_with_node_id(&self) -> builder::NodesStatsWithNodeId {
    builder::NodesStatsWithNodeId::new(self)
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
  /// let response = client.nodes_stats_with_metric_node_id()
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
  pub fn nodes_stats_with_metric_node_id(&self) -> builder::NodesStatsWithMetricNodeId {
    builder::NodesStatsWithMetricNodeId::new(self)
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
  /// let response = client.nodes_stats_with_index_metric_metric_node_id()
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
  pub fn nodes_stats_with_index_metric_metric_node_id(&self) -> builder::NodesStatsWithIndexMetricMetricNodeId {
    builder::NodesStatsWithIndexMetricMetricNodeId::new(self)
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
  /// let response = client.nodes_usage_with_node_id()
  ///    .node_id(node_id)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_usage_with_node_id(&self) -> builder::NodesUsageWithNodeId {
    builder::NodesUsageWithNodeId::new(self)
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
  /// let response = client.nodes_usage_with_metric_node_id()
  ///    .node_id(node_id)
  ///    .metric(metric)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_usage_with_metric_node_id(&self) -> builder::NodesUsageWithMetricNodeId {
    builder::NodesUsageWithMetricNodeId::new(self)
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
  /// let response = client.nodes_info_with_metric_node_id()
  ///    .node_id(node_id)
  ///    .metric(metric)
  ///    .flat_settings(flat_settings)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn nodes_info_with_metric_node_id(&self) -> builder::NodesInfoWithMetricNodeId {
    builder::NodesInfoWithMetricNodeId::new(self)
  }

  ///Returns account details for the current user.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/account`
  ///
  ///```ignore
  /// let response = client.get_account_details()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_account_details(&self) -> builder::GetAccountDetails {
    builder::GetAccountDetails::new(self)
  }

  ///Changes the password for the current user.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/account`
  ///
  ///```ignore
  /// let response = client.change_password()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn change_password(&self) -> builder::ChangePassword {
    builder::ChangePassword::new(self)
  }

  ///Creates, updates, or deletes multiple action groups in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/actiongroups`
  ///
  ///```ignore
  /// let response = client.patch_action_groups()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_action_groups(&self) -> builder::PatchActionGroups {
    builder::PatchActionGroups::new(self)
  }

  ///Retrieves all action groups.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/actiongroups/`
  ///
  ///```ignore
  /// let response = client.get_action_groups()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_action_groups(&self) -> builder::GetActionGroups {
    builder::GetActionGroups::new(self)
  }

  ///Retrieves one action group.
  ///
  ///Sends a `GET` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  ///
  ///Arguments:
  /// - `action_group`: Action group to retrieve.
  ///```ignore
  /// let response = client.get_action_group()
  ///    .action_group(action_group)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_action_group(&self) -> builder::GetActionGroup {
    builder::GetActionGroup::new(self)
  }

  ///Creates or replaces the specified action group.
  ///
  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  ///
  ///Arguments:
  /// - `action_group`: The name of the action group to create or replace
  /// - `body`
  ///```ignore
  /// let response = client.create_action_group()
  ///    .action_group(action_group)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_action_group(&self) -> builder::CreateActionGroup {
    builder::CreateActionGroup::new(self)
  }

  ///Delete a specified action group.
  ///
  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  ///
  ///Arguments:
  /// - `action_group`: Action group to delete.
  ///```ignore
  /// let response = client.delete_action_group()
  ///    .action_group(action_group)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_action_group(&self) -> builder::DeleteActionGroup {
    builder::DeleteActionGroup::new(self)
  }

  ///Updates individual attributes of an action group.
  ///
  ///Sends a `PATCH` request to
  /// `/_plugins/_security/api/actiongroups/{action_group}`
  ///
  ///```ignore
  /// let response = client.patch_action_group()
  ///    .action_group(action_group)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_action_group(&self) -> builder::PatchActionGroup {
    builder::PatchActionGroup::new(self)
  }

  ///Retrieves the audit configuration.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/audit`
  ///
  ///```ignore
  /// let response = client.get_audit_configuration()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_audit_configuration(&self) -> builder::GetAuditConfiguration {
    builder::GetAuditConfiguration::new(self)
  }

  ///A PATCH call is used to update specified fields in the audit
  /// configuration.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/audit`
  ///
  ///```ignore
  /// let response = client.patch_audit_configuration()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_audit_configuration(&self) -> builder::PatchAuditConfiguration {
    builder::PatchAuditConfiguration::new(self)
  }

  ///Updates the audit configuration.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/audit/config`
  ///
  ///```ignore
  /// let response = client.update_audit_configuration()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update_audit_configuration(&self) -> builder::UpdateAuditConfiguration {
    builder::UpdateAuditConfiguration::new(self)
  }

  ///Flushes the Security plugin user, authentication, and authorization
  /// cache.
  ///
  ///Sends a `DELETE` request to `/_plugins/_security/api/cache`
  ///
  ///```ignore
  /// let response = client.flush_cache()
  ///    .send()
  ///    .await;
  /// ```
  pub fn flush_cache(&self) -> builder::FlushCache {
    builder::FlushCache::new(self)
  }

  ///Retrieve all internal users.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/internalusers`
  ///
  ///```ignore
  /// let response = client.get_users()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_users(&self) -> builder::GetUsers {
    builder::GetUsers::new(self)
  }

  ///Creates, updates, or deletes multiple internal users in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/internalusers`
  ///
  ///```ignore
  /// let response = client.patch_users()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_users(&self) -> builder::PatchUsers {
    builder::PatchUsers::new(self)
  }

  ///Retrieve one internal user.
  ///
  ///Sends a `GET` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  ///
  ///```ignore
  /// let response = client.get_user()
  ///    .username(username)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_user(&self) -> builder::GetUser {
    builder::GetUser::new(self)
  }

  ///Creates or replaces the specified user.
  ///
  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  ///
  ///```ignore
  /// let response = client.create_user()
  ///    .username(username)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_user(&self) -> builder::CreateUser {
    builder::CreateUser::new(self)
  }

  ///Delete the specified user.
  ///
  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  ///
  ///```ignore
  /// let response = client.delete_user()
  ///    .username(username)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_user(&self) -> builder::DeleteUser {
    builder::DeleteUser::new(self)
  }

  ///Updates individual attributes of an internal user.
  ///
  ///Sends a `PATCH` request to
  /// `/_plugins/_security/api/internalusers/{username}`
  ///
  ///```ignore
  /// let response = client.patch_user()
  ///    .username(username)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_user(&self) -> builder::PatchUser {
    builder::PatchUser::new(self)
  }

  ///Retrieves all distinguished names in the allow list.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/nodesdn`
  ///
  ///```ignore
  /// let response = client.get_distinguished_names()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_distinguished_names(&self) -> builder::GetDistinguishedNames {
    builder::GetDistinguishedNames::new(self)
  }

  ///Bulk update of distinguished names.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/nodesdn`
  ///
  ///```ignore
  /// let response = client.patch_distinguished_names()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_distinguished_names(&self) -> builder::PatchDistinguishedNames {
    builder::PatchDistinguishedNames::new(self)
  }

  ///Retrieve distinguished names of a specified cluster.
  ///
  ///Sends a `GET` request to
  /// `/_plugins/_security/api/nodesdn/{cluster_name}`
  ///
  ///```ignore
  /// let response = client.get_distinguished_names_with_cluster_name()
  ///    .cluster_name(cluster_name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_distinguished_names_with_cluster_name(&self) -> builder::GetDistinguishedNamesWithClusterName {
    builder::GetDistinguishedNamesWithClusterName::new(self)
  }

  ///Adds or updates the specified distinguished names in the cluster’s or
  /// node’s allow list.
  ///
  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/nodesdn/{cluster_name}`
  ///
  ///```ignore
  /// let response = client.update_distinguished_names()
  ///    .cluster_name(cluster_name)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update_distinguished_names(&self) -> builder::UpdateDistinguishedNames {
    builder::UpdateDistinguishedNames::new(self)
  }

  ///Deletes all distinguished names in the specified cluster’s or node’s
  /// allow list.
  ///
  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/nodesdn/{cluster_name}`
  ///
  ///```ignore
  /// let response = client.delete_distinguished_names()
  ///    .cluster_name(cluster_name)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_distinguished_names(&self) -> builder::DeleteDistinguishedNames {
    builder::DeleteDistinguishedNames::new(self)
  }

  ///Creates, updates, or deletes multiple roles in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/roles`
  ///
  ///```ignore
  /// let response = client.patch_roles()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_roles(&self) -> builder::PatchRoles {
    builder::PatchRoles::new(self)
  }

  ///Retrieves all roles.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/roles/`
  ///
  ///```ignore
  /// let response = client.get_roles()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_roles(&self) -> builder::GetRoles {
    builder::GetRoles::new(self)
  }

  ///Retrieves one role.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/roles/{role}`
  ///
  ///```ignore
  /// let response = client.get_role()
  ///    .role(role)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_role(&self) -> builder::GetRole {
    builder::GetRole::new(self)
  }

  ///Creates or replaces the specified role.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/roles/{role}`
  ///
  ///```ignore
  /// let response = client.create_role()
  ///    .role(role)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_role(&self) -> builder::CreateRole {
    builder::CreateRole::new(self)
  }

  ///Delete the specified role.
  ///
  ///Sends a `DELETE` request to `/_plugins/_security/api/roles/{role}`
  ///
  ///```ignore
  /// let response = client.delete_role()
  ///    .role(role)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_role(&self) -> builder::DeleteRole {
    builder::DeleteRole::new(self)
  }

  ///Updates individual attributes of a role.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/roles/{role}`
  ///
  ///```ignore
  /// let response = client.patch_role()
  ///    .role(role)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_role(&self) -> builder::PatchRole {
    builder::PatchRole::new(self)
  }

  ///Retrieves all role mappings.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/rolesmapping`
  ///
  ///```ignore
  /// let response = client.get_role_mappings()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_role_mappings(&self) -> builder::GetRoleMappings {
    builder::GetRoleMappings::new(self)
  }

  ///Creates or updates multiple role mappings in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/rolesmapping`
  ///
  ///```ignore
  /// let response = client.patch_role_mappings()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_role_mappings(&self) -> builder::PatchRoleMappings {
    builder::PatchRoleMappings::new(self)
  }

  ///Retrieves one role mapping.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/rolesmapping/{role}`
  ///
  ///```ignore
  /// let response = client.get_role_mapping()
  ///    .role(role)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_role_mapping(&self) -> builder::GetRoleMapping {
    builder::GetRoleMapping::new(self)
  }

  ///Creates or replaces the specified role mapping.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/rolesmapping/{role}`
  ///
  ///```ignore
  /// let response = client.create_role_mapping()
  ///    .role(role)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_role_mapping(&self) -> builder::CreateRoleMapping {
    builder::CreateRoleMapping::new(self)
  }

  ///Deletes the specified role mapping.
  ///
  ///Sends a `DELETE` request to
  /// `/_plugins/_security/api/rolesmapping/{role}`
  ///
  ///```ignore
  /// let response = client.delete_role_mapping()
  ///    .role(role)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_role_mapping(&self) -> builder::DeleteRoleMapping {
    builder::DeleteRoleMapping::new(self)
  }

  ///Updates individual attributes of a role mapping.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/rolesmapping/{role}`
  ///
  ///```ignore
  /// let response = client.patch_role_mapping()
  ///    .role(role)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_role_mapping(&self) -> builder::PatchRoleMapping {
    builder::PatchRoleMapping::new(self)
  }

  ///Returns the current Security plugin configuration in JSON format.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/securityconfig`
  ///
  ///```ignore
  /// let response = client.get_configuration()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_configuration(&self) -> builder::GetConfiguration {
    builder::GetConfiguration::new(self)
  }

  ///A PATCH call is used to update the existing configuration using the REST
  /// API.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/securityconfig`
  ///
  ///```ignore
  /// let response = client.patch_configuration()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_configuration(&self) -> builder::PatchConfiguration {
    builder::PatchConfiguration::new(self)
  }

  ///Adds or updates the existing configuration using the REST API.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/securityconfig/config`
  ///
  ///```ignore
  /// let response = client.update_configuration()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update_configuration(&self) -> builder::UpdateConfiguration {
    builder::UpdateConfiguration::new(self)
  }

  ///Retrieves the cluster’s security certificates.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/ssl/certs`
  ///
  ///```ignore
  /// let response = client.get_certificates()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_certificates(&self) -> builder::GetCertificates {
    builder::GetCertificates::new(self)
  }

  ///Reload HTTP layer communication certificates.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/ssl/http/reloadcerts`
  ///
  ///```ignore
  /// let response = client.reload_http_certificates()
  ///    .send()
  ///    .await;
  /// ```
  pub fn reload_http_certificates(&self) -> builder::ReloadHttpCertificates {
    builder::ReloadHttpCertificates::new(self)
  }

  ///Reload transport layer communication certificates.
  ///
  ///Sends a `PUT` request to
  /// `/_plugins/_security/api/ssl/transport/reloadcerts`
  ///
  ///```ignore
  /// let response = client.reload_transport_certificates()
  ///    .send()
  ///    .await;
  /// ```
  pub fn reload_transport_certificates(&self) -> builder::ReloadTransportCertificates {
    builder::ReloadTransportCertificates::new(self)
  }

  ///Retrieves all tenants.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/tenants/`
  ///
  ///```ignore
  /// let response = client.get_tenants()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_tenants(&self) -> builder::GetTenants {
    builder::GetTenants::new(self)
  }

  ///Add, delete, or modify multiple tenants in a single call.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/tenants/`
  ///
  ///```ignore
  /// let response = client.patch_tenants()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_tenants(&self) -> builder::PatchTenants {
    builder::PatchTenants::new(self)
  }

  ///Retrieves one tenant.
  ///
  ///Sends a `GET` request to `/_plugins/_security/api/tenants/{tenant}`
  ///
  ///```ignore
  /// let response = client.get_tenant()
  ///    .tenant(tenant)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_tenant(&self) -> builder::GetTenant {
    builder::GetTenant::new(self)
  }

  ///Creates or replaces the specified tenant.
  ///
  ///Sends a `PUT` request to `/_plugins/_security/api/tenants/{tenant}`
  ///
  ///```ignore
  /// let response = client.create_tenant()
  ///    .tenant(tenant)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_tenant(&self) -> builder::CreateTenant {
    builder::CreateTenant::new(self)
  }

  ///Delete the specified tenant.
  ///
  ///Sends a `DELETE` request to `/_plugins/_security/api/tenants/{tenant}`
  ///
  ///```ignore
  /// let response = client.delete_tenant()
  ///    .tenant(tenant)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_tenant(&self) -> builder::DeleteTenant {
    builder::DeleteTenant::new(self)
  }

  ///Add, delete, or modify a single tenant.
  ///
  ///Sends a `PATCH` request to `/_plugins/_security/api/tenants/{tenant}`
  ///
  ///```ignore
  /// let response = client.patch_tenant()
  ///    .tenant(tenant)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn patch_tenant(&self) -> builder::PatchTenant {
    builder::PatchTenant::new(self)
  }

  ///Checks to see if the Security plugin is up and running.
  ///
  ///Sends a `GET` request to `/_plugins/_security/health`
  ///
  ///```ignore
  /// let response = client.security_health()
  ///    .send()
  ///    .await;
  /// ```
  pub fn security_health(&self) -> builder::SecurityHealth {
    builder::SecurityHealth::new(self)
  }

  ///Allows to evaluate the quality of ranked search results over a set of
  /// typical search queries.
  ///
  ///Sends a `GET` request to `/_rank_eval`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `search_type`: Search operation type.
  ///```ignore
  /// let response = client.rank_eval_get()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .search_type(search_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn rank_eval_get(&self) -> builder::RankEvalGet {
    builder::RankEvalGet::new(self)
  }

  ///Allows to evaluate the quality of ranked search results over a set of
  /// typical search queries.
  ///
  ///Sends a `POST` request to `/_rank_eval`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `search_type`: Search operation type.
  /// - `body`
  ///```ignore
  /// let response = client.rank_eval_post()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .search_type(search_type)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn rank_eval_post(&self) -> builder::RankEvalPost {
    builder::RankEvalPost::new(self)
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
  /// let response = client.indices_recovery()
  ///    .active_only(active_only)
  ///    .detailed(detailed)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_recovery(&self) -> builder::IndicesRecovery {
    builder::IndicesRecovery::new(self)
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
  /// let response = client.indices_refresh_get()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_refresh_get(&self) -> builder::IndicesRefreshGet {
    builder::IndicesRefreshGet::new(self)
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
  /// let response = client.indices_refresh_post()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_refresh_post(&self) -> builder::IndicesRefreshPost {
    builder::IndicesRefreshPost::new(self)
  }

  ///Allows to copy documents from one index to another, optionally filtering
  /// the source documents by a query, changing the destination index
  /// settings, or fetching the documents from a remote cluster.
  ///
  ///Sends a `POST` request to `/_reindex`
  ///
  ///Arguments:
  /// - `max_docs`: Maximum number of documents to process (default: all
  ///   documents).
  /// - `refresh`: Should the affected indexes be refreshed?.
  /// - `requests_per_second`: The throttle for this request in sub-requests per
  ///   second. -1 means no throttle.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `slices`: The number of slices this task should be divided into.
  ///   Defaults to 1, meaning the task isn't sliced into subtasks. Can be set
  ///   to `auto`.
  /// - `timeout`: Time each individual bulk request should wait for shards that
  ///   are unavailable.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  /// - `body`
  ///```ignore
  /// let response = client.reindex()
  ///    .max_docs(max_docs)
  ///    .refresh(refresh)
  ///    .requests_per_second(requests_per_second)
  ///    .scroll(scroll)
  ///    .slices(slices)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .wait_for_completion(wait_for_completion)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn reindex(&self) -> builder::Reindex {
    builder::Reindex::new(self)
  }

  ///Changes the number of requests per second for a particular Reindex
  /// operation.
  ///
  ///Sends a `POST` request to `/_reindex/{task_id}/_rethrottle`
  ///
  ///Arguments:
  /// - `task_id`: The task id to rethrottle.
  /// - `requests_per_second`: The throttle for this request in sub-requests per
  ///   second. -1 means no throttle.
  ///```ignore
  /// let response = client.reindex_rethrottle()
  ///    .task_id(task_id)
  ///    .requests_per_second(requests_per_second)
  ///    .send()
  ///    .await;
  /// ```
  pub fn reindex_rethrottle(&self) -> builder::ReindexRethrottle {
    builder::ReindexRethrottle::new(self)
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
    builder::ClusterRemoteInfo::new(self)
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
  /// let response = client.remote_store_restore()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .wait_for_completion(wait_for_completion)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn remote_store_restore(&self) -> builder::RemoteStoreRestore {
    builder::RemoteStoreRestore::new(self)
  }

  ///Allows to use the Mustache language to pre-render a search definition.
  ///
  ///Sends a `GET` request to `/_render/template`
  ///
  ///```ignore
  /// let response = client.render_search_template_get()
  ///    .send()
  ///    .await;
  /// ```
  pub fn render_search_template_get(&self) -> builder::RenderSearchTemplateGet {
    builder::RenderSearchTemplateGet::new(self)
  }

  ///Allows to use the Mustache language to pre-render a search definition.
  ///
  ///Sends a `POST` request to `/_render/template`
  ///
  ///```ignore
  /// let response = client.render_search_template_post()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn render_search_template_post(&self) -> builder::RenderSearchTemplatePost {
    builder::RenderSearchTemplatePost::new(self)
  }

  ///Allows to use the Mustache language to pre-render a search definition.
  ///
  ///Sends a `GET` request to `/_render/template/{id}`
  ///
  ///Arguments:
  /// - `id`: The id of the stored search template.
  ///```ignore
  /// let response = client.render_search_template_get_with_id()
  ///    .id(id)
  ///    .send()
  ///    .await;
  /// ```
  pub fn render_search_template_get_with_id(&self) -> builder::RenderSearchTemplateGetWithId {
    builder::RenderSearchTemplateGetWithId::new(self)
  }

  ///Allows to use the Mustache language to pre-render a search definition.
  ///
  ///Sends a `POST` request to `/_render/template/{id}`
  ///
  ///Arguments:
  /// - `id`: The id of the stored search template.
  /// - `body`
  ///```ignore
  /// let response = client.render_search_template_post_with_id()
  ///    .id(id)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn render_search_template_post_with_id(&self) -> builder::RenderSearchTemplatePostWithId {
    builder::RenderSearchTemplatePostWithId::new(self)
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
  /// let response = client.indices_resolve_index()
  ///    .name(name)
  ///    .expand_wildcards(expand_wildcards)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_resolve_index(&self) -> builder::IndicesResolveIndex {
    builder::IndicesResolveIndex::new(self)
  }

  ///Returns all script contexts.
  ///
  ///Sends a `GET` request to `/_script_context`
  ///
  ///```ignore
  /// let response = client.get_script_context()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_script_context(&self) -> builder::GetScriptContext {
    builder::GetScriptContext::new(self)
  }

  ///Returns available script types, languages and contexts.
  ///
  ///Sends a `GET` request to `/_script_language`
  ///
  ///```ignore
  /// let response = client.get_script_languages()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_script_languages(&self) -> builder::GetScriptLanguages {
    builder::GetScriptLanguages::new(self)
  }

  ///Allows an arbitrary script to be executed and a result to be returned.
  ///
  ///Sends a `GET` request to `/_scripts/painless/_execute`
  ///
  ///```ignore
  /// let response = client.scripts_painless_execute_get()
  ///    .send()
  ///    .await;
  /// ```
  pub fn scripts_painless_execute_get(&self) -> builder::ScriptsPainlessExecuteGet {
    builder::ScriptsPainlessExecuteGet::new(self)
  }

  ///Allows an arbitrary script to be executed and a result to be returned.
  ///
  ///Sends a `POST` request to `/_scripts/painless/_execute`
  ///
  ///```ignore
  /// let response = client.scripts_painless_execute_post()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn scripts_painless_execute_post(&self) -> builder::ScriptsPainlessExecutePost {
    builder::ScriptsPainlessExecutePost::new(self)
  }

  ///Returns a script.
  ///
  ///Sends a `GET` request to `/_scripts/{id}`
  ///
  ///Arguments:
  /// - `id`: Script ID.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  ///```ignore
  /// let response = client.get_script()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_script(&self) -> builder::GetScript {
    builder::GetScript::new(self)
  }

  ///Creates or updates a script.
  ///
  ///Sends a `PUT` request to `/_scripts/{id}`
  ///
  ///Arguments:
  /// - `id`: Script ID.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.put_script_put()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_script_put(&self) -> builder::PutScriptPut {
    builder::PutScriptPut::new(self)
  }

  ///Creates or updates a script.
  ///
  ///Sends a `POST` request to `/_scripts/{id}`
  ///
  ///Arguments:
  /// - `id`: Script ID.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.put_script_post()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_script_post(&self) -> builder::PutScriptPost {
    builder::PutScriptPost::new(self)
  }

  ///Deletes a script.
  ///
  ///Sends a `DELETE` request to `/_scripts/{id}`
  ///
  ///Arguments:
  /// - `id`: Script ID.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  ///```ignore
  /// let response = client.delete_script()
  ///    .id(id)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_script(&self) -> builder::DeleteScript {
    builder::DeleteScript::new(self)
  }

  ///Creates or updates a script.
  ///
  ///Sends a `PUT` request to `/_scripts/{id}/{context}`
  ///
  ///Arguments:
  /// - `id`: Script ID.
  /// - `context`: Script context.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.put_script_put_with_context()
  ///    .id(id)
  ///    .context(context)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_script_put_with_context(&self) -> builder::PutScriptPutWithContext {
    builder::PutScriptPutWithContext::new(self)
  }

  ///Creates or updates a script.
  ///
  ///Sends a `POST` request to `/_scripts/{id}/{context}`
  ///
  ///Arguments:
  /// - `id`: Script ID.
  /// - `context`: Script context.
  /// - `cluster_manager_timeout`: Operation timeout for connection to
  ///   cluster-manager node.
  /// - `master_timeout`: Operation timeout for connection to master node.
  /// - `timeout`: Operation timeout.
  /// - `body`
  ///```ignore
  /// let response = client.put_script_post_with_context()
  ///    .id(id)
  ///    .context(context)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn put_script_post_with_context(&self) -> builder::PutScriptPostWithContext {
    builder::PutScriptPostWithContext::new(self)
  }

  ///Returns results matching a query.
  ///
  ///Sends a `GET` request to `/_search`
  ///
  ///Arguments:
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `allow_partial_search_results`: Indicate if an error should be returned
  ///   if there is a partial search failure or timeout.
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `batched_reduce_size`: The number of shard results that should be
  ///   reduced at once on the coordinating node. This value should be used as a
  ///   protection mechanism to reduce the memory overhead per search request if
  ///   the potential number of shards in the request can be large.
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `docvalue_fields`: Comma-separated list of fields to return as the
  ///   docvalue representation of a field for each hit.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Specify whether to return detailed information about score
  ///   computation as part of a hit.
  /// - `from`: Starting offset.
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `max_concurrent_shard_requests`: The number of concurrent shard requests
  ///   per node this search executes concurrently. This value should be used to
  ///   limit the impact of the search on the cluster in order to limit the
  ///   number of concurrent shard requests.
  /// - `pre_filter_shard_size`: Threshold that enforces a pre-filter round-trip
  ///   to prefilter search shards based on query rewriting if the number of
  ///   shards the search request expands to exceeds the threshold. This filter
  ///   round-trip can limit the number of shards significantly if for instance
  ///   a shard can not match any documents based on its rewrite method ie. if
  ///   date filters are mandatory to match but the shard bounds and the query
  ///   are disjoint.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `request_cache`: Specify if request cache should be used for this
  ///   request or not, defaults to index level setting.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `search_type`: Search operation type.
  /// - `seq_no_primary_term`: Specify whether to return sequence number and
  ///   primary term of the last modification of each hit.
  /// - `size`: Number of hits to return.
  /// - `sort`: Comma-separated list of <field>:<direction> pairs.
  /// - `stats`: Specific 'tag' of the request for logging and statistical
  ///   purposes.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  /// - `suggest_field`: Specify which field to use for suggestions.
  /// - `suggest_mode`: Specify suggest mode.
  /// - `suggest_size`: How many suggestions to return in response.
  /// - `suggest_text`: The source text for which the suggestions should be
  ///   returned.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  /// - `timeout`: Operation timeout.
  /// - `track_scores`: Whether to calculate and return scores even if they are
  ///   not used for sorting.
  /// - `track_total_hits`: Indicate if the number of documents that match the
  ///   query should be tracked.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `version`: Whether to return document version as part of a hit.
  ///```ignore
  /// let response = client.search_get()
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .allow_no_indices(allow_no_indices)
  ///    .allow_partial_search_results(allow_partial_search_results)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .batched_reduce_size(batched_reduce_size)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .docvalue_fields(docvalue_fields)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .from(from)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .max_concurrent_shard_requests(max_concurrent_shard_requests)
  ///    .pre_filter_shard_size(pre_filter_shard_size)
  ///    .preference(preference)
  ///    .q(q)
  ///    .request_cache(request_cache)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .search_type(search_type)
  ///    .seq_no_primary_term(seq_no_primary_term)
  ///    .size(size)
  ///    .sort(sort)
  ///    .stats(stats)
  ///    .stored_fields(stored_fields)
  ///    .suggest_field(suggest_field)
  ///    .suggest_mode(suggest_mode)
  ///    .suggest_size(suggest_size)
  ///    .suggest_text(suggest_text)
  ///    .terminate_after(terminate_after)
  ///    .timeout(timeout)
  ///    .track_scores(track_scores)
  ///    .track_total_hits(track_total_hits)
  ///    .typed_keys(typed_keys)
  ///    .version(version)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_get(&self) -> builder::SearchGet {
    builder::SearchGet::new(self)
  }

  ///Returns results matching a query.
  ///
  ///Sends a `POST` request to `/_search`
  ///
  ///Arguments:
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `allow_partial_search_results`: Indicate if an error should be returned
  ///   if there is a partial search failure or timeout.
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `batched_reduce_size`: The number of shard results that should be
  ///   reduced at once on the coordinating node. This value should be used as a
  ///   protection mechanism to reduce the memory overhead per search request if
  ///   the potential number of shards in the request can be large.
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `docvalue_fields`: Comma-separated list of fields to return as the
  ///   docvalue representation of a field for each hit.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Specify whether to return detailed information about score
  ///   computation as part of a hit.
  /// - `from`: Starting offset.
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `max_concurrent_shard_requests`: The number of concurrent shard requests
  ///   per node this search executes concurrently. This value should be used to
  ///   limit the impact of the search on the cluster in order to limit the
  ///   number of concurrent shard requests.
  /// - `pre_filter_shard_size`: Threshold that enforces a pre-filter round-trip
  ///   to prefilter search shards based on query rewriting if the number of
  ///   shards the search request expands to exceeds the threshold. This filter
  ///   round-trip can limit the number of shards significantly if for instance
  ///   a shard can not match any documents based on its rewrite method ie. if
  ///   date filters are mandatory to match but the shard bounds and the query
  ///   are disjoint.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `request_cache`: Specify if request cache should be used for this
  ///   request or not, defaults to index level setting.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `search_type`: Search operation type.
  /// - `seq_no_primary_term`: Specify whether to return sequence number and
  ///   primary term of the last modification of each hit.
  /// - `size`: Number of hits to return.
  /// - `sort`: Comma-separated list of <field>:<direction> pairs.
  /// - `stats`: Specific 'tag' of the request for logging and statistical
  ///   purposes.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  /// - `suggest_field`: Specify which field to use for suggestions.
  /// - `suggest_mode`: Specify suggest mode.
  /// - `suggest_size`: How many suggestions to return in response.
  /// - `suggest_text`: The source text for which the suggestions should be
  ///   returned.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  /// - `timeout`: Operation timeout.
  /// - `track_scores`: Whether to calculate and return scores even if they are
  ///   not used for sorting.
  /// - `track_total_hits`: Indicate if the number of documents that match the
  ///   query should be tracked.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `version`: Whether to return document version as part of a hit.
  /// - `body`
  ///```ignore
  /// let response = client.search_post()
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .allow_no_indices(allow_no_indices)
  ///    .allow_partial_search_results(allow_partial_search_results)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .batched_reduce_size(batched_reduce_size)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .docvalue_fields(docvalue_fields)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .from(from)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .max_concurrent_shard_requests(max_concurrent_shard_requests)
  ///    .pre_filter_shard_size(pre_filter_shard_size)
  ///    .preference(preference)
  ///    .q(q)
  ///    .request_cache(request_cache)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .search_type(search_type)
  ///    .seq_no_primary_term(seq_no_primary_term)
  ///    .size(size)
  ///    .sort(sort)
  ///    .stats(stats)
  ///    .stored_fields(stored_fields)
  ///    .suggest_field(suggest_field)
  ///    .suggest_mode(suggest_mode)
  ///    .suggest_size(suggest_size)
  ///    .suggest_text(suggest_text)
  ///    .terminate_after(terminate_after)
  ///    .timeout(timeout)
  ///    .track_scores(track_scores)
  ///    .track_total_hits(track_total_hits)
  ///    .typed_keys(typed_keys)
  ///    .version(version)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_post(&self) -> builder::SearchPost {
    builder::SearchPost::new(self)
  }

  ///Deletes one or more point in time searches based on the IDs passed.
  ///
  ///Sends a `DELETE` request to `/_search/point_in_time`
  ///
  ///```ignore
  /// let response = client.delete_pit()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_pit(&self) -> builder::DeletePit {
    builder::DeletePit::new(self)
  }

  ///Lists all active point in time searches.
  ///
  ///Sends a `GET` request to `/_search/point_in_time/_all`
  ///
  ///```ignore
  /// let response = client.get_all_pits()
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_all_pits(&self) -> builder::GetAllPits {
    builder::GetAllPits::new(self)
  }

  ///Deletes all active point in time searches.
  ///
  ///Sends a `DELETE` request to `/_search/point_in_time/_all`
  ///
  ///```ignore
  /// let response = client.delete_all_pits()
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_all_pits(&self) -> builder::DeleteAllPits {
    builder::DeleteAllPits::new(self)
  }

  ///Allows to retrieve a large numbers of results from a single search
  /// request.
  ///
  ///Sends a `GET` request to `/_search/scroll`
  ///
  ///Arguments:
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `scroll_id`: Scroll ID.
  ///```ignore
  /// let response = client.scroll_get()
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .scroll(scroll)
  ///    .scroll_id(scroll_id)
  ///    .send()
  ///    .await;
  /// ```
  pub fn scroll_get(&self) -> builder::ScrollGet {
    builder::ScrollGet::new(self)
  }

  ///Allows to retrieve a large numbers of results from a single search
  /// request.
  ///
  ///Sends a `POST` request to `/_search/scroll`
  ///
  ///Arguments:
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `scroll_id`: Scroll ID.
  /// - `body`
  ///```ignore
  /// let response = client.scroll_post()
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .scroll(scroll)
  ///    .scroll_id(scroll_id)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn scroll_post(&self) -> builder::ScrollPost {
    builder::ScrollPost::new(self)
  }

  ///Explicitly clears the search context for a scroll.
  ///
  ///Sends a `DELETE` request to `/_search/scroll`
  ///
  ///```ignore
  /// let response = client.clear_scroll()
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn clear_scroll(&self) -> builder::ClearScroll {
    builder::ClearScroll::new(self)
  }

  ///Allows to retrieve a large numbers of results from a single search
  /// request.
  ///
  ///Sends a `GET` request to `/_search/scroll/{scroll_id}`
  ///
  ///Arguments:
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `scroll_id`: Scroll ID.
  ///```ignore
  /// let response = client.scroll_get_with_scroll_id()
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .scroll(scroll)
  ///    .scroll_id(scroll_id)
  ///    .send()
  ///    .await;
  /// ```
  pub fn scroll_get_with_scroll_id(&self) -> builder::ScrollGetWithScrollId {
    builder::ScrollGetWithScrollId::new(self)
  }

  ///Allows to retrieve a large numbers of results from a single search
  /// request.
  ///
  ///Sends a `POST` request to `/_search/scroll/{scroll_id}`
  ///
  ///Arguments:
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `scroll_id`: Scroll ID.
  /// - `body`
  ///```ignore
  /// let response = client.scroll_post_with_scroll_id()
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .scroll(scroll)
  ///    .scroll_id(scroll_id)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn scroll_post_with_scroll_id(&self) -> builder::ScrollPostWithScrollId {
    builder::ScrollPostWithScrollId::new(self)
  }

  ///Explicitly clears the search context for a scroll.
  ///
  ///Sends a `DELETE` request to `/_search/scroll/{scroll_id}`
  ///
  ///Arguments:
  /// - `scroll_id`: Comma-separated list of scroll IDs to clear.
  /// - `body`
  ///```ignore
  /// let response = client.clear_scroll_with_scroll_id()
  ///    .scroll_id(scroll_id)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn clear_scroll_with_scroll_id(&self) -> builder::ClearScrollWithScrollId {
    builder::ClearScrollWithScrollId::new(self)
  }

  ///Allows to use the Mustache language to pre-render a search definition.
  ///
  ///Sends a `GET` request to `/_search/template`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Specify whether to return detailed information about score
  ///   computation as part of a hit.
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `profile`: Specify whether to profile the query execution.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  ///```ignore
  /// let response = client.search_template_get()
  ///    .allow_no_indices(allow_no_indices)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .preference(preference)
  ///    .profile(profile)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_template_get(&self) -> builder::SearchTemplateGet {
    builder::SearchTemplateGet::new(self)
  }

  ///Allows to use the Mustache language to pre-render a search definition.
  ///
  ///Sends a `POST` request to `/_search/template`
  ///
  ///Arguments:
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Specify whether to return detailed information about score
  ///   computation as part of a hit.
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `profile`: Specify whether to profile the query execution.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `body`
  ///```ignore
  /// let response = client.search_template_post()
  ///    .allow_no_indices(allow_no_indices)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .preference(preference)
  ///    .profile(profile)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_template_post(&self) -> builder::SearchTemplatePost {
    builder::SearchTemplatePost::new(self)
  }

  ///Returns information about the indices and shards that a search request
  /// would be executed against.
  ///
  ///Sends a `GET` request to `/_search_shards`
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
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `routing`: Routing value.
  ///```ignore
  /// let response = client.search_shards_get()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .preference(preference)
  ///    .routing(routing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_shards_get(&self) -> builder::SearchShardsGet {
    builder::SearchShardsGet::new(self)
  }

  ///Returns information about the indices and shards that a search request
  /// would be executed against.
  ///
  ///Sends a `POST` request to `/_search_shards`
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
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `routing`: Routing value.
  ///```ignore
  /// let response = client.search_shards_post()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .preference(preference)
  ///    .routing(routing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_shards_post(&self) -> builder::SearchShardsPost {
    builder::SearchShardsPost::new(self)
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
  /// let response = client.indices_segments()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .verbose(verbose)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_segments(&self) -> builder::IndicesSegments {
    builder::IndicesSegments::new(self)
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
  /// let response = client.indices_get_settings()
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
  pub fn indices_get_settings(&self) -> builder::IndicesGetSettings {
    builder::IndicesGetSettings::new(self)
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
  /// let response = client.indices_put_settings()
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
  pub fn indices_put_settings(&self) -> builder::IndicesPutSettings {
    builder::IndicesPutSettings::new(self)
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
  /// let response = client.indices_get_settings_with_name()
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
  pub fn indices_get_settings_with_name(&self) -> builder::IndicesGetSettingsWithName {
    builder::IndicesGetSettingsWithName::new(self)
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
  /// let response = client.indices_shard_stores()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .status(status)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_shard_stores(&self) -> builder::IndicesShardStores {
    builder::IndicesShardStores::new(self)
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
    builder::SnapshotGetRepository::new(self)
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
    builder::SnapshotStatus::new(self)
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
    builder::SnapshotGetRepositoryWithRepository::new(self)
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
    builder::SnapshotCreateRepositoryPut::new(self)
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
    builder::SnapshotCreateRepositoryPost::new(self)
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
    builder::SnapshotDeleteRepository::new(self)
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
    builder::SnapshotCleanupRepository::new(self)
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
    builder::SnapshotStatusWithRepository::new(self)
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
    builder::SnapshotVerifyRepository::new(self)
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
    builder::SnapshotGet::new(self)
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
    builder::SnapshotCreatePut::new(self)
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
    builder::SnapshotCreatePost::new(self)
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
    builder::SnapshotDelete::new(self)
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
    builder::SnapshotClone::new(self)
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
    builder::SnapshotRestore::new(self)
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
    builder::SnapshotStatusWithRepositorySnapshot::new(self)
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
  /// let response = client.indices_stats()
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
  pub fn indices_stats(&self) -> builder::IndicesStats {
    builder::IndicesStats::new(self)
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
  /// let response = client.indices_stats_with_metric()
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
  pub fn indices_stats_with_metric(&self) -> builder::IndicesStatsWithMetric {
    builder::IndicesStatsWithMetric::new(self)
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
    builder::TasksList::new(self)
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
    builder::TasksCancel::new(self)
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
    builder::TasksGet::new(self)
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
    builder::TasksCancelWithTaskId::new(self)
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
  /// let response = client.indices_get_template()
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_template(&self) -> builder::IndicesGetTemplate {
    builder::IndicesGetTemplate::new(self)
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
  /// let response = client.indices_get_template_with_name()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_template_with_name(&self) -> builder::IndicesGetTemplateWithName {
    builder::IndicesGetTemplateWithName::new(self)
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
  /// let response = client.indices_put_template_put()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .order(order)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_put_template_put(&self) -> builder::IndicesPutTemplatePut {
    builder::IndicesPutTemplatePut::new(self)
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
  /// let response = client.indices_put_template_post()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .create(create)
  ///    .master_timeout(master_timeout)
  ///    .order(order)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_put_template_post(&self) -> builder::IndicesPutTemplatePost {
    builder::IndicesPutTemplatePost::new(self)
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
  /// let response = client.indices_delete_template()
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_delete_template(&self) -> builder::IndicesDeleteTemplate {
    builder::IndicesDeleteTemplate::new(self)
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
  /// let response = client.indices_exists_template()
  ///    .name(name)
  ///    .flat_settings(flat_settings)
  ///    .local(local)
  ///    .master_timeout(master_timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_exists_template(&self) -> builder::IndicesExistsTemplate {
    builder::IndicesExistsTemplate::new(self)
  }

  ///Changes the number of requests per second for a particular Update By
  /// Query operation.
  ///
  ///Sends a `POST` request to `/_update_by_query/{task_id}/_rethrottle`
  ///
  ///Arguments:
  /// - `task_id`: The task id to rethrottle.
  /// - `requests_per_second`: The throttle for this request in sub-requests per
  ///   second. -1 means no throttle.
  ///```ignore
  /// let response = client.update_by_query_rethrottle()
  ///    .task_id(task_id)
  ///    .requests_per_second(requests_per_second)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update_by_query_rethrottle(&self) -> builder::UpdateByQueryRethrottle {
    builder::UpdateByQueryRethrottle::new(self)
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
  /// let response = client.indices_get_upgrade()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_upgrade(&self) -> builder::IndicesGetUpgrade {
    builder::IndicesGetUpgrade::new(self)
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
  /// let response = client.indices_upgrade()
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .only_ancient_segments(only_ancient_segments)
  ///    .wait_for_completion(wait_for_completion)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_upgrade(&self) -> builder::IndicesUpgrade {
    builder::IndicesUpgrade::new(self)
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
  /// let response = client.indices_validate_query_get()
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
  pub fn indices_validate_query_get(&self) -> builder::IndicesValidateQueryGet {
    builder::IndicesValidateQueryGet::new(self)
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
  /// let response = client.indices_validate_query_post()
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
  pub fn indices_validate_query_post(&self) -> builder::IndicesValidateQueryPost {
    builder::IndicesValidateQueryPost::new(self)
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
  /// let response = client.indices_rollover()
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
  pub fn indices_rollover(&self) -> builder::IndicesRollover {
    builder::IndicesRollover::new(self)
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
  /// let response = client.indices_rollover_with_new_index()
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
  pub fn indices_rollover_with_new_index(&self) -> builder::IndicesRolloverWithNewIndex {
    builder::IndicesRolloverWithNewIndex::new(self)
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
  /// let response = client.indices_get()
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
  pub fn indices_get(&self) -> builder::IndicesGet {
    builder::IndicesGet::new(self)
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
  /// let response = client.indices_create()
  ///    .index(index)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_create(&self) -> builder::IndicesCreate {
    builder::IndicesCreate::new(self)
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
  /// let response = client.indices_delete()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_delete(&self) -> builder::IndicesDelete {
    builder::IndicesDelete::new(self)
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
  /// let response = client.indices_exists()
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
  pub fn indices_exists(&self) -> builder::IndicesExists {
    builder::IndicesExists::new(self)
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
  /// let response = client.indices_get_alias_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_alias_with_index(&self) -> builder::IndicesGetAliasWithIndex {
    builder::IndicesGetAliasWithIndex::new(self)
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
  /// let response = client.indices_get_alias_with_index_name()
  ///    .index(index)
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_alias_with_index_name(&self) -> builder::IndicesGetAliasWithIndexName {
    builder::IndicesGetAliasWithIndexName::new(self)
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
  /// let response = client.indices_put_alias_put()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_put_alias_put(&self) -> builder::IndicesPutAliasPut {
    builder::IndicesPutAliasPut::new(self)
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
  /// let response = client.indices_put_alias_post()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_put_alias_post(&self) -> builder::IndicesPutAliasPost {
    builder::IndicesPutAliasPost::new(self)
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
  /// let response = client.indices_delete_alias()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_delete_alias(&self) -> builder::IndicesDeleteAlias {
    builder::IndicesDeleteAlias::new(self)
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
  /// let response = client.indices_exists_alias_with_index()
  ///    .index(index)
  ///    .name(name)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_exists_alias_with_index(&self) -> builder::IndicesExistsAliasWithIndex {
    builder::IndicesExistsAliasWithIndex::new(self)
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
  /// let response = client.indices_put_alias_put_plural()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_put_alias_put_plural(&self) -> builder::IndicesPutAliasPutPlural {
    builder::IndicesPutAliasPutPlural::new(self)
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
  /// let response = client.indices_put_alias_post_plural()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_put_alias_post_plural(&self) -> builder::IndicesPutAliasPostPlural {
    builder::IndicesPutAliasPostPlural::new(self)
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
  /// let response = client.indices_delete_alias_plural()
  ///    .index(index)
  ///    .name(name)
  ///    .cluster_manager_timeout(cluster_manager_timeout)
  ///    .master_timeout(master_timeout)
  ///    .timeout(timeout)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_delete_alias_plural(&self) -> builder::IndicesDeleteAliasPlural {
    builder::IndicesDeleteAliasPlural::new(self)
  }

  ///Performs the analysis process on a text and return the tokens breakdown
  /// of the text.
  ///
  ///Sends a `GET` request to `/{index}/_analyze`
  ///
  ///Arguments:
  /// - `index`: The name of the index to scope the operation.
  ///```ignore
  /// let response = client.indices_analyze_get_with_index()
  ///    .index(index)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_analyze_get_with_index(&self) -> builder::IndicesAnalyzeGetWithIndex {
    builder::IndicesAnalyzeGetWithIndex::new(self)
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
  /// let response = client.indices_analyze_post_with_index()
  ///    .index(index)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_analyze_post_with_index(&self) -> builder::IndicesAnalyzePostWithIndex {
    builder::IndicesAnalyzePostWithIndex::new(self)
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
  /// let response = client.indices_add_block()
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
  pub fn indices_add_block(&self) -> builder::IndicesAddBlock {
    builder::IndicesAddBlock::new(self)
  }

  ///Allows to perform multiple index/update/delete operations in a single
  /// request.
  ///
  ///Sends a `PUT` request to `/{index}/_bulk`
  ///
  ///Arguments:
  /// - `index`: Default index for items which don't provide one.
  /// - `source`: True or false to return the _source field or not, or default
  ///   list of fields to return, can be overridden on each sub-request.
  /// - `source_excludes`: Default list of fields to exclude from the returned
  ///   _source field, can be overridden on each sub-request.
  /// - `source_includes`: Default list of fields to extract and return from the
  ///   _source field, can be overridden on each sub-request.
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `require_alias`: Sets require_alias for all incoming documents.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `type_`: Default document type for items which don't provide one.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.bulk_put_with_index()
  ///    .index(index)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .pipeline(pipeline)
  ///    .refresh(refresh)
  ///    .require_alias(require_alias)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn bulk_put_with_index(&self) -> builder::BulkPutWithIndex {
    builder::BulkPutWithIndex::new(self)
  }

  ///Allows to perform multiple index/update/delete operations in a single
  /// request.
  ///
  ///Sends a `POST` request to `/{index}/_bulk`
  ///
  ///Arguments:
  /// - `index`: Default index for items which don't provide one.
  /// - `source`: True or false to return the _source field or not, or default
  ///   list of fields to return, can be overridden on each sub-request.
  /// - `source_excludes`: Default list of fields to exclude from the returned
  ///   _source field, can be overridden on each sub-request.
  /// - `source_includes`: Default list of fields to extract and return from the
  ///   _source field, can be overridden on each sub-request.
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `require_alias`: Sets require_alias for all incoming documents.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `type_`: Default document type for items which don't provide one.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.bulk_post_with_index()
  ///    .index(index)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .pipeline(pipeline)
  ///    .refresh(refresh)
  ///    .require_alias(require_alias)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .type_(type_)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn bulk_post_with_index(&self) -> builder::BulkPostWithIndex {
    builder::BulkPostWithIndex::new(self)
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
  /// let response = client.indices_clear_cache_with_index()
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
  pub fn indices_clear_cache_with_index(&self) -> builder::IndicesClearCacheWithIndex {
    builder::IndicesClearCacheWithIndex::new(self)
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
  /// let response = client.indices_clone_put()
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
  pub fn indices_clone_put(&self) -> builder::IndicesClonePut {
    builder::IndicesClonePut::new(self)
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
  /// let response = client.indices_clone_post()
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
  pub fn indices_clone_post(&self) -> builder::IndicesClonePost {
    builder::IndicesClonePost::new(self)
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
  /// let response = client.indices_close()
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
  pub fn indices_close(&self) -> builder::IndicesClose {
    builder::IndicesClose::new(self)
  }

  ///Returns number of documents matching a query.
  ///
  ///Sends a `GET` request to `/{index}/_count`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to restrict the results.
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
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `min_score`: Include only documents with a specific `_score` value in
  ///   the result.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  ///```ignore
  /// let response = client.count_get_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .min_score(min_score)
  ///    .preference(preference)
  ///    .q(q)
  ///    .routing(routing)
  ///    .terminate_after(terminate_after)
  ///    .send()
  ///    .await;
  /// ```
  pub fn count_get_with_index(&self) -> builder::CountGetWithIndex {
    builder::CountGetWithIndex::new(self)
  }

  ///Returns number of documents matching a query.
  ///
  ///Sends a `POST` request to `/{index}/_count`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to restrict the results.
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
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `min_score`: Include only documents with a specific `_score` value in
  ///   the result.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  /// - `body`
  ///```ignore
  /// let response = client.count_post_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .min_score(min_score)
  ///    .preference(preference)
  ///    .q(q)
  ///    .routing(routing)
  ///    .terminate_after(terminate_after)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn count_post_with_index(&self) -> builder::CountPostWithIndex {
    builder::CountPostWithIndex::new(self)
  }

  ///Creates a new document in the index.
  ///
  ///Returns a 409 response when a document with a same ID already exists in
  /// the index.
  ///
  ///Sends a `PUT` request to `/{index}/_create/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.create_put()
  ///    .index(index)
  ///    .id(id)
  ///    .pipeline(pipeline)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_put(&self) -> builder::CreatePut {
    builder::CreatePut::new(self)
  }

  ///Creates a new document in the index.
  ///
  ///Returns a 409 response when a document with a same ID already exists in
  /// the index.
  ///
  ///Sends a `POST` request to `/{index}/_create/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.create_post()
  ///    .index(index)
  ///    .id(id)
  ///    .pipeline(pipeline)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_post(&self) -> builder::CreatePost {
    builder::CreatePost::new(self)
  }

  ///Deletes documents matching the provided query.
  ///
  ///Sends a `POST` request to `/{index}/_delete_by_query`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `conflicts`: What to do when the operation encounters version
  ///   conflicts?.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `from`: Starting offset.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `max_docs`: Maximum number of documents to process (default: all
  ///   documents).
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `refresh`: Refresh the shard containing the document before performing
  ///   the operation.
  /// - `request_cache`: Specify if request cache should be used for this
  ///   request or not, defaults to index level setting.
  /// - `requests_per_second`: The throttle for this request in sub-requests per
  ///   second. -1 means no throttle.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `scroll_size`: Size on the scroll request powering the operation.
  /// - `search_timeout`: Explicit timeout for each search request. Defaults to
  ///   no timeout.
  /// - `search_type`: Search operation type.
  /// - `size`: Deprecated, please use `max_docs` instead.
  /// - `slices`: The number of slices this task should be divided into.
  ///   Defaults to 1, meaning the task isn't sliced into subtasks. Can be set
  ///   to `auto`.
  /// - `sort`: Comma-separated list of <field>:<direction> pairs.
  /// - `stats`: Specific 'tag' of the request for logging and statistical
  ///   purposes.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  /// - `timeout`: Time each individual bulk request should wait for shards that
  ///   are unavailable.
  /// - `version`: Whether to return document version as part of a hit.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  /// - `body`
  ///```ignore
  /// let response = client.delete_by_query()
  ///    .index(index)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .conflicts(conflicts)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .from(from)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .max_docs(max_docs)
  ///    .preference(preference)
  ///    .q(q)
  ///    .refresh(refresh)
  ///    .request_cache(request_cache)
  ///    .requests_per_second(requests_per_second)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .scroll_size(scroll_size)
  ///    .search_timeout(search_timeout)
  ///    .search_type(search_type)
  ///    .size(size)
  ///    .slices(slices)
  ///    .sort(sort)
  ///    .stats(stats)
  ///    .terminate_after(terminate_after)
  ///    .timeout(timeout)
  ///    .version(version)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .wait_for_completion(wait_for_completion)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete_by_query(&self) -> builder::DeleteByQuery {
    builder::DeleteByQuery::new(self)
  }

  ///Creates or updates a document in an index.
  ///
  ///Sends a `POST` request to `/{index}/_doc`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `if_primary_term`: only perform the operation if the last operation that
  ///   has changed the document has the specified primary term.
  /// - `if_seq_no`: only perform the operation if the last operation that has
  ///   changed the document has the specified sequence number.
  /// - `op_type`: Explicit operation type. Defaults to `index` for requests
  ///   with an explicit document ID, and to `create`for requests without an
  ///   explicit document ID.
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `require_alias`: When true, requires destination to be an alias.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.index_post()
  ///    .index(index)
  ///    .if_primary_term(if_primary_term)
  ///    .if_seq_no(if_seq_no)
  ///    .op_type(op_type)
  ///    .pipeline(pipeline)
  ///    .refresh(refresh)
  ///    .require_alias(require_alias)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn index_post(&self) -> builder::IndexPost {
    builder::IndexPost::new(self)
  }

  ///Returns a document.
  ///
  ///Sends a `GET` request to `/{index}/_doc/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specify whether to perform the operation in realtime or
  ///   search mode.
  /// - `refresh`: Refresh the shard containing the document before performing
  ///   the operation.
  /// - `routing`: Routing value.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.get()
  ///    .index(index)
  ///    .id(id)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .stored_fields(stored_fields)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get(&self) -> builder::Get {
    builder::Get::new(self)
  }

  ///Creates or updates a document in an index.
  ///
  ///Sends a `PUT` request to `/{index}/_doc/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `if_primary_term`: only perform the operation if the last operation that
  ///   has changed the document has the specified primary term.
  /// - `if_seq_no`: only perform the operation if the last operation that has
  ///   changed the document has the specified sequence number.
  /// - `op_type`: Explicit operation type. Defaults to `index` for requests
  ///   with an explicit document ID, and to `create`for requests without an
  ///   explicit document ID.
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `require_alias`: When true, requires destination to be an alias.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.index_put_with_id()
  ///    .index(index)
  ///    .id(id)
  ///    .if_primary_term(if_primary_term)
  ///    .if_seq_no(if_seq_no)
  ///    .op_type(op_type)
  ///    .pipeline(pipeline)
  ///    .refresh(refresh)
  ///    .require_alias(require_alias)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn index_put_with_id(&self) -> builder::IndexPutWithId {
    builder::IndexPutWithId::new(self)
  }

  ///Creates or updates a document in an index.
  ///
  ///Sends a `POST` request to `/{index}/_doc/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `if_primary_term`: only perform the operation if the last operation that
  ///   has changed the document has the specified primary term.
  /// - `if_seq_no`: only perform the operation if the last operation that has
  ///   changed the document has the specified sequence number.
  /// - `op_type`: Explicit operation type. Defaults to `index` for requests
  ///   with an explicit document ID, and to `create`for requests without an
  ///   explicit document ID.
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `require_alias`: When true, requires destination to be an alias.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.index_post_with_id()
  ///    .index(index)
  ///    .id(id)
  ///    .if_primary_term(if_primary_term)
  ///    .if_seq_no(if_seq_no)
  ///    .op_type(op_type)
  ///    .pipeline(pipeline)
  ///    .refresh(refresh)
  ///    .require_alias(require_alias)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn index_post_with_id(&self) -> builder::IndexPostWithId {
    builder::IndexPostWithId::new(self)
  }

  ///Removes a document from the index.
  ///
  ///Sends a `DELETE` request to `/{index}/_doc/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `if_primary_term`: only perform the operation if the last operation that
  ///   has changed the document has the specified primary term.
  /// - `if_seq_no`: only perform the operation if the last operation that has
  ///   changed the document has the specified sequence number.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  ///```ignore
  /// let response = client.delete()
  ///    .index(index)
  ///    .id(id)
  ///    .if_primary_term(if_primary_term)
  ///    .if_seq_no(if_seq_no)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .send()
  ///    .await;
  /// ```
  pub fn delete(&self) -> builder::Delete {
    builder::Delete::new(self)
  }

  ///Returns information about whether a document exists in an index.
  ///
  ///Sends a `HEAD` request to `/{index}/_doc/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specify whether to perform the operation in realtime or
  ///   search mode.
  /// - `refresh`: Refresh the shard containing the document before performing
  ///   the operation.
  /// - `routing`: Routing value.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.exists()
  ///    .index(index)
  ///    .id(id)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .stored_fields(stored_fields)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn exists(&self) -> builder::Exists {
    builder::Exists::new(self)
  }

  ///Returns information about why a specific matches (or doesn't match) a
  /// query.
  ///
  ///Sends a `GET` request to `/{index}/_explain/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `analyze_wildcard`: Specify whether wildcards and prefix queries in the
  ///   query string query should be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The default field for query string query.
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `routing`: Routing value.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  ///```ignore
  /// let response = client.explain_get()
  ///    .index(index)
  ///    .id(id)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .lenient(lenient)
  ///    .preference(preference)
  ///    .q(q)
  ///    .routing(routing)
  ///    .stored_fields(stored_fields)
  ///    .send()
  ///    .await;
  /// ```
  pub fn explain_get(&self) -> builder::ExplainGet {
    builder::ExplainGet::new(self)
  }

  ///Returns information about why a specific matches (or doesn't match) a
  /// query.
  ///
  ///Sends a `POST` request to `/{index}/_explain/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `analyze_wildcard`: Specify whether wildcards and prefix queries in the
  ///   query string query should be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The default field for query string query.
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `routing`: Routing value.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  /// - `body`
  ///```ignore
  /// let response = client.explain_post()
  ///    .index(index)
  ///    .id(id)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .lenient(lenient)
  ///    .preference(preference)
  ///    .q(q)
  ///    .routing(routing)
  ///    .stored_fields(stored_fields)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn explain_post(&self) -> builder::ExplainPost {
    builder::ExplainPost::new(self)
  }

  ///Returns the information about the capabilities of fields among multiple
  /// indices.
  ///
  ///Sends a `GET` request to `/{index}/_field_caps`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fields`: Comma-separated list of field names.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `include_unmapped`: Indicates whether unmapped fields should be included
  ///   in the response.
  ///```ignore
  /// let response = client.field_caps_get_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fields(fields)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_unmapped(include_unmapped)
  ///    .send()
  ///    .await;
  /// ```
  pub fn field_caps_get_with_index(&self) -> builder::FieldCapsGetWithIndex {
    builder::FieldCapsGetWithIndex::new(self)
  }

  ///Returns the information about the capabilities of fields among multiple
  /// indices.
  ///
  ///Sends a `POST` request to `/{index}/_field_caps`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `fields`: Comma-separated list of field names.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `include_unmapped`: Indicates whether unmapped fields should be included
  ///   in the response.
  /// - `body`
  ///```ignore
  /// let response = client.field_caps_post_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .fields(fields)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .include_unmapped(include_unmapped)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn field_caps_post_with_index(&self) -> builder::FieldCapsPostWithIndex {
    builder::FieldCapsPostWithIndex::new(self)
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
  /// let response = client.indices_flush_get_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .force(force)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .wait_if_ongoing(wait_if_ongoing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_flush_get_with_index(&self) -> builder::IndicesFlushGetWithIndex {
    builder::IndicesFlushGetWithIndex::new(self)
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
  /// let response = client.indices_flush_post_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .force(force)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .wait_if_ongoing(wait_if_ongoing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_flush_post_with_index(&self) -> builder::IndicesFlushPostWithIndex {
    builder::IndicesFlushPostWithIndex::new(self)
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
  /// let response = client.indices_forcemerge_with_index()
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
  pub fn indices_forcemerge_with_index(&self) -> builder::IndicesForcemergeWithIndex {
    builder::IndicesForcemergeWithIndex::new(self)
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
  /// let response = client.indices_get_mapping_with_index()
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
  pub fn indices_get_mapping_with_index(&self) -> builder::IndicesGetMappingWithIndex {
    builder::IndicesGetMappingWithIndex::new(self)
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
  /// let response = client.indices_put_mapping_put()
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
  pub fn indices_put_mapping_put(&self) -> builder::IndicesPutMappingPut {
    builder::IndicesPutMappingPut::new(self)
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
  /// let response = client.indices_put_mapping_post()
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
  pub fn indices_put_mapping_post(&self) -> builder::IndicesPutMappingPost {
    builder::IndicesPutMappingPost::new(self)
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
  /// let response = client.indices_get_field_mapping_with_index()
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
  pub fn indices_get_field_mapping_with_index(&self) -> builder::IndicesGetFieldMappingWithIndex {
    builder::IndicesGetFieldMappingWithIndex::new(self)
  }

  ///Allows to get multiple documents in one request.
  ///
  ///Sends a `GET` request to `/{index}/_mget`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specify whether to perform the operation in realtime or
  ///   search mode.
  /// - `refresh`: Refresh the shard containing the document before performing
  ///   the operation.
  /// - `routing`: Routing value.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  ///```ignore
  /// let response = client.mget_get_with_index()
  ///    .index(index)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .stored_fields(stored_fields)
  ///    .send()
  ///    .await;
  /// ```
  pub fn mget_get_with_index(&self) -> builder::MgetGetWithIndex {
    builder::MgetGetWithIndex::new(self)
  }

  ///Allows to get multiple documents in one request.
  ///
  ///Sends a `POST` request to `/{index}/_mget`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specify whether to perform the operation in realtime or
  ///   search mode.
  /// - `refresh`: Refresh the shard containing the document before performing
  ///   the operation.
  /// - `routing`: Routing value.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  /// - `body`
  ///```ignore
  /// let response = client.mget_post_with_index()
  ///    .index(index)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .stored_fields(stored_fields)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn mget_post_with_index(&self) -> builder::MgetPostWithIndex {
    builder::MgetPostWithIndex::new(self)
  }

  ///Allows to execute several search operations in one request.
  ///
  ///Sends a `GET` request to `/{index}/_msearch`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to use as default.
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `max_concurrent_searches`: Controls the maximum number of concurrent
  ///   searches the multi search api will execute.
  /// - `max_concurrent_shard_requests`: The number of concurrent shard requests
  ///   each sub search executes concurrently per node. This value should be
  ///   used to limit the impact of the search on the cluster in order to limit
  ///   the number of concurrent shard requests.
  /// - `pre_filter_shard_size`: Threshold that enforces a pre-filter round-trip
  ///   to prefilter search shards based on query rewriting if the number of
  ///   shards the search request expands to exceeds the threshold. This filter
  ///   round-trip can limit the number of shards significantly if for instance
  ///   a shard can not match any documents based on its rewrite method ie. if
  ///   date filters are mandatory to match but the shard bounds and the query
  ///   are disjoint.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  ///```ignore
  /// let response = client.msearch_get_with_index()
  ///    .index(index)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .max_concurrent_searches(max_concurrent_searches)
  ///    .max_concurrent_shard_requests(max_concurrent_shard_requests)
  ///    .pre_filter_shard_size(pre_filter_shard_size)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .send()
  ///    .await;
  /// ```
  pub fn msearch_get_with_index(&self) -> builder::MsearchGetWithIndex {
    builder::MsearchGetWithIndex::new(self)
  }

  ///Allows to execute several search operations in one request.
  ///
  ///Sends a `POST` request to `/{index}/_msearch`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to use as default.
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `max_concurrent_searches`: Controls the maximum number of concurrent
  ///   searches the multi search api will execute.
  /// - `max_concurrent_shard_requests`: The number of concurrent shard requests
  ///   each sub search executes concurrently per node. This value should be
  ///   used to limit the impact of the search on the cluster in order to limit
  ///   the number of concurrent shard requests.
  /// - `pre_filter_shard_size`: Threshold that enforces a pre-filter round-trip
  ///   to prefilter search shards based on query rewriting if the number of
  ///   shards the search request expands to exceeds the threshold. This filter
  ///   round-trip can limit the number of shards significantly if for instance
  ///   a shard can not match any documents based on its rewrite method ie. if
  ///   date filters are mandatory to match but the shard bounds and the query
  ///   are disjoint.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `body`
  ///```ignore
  /// let response = client.msearch_post_with_index()
  ///    .index(index)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .max_concurrent_searches(max_concurrent_searches)
  ///    .max_concurrent_shard_requests(max_concurrent_shard_requests)
  ///    .pre_filter_shard_size(pre_filter_shard_size)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn msearch_post_with_index(&self) -> builder::MsearchPostWithIndex {
    builder::MsearchPostWithIndex::new(self)
  }

  ///Allows to execute several search template operations in one request.
  ///
  ///Sends a `GET` request to `/{index}/_msearch/template`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to use as default.
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `max_concurrent_searches`: Controls the maximum number of concurrent
  ///   searches the multi search api will execute.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  ///```ignore
  /// let response = client.msearch_template_get_with_index()
  ///    .index(index)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .max_concurrent_searches(max_concurrent_searches)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .send()
  ///    .await;
  /// ```
  pub fn msearch_template_get_with_index(&self) -> builder::MsearchTemplateGetWithIndex {
    builder::MsearchTemplateGetWithIndex::new(self)
  }

  ///Allows to execute several search template operations in one request.
  ///
  ///Sends a `POST` request to `/{index}/_msearch/template`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices to use as default.
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `max_concurrent_searches`: Controls the maximum number of concurrent
  ///   searches the multi search api will execute.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `body`
  ///```ignore
  /// let response = client.msearch_template_post_with_index()
  ///    .index(index)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .max_concurrent_searches(max_concurrent_searches)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn msearch_template_post_with_index(&self) -> builder::MsearchTemplatePostWithIndex {
    builder::MsearchTemplatePostWithIndex::new(self)
  }

  ///Returns multiple termvectors in one request.
  ///
  ///Sends a `GET` request to `/{index}/_mtermvectors`
  ///
  ///Arguments:
  /// - `index`: The index in which the document resides.
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  ///   Applies to all returned documents unless otherwise specified in body
  ///   'params' or 'docs'.
  /// - `fields`: Comma-separated list of fields to return. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `ids`: Comma-separated list of documents ids. You must define ids as
  ///   parameter or set 'ids' or 'docs' in the request body.
  /// - `offsets`: Specifies if term offsets should be returned. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `payloads`: Specifies if term payloads should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `positions`: Specifies if term positions should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on. Applies to all returned documents unless otherwise
  ///   specified in body 'params' or 'docs'.
  /// - `realtime`: Specifies if requests are real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.mtermvectors_get_with_index()
  ///    .index(index)
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .ids(ids)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn mtermvectors_get_with_index(&self) -> builder::MtermvectorsGetWithIndex {
    builder::MtermvectorsGetWithIndex::new(self)
  }

  ///Returns multiple termvectors in one request.
  ///
  ///Sends a `POST` request to `/{index}/_mtermvectors`
  ///
  ///Arguments:
  /// - `index`: The index in which the document resides.
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  ///   Applies to all returned documents unless otherwise specified in body
  ///   'params' or 'docs'.
  /// - `fields`: Comma-separated list of fields to return. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `ids`: Comma-separated list of documents ids. You must define ids as
  ///   parameter or set 'ids' or 'docs' in the request body.
  /// - `offsets`: Specifies if term offsets should be returned. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `payloads`: Specifies if term payloads should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `positions`: Specifies if term positions should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on. Applies to all returned documents unless otherwise
  ///   specified in body 'params' or 'docs'.
  /// - `realtime`: Specifies if requests are real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `body`
  ///```ignore
  /// let response = client.mtermvectors_post_with_index()
  ///    .index(index)
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .ids(ids)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn mtermvectors_post_with_index(&self) -> builder::MtermvectorsPostWithIndex {
    builder::MtermvectorsPostWithIndex::new(self)
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
  /// let response = client.indices_open()
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
  pub fn indices_open(&self) -> builder::IndicesOpen {
    builder::IndicesOpen::new(self)
  }

  ///Allows to evaluate the quality of ranked search results over a set of
  /// typical search queries.
  ///
  ///Sends a `GET` request to `/{index}/_rank_eval`
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
  /// - `search_type`: Search operation type.
  ///```ignore
  /// let response = client.rank_eval_get_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .search_type(search_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn rank_eval_get_with_index(&self) -> builder::RankEvalGetWithIndex {
    builder::RankEvalGetWithIndex::new(self)
  }

  ///Allows to evaluate the quality of ranked search results over a set of
  /// typical search queries.
  ///
  ///Sends a `POST` request to `/{index}/_rank_eval`
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
  /// - `search_type`: Search operation type.
  /// - `body`
  ///```ignore
  /// let response = client.rank_eval_post_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .search_type(search_type)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn rank_eval_post_with_index(&self) -> builder::RankEvalPostWithIndex {
    builder::RankEvalPostWithIndex::new(self)
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
  /// let response = client.indices_recovery_with_index()
  ///    .index(index)
  ///    .active_only(active_only)
  ///    .detailed(detailed)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_recovery_with_index(&self) -> builder::IndicesRecoveryWithIndex {
    builder::IndicesRecoveryWithIndex::new(self)
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
  /// let response = client.indices_refresh_get_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_refresh_get_with_index(&self) -> builder::IndicesRefreshGetWithIndex {
    builder::IndicesRefreshGetWithIndex::new(self)
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
  /// let response = client.indices_refresh_post_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_refresh_post_with_index(&self) -> builder::IndicesRefreshPostWithIndex {
    builder::IndicesRefreshPostWithIndex::new(self)
  }

  ///Returns results matching a query.
  ///
  ///Sends a `GET` request to `/{index}/_search`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `allow_partial_search_results`: Indicate if an error should be returned
  ///   if there is a partial search failure or timeout.
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `batched_reduce_size`: The number of shard results that should be
  ///   reduced at once on the coordinating node. This value should be used as a
  ///   protection mechanism to reduce the memory overhead per search request if
  ///   the potential number of shards in the request can be large.
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `docvalue_fields`: Comma-separated list of fields to return as the
  ///   docvalue representation of a field for each hit.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Specify whether to return detailed information about score
  ///   computation as part of a hit.
  /// - `from`: Starting offset.
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `max_concurrent_shard_requests`: The number of concurrent shard requests
  ///   per node this search executes concurrently. This value should be used to
  ///   limit the impact of the search on the cluster in order to limit the
  ///   number of concurrent shard requests.
  /// - `pre_filter_shard_size`: Threshold that enforces a pre-filter round-trip
  ///   to prefilter search shards based on query rewriting if the number of
  ///   shards the search request expands to exceeds the threshold. This filter
  ///   round-trip can limit the number of shards significantly if for instance
  ///   a shard can not match any documents based on its rewrite method ie. if
  ///   date filters are mandatory to match but the shard bounds and the query
  ///   are disjoint.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `request_cache`: Specify if request cache should be used for this
  ///   request or not, defaults to index level setting.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `search_type`: Search operation type.
  /// - `seq_no_primary_term`: Specify whether to return sequence number and
  ///   primary term of the last modification of each hit.
  /// - `size`: Number of hits to return.
  /// - `sort`: Comma-separated list of <field>:<direction> pairs.
  /// - `stats`: Specific 'tag' of the request for logging and statistical
  ///   purposes.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  /// - `suggest_field`: Specify which field to use for suggestions.
  /// - `suggest_mode`: Specify suggest mode.
  /// - `suggest_size`: How many suggestions to return in response.
  /// - `suggest_text`: The source text for which the suggestions should be
  ///   returned.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  /// - `timeout`: Operation timeout.
  /// - `track_scores`: Whether to calculate and return scores even if they are
  ///   not used for sorting.
  /// - `track_total_hits`: Indicate if the number of documents that match the
  ///   query should be tracked.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `version`: Whether to return document version as part of a hit.
  ///```ignore
  /// let response = client.search_get_with_index()
  ///    .index(index)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .allow_no_indices(allow_no_indices)
  ///    .allow_partial_search_results(allow_partial_search_results)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .batched_reduce_size(batched_reduce_size)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .docvalue_fields(docvalue_fields)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .from(from)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .max_concurrent_shard_requests(max_concurrent_shard_requests)
  ///    .pre_filter_shard_size(pre_filter_shard_size)
  ///    .preference(preference)
  ///    .q(q)
  ///    .request_cache(request_cache)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .search_type(search_type)
  ///    .seq_no_primary_term(seq_no_primary_term)
  ///    .size(size)
  ///    .sort(sort)
  ///    .stats(stats)
  ///    .stored_fields(stored_fields)
  ///    .suggest_field(suggest_field)
  ///    .suggest_mode(suggest_mode)
  ///    .suggest_size(suggest_size)
  ///    .suggest_text(suggest_text)
  ///    .terminate_after(terminate_after)
  ///    .timeout(timeout)
  ///    .track_scores(track_scores)
  ///    .track_total_hits(track_total_hits)
  ///    .typed_keys(typed_keys)
  ///    .version(version)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_get_with_index(&self) -> builder::SearchGetWithIndex {
    builder::SearchGetWithIndex::new(self)
  }

  ///Returns results matching a query.
  ///
  ///Sends a `POST` request to `/{index}/_search`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `allow_partial_search_results`: Indicate if an error should be returned
  ///   if there is a partial search failure or timeout.
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `batched_reduce_size`: The number of shard results that should be
  ///   reduced at once on the coordinating node. This value should be used as a
  ///   protection mechanism to reduce the memory overhead per search request if
  ///   the potential number of shards in the request can be large.
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `docvalue_fields`: Comma-separated list of fields to return as the
  ///   docvalue representation of a field for each hit.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Specify whether to return detailed information about score
  ///   computation as part of a hit.
  /// - `from`: Starting offset.
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `max_concurrent_shard_requests`: The number of concurrent shard requests
  ///   per node this search executes concurrently. This value should be used to
  ///   limit the impact of the search on the cluster in order to limit the
  ///   number of concurrent shard requests.
  /// - `pre_filter_shard_size`: Threshold that enforces a pre-filter round-trip
  ///   to prefilter search shards based on query rewriting if the number of
  ///   shards the search request expands to exceeds the threshold. This filter
  ///   round-trip can limit the number of shards significantly if for instance
  ///   a shard can not match any documents based on its rewrite method ie. if
  ///   date filters are mandatory to match but the shard bounds and the query
  ///   are disjoint.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `request_cache`: Specify if request cache should be used for this
  ///   request or not, defaults to index level setting.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `search_type`: Search operation type.
  /// - `seq_no_primary_term`: Specify whether to return sequence number and
  ///   primary term of the last modification of each hit.
  /// - `size`: Number of hits to return.
  /// - `sort`: Comma-separated list of <field>:<direction> pairs.
  /// - `stats`: Specific 'tag' of the request for logging and statistical
  ///   purposes.
  /// - `stored_fields`: Comma-separated list of stored fields to return.
  /// - `suggest_field`: Specify which field to use for suggestions.
  /// - `suggest_mode`: Specify suggest mode.
  /// - `suggest_size`: How many suggestions to return in response.
  /// - `suggest_text`: The source text for which the suggestions should be
  ///   returned.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  /// - `timeout`: Operation timeout.
  /// - `track_scores`: Whether to calculate and return scores even if they are
  ///   not used for sorting.
  /// - `track_total_hits`: Indicate if the number of documents that match the
  ///   query should be tracked.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `version`: Whether to return document version as part of a hit.
  /// - `body`
  ///```ignore
  /// let response = client.search_post_with_index()
  ///    .index(index)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .allow_no_indices(allow_no_indices)
  ///    .allow_partial_search_results(allow_partial_search_results)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .batched_reduce_size(batched_reduce_size)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .docvalue_fields(docvalue_fields)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .from(from)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .max_concurrent_shard_requests(max_concurrent_shard_requests)
  ///    .pre_filter_shard_size(pre_filter_shard_size)
  ///    .preference(preference)
  ///    .q(q)
  ///    .request_cache(request_cache)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .search_type(search_type)
  ///    .seq_no_primary_term(seq_no_primary_term)
  ///    .size(size)
  ///    .sort(sort)
  ///    .stats(stats)
  ///    .stored_fields(stored_fields)
  ///    .suggest_field(suggest_field)
  ///    .suggest_mode(suggest_mode)
  ///    .suggest_size(suggest_size)
  ///    .suggest_text(suggest_text)
  ///    .terminate_after(terminate_after)
  ///    .timeout(timeout)
  ///    .track_scores(track_scores)
  ///    .track_total_hits(track_total_hits)
  ///    .typed_keys(typed_keys)
  ///    .version(version)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_post_with_index(&self) -> builder::SearchPostWithIndex {
    builder::SearchPostWithIndex::new(self)
  }

  ///Creates point in time context.
  ///
  ///Sends a `POST` request to `/{index}/_search/point_in_time`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_partial_pit_creation`: Allow if point in time can be created with
  ///   partial failures.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `keep_alive`: Specify the keep alive for point in time.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `routing`: Comma-separated list of specific routing values.
  ///```ignore
  /// let response = client.create_pit()
  ///    .index(index)
  ///    .allow_partial_pit_creation(allow_partial_pit_creation)
  ///    .expand_wildcards(expand_wildcards)
  ///    .keep_alive(keep_alive)
  ///    .preference(preference)
  ///    .routing(routing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn create_pit(&self) -> builder::CreatePit {
    builder::CreatePit::new(self)
  }

  ///Allows to use the Mustache language to pre-render a search definition.
  ///
  ///Sends a `GET` request to `/{index}/_search/template`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Specify whether to return detailed information about score
  ///   computation as part of a hit.
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `profile`: Specify whether to profile the query execution.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  ///```ignore
  /// let response = client.search_template_get_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .preference(preference)
  ///    .profile(profile)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_template_get_with_index(&self) -> builder::SearchTemplateGetWithIndex {
    builder::SearchTemplateGetWithIndex::new(self)
  }

  ///Allows to use the Mustache language to pre-render a search definition.
  ///
  ///Sends a `POST` request to `/{index}/_search/template`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `ccs_minimize_roundtrips`: Indicates whether network round-trips should
  ///   be minimized as part of cross-cluster search requests execution.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `explain`: Specify whether to return detailed information about score
  ///   computation as part of a hit.
  /// - `ignore_throttled`: Whether specified concrete, expanded or aliased
  ///   indices should be ignored when throttled.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `profile`: Specify whether to profile the query execution.
  /// - `rest_total_hits_as_int`: Indicates whether hits.total should be
  ///   rendered as an integer or an object in the rest search response.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `search_type`: Search operation type.
  /// - `typed_keys`: Specify whether aggregation and suggester names should be
  ///   prefixed by their respective types in the response.
  /// - `body`
  ///```ignore
  /// let response = client.search_template_post_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .ccs_minimize_roundtrips(ccs_minimize_roundtrips)
  ///    .expand_wildcards(expand_wildcards)
  ///    .explain(explain)
  ///    .ignore_throttled(ignore_throttled)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .preference(preference)
  ///    .profile(profile)
  ///    .rest_total_hits_as_int(rest_total_hits_as_int)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .search_type(search_type)
  ///    .typed_keys(typed_keys)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_template_post_with_index(&self) -> builder::SearchTemplatePostWithIndex {
    builder::SearchTemplatePostWithIndex::new(self)
  }

  ///Returns information about the indices and shards that a search request
  /// would be executed against.
  ///
  ///Sends a `GET` request to `/{index}/_search_shards`
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
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `routing`: Routing value.
  ///```ignore
  /// let response = client.search_shards_get_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .preference(preference)
  ///    .routing(routing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_shards_get_with_index(&self) -> builder::SearchShardsGetWithIndex {
    builder::SearchShardsGetWithIndex::new(self)
  }

  ///Returns information about the indices and shards that a search request
  /// would be executed against.
  ///
  ///Sends a `POST` request to `/{index}/_search_shards`
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
  /// - `local`: Return local information, do not retrieve the state from
  ///   cluster-manager node.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `routing`: Routing value.
  ///```ignore
  /// let response = client.search_shards_post_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .local(local)
  ///    .preference(preference)
  ///    .routing(routing)
  ///    .send()
  ///    .await;
  /// ```
  pub fn search_shards_post_with_index(&self) -> builder::SearchShardsPostWithIndex {
    builder::SearchShardsPostWithIndex::new(self)
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
  /// let response = client.indices_segments_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .verbose(verbose)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_segments_with_index(&self) -> builder::IndicesSegmentsWithIndex {
    builder::IndicesSegmentsWithIndex::new(self)
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
  /// let response = client.indices_get_settings_with_index()
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
  pub fn indices_get_settings_with_index(&self) -> builder::IndicesGetSettingsWithIndex {
    builder::IndicesGetSettingsWithIndex::new(self)
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
  /// let response = client.indices_put_settings_with_index()
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
  pub fn indices_put_settings_with_index(&self) -> builder::IndicesPutSettingsWithIndex {
    builder::IndicesPutSettingsWithIndex::new(self)
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
  /// let response = client.indices_get_settings_with_index_name()
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
  pub fn indices_get_settings_with_index_name(&self) -> builder::IndicesGetSettingsWithIndexName {
    builder::IndicesGetSettingsWithIndexName::new(self)
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
  /// let response = client.indices_shard_stores_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .status(status)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_shard_stores_with_index(&self) -> builder::IndicesShardStoresWithIndex {
    builder::IndicesShardStoresWithIndex::new(self)
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
  /// let response = client.indices_shrink_put()
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
  pub fn indices_shrink_put(&self) -> builder::IndicesShrinkPut {
    builder::IndicesShrinkPut::new(self)
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
  /// let response = client.indices_shrink_post()
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
  pub fn indices_shrink_post(&self) -> builder::IndicesShrinkPost {
    builder::IndicesShrinkPost::new(self)
  }

  ///Returns the source of a document.
  ///
  ///Sends a `GET` request to `/{index}/_source/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specify whether to perform the operation in realtime or
  ///   search mode.
  /// - `refresh`: Refresh the shard containing the document before performing
  ///   the operation.
  /// - `routing`: Routing value.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.get_source()
  ///    .index(index)
  ///    .id(id)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_source(&self) -> builder::GetSource {
    builder::GetSource::new(self)
  }

  ///Returns information about whether a document source exists in an index.
  ///
  ///Sends a `HEAD` request to `/{index}/_source/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specify whether to perform the operation in realtime or
  ///   search mode.
  /// - `refresh`: Refresh the shard containing the document before performing
  ///   the operation.
  /// - `routing`: Routing value.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.exists_source()
  ///    .index(index)
  ///    .id(id)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .refresh(refresh)
  ///    .routing(routing)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn exists_source(&self) -> builder::ExistsSource {
    builder::ExistsSource::new(self)
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
  /// let response = client.indices_split_put()
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
  pub fn indices_split_put(&self) -> builder::IndicesSplitPut {
    builder::IndicesSplitPut::new(self)
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
  /// let response = client.indices_split_post()
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
  pub fn indices_split_post(&self) -> builder::IndicesSplitPost {
    builder::IndicesSplitPost::new(self)
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
  /// let response = client.indices_stats_with_index()
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
  pub fn indices_stats_with_index(&self) -> builder::IndicesStatsWithIndex {
    builder::IndicesStatsWithIndex::new(self)
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
  /// let response = client.indices_stats_with_index_metric()
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
  pub fn indices_stats_with_index_metric(&self) -> builder::IndicesStatsWithIndexMetric {
    builder::IndicesStatsWithIndexMetric::new(self)
  }

  ///Returns information and statistics about terms in the fields of a
  /// particular document.
  ///
  ///Sends a `GET` request to `/{index}/_termvectors`
  ///
  ///Arguments:
  /// - `index`: The index in which the document resides.
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  /// - `fields`: Comma-separated list of fields to return.
  /// - `offsets`: Specifies if term offsets should be returned.
  /// - `payloads`: Specifies if term payloads should be returned.
  /// - `positions`: Specifies if term positions should be returned.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specifies if request is real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.termvectors_get()
  ///    .index(index)
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn termvectors_get(&self) -> builder::TermvectorsGet {
    builder::TermvectorsGet::new(self)
  }

  ///Returns information and statistics about terms in the fields of a
  /// particular document.
  ///
  ///Sends a `POST` request to `/{index}/_termvectors`
  ///
  ///Arguments:
  /// - `index`: The index in which the document resides.
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  /// - `fields`: Comma-separated list of fields to return.
  /// - `offsets`: Specifies if term offsets should be returned.
  /// - `payloads`: Specifies if term payloads should be returned.
  /// - `positions`: Specifies if term positions should be returned.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specifies if request is real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `body`
  ///```ignore
  /// let response = client.termvectors_post()
  ///    .index(index)
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn termvectors_post(&self) -> builder::TermvectorsPost {
    builder::TermvectorsPost::new(self)
  }

  ///Returns information and statistics about terms in the fields of a
  /// particular document.
  ///
  ///Sends a `GET` request to `/{index}/_termvectors/{id}`
  ///
  ///Arguments:
  /// - `index`: The index in which the document resides.
  /// - `id`: Document ID. When not specified a doc param should be supplied.
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  /// - `fields`: Comma-separated list of fields to return.
  /// - `offsets`: Specifies if term offsets should be returned.
  /// - `payloads`: Specifies if term payloads should be returned.
  /// - `positions`: Specifies if term positions should be returned.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specifies if request is real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.termvectors_get_with_id()
  ///    .index(index)
  ///    .id(id)
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn termvectors_get_with_id(&self) -> builder::TermvectorsGetWithId {
    builder::TermvectorsGetWithId::new(self)
  }

  ///Returns information and statistics about terms in the fields of a
  /// particular document.
  ///
  ///Sends a `POST` request to `/{index}/_termvectors/{id}`
  ///
  ///Arguments:
  /// - `index`: The index in which the document resides.
  /// - `id`: Document ID. When not specified a doc param should be supplied.
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  /// - `fields`: Comma-separated list of fields to return.
  /// - `offsets`: Specifies if term offsets should be returned.
  /// - `payloads`: Specifies if term payloads should be returned.
  /// - `positions`: Specifies if term positions should be returned.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `realtime`: Specifies if request is real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `body`
  ///```ignore
  /// let response = client.termvectors_post_with_id()
  ///    .index(index)
  ///    .id(id)
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn termvectors_post_with_id(&self) -> builder::TermvectorsPostWithId {
    builder::TermvectorsPostWithId::new(self)
  }

  ///Updates a document with a script or partial document.
  ///
  ///Sends a `POST` request to `/{index}/_update/{id}`
  ///
  ///Arguments:
  /// - `index`: Index name.
  /// - `id`: Document ID.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `if_primary_term`: only perform the operation if the last operation that
  ///   has changed the document has the specified primary term.
  /// - `if_seq_no`: only perform the operation if the last operation that has
  ///   changed the document has the specified sequence number.
  /// - `lang`: The script language.
  /// - `refresh`: If `true` then refresh the affected shards to make this
  ///   operation visible to search, if `wait_for` then wait for a refresh to
  ///   make this operation visible to search, if `false` (the default) then do
  ///   nothing with refreshes.
  /// - `require_alias`: When true, requires destination to be an alias.
  /// - `retry_on_conflict`: Specify how many times should the operation be
  ///   retried when a conflict occurs.
  /// - `routing`: Routing value.
  /// - `timeout`: Operation timeout.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `body`
  ///```ignore
  /// let response = client.update()
  ///    .index(index)
  ///    .id(id)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .if_primary_term(if_primary_term)
  ///    .if_seq_no(if_seq_no)
  ///    .lang(lang)
  ///    .refresh(refresh)
  ///    .require_alias(require_alias)
  ///    .retry_on_conflict(retry_on_conflict)
  ///    .routing(routing)
  ///    .timeout(timeout)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update(&self) -> builder::Update {
    builder::Update::new(self)
  }

  ///Performs an update on every document in the index without changing the
  /// source, for example to pick up a mapping change.
  ///
  ///Sends a `POST` request to `/{index}/_update_by_query`
  ///
  ///Arguments:
  /// - `index`: Comma-separated list of indices; use `_all` or empty string to
  ///   perform the operation on all indices.
  /// - `source`: True or false to return the _source field or not, or a list of
  ///   fields to return.
  /// - `source_excludes`: List of fields to exclude from the returned _source
  ///   field.
  /// - `source_includes`: List of fields to extract and return from the _source
  ///   field.
  /// - `allow_no_indices`: Whether to ignore if a wildcard indices expression
  ///   resolves into no concrete indices. (This includes `_all` string or when
  ///   no indices have been specified).
  /// - `analyze_wildcard`: Specify whether wildcard and prefix queries should
  ///   be analyzed.
  /// - `analyzer`: The analyzer to use for the query string.
  /// - `conflicts`: What to do when the operation encounters version
  ///   conflicts?.
  /// - `default_operator`: The default operator for query string query (AND or
  ///   OR).
  /// - `df`: The field to use as default where no field prefix is given in the
  ///   query string.
  /// - `expand_wildcards`: Whether to expand wildcard expression to concrete
  ///   indices that are open, closed or both.
  /// - `from`: Starting offset.
  /// - `ignore_unavailable`: Whether specified concrete indices should be
  ///   ignored when unavailable (missing or closed).
  /// - `lenient`: Specify whether format-based query failures (such as
  ///   providing text to a numeric field) should be ignored.
  /// - `max_docs`: Maximum number of documents to process (default: all
  ///   documents).
  /// - `pipeline`: The pipeline id to preprocess incoming documents with.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on.
  /// - `q`: Query in the Lucene query string syntax.
  /// - `refresh`: Should the affected indexes be refreshed?.
  /// - `request_cache`: Specify if request cache should be used for this
  ///   request or not, defaults to index level setting.
  /// - `requests_per_second`: The throttle for this request in sub-requests per
  ///   second. -1 means no throttle.
  /// - `routing`: Comma-separated list of specific routing values.
  /// - `scroll`: Specify how long a consistent view of the index should be
  ///   maintained for scrolled search.
  /// - `scroll_size`: Size on the scroll request powering the operation.
  /// - `search_timeout`: Explicit timeout for each search request. Defaults to
  ///   no timeout.
  /// - `search_type`: Search operation type.
  /// - `size`: Deprecated, please use `max_docs` instead.
  /// - `slices`: The number of slices this task should be divided into.
  ///   Defaults to 1, meaning the task isn't sliced into subtasks. Can be set
  ///   to `auto`.
  /// - `sort`: Comma-separated list of <field>:<direction> pairs.
  /// - `stats`: Specific 'tag' of the request for logging and statistical
  ///   purposes.
  /// - `terminate_after`: The maximum number of documents to collect for each
  ///   shard, upon reaching which the query execution will terminate early.
  /// - `timeout`: Time each individual bulk request should wait for shards that
  ///   are unavailable.
  /// - `version`: Whether to return document version as part of a hit.
  /// - `wait_for_active_shards`: Sets the number of shard copies that must be
  ///   active before proceeding with the operation. Defaults to 1, meaning the
  ///   primary shard only. Set to `all` for all shard copies, otherwise set to
  ///   any non-negative value less than or equal to the total number of copies
  ///   for the shard (number of replicas + 1).
  /// - `wait_for_completion`: Should this request wait until the operation has
  ///   completed before returning.
  /// - `body`
  ///```ignore
  /// let response = client.update_by_query()
  ///    .index(index)
  ///    .source(source)
  ///    .source_excludes(source_excludes)
  ///    .source_includes(source_includes)
  ///    .allow_no_indices(allow_no_indices)
  ///    .analyze_wildcard(analyze_wildcard)
  ///    .analyzer(analyzer)
  ///    .conflicts(conflicts)
  ///    .default_operator(default_operator)
  ///    .df(df)
  ///    .expand_wildcards(expand_wildcards)
  ///    .from(from)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .lenient(lenient)
  ///    .max_docs(max_docs)
  ///    .pipeline(pipeline)
  ///    .preference(preference)
  ///    .q(q)
  ///    .refresh(refresh)
  ///    .request_cache(request_cache)
  ///    .requests_per_second(requests_per_second)
  ///    .routing(routing)
  ///    .scroll(scroll)
  ///    .scroll_size(scroll_size)
  ///    .search_timeout(search_timeout)
  ///    .search_type(search_type)
  ///    .size(size)
  ///    .slices(slices)
  ///    .sort(sort)
  ///    .stats(stats)
  ///    .terminate_after(terminate_after)
  ///    .timeout(timeout)
  ///    .version(version)
  ///    .wait_for_active_shards(wait_for_active_shards)
  ///    .wait_for_completion(wait_for_completion)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn update_by_query(&self) -> builder::UpdateByQuery {
    builder::UpdateByQuery::new(self)
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
  /// let response = client.indices_get_upgrade_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_get_upgrade_with_index(&self) -> builder::IndicesGetUpgradeWithIndex {
    builder::IndicesGetUpgradeWithIndex::new(self)
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
  /// let response = client.indices_upgrade_with_index()
  ///    .index(index)
  ///    .allow_no_indices(allow_no_indices)
  ///    .expand_wildcards(expand_wildcards)
  ///    .ignore_unavailable(ignore_unavailable)
  ///    .only_ancient_segments(only_ancient_segments)
  ///    .wait_for_completion(wait_for_completion)
  ///    .send()
  ///    .await;
  /// ```
  pub fn indices_upgrade_with_index(&self) -> builder::IndicesUpgradeWithIndex {
    builder::IndicesUpgradeWithIndex::new(self)
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
  /// let response = client.indices_validate_query_get_with_index()
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
  pub fn indices_validate_query_get_with_index(&self) -> builder::IndicesValidateQueryGetWithIndex {
    builder::IndicesValidateQueryGetWithIndex::new(self)
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
  /// let response = client.indices_validate_query_post_with_index()
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
  pub fn indices_validate_query_post_with_index(&self) -> builder::IndicesValidateQueryPostWithIndex {
    builder::IndicesValidateQueryPostWithIndex::new(self)
  }
}

pub mod prelude {
  pub use self::super::Client;
}
