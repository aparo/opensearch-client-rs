//! Rescore clause to run second query over original one results and that way
//! give more accuracy for final results <https://www.elastic.co/guide/en/opensearch/reference/6.8/search-request-rescore.html>

mod rescore_;
mod rescore_collection;

pub use self::{rescore_::*, rescore_collection::*};
