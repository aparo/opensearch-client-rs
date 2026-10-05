/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Date field type property structs.
 */

use serde::{Deserialize, Serialize};

/// `date` field: stores dates as milliseconds since epoch.
///
/// # Example
/// ```json
/// {
///   "type": "date",
///   "format": "yyyy-MM-dd||epoch_millis"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateProperty {
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Store column-oriented data for sorting/aggregations (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// One or more date formats separated by `||` (default: strict_date_optional_time||epoch_millis).
    #[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Store malformed values instead of rejecting the document (default false).
    #[serde(rename = "ignore_malformed", default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// Locale used to parse dates (default ROOT).
    #[serde(rename = "locale", default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Value used to represent explicit null values.
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    /// Format used when printing/returning dates in derived source reconstruction.
    #[serde(rename = "print_format", default, skip_serializing_if = "Option::is_none")]
    pub print_format: Option<String>,
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

impl DateProperty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
}

/// `date_nanos` field: nanosecond-precision date (range approx. 1970–2262).
///
/// # Example
/// ```json
/// {
///   "type": "date_nanos",
///   "format": "strict_date_optional_time_nanos"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateNanosProperty {
    /// Relevance score boost factor (default 1.0).
    #[serde(rename = "boost", default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Store column-oriented data (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Date format (default: strict_date_optional_time||epoch_millis).
    #[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Store malformed values instead of rejecting the document (default false).
    #[serde(rename = "ignore_malformed", default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    /// Index this field for searching (default true).
    #[serde(rename = "index", default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    /// Locale used to parse dates (default ROOT).
    #[serde(rename = "locale", default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Value used to represent explicit null values.
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    /// Format used when printing/returning dates.
    #[serde(rename = "print_format", default, skip_serializing_if = "Option::is_none")]
    pub print_format: Option<String>,
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

impl DateNanosProperty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
}
