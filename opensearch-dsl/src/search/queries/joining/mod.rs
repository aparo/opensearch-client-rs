//! Performing full SQL-style joins in a distributed system like OpenSearch is
//! prohibitively expensive. Instead, OpenSearch offers two forms of join which
//! are designed to scale horizontally.
//!
//! <https://www.elastic.co/guide/en/opensearch/reference/current/joining-queries.html>

mod has_child_query;
mod has_parent_query;
mod nested_query;
mod parent_id_query;

pub use self::{has_child_query::*, has_parent_query::*, nested_query::*, parent_id_query::*};
