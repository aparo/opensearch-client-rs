/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Top-level index type mapping definition.
 */

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common;

use super::{enums::DynamicMapping, DynamicTemplate, Property};

/// Top-level mapping definition for an OpenSearch index.
///
/// # Example
/// ```json
/// {
///   "mappings": {
///     "dynamic": "strict",
///     "_source": {"enabled": true},
///     "properties": {
///       "title":   {"type": "text", "analyzer": "standard"},
///       "status":  {"type": "keyword"},
///       "created": {"type": "date"}
///     }
///   }
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeMapping {
    /// Enable numeric string detection (default false).
    #[serde(rename = "numeric_detection", default, skip_serializing_if = "Option::is_none")]
    pub numeric_detection: Option<bool>,

    /// Legacy all-field configuration.
    #[serde(rename = "all_field", default, skip_serializing_if = "Option::is_none")]
    pub all_field: Option<common::mapping::AllField>,

    /// Whether this mapping is enabled for indexing.
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Data stream timestamp field configuration.
    #[serde(rename = "_data_stream_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub data_stream_timestamp: Option<common::mapping::DataStreamTimestamp>,

    /// Dynamic field mapping behavior.
    #[serde(rename = "dynamic", default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<DynamicMapping>,

    /// Custom routing configuration.
    #[serde(rename = "_routing", default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<common::mapping::RoutingField>,

    /// Source field configuration (storage, includes/excludes).
    #[serde(rename = "_source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<common::mapping::SourceField>,

    /// Size field configuration.
    #[serde(rename = "_size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<common::mapping::SizeField>,

    /// Field mappings (field name → property definition).
    #[serde(rename = "properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Property>>,

    /// Index field configuration.
    #[serde(rename = "index_field", default, skip_serializing_if = "Option::is_none")]
    pub index_field: Option<common::mapping::IndexField>,

    /// Arbitrary mapping-level metadata.
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<common::Metadata>,

    /// Field names field configuration.
    #[serde(rename = "_field_names", default, skip_serializing_if = "Option::is_none")]
    pub field_names: Option<common::mapping::FieldNamesField>,

    /// Enable date string auto-detection (default true).
    #[serde(rename = "date_detection", default, skip_serializing_if = "Option::is_none")]
    pub date_detection: Option<bool>,

    /// Date formats recognized during date detection.
    #[serde(rename = "dynamic_date_formats", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_date_formats: Option<Vec<String>>,

    /// Dynamic template rules (ordered list of named templates).
    #[serde(rename = "dynamic_templates", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_templates: Option<Vec<HashMap<String, DynamicTemplate>>>,
}

impl TypeMapping {
    pub fn new() -> TypeMapping {
        TypeMapping::default()
    }

    /// Add a named field to the properties map.
    pub fn with_property(mut self, name: impl Into<String>, property: Property) -> Self {
        self.properties
            .get_or_insert_with(HashMap::new)
            .insert(name.into(), property);
        self
    }

    /// Set the dynamic mapping behavior.
    pub fn with_dynamic(mut self, dynamic: DynamicMapping) -> Self {
        self.dynamic = Some(dynamic);
        self
    }
}
