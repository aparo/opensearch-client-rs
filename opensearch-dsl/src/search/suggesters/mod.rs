//! Suggests similar looking terms based on a provided text by using a
//! suggester.
//!
//! <https://www.elastic.co/guide/en/opensearch/reference/current/search-suggesters.html>

mod completion_suggester;
mod suggest_context_query;
mod suggest_fuzziness;
mod suggester;

pub use self::{completion_suggester::*, suggest_context_query::*, suggest_fuzziness::*, suggester::*};
