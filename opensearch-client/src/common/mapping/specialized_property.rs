/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Specialized field type property structs.
 */

use serde::{Deserialize, Serialize};

/// `binary` field: stores Base64-encoded binary values. Not searchable.
///
/// # Example
/// ```json
/// {
///   "type": "binary",
///   "doc_values": false
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinaryProperty {
    /// Store column-oriented data (default false for binary).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl BinaryProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `alias` field: alternate name pointing to an existing indexed field.
///
/// Constraints: one original field per alias; must be at the same nesting level;
/// original cannot be an object or another alias.
///
/// # Example
/// ```json
/// {
///   "type": "alias",
///   "path": "user.name"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasProperty {
    /// Dot-notation path to the target field (required).
    #[serde(rename = "path")]
    pub path: String,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl AliasProperty {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            meta: None,
        }
    }
}

/// `percolator` field: stores a query DSL object for use with the `percolate` query.
///
/// Documents stored to a percolator field contain a query, and the `percolate` query
/// finds which stored queries match an incoming document.
///
/// # Example (mapping)
/// ```json
/// {
///   "type": "percolator"
/// }
/// ```
/// (document stored to this field): `{"query": {"term": {"status": "active"}}}`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PercolatorProperty {
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl PercolatorProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `histogram` field: pre-aggregated numeric data stored as a pair of arrays.
///
/// Document value: `{"values": [1, 5, 10], "counts": [3, 7, 2]}`
///
/// # Example
/// ```json
/// {
///   "type": "histogram"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistogramProperty {
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl HistogramProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `aggregate_metric_double` field: stores pre-aggregated numeric metrics.
///
/// # Example
/// ```json
/// {
///   "type": "aggregate_metric_double",
///   "metrics": ["min", "max", "sum", "value_count"],
///   "default_metric": "max"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateMetricDoubleProperty {
    /// Which metrics to store: min, max, sum, value_count, avg (derived).
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    /// Default metric used when the field is queried directly.
    #[serde(rename = "default_metric", default, skip_serializing_if = "Option::is_none")]
    pub default_metric: Option<String>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl AggregateMetricDoubleProperty {
    pub fn new(metrics: Vec<String>, default_metric: impl Into<String>) -> Self {
        Self {
            metrics: Some(metrics),
            default_metric: Some(default_metric.into()),
            meta: None,
        }
    }
}

/// `murmur3` field: computes and stores a MurmurHash3 of field values at index time.
///
/// Used for efficient cardinality estimation (HyperLogLog++ aggregation).
///
/// # Example
/// ```json
/// {
///   "type": "murmur3"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Murmur3HashProperty {
    /// Store column-oriented data (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl Murmur3HashProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
