/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * IP field type property struct.
 */

use serde::{Deserialize, Serialize};

/// `ip` field: stores IPv4 or IPv6 addresses.
///
/// CIDR notation is supported in term queries: `"192.168.0.0/16"`.
///
/// # Example
/// ```json
/// {
///   "type": "ip",
///   "ignore_malformed": false
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpProperty {
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Store column-oriented data for sorting/aggregations (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Store malformed IP values instead of rejecting the document (default false).
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
    pub null_value: Option<String>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Copy field values into one or more target fields.
    #[serde(rename = "copy_to", default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::common::Fields>,
}

impl IpProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
