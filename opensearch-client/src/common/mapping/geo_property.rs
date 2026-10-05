/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Geographic field type property structs.
 */

use serde::{Deserialize, Serialize};

use super::enums::Orientation;

/// `geo_point` field: latitude/longitude coordinates.
///
/// Accepted input formats: `{lat, lon}` object, `"lat,lon"` string, geohash,
/// `[lon, lat]` array, WKT `POINT(lon lat)`, GeoJSON Point.
///
/// # Example
/// ```json
/// {
///   "type": "geo_point",
///   "ignore_malformed": true
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoPointProperty {
    /// Store column-oriented data (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Store malformed geo values instead of rejecting (default false).
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

impl GeoPointProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `geo_shape` field: arbitrary geographic shapes tessellated into a BKD tree.
///
/// Supports GeoJSON and WKT formats. Coordinates are in `[longitude, latitude]` order.
///
/// Shape types: point, linestring, polygon (with holes), multipoint, multilinestring,
/// multipolygon, geometrycollection, envelope/BBOX.
///
/// # Example
/// ```json
/// {
///   "type": "geo_shape",
///   "orientation": "RIGHT"
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoShapeProperty {
    /// Auto-close unclosed linear rings in polygons (default false).
    #[serde(rename = "coerce", default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    /// Store column-oriented data (default true).
    #[serde(rename = "doc_values", default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    /// Store malformed geo values instead of rejecting (default false).
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

impl GeoShapeProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
