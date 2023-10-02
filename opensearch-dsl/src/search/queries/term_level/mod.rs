//! You can use **term-level queries** to find documents based on precise values
//! in structured data. Examples of structured data include date ranges, IP
//! addresses, prices, or product IDs.
//!
//! Unlike full-text queries, term-level queries do not analyze search terms.
//! Instead, term-level queries match the exact terms stored in a field.

mod exists_query;
mod fuzzy_query;
mod ids_query;
mod prefix_query;
mod range_query;
mod regexp_query;
mod term_query;
mod terms_lookup_query;
mod terms_query;
mod terms_set_query;
mod wildcard_query;

pub use self::{
  exists_query::*, fuzzy_query::*, ids_query::*, prefix_query::*, range_query::*, regexp_query::*, term_query::*,
  terms_lookup_query::*, terms_query::*, terms_set_query::*, wildcard_query::*,
};
