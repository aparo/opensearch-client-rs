use std::sync::Arc;

use crate::{Error, OsClient};
use opensearch_dsl::{util::ShouldSkip, InnerHitsResult, Map};
use opensearch_dsl::{
    Explanation, NestedIdentity, Query, SearchResponse, ShardStatistics, SortCollection, Terms,
    TotalHits,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the state of a search operation that uses the "search after"
/// feature.
pub struct SearchAfterState {
    pub client: Arc<OsClient>,
    pub index: String,
    pub stop: bool,
    pub size: u64,
    pub query: Query,
    pub sort: SortCollection,
    pub search_after: Option<Terms>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct TypedSearchResult<T> {
    #[serde(default)]
    pub hits: TypedHitsMetadata<T>,
    #[serde(
        rename = "_scroll_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_id: Option<String>,
    #[serde(rename = "_shards", default)]
    pub shards: ShardStatistics,
    #[serde(default)]
    pub timed_out: bool,
    #[serde(default)]
    pub took: u32,
}

impl<T: serde::de::DeserializeOwned + std::default::Default> TypedSearchResult<T> {
    pub fn from_response(response: SearchResponse) -> Result<Self, crate::Error> {
        // Implement conversion logic from SearchSuccess to TypedSearchResult<T>
        // Example stub:
        let hits: TypedHitsMetadata<T> = TypedHitsMetadata::from_response(response.hits)?;

        Ok(TypedSearchResult {
            // fill fields from response
            hits,
            scroll_id: response.scroll_id,
            shards: response.shards,
            timed_out: response.timed_out,
            took: response.took,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TypedHit<T> {
    /// Search explanation
    #[serde(
        default,
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_explanation"
    )]
    pub explanation: Option<Explanation>,

    /// Document index
    #[serde(
        default,
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_index"
    )]
    pub index: String,

    /// Document ID
    #[serde(
        default,
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_id"
    )]
    pub id: String,

    /// Document score. [`None`] when documents are implicitly sorted by a
    /// field other than `_score`
    #[serde(
        default,
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_score"
    )]
    pub score: Option<f32>,

    /// Nested document identity
    #[serde(
        default,
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_nested"
    )]
    pub nested: Option<NestedIdentity>,

    /// Document source
    #[serde(
        default,
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_source"
    )]
    pub source: Option<T>,

    /// Highlighted matches
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub highlight: Map<String, Vec<String>>,

    /// Inner hits
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub inner_hits: Map<String, InnerHitsResult>,

    /// Matched queries
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub matched_queries: Vec<String>,

    /// Values document was sorted by
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub sort: Vec<serde_json::Value>,

    /// Field values for the documents. Need to be specified in the request
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub fields: Map<String, serde_json::Value>,
}

impl<T> From<&TypedHit<T>> for TypedHit<T> {
    fn from(value: &TypedHit<T>) -> Self {
        value.into()
    }
}

impl<T: serde::de::DeserializeOwned> TypedHit<T> {
    pub fn from_hit(hit: opensearch_dsl::Hit) -> TypedHit<T> {
        let parsed: Result<T, serde_json::Error> = hit.source.parse();
        let source: Option<T> = if let Ok(src) = parsed {
            Some(src)
        } else {
            None
        };
        TypedHit {
            explanation: hit.explanation,
            index: hit.index,
            id: hit.id,
            score: hit.score,
            nested: hit.nested,
            source,
            highlight: hit.highlight,
            inner_hits: hit.inner_hits,
            matched_queries: hit.matched_queries,
            sort: hit.sort,
            fields: hit.fields,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypedHitsMetadata<T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hits: Vec<TypedHit<T>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<TotalHits>,
}

impl<T> TypedHitsMetadata<T> {
    pub fn get_total_value(&self) -> Option<u64> {
        self.total.as_ref().map(|t| t.value)
    }
}

impl<T> From<&TypedHitsMetadata<T>> for TypedHitsMetadata<T> {
    fn from(value: &TypedHitsMetadata<T>) -> Self {
        value.into()
    }
}

impl<T> Default for TypedHitsMetadata<T> {
    fn default() -> Self {
        Self {
            hits: Vec::new(),
            max_score: None,
            total: None,
        }
    }
}

impl<T: serde::de::DeserializeOwned> TypedHitsMetadata<T> {
    pub fn from_response(hits: opensearch_dsl::HitsMetadata) -> Result<Self, crate::Error> {
        let typed_hits = hits
            .hits
            .into_iter()
            .map(|hit| TypedHit::from_hit(hit))
            .collect::<Vec<_>>();

        Ok(TypedHitsMetadata {
            hits: typed_hits,
            max_score: hits.max_score,
            total: hits.total,
        })
    }
}

// impl<T> From<&TypedHitsMetadata<T>> for TypedHitsMetadata<T> {
//     fn from(value: &TypedHitsMetadata<T>) -> Self {
//         value.clone()
//     }
// }

// impl<T> SearchResult<T> {
//     pub fn builder() -> builder::SearchResult<T> {
//         builder::SearchResult::default()
//     }
// }
