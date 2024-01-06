pub extern crate url;

mod client;
mod credentials;
mod auth_middleware;
pub mod bulker;

#[cfg(feature = "cat")]
mod cat;
#[cfg(feature = "cluster")]
mod cluster;
#[cfg(feature = "indices")]
pub mod indices;
#[cfg(feature = "ingest")]
mod ingest;
#[cfg(feature = "nodes")]
mod nodes;
#[cfg(feature = "ml")]
mod ml;
#[cfg(feature = "mtermvectors")]
mod mtermvectors;
#[cfg(feature = "remote")]
mod remote;
#[cfg(feature = "security")]
mod security;
#[cfg(feature = "snapshot")]
mod snapshot;
#[cfg(feature = "tasks")]
mod tasks;
#[cfg(feature = "tools")]
mod tools;

use std::sync::{Arc, Mutex};

use bulker::{Bulker, BulkerBuilder};
pub use opensearch_dsl as dsl;
use opensearch_dsl::{Query, Search, SortCollection, Terms};
#[allow(unused_imports)]
use client::{encode_path, encode_path_option_vec_string, RequestBuilderExt};
pub use client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{de::DeserializeOwned, Serialize};
use tokio::task::JoinHandle;
use tracing::info;
use types::bulk::{BulkAction, BulkResponse, CreateAction, DeleteAction, IndexAction, UpdateAction, UpdateActionBody};
use futures::{
  stream::{self, StreamExt},
  Stream,
};
pub mod types;
pub mod builder;

#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
#[cfg(target_arch = "wasm32")]
use reqwest::Client;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::ClientBuilder;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::{NoProxy, Proxy};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use url::Url;
use client::ReqwestResponse;

#[cfg(not(target_arch = "wasm32"))]
use crate::{auth_middleware::AuthMiddleware, credentials::Credentials};

#[derive(Clone, Debug)]
pub struct OsClientBuilder {
  baseurl: Url,
  retries: u32,
  credentials: HashMap<String, Credentials>,
  accept_invalid_certificates: bool,
  max_bulk_size: u32,
  #[cfg(not(target_arch = "wasm32"))]
  cache: Option<PathBuf>,
  #[cfg(not(target_arch = "wasm32"))]
  proxy: bool,
  #[cfg(not(target_arch = "wasm32"))]
  proxy_url: Option<Proxy>,
  #[cfg(not(target_arch = "wasm32"))]
  no_proxy_domain: Option<String>,
}

impl Default for OsClientBuilder {
  fn default() -> Self {
    Self {
      baseurl: Url::parse("http://localhost:9200").unwrap(),
      credentials: HashMap::new(),
      accept_invalid_certificates: false,
      max_bulk_size: 200,
      #[cfg(not(target_arch = "wasm32"))]
      cache: None,
      #[cfg(not(target_arch = "wasm32"))]
      proxy: false,
      #[cfg(not(target_arch = "wasm32"))]
      proxy_url: None,
      #[cfg(not(target_arch = "wasm32"))]
      no_proxy_domain: None,
      #[cfg(not(test))]
      retries: 2,
      #[cfg(test)]
      retries: 0,
    }
  }
}

impl OsClientBuilder {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn base_url(mut self, baseurl: Url) -> Self {
    self.baseurl = baseurl;
    self
  }

  pub fn accept_invalid_certificates(mut self, accept_invalid_certificates: bool) -> Self {
    self.accept_invalid_certificates = accept_invalid_certificates;
    self
  }

  pub fn max_bulk_size(mut self, max_bulk_size: u32) -> Self {
    self.max_bulk_size = max_bulk_size;
    self
  }

