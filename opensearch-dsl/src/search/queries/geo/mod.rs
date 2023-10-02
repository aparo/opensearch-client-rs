//! OpenSearch supports two types of geo data: geo_point fields which support
//! lat/lon pairs, and geo_shape fields, which support points, lines, circles,
//! polygons, multi-polygons, etc.

mod geo_bounding_box_query;
mod geo_distance_query;
mod geo_shape_lookup_query;
mod geo_shape_query;

pub use self::{geo_bounding_box_query::*, geo_distance_query::*, geo_shape_lookup_query::*, geo_shape_query::*};
