//! Search APIs are used to search and aggregate data stored in OpenSearch
//! indices and data streams. For an overview and related tutorials, see
//! [Search your data](https://www.elastic.co/guide/en/opensearch/reference/current/search-your-data.html).
//!
//! Most search APIs support
//! [multi-target syntax](https://www.elastic.co/guide/en/opensearch/reference/current/multi-index.html),
//! with the exception of the
//! [explain API](https://www.elastic.co/guide/en/opensearch/reference/current/search-explain.html).
//!
//! <https://www.elastic.co/guide/en/opensearch/reference/current/search.html>

// Private modules
mod response;

// Public modules
pub mod aggregations;
pub mod collapse;
pub mod highlight;
pub mod knn;
pub mod params;
pub mod queries;
pub mod request;
pub mod rescoring;
pub mod runtime_mappings;
pub mod script_fields;
pub mod sort;
pub mod suggesters;

// Public re-exports
pub use self::{
    aggregations::*,
    collapse::*,
    highlight::*,
    knn::*,
    params::*,
    queries::{params::*, *},
    request::*,
    rescoring::*,
    response::*,
    runtime_mappings::*,
    script_fields::*,
    sort::*,
    suggesters::*,
};