  pub fn basic_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
    self.credentials.insert(
      auth_middleware::nerf_dart(&self.baseurl),
      Credentials::Basic {
        username: username.into(),
        password: Some(password.into()),
      },
    );
    self
  }

  pub fn token_auth(mut self, token: impl Into<String>) -> Self {
    self.credentials.insert(
      auth_middleware::nerf_dart(&self.baseurl),
      Credentials::Token(token.into()),
    );
    self
  }

  pub fn legacy_auth(mut self, legacy_auth_token: impl Into<String>) -> Self {
    self.credentials.insert(
      auth_middleware::nerf_dart(&self.baseurl),
      Credentials::EncodedBasic(legacy_auth_token.into()),
    );
    self
  }

  pub fn retries(mut self, retries: u32) -> Self {
    self.retries = retries;
    self
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn cache(mut self, cache: impl AsRef<Path>) -> Self {
    self.cache = Some(PathBuf::from(cache.as_ref()));
    self
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn proxy(mut self, proxy: bool) -> Self {
    self.proxy = proxy;
    self
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn proxy_url(mut self, proxy_url: impl AsRef<str>) -> Result<Self, Error> {
    match Url::parse(proxy_url.as_ref()) {
      Ok(url_info) => {
        let username = url_info.username();
        let password = url_info.password();
        let mut proxy = Proxy::all(url_info.as_ref())?;

        if let Some(password_str) = password {
          proxy = proxy.basic_auth(username, password_str);
        }

        proxy = proxy.no_proxy(self.get_no_proxy_domain());
        self.proxy_url = Some(proxy);
        self.proxy = true;
        Ok(self)
      }
      Err(e) => Err(Error::UrlParseError(e)),
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn no_proxy_domain(mut self, no_proxy_domain: impl AsRef<str>) -> Self {
    self.no_proxy_domain = Some(no_proxy_domain.as_ref().into());
    self
  }

  pub fn build(self) -> OsClient {
    #[cfg(target_arch = "wasm32")]
    let client_raw = {
      let mut client_core = ClientBuilder::new();
      if self.accept_invalid_certificates {
        let mut builder = client_raw.clone().builder();
        builder = builder.danger_accept_invalid_certs(true);
        builder = builder.danger_accept_invalid_hostnames(true);
        client_raw = builder.build().unwrap();
      }
      client_core.build().expect("Fail to build HTTP client.")
    };

    #[cfg(not(target_arch = "wasm32"))]
    let client_raw = {
      let mut client_core = ClientBuilder::new()
        .user_agent("opensearch-client/0.1.0")
        .pool_max_idle_per_host(20)
        .timeout(std::time::Duration::from_secs(60 * 5));

      if let Some(url) = self.proxy_url {
        client_core = client_core.proxy(url);
      }

      if !self.proxy {
        client_core = client_core.no_proxy();
      }
      if self.accept_invalid_certificates {
        client_core = client_core.danger_accept_invalid_certs(true);
        client_core = client_core.danger_accept_invalid_hostnames(true);
      }

      client_core.build().expect("Fail to build HTTP client.")
    };

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(self.retries);
    let retry_strategy = RetryTransientMiddleware::new_with_policy(retry_policy);
    let credentials = Arc::new(self.credentials);

    #[allow(unused_mut)]
    let mut client_builder = reqwest_middleware::ClientBuilder::new(client_raw.clone())
      .with(retry_strategy)
      .with(AuthMiddleware(credentials.clone()));

    #[cfg(not(target_arch = "wasm32"))]
    if let Some(cache_loc) = self.cache {
      client_builder = client_builder.with(Cache(HttpCache {
        mode: CacheMode::Default,
        manager: CACacheManager { path: cache_loc },
        options: HttpCacheOptions::default(),
      }));
    }

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(self.retries);
    let retry_strategy = RetryTransientMiddleware::new_with_policy(retry_policy);

    let client_uncached_builder = reqwest_middleware::ClientBuilder::new(client_raw)
      .with(retry_strategy)
      .with(AuthMiddleware(credentials));

    OsClient {
      baseurl: Arc::new(self.baseurl),
      client: client_builder.build(),
      client_uncached: client_uncached_builder.build(),
      bulker: Arc::new(Mutex::new(String::new())),
      bulker_size: Arc::new(Mutex::new(0)),
      max_bulk_size: self.max_bulk_size,
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  fn get_no_proxy_domain(&self) -> Option<NoProxy> {
    if let Some(ref no_proxy_conf) = self.no_proxy_domain {
      if !no_proxy_conf.is_empty() {
        return NoProxy::from_string(no_proxy_conf);
      }
    }

    NoProxy::from_env().or(None)
  }
}

#[derive(Clone, Debug)]
///Client for OpenSearch
///
///Version: 2021-11-23
pub struct OsClient {
  pub(crate) baseurl: Arc<Url>,
  pub(crate) client: ClientWithMiddleware,
  pub(crate) client_uncached: ClientWithMiddleware,
  pub(crate) bulker: Arc<Mutex<String>>,
  pub(crate) bulker_size: Arc<Mutex<u32>>,
  pub(crate) max_bulk_size: u32,
}

pub trait Request {
  type Response: DeserializeOwned + Send + Sync;
  fn method(&self) -> reqwest::Method;
  fn path(&self) -> Result<String, Error>;
  fn body(&self) -> Result<Option<String>, Error>;
  fn query_args(&self) -> Result<Option<HashMap<String, String>>, Error>;
  fn url(&self, base_url: &Url) -> Result<Url, Error> {
    let mut url = base_url.clone();
    url.set_path(&self.path()?);
    if let Some(query_args) = self.query_args()? {
      url.query_pairs_mut().clear().extend_pairs(query_args.iter());
    }
    Ok(url)
  }
}

impl OsClient {
  /// Create a new client.
  ///
  /// `baseurl` is the base URL provided to the internal
  /// `reqwest::Client`, and should include a scheme and hostname,
  /// as well as port and a path stem if applicable.
  pub fn new(baseurl: Url) -> Self {
    let builder = OsClientBuilder::new().base_url(baseurl);
    builder.build()
  }

  pub fn from_environment() -> Result<OsClient, Error> {
    let accept_invalid_certificates: bool = match std::env::var("OPENSEARCH_SSL_VERIFY") {
      Ok(value) => value.eq_ignore_ascii_case("false"),
      Err(_) => false,
    };
    let user: String = match std::env::var("OPENSEARCH_USER") {
      Ok(user) => user,
      Err(_) => "admin".into(),
    };
    let password: String = match std::env::var("OPENSEARCH_PASSWORD") {
      Ok(password) => password,
      Err(_) => "admin".into(),
    };

    let server = match std::env::var("OPENSEARCH_URL") {
      Ok(server) => server,
      Err(_) => "https://localhost:9200".into(),
    };

    let mut builder = OsClientBuilder::new().base_url(Url::parse(&server)?);
    if accept_invalid_certificates {
      builder = builder.accept_invalid_certificates(true);
    }
    builder = builder.basic_auth(user, password);

    if let Ok(max_bulk_size) = std::env::var("OPENSEARCH_MAX_BULK_SIZE") {
      match max_bulk_size.parse::<u32>() {
        Ok(max_bulk_size) => builder = builder.max_bulk_size(max_bulk_size),
        Err(_) => info!("Invalid value for OPENSEARCH_MAX_BULK_SIZE, using default"),
      }
    };

    Ok(builder.build())
  }

  #[cfg(feature = "cat")]
  pub fn cat(&self) -> cat::Cat {
    cat::Cat::new(&self)
  }

  #[cfg(feature = "cluster")]
  pub fn cluster(&self) -> cluster::Cluster {
    cluster::Cluster::new(&self)
  }

  #[cfg(feature = "indices")]
  pub fn indices(&self) -> indices::Indices {
    indices::Indices::new(&self)
  }

  #[cfg(feature = "ingest")]
  pub fn ingest(&self) -> ingest::Ingest {
    ingest::Ingest::new(&self)
  }

  #[cfg(feature = "mtermvectors")]
  pub fn mtermvectors(&self) -> mtermvectors::Mtermvectors {
    mtermvectors::Mtermvectors::new(&self)
  }

  #[cfg(feature = "ml")]
  pub fn ml(&self) -> ml::ML {
    ml::ML::new(&self)
  }

  #[cfg(feature = "nodes")]
  pub fn nodes(&self) -> nodes::Nodes {
    nodes::Nodes::new(&self)
  }

  #[cfg(feature = "remote")]
  pub fn remote(&self) -> remote::Remote {
    remote::Remote::new(&self)
  }

  #[cfg(feature = "security")]
  pub fn security(&self) -> security::Security {
    security::Security::new(&self)
  }

  #[cfg(feature = "snapshot")]
  pub fn snapshot(&self) -> snapshot::Snapshot {
    snapshot::Snapshot::new(&self)
  }

  #[cfg(feature = "tasks")]
  pub fn tasks(&self) -> tasks::Tasks {
    tasks::Tasks::new(&self)
  }

  #[cfg(feature = "tools")]
  pub fn tools(&self) -> tools::Tools {
    tools::Tools::new(&self)
  }

  /// Get the base URL to which requests are made.
  pub fn baseurl(&self) -> &Url {
    &self.baseurl
  }

  /// Get the internal `reqwest_middleware::ClientWithMiddleware` used to make
  /// requests.
  pub fn client(&self) -> &reqwest_middleware::ClientWithMiddleware {
    &self.client
  }

  /// Get the version of this API.
  ///
  /// This string is pulled directly from the source OpenAPI
  /// document and may be in any format the API selects.
  pub fn api_version(&self) -> &'static str {
    "2021-11-23"
  }

  pub async fn send<T: Request + Serialize>(&self, request: T) -> Result<ResponseValue<T::Response>, Error> {
    let body = request.body()?;
    let url = request.url(self.baseurl.clone().as_ref())?;
    let mut request_builder = self.client.request(request.method(), url);
    if let Some(body) = body {
      request_builder = request_builder.body(body);
    }
    let response = request_builder.send().await?;
    match response.status().as_u16() {
      200u16 => ResponseValue::from_response(response).await,
      _ => {
        Err(Error::UnexpectedResponse(
          ReqwestResponse::from_response(response).await,
        ))
      }
    }
  }
}

impl OsClient {
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
  pub fn bulk(&self) -> builder::BulkPost {
    builder::BulkPost::new(self)
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
  /// let response = client.count()
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
  pub fn count(&self) -> builder::Count {
    builder::Count::new(self)
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
  pub fn index_post(&self) -> builder::IndexPost {
    builder::IndexPost::new(self)
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
  /// let response = client.explain()
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
  pub fn explain(&self) -> builder::ExplainPost {
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
  /// let response = client.mget_with_index()
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
  pub fn mget_with_index(&self) -> builder::MgetWithIndex {
    builder::MgetWithIndex::new(self)
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
  /// let response = client.msearch_template_with_index()
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
  pub fn msearch_template_with_index(&self) -> builder::MsearchTemplatePostWithIndex {
    builder::MsearchTemplatePostWithIndex::new(self)
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
  pub fn search(&self) -> builder::SearchPostWithIndex {
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
  /// let response = client.search_template_with_index()
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
  pub fn search_template_with_index(&self) -> builder::SearchTemplatePostWithIndex {
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

  /// Sends a bulk index request to OpenSearch with the specified index, id and
  /// document body.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index.
  /// * `id` - An optional string slice that holds the id of the document.
  /// * `body` - A reference to a serializable document body.
  ///
  /// # Returns
  ///
  /// Returns a Result containing a serde_json::Value representing the response
  /// from OpenSearch or an Error if the request fails.
  ///
  /// # Example
  ///
  /// ```
  /// use opensearch_client::OpenSearchClient;
  ///
  /// #[derive(Serialize)]
  /// struct MyDocument {
  ///   title: String,
  ///   content: String,
  /// }
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let client = OpenSearchClient::new("http://localhost:9200");
  ///   let document = MyDocument {
  ///     title: "My Title".to_string(),
  ///     content: "My Content".to_string(),
  ///   };
  ///   let response = client
  ///     .bulk_index_document("my_index", Some("my_id".to_string()), &document)
  ///     .await?;
  ///   Ok(())
  /// }
  /// ```
  pub async fn bulk_index_document<T: Serialize>(
    &self,
    index: &str,
    id: Option<String>,
    body: &T,
  ) -> Result<(), Error> {
    let body_json = serde_json::to_value(body)?;
    let action = BulkAction::Index(IndexAction {
      index: index.to_owned(),
      id: id.clone(),
      pipeline: None,
    });
    self.bulk_action(action, Some(&body_json)).await
  }

  /// Sends a bulk action to the OpenSearch server.
  ///
  /// # Arguments
  ///
  /// * `command` - A string slice that holds the command to be executed.
  /// * `action` - A `BulkAction` enum that specifies the action to be taken.
  /// * `body` - An optional `serde_json::Value` that holds the request body.
  ///
  /// # Returns
  ///
  /// A `Result` containing a `serde_json::Value` object representing the
  /// response from the server, or an `Error` if the request failed.
  ///
  /// # Examples
  ///
  /// ```
  /// use opensearch_client::{BulkAction, OpenSearchClient};
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let client = OpenSearchClient::new("http://localhost:9200")?;
  ///   let action = BulkAction::Index {
  ///     index: "my_index".to_string(),
  ///     id: Some("1".to_string()),
  ///   };
  ///   let response = client.bulk_action("index", action, None).await?;
  ///   Ok(())
  /// }
  /// ```
  pub async fn bulk_action(&self, action: BulkAction, body: Option<&serde_json::Value>) -> Result<(), Error> {
    let j = serde_json::to_string(&action)?;
    let bulker_arc = Arc::clone(&self.bulker);
    let mut bulker = bulker_arc.lock().unwrap();
    bulker.push_str(j.as_str());
    bulker.push('\n');
    match body {
      None => {}
      Some(js) => {
        let j = serde_json::to_string(js)?;
        bulker.push_str(j.as_str());
        bulker.push('\n');
      }
    }

    let bulker_size_arc = Arc::clone(&self.bulker_size);
    let mut bulker_size = bulker_size_arc.lock().unwrap();
    *bulker_size += 1;
    if *bulker_size >= self.max_bulk_size {
      drop(bulker_size);
      drop(bulker);
      self.flush_bulk().await?;
    }
    Ok(())
  }

  /// Sends a bulk create request to the OpenSearch cluster with the specified
  /// index, id and body.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index.
  /// * `id` - A string slice that holds the id of the document.
  /// * `body` - A generic type `T` that holds the body of the document to be
  ///   created.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing a `serde_json::Value` on success, or an
  /// `Error` on failure.
  pub async fn bulk_create_document<T: Serialize>(&self, index: &str, id: &str, body: &T) -> Result<(), Error> {
    let body_json = serde_json::to_value(body)?;

    let action = BulkAction::Create(CreateAction {
      index: index.to_owned(),
      id: id.to_owned(),
      ..Default::default()
    });

    self.bulk_action(action, Some(&body_json)).await
  }

  /// Asynchronously updates a document in bulk.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index.
  /// * `id` - A string slice that holds the ID of the document to update.
  /// * `body` - An `UpdateAction` struct that holds the update action to
  ///   perform.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing a `serde_json::Value` on success, or an
  /// `Error` on failure.
  pub async fn bulk_update_document(&self, index: &str, id: &str, body: &UpdateActionBody) -> Result<(), Error> {
    let action = BulkAction::Update(UpdateAction {
      index: index.to_owned(),
      id: id.to_owned(),
      ..Default::default()
    });
    let j = serde_json::to_value(body)?;
    self.bulk_action(action, Some(&j)).await
  }

  /// Sends a bulk request to the OpenSearch server and returns a
  /// `BulkResponse`. If the bulker size is 0, it returns an empty
  /// `BulkResponse`. If the bulk request contains errors, it logs the errors
  /// and returns the `BulkResponse`.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use opensearch_client::OpenSearchClient;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let client = OpenSearchClient::new("http://localhost:9200", "user", "password");
  ///   let response = client.flush_bulk().await?;
  ///   println!("{:?}", response);
  ///   Ok(())
  /// }
  /// ```
  pub async fn flush_bulk(&self) -> Result<BulkResponse, Error> {
    let bulker_size_arc = Arc::clone(&self.bulker_size);
    let mut bulker_size = bulker_size_arc.lock().unwrap();
    if *bulker_size > 0 {
      let bulker_arc = Arc::clone(&self.bulker);
      let mut bulker = bulker_arc.lock().unwrap();

      // let request_url = format!("{}_bulk", self.server);

      match self.bulk().body(bulker.to_owned()).send().await
        // .client
        // .post(request_url)
        // .basic_auth(self.user.as_str(), Some(self.password.as_str()))
        // .body()
        // .header("Content-Type", "application/x-ndjson")
        // .send()
        // .await
      {
        Ok(response) => {
          let result: BulkResponse = response.into_inner();
          *bulker = String::new();
          *bulker_size = 0;
          // debug!("{:?}", &result);
          if result.errors {
            for map in &result.items {
              for (_, value) in map.iter() {
                if let Some(error) = &value.error {
                  if !error.kind.eq_ignore_ascii_case("version_conflict_engine_exception") {
                    info!("{:?}", &value);
                  }
                }
              }
            }
          }

          Ok(result)
        }
        Err(err) => {
          println!("{:?}", &err);
          Err(err)
        }
      }
    } else {
      Ok(BulkResponse {
        took: 0,
        errors: false,
        items: vec![],
      })
    }
  }

  /// Indexes a document in the specified index with the given body and optional
  /// ID.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index to which the
  ///   document will be added.
  /// * `body` - A reference to a serializable object that represents the
  ///   document to be added.
  /// * `id` - An optional string slice that holds the ID of the document to be
  ///   added. If not provided, a new ID will be generated.
  ///
  /// # Returns
  ///
  /// A `Result` containing an `IndexResponse` object if the operation was
  /// successful, or an `Error` if an error occurred.
  ///
  /// # Examples
  ///
  /// ```
  /// use opensearch_client::Client;
  ///
  /// #[derive(Serialize)]
  /// struct MyDocument {
  ///   title: String,
  ///   content: String,
  /// }
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let client = Client::new("http://localhost:9200")?;
  ///
  ///   let document = MyDocument {
  ///     title: "My Title".to_string(),
  ///     content: "My Content".to_string(),
  ///   };
  ///
  ///   let response = client.index_document("my_index", &document, None).await?;
  ///
  ///   println!("Document ID: {}", response._id);
  ///
  ///   Ok(())
  /// }
  /// ```
  pub async fn index_document<T: Serialize>(
    &self,
    index: &String,
    body: &T,
    id: Option<String>,
  ) -> Result<types::IndexResponse, Error> {
    let body_json = serde_json::to_value(body)?;
    let partial_request = self.index_post().index(index).body(body_json);
    let request = match id {
      None => partial_request,
      Some(id) => partial_request.id(id),
    };
    let response = request.send().await?;
    Ok(response.into_inner())
  }

  /// Creates a new document in the specified index with the given ID and body.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index.
  /// * `id` - A string slice that holds the ID of the document.
  /// * `body` - A generic type `T` that holds the body of the document. The
  ///   type `T` must implement the `Serialize` trait from the `serde` crate.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing an `IndexResponse` on success, or an `Error`
  /// on failure.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use opensearch_client::OpenSearchClient;
  ///
  /// #[derive(Serialize)]
  /// struct MyDocument {
  ///   title: String,
  ///   content: String,
  /// }
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let client = OpenSearchClient::new("http://localhost:9200")?;
  ///
  ///   let document = MyDocument {
  ///     title: "My Title".to_string(),
  ///     content: "My Content".to_string(),
  ///   };
  ///
  ///   let response = client.create_document("my_index", "1", &document).await?;
  ///
  ///   Ok(())
  /// }
  /// ```
  pub async fn create_document<T: Serialize>(
    &self,
    index: &String,
    id: &String,
    body: &T,
  ) -> Result<types::IndexResponse, Error> {
    let body_json = serde_json::to_value(body)?;

    let response = self.create_put().index(index).id(id).body(body_json).send().await?;
    let result = response.into_inner();

    Ok(result)
  }

  /// Asynchronously retrieves a typed document from the specified index and ID.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index to retrieve
  ///   the document from.
  /// * `id` - A string slice that holds the ID of the document to retrieve.
  ///
  /// # Returns
  ///
  /// A `Result` containing the deserialized content of the retrieved document,
  /// or an `Error` if the operation failed.
  ///
  /// # Generic Type Parameters
  ///
  /// * `T` - The type of the document to retrieve. Must implement the
  ///   `DeserializeOwned` and
  /// `std::default::Default` traits.
  pub async fn get_typed<T: DeserializeOwned + std::default::Default>(
    &self,
    index: &str,
    id: &str,
  ) -> Result<types::GetResponseContent<T>, Error> {
    let response = self.get().index(index).id(id).send::<T>().await?;
    let result = response.into_inner();

    Ok(result)
  }

  /// Updates a document in the specified index with the given ID using the
  /// provided update action.
  ///
  /// # Arguments
  ///
  /// * `index` - A string slice that holds the name of the index to update the
  ///   document in.
  /// * `id` - A string slice that holds the ID of the document to update.
  /// * `action` - A reference to an `UpdateAction` enum that specifies the
  ///   update action to perform.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing an `IndexResponse` struct if the update was
  /// successful, or an `Error` if an error occurred.
  ///
  /// # Example
  ///
  /// ```rust
  /// use opensearch_client::{Client, UpdateAction};
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///     let client = Client::new("http://localhost:9200")?;
  ///
  ///     let index = "my_index";
  ///     let id = "1";
  ///     let action = UpdateAction::new().doc(json!({"foo": "bar"}));
  ///
  ///     let response = client.update_document(index, id, &action).await?;
  ///
  ///     Ok(())
  /// }
  /// ```
  pub async fn update_document(
    &self,
    index: &str,
    id: &str,
    action: &UpdateAction,
  ) -> Result<types::IndexResponse, Error> {
    let body = serde_json::to_value(&action)?;

    let response = self.update().body(body).index(index).id(id).send().await?;
    Ok(response.into_inner())
  }

  pub fn get_bulker(&self, bulk_size: u32, max_concurrent_connections: u32) -> (JoinHandle<()>, Bulker) {
    Bulker::new(Arc::new(self.clone()), bulk_size, max_concurrent_connections)
  }

  pub fn bulker(&self) -> BulkerBuilder {
    BulkerBuilder::new(Arc::new(self.clone()), self.max_bulk_size)
  }

  pub async fn search_typed<T: DeserializeOwned + std::default::Default>(
    &self,
    index: &str,
    search: Search,
  ) -> Result<types::SearchResult<T>, Error> {
    let response = self.search().index(index).body(search).send().await?;
    let result = response.into_inner();
    Ok(result)
  }

  /// Searches for documents in the specified index and returns a stream of
  /// hits.
  ///
  /// # Arguments
  ///
  /// * `index` - The name of the index to search in.
  /// * `query` - The query to execute.
  /// * `sort` - The sort criteria to use.
  /// * `size` - The maximum number of hits to return.
  ///
  /// # Returns
  ///
  /// A stream of hits that match the specified search criteria.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use opensearch_client::{Client, Query, SortCollection};
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
  ///   let client = Client::new("http://localhost:9200")?;
  ///   let query = Query::match_all();
  ///   let sort = SortCollection::new().add_field("_doc", "asc");
  ///   let stream = client.search_stream("my_index", &query, &sort, 10).await?;
  ///   stream
  ///     .for_each(|hit| {
  ///       println!("{:?}", hit);
  ///       futures::future::ready(())
  ///     })
  ///     .await;
  ///   Ok(())
  /// }
  /// ```
  pub async fn search_stream<T: DeserializeOwned + std::default::Default>(
    &self,
    index: &str,
    query: &Query,
    sort: &SortCollection,
    size: u64,
  ) -> Result<impl Stream<Item = types::Hit<T>> + 'static, Error> {
    let start_state = SearchAfterState {
      client: Arc::new(self.clone()),
      stop: false,
      search_after: None,
      index: index.to_owned(),
      query: query.clone(),
      sort: sort.clone(),
      size,
    };

    async fn stream_next<T: DeserializeOwned + std::default::Default>(
      state: SearchAfterState,
    ) -> Result<(Vec<types::Hit<T>>, SearchAfterState), Error> {
      let mut body: Search = Search::new()
        .size(state.size)
        .query(state.query.clone())
        .sort(state.sort.clone());

      if let Some(search_after) = state.search_after.clone() {
        body = body.search_after(search_after.clone());
      }
      let response = state
        .client
        .clone()
        .search()
        .index(&state.index)
        .body(body)
        .send::<T>()
        .await?;
      let hits = response.into_inner().hits.hits;
      let next_state = SearchAfterState {
        stop: (hits.len() as u64) < state.size,
        search_after: hits.iter().last().and_then(|f| {
          if let Some(last_sort) = f.sort.clone().unwrap().as_array() {
            if last_sort.is_empty() {
              None
            } else {
              Some(Terms::from(last_sort))
            }
          } else {
            None
          }
        }),
        ..state
      };

      Ok((hits, next_state))
    }

    let stream = stream::unfold(start_state, move |state| {
      async move {
        if state.stop {
          None
        } else {
          let result = stream_next::<T>(state).await;
          match result {
            Ok((items, state)) => Some((stream::iter(items), state)),
            Err(_err) => None,
          }
        }
      }
    });

    Ok(stream.flatten())
  }
}

/// Represents the state of a search operation that uses the "search after"
/// feature.
struct SearchAfterState {
  client: Arc<OsClient>,
  index: String,
  stop: bool,
  size: u64,
  query: Query,
  sort: SortCollection,
  search_after: Option<Terms>,
}

pub mod prelude {
  pub use self::super::OsClient;
}

#[cfg(test)]
mod tests {

  use std::path::PathBuf;

  use serde::de::DeserializeOwned;

  use super::*;
  fn load_entity<T: DeserializeOwned>(name: &str) -> T {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/base/{name}"));
    let text = std::fs::read_to_string(filename).unwrap();
    serde_json::from_str(&text).unwrap()
  }

  #[test]
  fn test_document_delete_response() {
    let decoded: types::DocumentDeleteResponse = load_entity("document_delete.response.json");
    assert_eq!(decoded.id, String::from("MzcIJX8BA7mbufL6DOwl"));
  }
}
