use reqwest::Body;
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "search")]
use opensearch_dsl::Search;
use tracing::debug;

use crate::types::bulk::BulkResponse;
use super::types;
#[allow(unused_imports)]
use super::{
  encode_path, encode_path_option_vec_string, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
  ReqwestResponse, ResponseValue,
};

///Builder for [`Client::count`]
///
///[`Client::count`]: super::OsClient::count
#[derive(Debug, Clone)]
pub struct Count<'a> {
  client: &'a super::OsClient,
  index: Result<Vec<types::OpenSearchNameValue>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  default_operator: Result<Option<types::DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  ignore_throttled: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  lenient: Result<Option<bool>, String>,
  min_score: Result<Option<i32>, String>,
  preference: Result<Option<String>, String>,
  q: Result<Option<String>, String>,
  routing: Result<Option<Vec<String>>, String>,
  terminate_after: Result<Option<i32>, String>,
  body: Result<Search, String>,
}

impl<'a> Count<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      allow_no_indices: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      expand_wildcards: Ok(None),
      ignore_throttled: Ok(None),
      ignore_unavailable: Ok(None),
      lenient: Ok(None),
      min_score: Ok(None),
      preference: Ok(None),
      q: Ok(None),
      routing: Ok(None),
      terminate_after: Ok(None),
      body: Ok(Search::default()),
    }
  }

  pub fn indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<types::OpenSearchNameValue>>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string());
    self
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::OpenSearchNameValue>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `OpenSearchNameValue` for index failed".to_string())
      .map(|i| vec![i]);
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
    self
  }

  pub fn analyze_wildcard<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.analyze_wildcard = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for analyze_wildcard failed".to_string());
    self
  }

  pub fn analyzer<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.analyzer = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for analyzer failed".to_string());
    self
  }

  pub fn default_operator<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::DefaultOperator>, {
    self.default_operator = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `DefaultOperator` for default_operator failed".to_string());
    self
  }

  pub fn df<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.df = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for df failed".to_string());
    self
  }

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn ignore_throttled<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_throttled = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_throttled failed".to_string());
    self
  }

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn lenient<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.lenient = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for lenient failed".to_string());
    self
  }

  pub fn min_score<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.min_score = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for min_score failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn q<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.q = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for q failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for routing failed".to_string());
    self
  }

  pub fn terminate_after<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.terminate_after = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for terminate_after failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Search>, {
    self.body = value
      .try_into()
      .map_err(|_| "conversion to `CountBodyParams` for body failed".to_string());
    self
  }

  ///Sends a `POST` request to `/{index}/_count`
  pub async fn send(self) -> Result<ResponseValue<types::CountResponse>, Error> {
    let Self {
      client,
      index,
      allow_no_indices,
      analyze_wildcard,
      analyzer,
      default_operator,
      df,
      expand_wildcards,
      ignore_throttled,
      ignore_unavailable,
      lenient,
      min_score,
      preference,
      q,
      routing,
      terminate_after,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let ignore_throttled = ignore_throttled.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let min_score = min_score.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let terminate_after = terminate_after.map_err(Error::InvalidRequest)?;
    let body = body.map_err(Error::InvalidRequest)?;
    let indices = index.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
    let url = if indices.len() > 0 {
      format!("{}{}/_count", client.baseurl, encode_path(&indices),)
    } else {
      format!("{}_count", client.baseurl)
    };
    let mut query = Vec::with_capacity(14usize);
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &analyze_wildcard {
      query.push(("analyze_wildcard", v.to_string()));
    }
    if let Some(v) = &analyzer {
      query.push(("analyzer", v.to_string()));
    }
    if let Some(v) = &default_operator {
      query.push(("default_operator", v.to_string()));
    }
    if let Some(v) = &df {
      query.push(("df", v.to_string()));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &ignore_throttled {
      query.push(("ignore_throttled", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &min_score {
      query.push(("min_score", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.join(",")));
    }
    if let Some(v) = &terminate_after {
      query.push(("terminate_after", v.to_string()));
    }
    let request = client.client.post(url).json(&body).query(&query).build()?;
    let result = client.client.execute(request).await;
    let response = result?;
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

///Builder for [`Client::search_post_with_index`]
///
///[`Client::search_post_with_index`]: super::OsClient::search_post_with_index
#[derive(Debug, Clone)]
pub struct SearchPostWithIndex<'a> {
  client: &'a super::OsClient,
  index: Result<types::IndexNames, String>,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  allow_partial_search_results: Result<Option<bool>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  batched_reduce_size: Result<Option<i32>, String>,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  default_operator: Result<Option<types::DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  docvalue_fields: Result<Option<Vec<String>>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  explain: Result<Option<bool>, String>,
  from: Result<Option<i32>, String>,
  ignore_throttled: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  lenient: Result<Option<bool>, String>,
  max_concurrent_shard_requests: Result<Option<i32>, String>,
  pre_filter_shard_size: Result<Option<i32>, String>,
  preference: Result<Option<String>, String>,
  q: Result<Option<String>, String>,
  request_cache: Result<Option<bool>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  routing: Result<Option<Vec<String>>, String>,
  scroll: Result<Option<types::SearchPostWithIndexScroll>, String>,
  search_type: Result<Option<types::SearchType>, String>,
  seq_no_primary_term: Result<Option<bool>, String>,
  size: Result<Option<i32>, String>,
  sort: Result<Option<Vec<String>>, String>,
  stats: Result<Option<Vec<String>>, String>,
  stored_fields: Result<Option<Vec<String>>, String>,
  suggest_field: Result<Option<String>, String>,
  suggest_mode: Result<Option<types::SuggestMode>, String>,
  suggest_size: Result<Option<i32>, String>,
  suggest_text: Result<Option<String>, String>,
  terminate_after: Result<Option<i32>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  track_scores: Result<Option<bool>, String>,
  track_total_hits: Result<Option<bool>, String>,
  typed_keys: Result<Option<bool>, String>,
  version: Result<Option<bool>, String>,
  body: Result<Option<Search>, String>,
}

impl<'a> SearchPostWithIndex<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      index: Err("index was not initialized".to_string()),
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      allow_no_indices: Ok(None),
      allow_partial_search_results: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      batched_reduce_size: Ok(None),
      ccs_minimize_roundtrips: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      docvalue_fields: Ok(None),
      expand_wildcards: Ok(None),
      explain: Ok(None),
      from: Ok(None),
      ignore_throttled: Ok(None),
      ignore_unavailable: Ok(None),
      lenient: Ok(None),
      max_concurrent_shard_requests: Ok(None),
      pre_filter_shard_size: Ok(None),
      preference: Ok(None),
      q: Ok(None),
      request_cache: Ok(None),
      rest_total_hits_as_int: Ok(None),
      routing: Ok(None),
      scroll: Ok(None),
      search_type: Ok(None),
      seq_no_primary_term: Ok(None),
      size: Ok(None),
      sort: Ok(None),
      stats: Ok(None),
      stored_fields: Ok(None),
      suggest_field: Ok(None),
      suggest_mode: Ok(None),
      suggest_size: Ok(None),
      suggest_text: Ok(None),
      terminate_after: Ok(None),
      timeout: Ok(None),
      track_scores: Ok(None),
      track_total_hits: Ok(None),
      typed_keys: Ok(None),
      version: Ok(None),
      body: Ok(None),
    }
  }

  pub fn index<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::IndexNames>, {
    self.index = value
      .try_into()
      .map_err(|_| "conversion to `IndexNames` for index failed".to_string());
    self
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
    self
  }

  pub fn allow_partial_search_results<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_partial_search_results = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_partial_search_results failed".to_string());
    self
  }

  pub fn analyze_wildcard<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.analyze_wildcard = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for analyze_wildcard failed".to_string());
    self
  }

  pub fn analyzer<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.analyzer = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for analyzer failed".to_string());
    self
  }

  pub fn batched_reduce_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.batched_reduce_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for batched_reduce_size failed".to_string());
    self
  }

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn default_operator<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::DefaultOperator>, {
    self.default_operator = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `DefaultOperator` for default_operator failed".to_string());
    self
  }

  pub fn df<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.df = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for df failed".to_string());
    self
  }

  pub fn docvalue_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.docvalue_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for docvalue_fields failed".to_string());
    self
  }

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn explain<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.explain = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for explain failed".to_string());
    self
  }

  pub fn from<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.from = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for from failed".to_string());
    self
  }

  pub fn ignore_throttled<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_throttled = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_throttled failed".to_string());
    self
  }

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn lenient<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.lenient = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for lenient failed".to_string());
    self
  }

  pub fn max_concurrent_shard_requests<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_shard_requests = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_shard_requests failed".to_string());
    self
  }

  pub fn pre_filter_shard_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.pre_filter_shard_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for pre_filter_shard_size failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn q<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.q = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for q failed".to_string());
    self
  }

  pub fn request_cache<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.request_cache = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for request_cache failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for routing failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchPostWithIndexScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchPostWithIndexScroll` for scroll failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchType>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchType` for search_type failed".to_string());
    self
  }

  pub fn seq_no_primary_term<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.seq_no_primary_term = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for seq_no_primary_term failed".to_string());
    self
  }

  pub fn size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for size failed".to_string());
    self
  }

  pub fn sort<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.sort = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for sort failed".to_string());
    self
  }

  pub fn stats<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stats = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stats failed".to_string());
    self
  }

  pub fn stored_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stored_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stored_fields failed".to_string());
    self
  }

  pub fn suggest_field<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.suggest_field = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for suggest_field failed".to_string());
    self
  }

  pub fn suggest_mode<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SuggestMode>, {
    self.suggest_mode = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SuggestMode` for suggest_mode failed".to_string());
    self
  }

  pub fn suggest_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.suggest_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for suggest_size failed".to_string());
    self
  }

  pub fn suggest_text<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.suggest_text = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for suggest_text failed".to_string());
    self
  }

  pub fn terminate_after<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.terminate_after = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for terminate_after failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn track_scores<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.track_scores = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for track_scores failed".to_string());
    self
  }

  pub fn track_total_hits<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.track_total_hits = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for track_total_hits failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for version failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Search>, {
    self.body = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchBodyParams` for body failed".to_string());
    self
  }

  // pub fn body_map<F>(mut self, f: F) -> Self
  // where
  //   F: std::ops::FnOnce(Search) -> Search, {
  //   self.body = self.body.map(f);
  //   self
  // }

  ///Sends a `POST` request to `/{index}/_search`
  pub async fn send<T: DeserializeOwned + std::default::Default>(
    self,
  ) -> Result<ResponseValue<types::SearchResult<T>>, Error> {
    let Self {
      client,
      index,
      source,
      source_excludes,
      source_includes,
      allow_no_indices,
      allow_partial_search_results,
      analyze_wildcard,
      analyzer,
      batched_reduce_size,
      ccs_minimize_roundtrips,
      default_operator,
      df,
      docvalue_fields,
      expand_wildcards,
      explain,
      from,
      ignore_throttled,
      ignore_unavailable,
      lenient,
      max_concurrent_shard_requests,
      pre_filter_shard_size,
      preference,
      q,
      request_cache,
      rest_total_hits_as_int,
      routing,
      scroll,
      search_type,
      seq_no_primary_term,
      size,
      sort,
      stats,
      stored_fields,
      suggest_field,
      suggest_mode,
      suggest_size,
      suggest_text,
      terminate_after,
      timeout,
      track_scores,
      track_total_hits,
      typed_keys,
      version,
      body,
    } = self;
    let index = index.map_err(Error::InvalidRequest)?;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let allow_partial_search_results = allow_partial_search_results.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let batched_reduce_size = batched_reduce_size.map_err(Error::InvalidRequest)?;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let docvalue_fields = docvalue_fields.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let from = from.map_err(Error::InvalidRequest)?;
    let ignore_throttled = ignore_throttled.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let max_concurrent_shard_requests = max_concurrent_shard_requests.map_err(Error::InvalidRequest)?;
    let pre_filter_shard_size = pre_filter_shard_size.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let request_cache = request_cache.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let seq_no_primary_term = seq_no_primary_term.map_err(Error::InvalidRequest)?;
    let size = size.map_err(Error::InvalidRequest)?;
    let sort = sort.map_err(Error::InvalidRequest)?;
    let stats = stats.map_err(Error::InvalidRequest)?;
    let stored_fields = stored_fields.map_err(Error::InvalidRequest)?;
    let suggest_field = suggest_field.map_err(Error::InvalidRequest)?;
    let suggest_mode = suggest_mode.map_err(Error::InvalidRequest)?;
    let suggest_size = suggest_size.map_err(Error::InvalidRequest)?;
    let suggest_text = suggest_text.map_err(Error::InvalidRequest)?;
    let terminate_after = terminate_after.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let track_scores = track_scores.map_err(Error::InvalidRequest)?;
    let track_total_hits = track_total_hits.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let body = match body.map_err(Error::InvalidRequest)? {
      Some(body) => body,
      None => Search::default(),
    };
    let url = format!("{}{}/_search", client.baseurl, encode_path(&index.to_string()),);
    let mut query = Vec::with_capacity(42usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &allow_partial_search_results {
      query.push(("allow_partial_search_results", v.to_string()));
    }
    if let Some(v) = &analyze_wildcard {
      query.push(("analyze_wildcard", v.to_string()));
    }
    if let Some(v) = &analyzer {
      query.push(("analyzer", v.to_string()));
    }
    if let Some(v) = &batched_reduce_size {
      query.push(("batched_reduce_size", v.to_string()));
    }
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &default_operator {
      query.push(("default_operator", v.to_string()));
    }
    if let Some(v) = &df {
      query.push(("df", v.to_string()));
    }
    if let Some(v) = &docvalue_fields {
      query.push(("docvalue_fields", v.join(",")));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &from {
      query.push(("from", v.to_string()));
    }
    if let Some(v) = &ignore_throttled {
      query.push(("ignore_throttled", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &max_concurrent_shard_requests {
      query.push(("max_concurrent_shard_requests", v.to_string()));
    }
    if let Some(v) = &pre_filter_shard_size {
      query.push(("pre_filter_shard_size", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &request_cache {
      query.push(("request_cache", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.join(",")));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &seq_no_primary_term {
      query.push(("seq_no_primary_term", v.to_string()));
    }
    if let Some(v) = &size {
      query.push(("size", v.to_string()));
    }
    if let Some(v) = &sort {
      query.push(("sort", v.join(",")));
    }
    if let Some(v) = &stats {
      query.push(("stats", v.join(",")));
    }
    if let Some(v) = &stored_fields {
      query.push(("stored_fields", v.join(",")));
    }
    if let Some(v) = &suggest_field {
      query.push(("suggest_field", v.to_string()));
    }
    if let Some(v) = &suggest_mode {
      query.push(("suggest_mode", v.to_string()));
    }
    if let Some(v) = &suggest_size {
      query.push(("suggest_size", v.to_string()));
    }
    if let Some(v) = &suggest_text {
      query.push(("suggest_text", v.to_string()));
    }
    if let Some(v) = &terminate_after {
      query.push(("terminate_after", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &track_scores {
      query.push(("track_scores", v.to_string()));
    }
    if let Some(v) = &track_total_hits {
      query.push(("track_total_hits", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    debug!("{}/_search\n{}", &url, serde_json::to_string(&body)?);

    let request = client
      .client
      .post(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .query(&query)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
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

///Builder for [`Client::search_post`]
///
///[`Client::search_post`]: super::OsClient::search_post
#[derive(Debug, Clone)]
pub struct SearchPost<'a> {
  client: &'a super::OsClient,
  source: Result<Option<Vec<String>>, String>,
  source_excludes: Result<Option<Vec<String>>, String>,
  source_includes: Result<Option<Vec<String>>, String>,
  allow_no_indices: Result<Option<bool>, String>,
  allow_partial_search_results: Result<Option<bool>, String>,
  analyze_wildcard: Result<Option<bool>, String>,
  analyzer: Result<Option<String>, String>,
  batched_reduce_size: Result<Option<i32>, String>,
  ccs_minimize_roundtrips: Result<Option<bool>, String>,
  default_operator: Result<Option<types::DefaultOperator>, String>,
  df: Result<Option<String>, String>,
  docvalue_fields: Result<Option<Vec<String>>, String>,
  expand_wildcards: Result<Option<types::ExpandWildcards>, String>,
  explain: Result<Option<bool>, String>,
  from: Result<Option<i32>, String>,
  ignore_throttled: Result<Option<bool>, String>,
  ignore_unavailable: Result<Option<bool>, String>,
  lenient: Result<Option<bool>, String>,
  max_concurrent_shard_requests: Result<Option<i32>, String>,
  pre_filter_shard_size: Result<Option<i32>, String>,
  preference: Result<Option<String>, String>,
  q: Result<Option<String>, String>,
  request_cache: Result<Option<bool>, String>,
  rest_total_hits_as_int: Result<Option<bool>, String>,
  routing: Result<Option<Vec<String>>, String>,
  scroll: Result<Option<types::SearchPostScroll>, String>,
  search_type: Result<Option<types::SearchType>, String>,
  seq_no_primary_term: Result<Option<bool>, String>,
  size: Result<Option<i32>, String>,
  sort: Result<Option<Vec<String>>, String>,
  stats: Result<Option<Vec<String>>, String>,
  stored_fields: Result<Option<Vec<String>>, String>,
  suggest_field: Result<Option<String>, String>,
  suggest_mode: Result<Option<types::SuggestMode>, String>,
  suggest_size: Result<Option<i32>, String>,
  suggest_text: Result<Option<String>, String>,
  terminate_after: Result<Option<i32>, String>,
  timeout: Result<Option<types::Timeout>, String>,
  track_scores: Result<Option<bool>, String>,
  track_total_hits: Result<Option<bool>, String>,
  typed_keys: Result<Option<bool>, String>,
  version: Result<Option<bool>, String>,
  body: Result<Option<Search>, String>,
}

impl<'a> SearchPost<'a> {
  pub fn new(client: &'a super::OsClient) -> Self {
    Self {
      client,
      source: Ok(None),
      source_excludes: Ok(None),
      source_includes: Ok(None),
      allow_no_indices: Ok(None),
      allow_partial_search_results: Ok(None),
      analyze_wildcard: Ok(None),
      analyzer: Ok(None),
      batched_reduce_size: Ok(None),
      ccs_minimize_roundtrips: Ok(None),
      default_operator: Ok(None),
      df: Ok(None),
      docvalue_fields: Ok(None),
      expand_wildcards: Ok(None),
      explain: Ok(None),
      from: Ok(None),
      ignore_throttled: Ok(None),
      ignore_unavailable: Ok(None),
      lenient: Ok(None),
      max_concurrent_shard_requests: Ok(None),
      pre_filter_shard_size: Ok(None),
      preference: Ok(None),
      q: Ok(None),
      request_cache: Ok(None),
      rest_total_hits_as_int: Ok(None),
      routing: Ok(None),
      scroll: Ok(None),
      search_type: Ok(None),
      seq_no_primary_term: Ok(None),
      size: Ok(None),
      sort: Ok(None),
      stats: Ok(None),
      stored_fields: Ok(None),
      suggest_field: Ok(None),
      suggest_mode: Ok(None),
      suggest_size: Ok(None),
      suggest_text: Ok(None),
      terminate_after: Ok(None),
      timeout: Ok(None),
      track_scores: Ok(None),
      track_total_hits: Ok(None),
      typed_keys: Ok(None),
      version: Ok(None),
      body: Ok(Some(Search::new())),
    }
  }

  pub fn source<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source failed".to_string());
    self
  }

  pub fn source_excludes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_excludes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_excludes failed".to_string());
    self
  }

  pub fn source_includes<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.source_includes = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for source_includes failed".to_string());
    self
  }

  pub fn allow_no_indices<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_no_indices = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_no_indices failed".to_string());
    self
  }

  pub fn allow_partial_search_results<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.allow_partial_search_results = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for allow_partial_search_results failed".to_string());
    self
  }

  pub fn analyze_wildcard<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.analyze_wildcard = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for analyze_wildcard failed".to_string());
    self
  }

  pub fn analyzer<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.analyzer = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for analyzer failed".to_string());
    self
  }

  pub fn batched_reduce_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.batched_reduce_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for batched_reduce_size failed".to_string());
    self
  }

  pub fn ccs_minimize_roundtrips<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ccs_minimize_roundtrips = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ccs_minimize_roundtrips failed".to_string());
    self
  }

  pub fn default_operator<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::DefaultOperator>, {
    self.default_operator = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `DefaultOperator` for default_operator failed".to_string());
    self
  }

  pub fn df<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.df = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for df failed".to_string());
    self
  }

  pub fn docvalue_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.docvalue_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for docvalue_fields failed".to_string());
    self
  }

  pub fn expand_wildcards<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::ExpandWildcards>, {
    self.expand_wildcards = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `ExpandWildcards` for expand_wildcards failed".to_string());
    self
  }

  pub fn explain<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.explain = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for explain failed".to_string());
    self
  }

  pub fn from<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.from = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for from failed".to_string());
    self
  }

  pub fn ignore_throttled<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_throttled = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_throttled failed".to_string());
    self
  }

  pub fn ignore_unavailable<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.ignore_unavailable = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for ignore_unavailable failed".to_string());
    self
  }

  pub fn lenient<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.lenient = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for lenient failed".to_string());
    self
  }

  pub fn max_concurrent_shard_requests<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.max_concurrent_shard_requests = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for max_concurrent_shard_requests failed".to_string());
    self
  }

  pub fn pre_filter_shard_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.pre_filter_shard_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for pre_filter_shard_size failed".to_string());
    self
  }

  pub fn preference<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.preference = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for preference failed".to_string());
    self
  }

  pub fn q<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.q = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for q failed".to_string());
    self
  }

  pub fn request_cache<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.request_cache = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for request_cache failed".to_string());
    self
  }

  pub fn rest_total_hits_as_int<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.rest_total_hits_as_int = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for rest_total_hits_as_int failed".to_string());
    self
  }

  pub fn routing<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.routing = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for routing failed".to_string());
    self
  }

  pub fn scroll<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchPostScroll>, {
    self.scroll = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchPostScroll` for scroll failed".to_string());
    self
  }

  pub fn search_type<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SearchType>, {
    self.search_type = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SearchType` for search_type failed".to_string());
    self
  }

  pub fn seq_no_primary_term<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.seq_no_primary_term = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for seq_no_primary_term failed".to_string());
    self
  }

  pub fn size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for size failed".to_string());
    self
  }

  pub fn sort<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.sort = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for sort failed".to_string());
    self
  }

  pub fn stats<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stats = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stats failed".to_string());
    self
  }

  pub fn stored_fields<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Vec<String>>, {
    self.stored_fields = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Vec < String >` for stored_fields failed".to_string());
    self
  }

  pub fn suggest_field<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.suggest_field = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for suggest_field failed".to_string());
    self
  }

  pub fn suggest_mode<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::SuggestMode>, {
    self.suggest_mode = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `SuggestMode` for suggest_mode failed".to_string());
    self
  }

  pub fn suggest_size<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.suggest_size = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for suggest_size failed".to_string());
    self
  }

  pub fn suggest_text<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<String>, {
    self.suggest_text = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `String` for suggest_text failed".to_string());
    self
  }

  pub fn terminate_after<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<i32>, {
    self.terminate_after = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `i32` for terminate_after failed".to_string());
    self
  }

  pub fn timeout<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<types::Timeout>, {
    self.timeout = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Timeout` for timeout failed".to_string());
    self
  }

  pub fn track_scores<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.track_scores = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for track_scores failed".to_string());
    self
  }

  pub fn track_total_hits<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.track_total_hits = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for track_total_hits failed".to_string());
    self
  }

  pub fn typed_keys<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.typed_keys = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for typed_keys failed".to_string());
    self
  }

  pub fn version<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<bool>, {
    self.version = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `bool` for version failed".to_string());
    self
  }

  pub fn body<V>(mut self, value: V) -> Self
  where
    V: std::convert::TryInto<Search>, {
    self.body = value
      .try_into()
      .map(Some)
      .map_err(|_| "conversion to `Search` for body failed".to_string());
    self
  }

  // pub fn body_map<F>(mut self, f: F) -> Self
  // where
  //   F: std::ops::FnOnce(Search) -> Search, {
  //   self.body = self.body.map(f);
  //   self
  // }

  ///Sends a `POST` request to `/_search`
  pub async fn send<T: DeserializeOwned + std::default::Default>(
    self,
  ) -> Result<ResponseValue<types::SearchPostResponseContent<T>>, Error> {
    let Self {
      client,
      source,
      source_excludes,
      source_includes,
      allow_no_indices,
      allow_partial_search_results,
      analyze_wildcard,
      analyzer,
      batched_reduce_size,
      ccs_minimize_roundtrips,
      default_operator,
      df,
      docvalue_fields,
      expand_wildcards,
      explain,
      from,
      ignore_throttled,
      ignore_unavailable,
      lenient,
      max_concurrent_shard_requests,
      pre_filter_shard_size,
      preference,
      q,
      request_cache,
      rest_total_hits_as_int,
      routing,
      scroll,
      search_type,
      seq_no_primary_term,
      size,
      sort,
      stats,
      stored_fields,
      suggest_field,
      suggest_mode,
      suggest_size,
      suggest_text,
      terminate_after,
      timeout,
      track_scores,
      track_total_hits,
      typed_keys,
      version,
      body,
    } = self;
    let source = source.map_err(Error::InvalidRequest)?;
    let source_excludes = source_excludes.map_err(Error::InvalidRequest)?;
    let source_includes = source_includes.map_err(Error::InvalidRequest)?;
    let allow_no_indices = allow_no_indices.map_err(Error::InvalidRequest)?;
    let allow_partial_search_results = allow_partial_search_results.map_err(Error::InvalidRequest)?;
    let analyze_wildcard = analyze_wildcard.map_err(Error::InvalidRequest)?;
    let analyzer = analyzer.map_err(Error::InvalidRequest)?;
    let batched_reduce_size = batched_reduce_size.map_err(Error::InvalidRequest)?;
    let ccs_minimize_roundtrips = ccs_minimize_roundtrips.map_err(Error::InvalidRequest)?;
    let default_operator = default_operator.map_err(Error::InvalidRequest)?;
    let df = df.map_err(Error::InvalidRequest)?;
    let docvalue_fields = docvalue_fields.map_err(Error::InvalidRequest)?;
    let expand_wildcards = expand_wildcards.map_err(Error::InvalidRequest)?;
    let explain = explain.map_err(Error::InvalidRequest)?;
    let from = from.map_err(Error::InvalidRequest)?;
    let ignore_throttled = ignore_throttled.map_err(Error::InvalidRequest)?;
    let ignore_unavailable = ignore_unavailable.map_err(Error::InvalidRequest)?;
    let lenient = lenient.map_err(Error::InvalidRequest)?;
    let max_concurrent_shard_requests = max_concurrent_shard_requests.map_err(Error::InvalidRequest)?;
    let pre_filter_shard_size = pre_filter_shard_size.map_err(Error::InvalidRequest)?;
    let preference = preference.map_err(Error::InvalidRequest)?;
    let q = q.map_err(Error::InvalidRequest)?;
    let request_cache = request_cache.map_err(Error::InvalidRequest)?;
    let rest_total_hits_as_int = rest_total_hits_as_int.map_err(Error::InvalidRequest)?;
    let routing = routing.map_err(Error::InvalidRequest)?;
    let scroll = scroll.map_err(Error::InvalidRequest)?;
    let search_type = search_type.map_err(Error::InvalidRequest)?;
    let seq_no_primary_term = seq_no_primary_term.map_err(Error::InvalidRequest)?;
    let size = size.map_err(Error::InvalidRequest)?;
    let sort = sort.map_err(Error::InvalidRequest)?;
    let stats = stats.map_err(Error::InvalidRequest)?;
    let stored_fields = stored_fields.map_err(Error::InvalidRequest)?;
    let suggest_field = suggest_field.map_err(Error::InvalidRequest)?;
    let suggest_mode = suggest_mode.map_err(Error::InvalidRequest)?;
    let suggest_size = suggest_size.map_err(Error::InvalidRequest)?;
    let suggest_text = suggest_text.map_err(Error::InvalidRequest)?;
    let terminate_after = terminate_after.map_err(Error::InvalidRequest)?;
    let timeout = timeout.map_err(Error::InvalidRequest)?;
    let track_scores = track_scores.map_err(Error::InvalidRequest)?;
    let track_total_hits = track_total_hits.map_err(Error::InvalidRequest)?;
    let typed_keys = typed_keys.map_err(Error::InvalidRequest)?;
    let version = version.map_err(Error::InvalidRequest)?;
    let body = match body.map_err(Error::InvalidRequest)? {
      Some(_) => todo!(),
      None => todo!(),
    };
    let url = format!("{}_search", client.baseurl,);
    let mut query = Vec::with_capacity(42usize);
    if let Some(v) = &source {
      query.push(("_source", v.join(",")));
    }
    if let Some(v) = &source_excludes {
      query.push(("_source_excludes", v.join(",")));
    }
    if let Some(v) = &source_includes {
      query.push(("_source_includes", v.join(",")));
    }
    if let Some(v) = &allow_no_indices {
      query.push(("allow_no_indices", v.to_string()));
    }
    if let Some(v) = &allow_partial_search_results {
      query.push(("allow_partial_search_results", v.to_string()));
    }
    if let Some(v) = &analyze_wildcard {
      query.push(("analyze_wildcard", v.to_string()));
    }
    if let Some(v) = &analyzer {
      query.push(("analyzer", v.to_string()));
    }
    if let Some(v) = &batched_reduce_size {
      query.push(("batched_reduce_size", v.to_string()));
    }
    if let Some(v) = &ccs_minimize_roundtrips {
      query.push(("ccs_minimize_roundtrips", v.to_string()));
    }
    if let Some(v) = &default_operator {
      query.push(("default_operator", v.to_string()));
    }
    if let Some(v) = &df {
      query.push(("df", v.to_string()));
    }
    if let Some(v) = &docvalue_fields {
      query.push(("docvalue_fields", v.join(",")));
    }
    if let Some(v) = &expand_wildcards {
      query.push(("expand_wildcards", v.to_string()));
    }
    if let Some(v) = &explain {
      query.push(("explain", v.to_string()));
    }
    if let Some(v) = &from {
      query.push(("from", v.to_string()));
    }
    if let Some(v) = &ignore_throttled {
      query.push(("ignore_throttled", v.to_string()));
    }
    if let Some(v) = &ignore_unavailable {
      query.push(("ignore_unavailable", v.to_string()));
    }
    if let Some(v) = &lenient {
      query.push(("lenient", v.to_string()));
    }
    if let Some(v) = &max_concurrent_shard_requests {
      query.push(("max_concurrent_shard_requests", v.to_string()));
    }
    if let Some(v) = &pre_filter_shard_size {
      query.push(("pre_filter_shard_size", v.to_string()));
    }
    if let Some(v) = &preference {
      query.push(("preference", v.to_string()));
    }
    if let Some(v) = &q {
      query.push(("q", v.to_string()));
    }
    if let Some(v) = &request_cache {
      query.push(("request_cache", v.to_string()));
    }
    if let Some(v) = &rest_total_hits_as_int {
      query.push(("rest_total_hits_as_int", v.to_string()));
    }
    if let Some(v) = &routing {
      query.push(("routing", v.join(",")));
    }
    if let Some(v) = &scroll {
      query.push(("scroll", v.to_string()));
    }
    if let Some(v) = &search_type {
      query.push(("search_type", v.to_string()));
    }
    if let Some(v) = &seq_no_primary_term {
      query.push(("seq_no_primary_term", v.to_string()));
    }
    if let Some(v) = &size {
      query.push(("size", v.to_string()));
    }
    if let Some(v) = &sort {
      query.push(("sort", v.join(",")));
    }
    if let Some(v) = &stats {
      query.push(("stats", v.join(",")));
    }
    if let Some(v) = &stored_fields {
      query.push(("stored_fields", v.join(",")));
    }
    if let Some(v) = &suggest_field {
      query.push(("suggest_field", v.to_string()));
    }
    if let Some(v) = &suggest_mode {
      query.push(("suggest_mode", v.to_string()));
    }
    if let Some(v) = &suggest_size {
      query.push(("suggest_size", v.to_string()));
    }
    if let Some(v) = &suggest_text {
      query.push(("suggest_text", v.to_string()));
    }
    if let Some(v) = &terminate_after {
      query.push(("terminate_after", v.to_string()));
    }
    if let Some(v) = &timeout {
      query.push(("timeout", v.to_string()));
    }
    if let Some(v) = &track_scores {
      query.push(("track_scores", v.to_string()));
    }
    if let Some(v) = &track_total_hits {
      query.push(("track_total_hits", v.to_string()));
    }
    if let Some(v) = &typed_keys {
      query.push(("typed_keys", v.to_string()));
    }
    if let Some(v) = &version {
      query.push(("version", v.to_string()));
    }
    let request = client
      .client
      .post(url)
      .header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
      )
      .json(&body)
      .query(&query)
      .build()?;
    let result = client.client.execute(request).await;
    let response = result?;
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
