/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Boolean field type property struct.
 */

use serde::{Deserialize, Serialize};

/// `boolean` field: accepts true/false values.
///
/// # Example
/// ```json
/// {
///   "type": "boolean",
///   "null_value": false
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanProperty {
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Store column-oriented data for sorting/aggregations (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Value used to replace explicit null values (true or false).
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<bool>,
    /// Store field value separately from _source (default false).
    #[serde(rename = "store", default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Copy field values into one or more target fields.
    #[serde(rename = "copy_to", default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::common::Fields>,
}

impl BooleanProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
