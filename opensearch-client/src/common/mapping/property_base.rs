/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Base parameters shared across many property types.
 */

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::enums::DynamicMapping;

/// Common base properties used as a building block for property types that
/// support nested sub-fields and multi-fields.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyBase {
    /// Strings exceeding this byte threshold are not indexed.
    #[serde(rename = "ignore_above", default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<u32>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Nested subfield mappings (raw JSON for flexibility).
    #[serde(rename = "properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Controls dynamic mapping of new fields within this property.
    #[serde(rename = "dynamic", default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<DynamicMapping>,
    /// Multi-field definitions (raw JSON for flexibility).
    #[serde(rename = "fields", default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<serde_json::Value>,
}

impl PropertyBase {
    pub fn new() -> PropertyBase {
        PropertyBase::default()
    }
}
