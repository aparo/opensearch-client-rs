/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Dynamic template mapping configuration.
 */

use serde::{Deserialize, Serialize};

use super::enums::MatchPattern;
use super::Property;

/// A dynamic template rule that controls how dynamically detected fields are mapped.
///
/// Dynamic templates are applied in order; the first matching template wins.
/// Templates match based on the field name (`match`/`unmatch`), the JSON type
/// (`match_mapping_type`), or the full dotted path (`path_match`/`path_unmatch`).
///
/// # Example
/// ```json
/// {
///   "strings_as_keyword": {
///     "match_mapping_type": "string",
///     "mapping": {"type": "keyword", "ignore_above": 256}
///   }
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicTemplate {
    /// Glob pattern matching the full dot-notation field path.
    #[serde(rename = "path_match", default, skip_serializing_if = "Option::is_none")]
    pub path_match: Option<String>,
    /// Glob pattern that excludes field paths from matching.
    #[serde(rename = "path_unmatch", default, skip_serializing_if = "Option::is_none")]
    pub path_unmatch: Option<String>,
    /// Glob pattern that excludes field names from matching.
    #[serde(rename = "unmatch", default, skip_serializing_if = "Option::is_none")]
    pub unmatch: Option<String>,
    /// Glob (or regex when match_pattern=regex) matching the field name.
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
    /// JSON type filter: `string`, `long`, `double`, `boolean`, `date`, `object`, `knn_vector`.
    #[serde(rename = "match_mapping_type", default, skip_serializing_if = "Option::is_none")]
    pub match_mapping_type: Option<String>,
    /// The mapping to apply to matched fields.
    #[serde(rename = "mapping", default, skip_serializing_if = "Option::is_none")]
    pub mapping: Option<Property>,
    /// Pattern type for the `match` field: `simple` (glob, default) or `regex`.
    #[serde(rename = "match_pattern", default, skip_serializing_if = "Option::is_none")]
    pub match_pattern: Option<MatchPattern>,
}

impl DynamicTemplate {
    pub fn new() -> DynamicTemplate {
        DynamicTemplate::default()
    }

    /// Create a template that matches all string fields and maps them as keyword.
    pub fn strings_as_keyword() -> DynamicTemplate {
        use super::{KeywordProperty, Property};
        DynamicTemplate {
            match_mapping_type: Some("string".into()),
            mapping: Some(Property::keyword(KeywordProperty::new())),
            ..Default::default()
        }
    }

    /// Create a template that matches all string fields and maps them as text.
    pub fn strings_as_text() -> DynamicTemplate {
        use super::{Property, TextProperty};
        DynamicTemplate {
            match_mapping_type: Some("string".into()),
            mapping: Some(Property::text(TextProperty::new())),
            ..Default::default()
        }
    }
}
