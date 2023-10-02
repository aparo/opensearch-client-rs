//! Performs
//! [analysis](https://www.elastic.co/guide/en/opensearch/reference/current/analysis.html)
//! on a text string and returns the resulting tokens.
//!
//! <https://www.elastic.co/guide/en/opensearch/reference/current/indices-analyze.html#analyze-api-query-params>

mod request;
mod response;

pub use self::{request::*, response::*};
