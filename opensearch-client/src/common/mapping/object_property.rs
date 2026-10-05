/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Object and nested field type property structs.
 */

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::enums::DynamicMapping;

/// Type alias for a properties map (field name → typed JSON mapping).
///
/// Use `serde_json::from_value::<Property>(v)` to deserialize individual entries.
pub type PropertiesMap = HashMap<String, serde_json::Value>;

/// `object` field: default type for JSON objects.
///
/// Stored internally as flat key-value pairs; subfields accessed via dot notation.
///
/// # Example
/// ```json
/// {
///   "type": "object",
///   "dynamic": "strict",
///   "properties": {
///     "name": {"type": "keyword"}
///   }
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectProperty {
    /// Controls dynamic mapping of new fields within this object.
    #[serde(rename = "dynamic", default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<DynamicMapping>,
    /// Whether this field is parsed and indexed (default true).
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Subfield mappings (use `serde_json::to_value` to convert `Property` values).
    #[serde(rename = "properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesMap>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl ObjectProperty {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a subfield to this object's properties.
    pub fn with_property(mut self, name: impl Into<String>, field: serde_json::Value) -> Self {
        self.properties
            .get_or_insert_with(HashMap::new)
            .insert(name.into(), field);
        self
    }
}

/// `nested` field: each nested object is stored as a separate Lucene document.
///
/// Required when you need to query related fields within the same array element
/// as a unit. Standard objects flatten arrays, losing element boundaries.
///
/// # Example
/// ```json
/// {
///   "type": "nested",
///   "properties": {
///     "tag": {"type": "keyword"},
///     "value": {"type": "float"}
///   }
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedProperty {
    /// Controls dynamic mapping within this nested object.
    #[serde(rename = "dynamic", default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<DynamicMapping>,
    /// Copy nested fields to the parent document as flat fields (default false).
    #[serde(rename = "include_in_parent", default, skip_serializing_if = "Option::is_none")]
    pub include_in_parent: Option<bool>,
    /// Copy nested fields to the root document as flat fields (default false).
    #[serde(rename = "include_in_root", default, skip_serializing_if = "Option::is_none")]
    pub include_in_root: Option<bool>,
    /// Subfield mappings (use `serde_json::to_value` to convert `Property` values).
    #[serde(rename = "properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesMap>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl NestedProperty {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a subfield to this nested object's properties.
    pub fn with_property(mut self, name: impl Into<String>, field: serde_json::Value) -> Self {
        self.properties
            .get_or_insert_with(HashMap::new)
            .insert(name.into(), field);
        self
    }
}

/// `flat_object` field: entire JSON stored as a string; subfields accessible via dot notation.
///
/// Prevents mapping explosion. Not analyzed; no aggregations via dot notation;
/// no numerical operations on subfields.
///
/// Supported queries: term, terms, terms_set, prefix, range, match, multi-match,
/// query_string, simple_query_string, exists, wildcard.
///
/// # Example
/// ```json
/// {
///   "type": "flat_object"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlatObjectProperty {
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl FlatObjectProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `join` field: defines parent/child relationships within a single index.
///
/// Only one join field per index. Child documents must be routed to the same shard
/// as their parent. Use `has_parent` and `has_child` queries.
///
/// # Example
/// ```json
/// {
///   "type": "join",
///   "relations": {
///     "question": "answer"
///   }
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JoinProperty {
    /// Map of parent name(s) to child name(s).
    #[serde(rename = "relations", default, skip_serializing_if = "Option::is_none")]
    pub relations: Option<HashMap<String, serde_json::Value>>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl JoinProperty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_relation(mut self, parent: impl Into<String>, child: serde_json::Value) -> Self {
        self.relations
            .get_or_insert_with(HashMap::new)
            .insert(parent.into(), child);
        self
    }
}
