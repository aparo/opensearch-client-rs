//! # OpenSearch DSL for Rust
//!
//! A high-level, strongly typed Domain Specific Language (DSL) for building OpenSearch queries in Rust.
//! This library provides a complete mapping to the OpenSearch Query DSL with compile-time type safety.
//!
//! *Based on the excellent [elasticsearch-dsl-rs](https://github.com/vinted/elasticsearch-dsl-rs)
//! project, adapted for OpenSearch.*
//!
//! ## Features
//!
//! - **🔒 Type Safety**: Strongly typed queries, aggregations, and responses with compile-time validation
//! - **🎯 Complete Coverage**: Full support for OpenSearch Query DSL including complex nested queries
//! - **📊 Rich Aggregations**: Support for all aggregation types with proper result parsing
//! - **🧩 Composable**: Build complex queries by composing smaller query components
//! - **⚡ Zero-Cost Abstractions**: Compiles to efficient JSON with no runtime overhead
//! - **🔌 Client Agnostic**: Works with any HTTP client, not tied to specific OpenSearch client libraries
//! - **📝 Auto-Generated JSON**: Automatically produces valid OpenSearch JSON from Rust code
//! - **🎨 Fluent API**: Chainable method calls for intuitive query building
//!
//! ## Installation
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! opensearch-dsl = "0.3"
//! ```
//!
//! ## Quick Start
//!
//! ### Basic Search Query
//!
//! ```rust
//! use opensearch_dsl::*;
//!
//! let search = Search::new()
//!     .source(false)
//!     .from(0)
//!     .size(10)
//!     .query(Query::match_all())
//!     .sort(vec![Sort::field("timestamp").desc()]);
//!
//! // Generates valid OpenSearch JSON
//! let json = serde_json::to_string(&search)?;
//! ```
//!
//! ### Complex Boolean Query
//!
//! ```rust
//! let search = Search::new()
//!     .query(
//!         Query::bool()
//!             .must(vec![
//!                 Query::match_("title", "OpenSearch"),
//!                 Query::range("date").gte("2023-01-01")
//!             ])
//!             .should(vec![
//!                 Query::term("category", "tutorial"),
//!                 Query::term("featured", true)
//!             ])
//!             .filter(vec![
//!                 Query::term("status", "published")
//!             ])
//!             .minimum_should_match(1)
//!     );
//! ```
//!
//! ### Aggregations
//!
//! ```rust
//! let search = Search::new()
//!     .size(0)
//!     .aggregations(vec![
//!         ("categories", Aggregation::terms("category")),
//!         ("monthly_sales",
//!             Aggregation::date_histogram("date", "month")
//!                 .sub_aggregation("total_revenue", Aggregation::sum("price"))
//!         )
//!     ]);
//! ```
//!
//! ## Module Organization
//!
//! - [`search`] - Search queries and request building
//! - [`query`] - All query types (term, match, bool, etc.)
//! - [`aggregation`] - Aggregation types and builders
//! - [`sort`] - Sorting options and configurations
//! - [`types`] - Common types and utilities
//!
//! ## Integration with OpenSearch Client
//!
//! This DSL works seamlessly with the opensearch-client:
//!
//! ```rust
//! use opensearch_client::*;
//! use opensearch_dsl::*;
//!
//! let client = OsClient::new(/* configuration */);
//! let search = Search::new().query(Query::match_("title", "rust"));
//! let response = client.search(&search).index("articles").await?;
//! ```
//!
//! ## Examples
//!
//! For comprehensive examples including e-commerce search, log analytics, and time series analysis,
//! see the [examples directory](https://github.com/aparo/opensearch-client-rs/tree/main/examples).
//!
//! use opensearch_dsl::*;
//!
//! fn main() {
//!   let query = Search::new()
//!     .source(false)
//!     .stats("statistics")
//!     .from(0)
//!     .size(30)
//!     .query(
//!       Query::bool()
//!         .must(Query::multi_match(
//!           ["title", "description"],
//!           "you know, for search",
//!         ))
//!         .filter(Query::terms("tags", ["opensearch"]))
//!         .should(Query::term("verified", true).boost(10)),
//!     )
//!     .aggregate(
//!       "country_ids",
//!       Aggregation::terms("country_id")
//!         .aggregate("catalog_ids", Aggregation::terms("catalog_id"))
//!         .aggregate("company_ids", Aggregation::terms("company_id"))
//!         .aggregate(
//!           "top1",
//!           Aggregation::top_hits()
//!             .size(1)
//!             .sort(FieldSort::ascending("user_id")),
//!         ),
//!     )
//!     .rescore(Rescore::new(Query::term("field", 1)).query_weight(1.2));
//! }
//! ```
//!
//! See examples for more.
//!
//! #### License
//!
//! <sup>
//! Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
//! 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
//! </sup>
#![doc(
    html_logo_url = "https://play-lh.googleusercontent.com/VvtT2Dvf_oOC3DL4c2i5hfNvwIqzdU2apScRMlmeRW10Yf-vJXnXqAjdNWE9KW5YvK0"
)]
#![deny(
  bad_style,
  dead_code,
  deprecated,
  improper_ctypes,
  missing_debug_implementations,
  missing_docs,
  non_shorthand_field_patterns,
  no_mangle_generic_items,
  overflowing_literals,
  path_statements,
  patterns_in_fns_without_body,
  // private_in_public,
  trivial_casts,
  trivial_numeric_casts,
  unconditional_recursion,
  unknown_lints,
  unreachable_code,
  unreachable_pub,
  unused,
  unused_allocation,
  unused_comparisons,
  unused_extern_crates,
  unused_import_braces,
  unused_mut,
  unused_parens,
  unused_qualifications,
  unused_results,
  warnings,
  while_true
)]
#![allow(ambiguous_glob_reexports)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

// Macro modules
#[macro_use]
mod macros;
mod types;

// Crate modules
#[macro_use]
pub mod util;
pub use self::types::*;

// Public modules
pub mod analyze;
pub mod search;

// Public re-exports
pub use self::{analyze::*, search::*};
