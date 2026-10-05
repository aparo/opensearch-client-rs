/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Cartesian (2D plane) field type property structs.
 */

use serde::{Deserialize, Serialize};

use super::enums::Orientation;

/// `xy_point` field: single point in 2D Cartesian space (not geographic).
///
/// Accepted input formats: `{x, y}` object, `"x,y"` string, `[x, y]` array,
/// WKT `POINT(x y)`, GeoJSON Point (with x,y coordinates).
///
/// Useful for VR, CAD, venue mapping, game maps. Faster distance calculation
/// than geographic types since no Earth-curve correction is needed.
///
/// # Example
/// ```json
/// {
///   "type": "xy_point",
///   "ignore_malformed": false
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct XyPointProperty {
    /// Store column-oriented data (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Store malformed values instead of rejecting (default false).
    #[serde(rename = "ignore_malformed", default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    /// Silently ignore the Z coordinate in 3D points (default true).
    #[serde(rename = "ignore_z_value", default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    /// Value used to represent explicit null values.
    #[serde(rename = "null_value", default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<serde_json::Value>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl XyPointProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `xy_shape` field: shapes in 2D Cartesian space tessellated into a BKD tree.
///
/// Supports GeoJSON and WKT formats. Coordinates are in `x, y` order.
///
/// Shape types: point, linestring, polygon, multipoint, multilinestring,
/// multipolygon, geometrycollection, envelope.
///
/// # Example
/// ```json
/// {
///   "type": "xy_shape",
///   "orientation": "RIGHT"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct XyShapeProperty {
    /// Auto-close unclosed linear rings in polygons (default false).
    #[serde(rename = "coerce", default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    /// Store column-oriented data (default false for xy_shape).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Store malformed values instead of rejecting (default false).
    #[serde(rename = "ignore_malformed", default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    /// Silently ignore the Z coordinate (default true).
    #[serde(rename = "ignore_z_value", default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    /// Default polygon vertex winding order (default RIGHT = counterclockwise).
    #[serde(rename = "orientation", default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl XyShapeProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
