/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Numeric field type property structs.
 */

use serde::{Deserialize, Serialize};

/// Numeric field (byte, short, integer, long, unsigned_long, float, double, half_float).
///
/// # Example (integer)
/// ```json
/// {
///   "type": "integer",
///   "coerce": true,
///   "null_value": 0
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericProperty {
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Coerce strings to numbers and truncate floats to integers (default true).
    #[serde(rename = "coerce", default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    /// Store column-oriented data for sorting/aggregations (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Store malformed values instead of rejecting the document (default false).
    #[serde(rename = "ignore_malformed", default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Value used to represent explicit null values.
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<serde_json::Value>,
    /// Use skip-list doc values for faster range queries (default false).
    #[serde(rename = "skip_list", default, skip_serializing_if = "Option::is_none")]
    pub skip_list: Option<bool>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Copy field values into one or more target fields.
    #[serde(rename = "copy_to", default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::common::Fields>,
}

impl NumericProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `scaled_float` field: floating-point backed by a long scaled by a fixed factor.
///
/// # Example
/// ```json
/// {
///   "type": "scaled_float",
///   "scaling_factor": 100
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaledFloatProperty {
    /// Scale factor applied before storing as long (required).
    #[serde(rename = "scaling_factor")]
    pub scaling_factor: f64,
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Coerce strings to numbers (default true).
    #[serde(rename = "coerce", default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    /// Store column-oriented data (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Store malformed values instead of rejecting the document (default false).
    #[serde(rename = "ignore_malformed", default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Value used to represent explicit null values.
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    /// Use skip-list doc values for faster range queries (default false).
    #[serde(rename = "skip_list", default, skip_serializing_if = "Option::is_none")]
    pub skip_list: Option<bool>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Copy field values into one or more target fields.
    #[serde(rename = "copy_to", default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::common::Fields>,
}

impl ScaledFloatProperty {
    pub fn new(scaling_factor: f64) -> Self {
        Self {
            scaling_factor,
            ..Default::default()
        }
    }
}
