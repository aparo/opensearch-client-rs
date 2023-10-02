//! Like geo_shape OpenSearch supports the ability to index arbitrary two
//! dimension (non Geospatial) geometries making it possible to map out virtual
//! worlds, sporting venues, theme parks, and CAD diagrams.
//!
//! OpenSearch supports two types of cartesian data: point fields which support
//! x/y pairs, and shape fields, which support points, lines, circles, polygons,
//! multi-polygons, etc.

mod shape_lookup_query;
mod shape_query;

pub use self::{shape_lookup_query::*, shape_query::*};
