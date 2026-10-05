/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Autocomplete field type property structs.
 */

use serde::{Deserialize, Serialize};

use super::enums::{IndexOptions, TermVector};

/// Fuzzy completion options for the `completion` field suggester.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompletionFuzzy {
    /// Maximum edit distance (default 0 = no fuzzy).
    #[serde(rename = "fuzziness", default, skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<serde_json::Value>,
    /// Minimum input length before fuzzy matching kicks in (default 3).
    #[serde(rename = "min_length", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    /// Number of characters at the start that must match exactly (default 1).
    #[serde(rename = "prefix_length", default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<u32>,
    /// Allow transpositions in fuzzy matching (default true).
    #[serde(rename = "transpositions", default, skip_serializing_if = "Option::is_none")]
    pub transpositions: Option<bool>,
    /// Apply fuzzy matching to Unicode code points, not bytes (default false).
    #[serde(rename = "unicode_aware", default, skip_serializing_if = "Option::is_none")]
    pub unicode_aware: Option<bool>,
}

/// `completion` field: in-memory prefix suggester for autocomplete (matches from the start).
///
/// For both prefix and infix completion, use `search_as_you_type` instead.
///
/// # Example
/// ```json
/// {
///   "type": "completion",
///   "analyzer": "simple"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompletionProperty {
    /// Analyzer for index-time tokenization (default: simple).
    #[serde(rename = "analyzer", default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    /// Analyzer for search-time tokenization (defaults to index analyzer).
    #[serde(rename = "search_analyzer", default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
    /// Keep separator tokens in completion inputs (default true).
    #[serde(rename = "preserve_separators", default, skip_serializing_if = "Option::is_none")]
    pub preserve_separators: Option<bool>,
    /// Preserve position increments from filtered tokens (default true).
    #[serde(rename = "preserve_position_increments", default, skip_serializing_if = "Option::is_none")]
    pub preserve_position_increments: Option<bool>,
    /// Maximum input length in UTF-16 code points (default 50).
    #[serde(rename = "max_input_length", default, skip_serializing_if = "Option::is_none")]
    pub max_input_length: Option<u32>,
    /// Fuzzy completion options.
    #[serde(rename = "fuzzy", default, skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<CompletionFuzzy>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl CompletionProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `search_as_you_type` field: both prefix and infix completion.
///
/// Auto-creates n-gram shingle subfields (`._2gram`, `._3gram`) and `._index_prefix`
/// subfield. Query with `multi_match` type `bool_prefix` for any-order completion,
/// or `match_phrase_prefix` for in-order prefix.
///
/// # Example
/// ```json
/// {
///   "type": "search_as_you_type",
///   "max_shingle_size": 3
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchAsYouTypeProperty {
    /// Analyzer for index-time tokenization.
    #[serde(rename = "analyzer", default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// What information is stored in the inverted index (default positions).
    #[serde(rename = "index_options", default, skip_serializing_if = "Option::is_none")]
    pub index_options: Option<IndexOptions>,
    /// Maximum shingle size for auto-created subfields (default 3, range 2–4).
    #[serde(rename = "max_shingle_size", default, skip_serializing_if = "Option::is_none")]
    pub max_shingle_size: Option<u32>,
    /// Enable field-length normalization (default false for subfields).
    #[serde(rename = "norms", default, skip_serializing_if = "Option::is_none")]
    pub norms: Option<bool>,
    /// Search-time analyzer (defaults to index analyzer).
    #[serde(rename = "search_analyzer", default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
    /// Analyzer used only for phrase queries.
    #[serde(rename = "search_quote_analyzer", default, skip_serializing_if = "Option::is_none")]
    pub search_quote_analyzer: Option<String>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Per-document term information storage (default no).
    #[serde(rename = "term_vector", default, skip_serializing_if = "Option::is_none")]
    pub term_vector: Option<TermVector>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl SearchAsYouTypeProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
