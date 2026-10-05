/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Range field type property structs.
 */

use serde::{Deserialize, Serialize};

/// Common parameters for all range field types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RangeProperty {
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Coerce strings and fractions (default true).
    #[serde(rename = "coerce", default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    /// Store column-oriented data for sorting/aggregations (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl RangeProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `integer_range` field: range of signed 32-bit integers.
///
/// # Example
/// ```json
/// {
///   "type": "integer_range"
/// }
/// ```
/// Document value: `{"gte": 0, "lte": 100}`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegerRangeProperty {
    #[serde(flatten)]
    pub base: RangeProperty,
}

/// `long_range` field: range of signed 64-bit integers.
///
/// # Example
/// ```json
/// {
///   "type": "long_range"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LongRangeProperty {
    #[serde(flatten)]
    pub base: RangeProperty,
}

/// `float_range` field: range of single-precision 32-bit floats.
///
/// # Example
/// ```json
/// {
///   "type": "float_range"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FloatRangeProperty {
    #[serde(flatten)]
    pub base: RangeProperty,
}

/// `double_range` field: range of double-precision 64-bit floats.
///
/// # Example
/// ```json
/// {
///   "type": "double_range"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoubleRangeProperty {
    #[serde(flatten)]
    pub base: RangeProperty,
}

/// `date_range` field: range of date values (stored as unsigned 64-bit ms since epoch).
///
/// Supports the `format` parameter for custom date parsing.
///
/// # Example
/// ```json
/// {
///   "type": "date_range",
///   "format": "yyyy-MM-dd"
/// }
/// ```
/// Document value: `{"gte": "2024-01-01", "lte": "2024-12-31"}`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateRangeProperty {
    #[serde(flatten)]
    pub base: RangeProperty,
    /// Date format for parsing range bounds.
    #[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl DateRangeProperty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
}

/// `ip_range` field: range of IPv4/IPv6 addresses. Also accepts CIDR notation.
///
/// # Example
/// ```json
/// {
///   "type": "ip_range"
/// }
/// ```
/// Document value: `{"gte": "192.168.0.1", "lte": "192.168.0.254"}` or `"192.168.0.0/24"`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRangeProperty {
    #[serde(flatten)]
    pub base: RangeProperty,
}
