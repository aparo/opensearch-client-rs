/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Text-based field type property structs.
 */

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::enums::{IndexOptions, Similarity, TermVector};
use super::text_index_prefixes::TextIndexPrefixes;

/// Fielddata frequency filter for text fields.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FielddataFrequencyFilter {
    /// Minimum document frequency ratio (0–1) or absolute count.
    #[serde(rename = "min", default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Maximum document frequency ratio (0–1) or absolute count.
    #[serde(rename = "max", default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// Minimum segment size required to apply the filter.
    #[serde(rename = "min_segment_size", default, skip_serializing_if = "Option::is_none")]
    pub min_segment_size: Option<u32>,
}

/// Full-text `text` field: analyzed and tokenized for full-text search.
///
/// # Example
/// ```json
/// {
///   "type": "text",
///   "analyzer": "standard",
///   "index_options": "positions"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextProperty {
    /// Index-time analyzer name.
    #[serde(rename = "analyzer", default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Copy field values into one or more target fields.
    #[serde(rename = "copy_to", default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::common::Fields>,
    /// Build global ordinals at refresh time (default false).
    #[serde(rename = "eager_global_ordinals", default, skip_serializing_if = "Option::is_none")]
    pub eager_global_ordinals: Option<bool>,
    /// Enable fielddata for sorting/aggregations on text (expensive, default false).
    #[serde(rename = "fielddata", default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<bool>,
    /// Filter applied when loading fielddata.
    #[serde(rename = "fielddata_frequency_filter", default, skip_serializing_if = "Option::is_none")]
    pub fielddata_frequency_filter: Option<FielddataFrequencyFilter>,
    /// Multi-field definitions (index same value in different ways).
    #[serde(rename = "fields", default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// What information is stored in the inverted index.
    #[serde(rename = "index_options", default, skip_serializing_if = "Option::is_none")]
    pub index_options: Option<IndexOptions>,
    /// Index bigrams of adjacent words for faster phrase queries (default false).
    #[serde(rename = "index_phrases", default, skip_serializing_if = "Option::is_none")]
    pub index_phrases: Option<bool>,
    /// Index term prefixes to speed up prefix/autocomplete queries.
    #[serde(rename = "index_prefixes", default, skip_serializing_if = "Option::is_none")]
    pub index_prefixes: Option<TextIndexPrefixes>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Enable field-length normalization for scoring (default true).
    #[serde(rename = "norms", default, skip_serializing_if = "Option::is_none")]
    pub norms: Option<bool>,
    /// Virtual gap inserted between multi-value elements (default 100).
    #[serde(rename = "position_increment_gap", default, skip_serializing_if = "Option::is_none")]
    pub position_increment_gap: Option<u32>,
    /// Search-time analyzer (defaults to index analyzer).
    #[serde(rename = "search_analyzer", default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
    /// Analyzer used only for phrase queries.
    #[serde(rename = "search_quote_analyzer", default, skip_serializing_if = "Option::is_none")]
    pub search_quote_analyzer: Option<String>,
    /// Similarity algorithm (default BM25).
    #[serde(rename = "similarity", default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<Similarity>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Per-document term information storage.
    #[serde(rename = "term_vector", default, skip_serializing_if = "Option::is_none")]
    pub term_vector: Option<TermVector>,
}

impl TextProperty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_analyzer(mut self, analyzer: impl Into<String>) -> Self {
        self.analyzer = Some(analyzer.into());
        self
    }
}

/// Exact-value `keyword` field: not analyzed, used for filtering, sorting, aggregations.
///
/// # Example
/// ```json
/// {
///   "type": "keyword",
///   "ignore_above": 256
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeywordProperty {
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Copy field values into one or more target fields.
    #[serde(rename = "copy_to", default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::common::Fields>,
    /// Store column-oriented data for sorting/aggregations (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Build global ordinals at refresh time (default false).
    #[serde(rename = "eager_global_ordinals", default, skip_serializing_if = "Option::is_none")]
    pub eager_global_ordinals: Option<bool>,
    /// Multi-field definitions.
    #[serde(rename = "fields", default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    /// Strings longer than this are not indexed (default unlimited).
    #[serde(rename = "ignore_above", default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<u32>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// What information is stored in the inverted index (default docs).
    #[serde(rename = "index_options", default, skip_serializing_if = "Option::is_none")]
    pub index_options: Option<IndexOptions>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Normalizer to apply before indexing (e.g. lowercase).
    #[serde(rename = "normalizer", default, skip_serializing_if = "Option::is_none")]
    pub normalizer: Option<String>,
    /// Enable field-length normalization for scoring (default false for keyword).
    #[serde(rename = "norms", default, skip_serializing_if = "Option::is_none")]
    pub norms: Option<bool>,
    /// Value used to represent explicit null values.
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    /// Similarity algorithm (default BM25).
    #[serde(rename = "similarity", default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<Similarity>,
    /// Split query tokens on whitespace for term-level queries (default false).
    #[serde(rename = "split_queries_on_whitespace", default, skip_serializing_if = "Option::is_none")]
    pub split_queries_on_whitespace: Option<bool>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Enable BM25 scoring instead of constant_score rewriting (default false).
    #[serde(rename = "use_similarity", default, skip_serializing_if = "Option::is_none")]
    pub use_similarity: Option<bool>,
}

impl KeywordProperty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ignore_above(mut self, limit: u32) -> Self {
        self.ignore_above = Some(limit);
        self
    }

    pub fn with_normalizer(mut self, normalizer: impl Into<String>) -> Self {
        self.normalizer = Some(normalizer.into());
        self
    }
}

/// `match_only_text` field: storage-optimized text variant; all matches score 1.0.
///
/// # Example
/// ```json
/// {
///   "type": "match_only_text"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchOnlyTextPropertyValue {
    /// Copy field values into one or more target fields.
    #[serde(rename = "copy_to", default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::common::Fields>,
    /// Multi-field definitions.
    #[serde(rename = "fields", default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl MatchOnlyTextPropertyValue {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `wildcard` field: optimized for arbitrary substring/regex matching via trigram index.
///
/// # Example
/// ```json
/// {
///   "type": "wildcard",
///   "ignore_above": 1024
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WildcardProperty {
    /// Copy field values into one or more target fields.
    #[serde(rename = "copy_to", default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::common::Fields>,
    /// Store column-oriented data for sorting/aggregations (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Multi-field definitions.
    #[serde(rename = "fields", default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
    /// Strings longer than this are not indexed (default unlimited).
    #[serde(rename = "ignore_above", default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<u32>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Normalizer to apply before indexing.
    #[serde(rename = "normalizer", default, skip_serializing_if = "Option::is_none")]
    pub normalizer: Option<String>,
    /// Value used to represent explicit null values.
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
}

impl WildcardProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `constant_keyword` field: all documents in the index share the same fixed value.
///
/// # Example
/// ```json
/// {
///   "type": "constant_keyword",
///   "value": "log"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConstantKeywordProperty {
    /// The fixed value shared by all documents in the index.
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl ConstantKeywordProperty {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: Some(value.into()),
            meta: None,
        }
    }
}

/// `token_count` field: stores the integer count of analyzed tokens from a string.
///
/// # Example
/// ```json
/// {
///   "type": "token_count",
///   "analyzer": "standard"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenCountProperty {
    /// Analyzer used to count tokens (required).
    #[serde(rename = "analyzer")]
    pub analyzer: String,
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Store column-oriented data (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Include position increments in token count (default true).
    #[serde(rename = "enable_position_increments", default, skip_serializing_if = "Option::is_none")]
    pub enable_position_increments: Option<bool>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Value used to represent explicit null values.
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<i64>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
}

impl TokenCountProperty {
    pub fn new(analyzer: impl Into<String>) -> Self {
        Self {
            analyzer: analyzer.into(),
            ..Default::default()
        }
    }
}

/// `version` field: stores Semantic Versioning (SemVer) strings with correct ordering.
///
/// # Example
/// ```json
/// {
///   "type": "version"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionProperty {
    /// Store column-oriented data for sorting (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
}

impl VersionProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `icu_collation_keyword` field: language-aware sorting and range queries (requires analysis-icu plugin).
///
/// # Example
/// ```json
/// {
///   "type": "icu_collation_keyword",
///   "language": "de",
///   "strength": "tertiary"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IcuCollationKeywordProperty {
    /// BCP 47 language tag (e.g. "de", "en").
    #[serde(rename = "language", default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// ISO 3166 country code (e.g. "DE").
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Collation variant.
    #[serde(rename = "variant", default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// Collation strength: primary, secondary, tertiary, quaternary, identical.
    #[serde(rename = "strength", default, skip_serializing_if = "Option::is_none")]
    pub strength: Option<String>,
    /// Decomposition mode: no, canonical, full.
    #[serde(rename = "decomposition", default, skip_serializing_if = "Option::is_none")]
    pub decomposition: Option<String>,
    /// Alternate handling: non-ignorable, shifted.
    #[serde(rename = "alternate", default, skip_serializing_if = "Option::is_none")]
    pub alternate: Option<String>,
    /// Add extra secondary level for case differences (default false).
    #[serde(rename = "case_level", default, skip_serializing_if = "Option::is_none")]
    pub case_level: Option<bool>,
    /// Case ordering: lower, upper.
    #[serde(rename = "case_first", default, skip_serializing_if = "Option::is_none")]
    pub case_first: Option<String>,
    /// Numeric collation: sort "10" after "9" (default false).
    #[serde(rename = "numeric", default, skip_serializing_if = "Option::is_none")]
    pub numeric: Option<bool>,
    /// Store column-oriented data (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// Value used to represent explicit null values.
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Multi-field definitions.
    #[serde(rename = "fields", default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
}

impl IcuCollationKeywordProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
