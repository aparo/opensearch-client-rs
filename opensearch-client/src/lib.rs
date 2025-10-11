//! # OpenSearch Client for Rust
//!
//! A comprehensive Rust client library for OpenSearch with strongly typed APIs and async support.
//!
//! ## Features
//!
//! - **Strongly Typed**: Type-safe APIs with compile-time guarantees
//! - **Async/Await**: Built on tokio for high-performance async operations
//! - **Comprehensive Coverage**: Support for search, indices, cluster management, and more
//! - **Modular Design**: Feature flags for optional functionality
//! - **Production Ready**: Includes retry logic, connection pooling, and error handling
//!
//! ## Quick Start
//!
//! ```rust
//! use opensearch_client::{ConfigurationBuilder, OsClient};
//! use opensearch_dsl::*;
//! use url::Url;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client configuration
//!     let config = ConfigurationBuilder::new()
//!         .base_url(Url::parse("http://localhost:9200")?)
//!         .basic_auth("admin".to_string(), "admin".to_string())
//!         .build();
//!     
//!     let client = OsClient::new(config);
//!     
//!     // Perform a search
//!     let search = Search::new()
//!         .query(Query::match_("title", "opensearch"))
//!         .size(10);
//!     
//!     let response = client.search(&search).index("my_index").await?;
//!     
//!     for hit in response.hits.hits {
//!         println!("Document: {:?}", hit.source);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Module Organization
//!
//! - [`search`] - Search operations and query building
//! - [`indices`] - Index management and operations
//! - [`cluster`] - Cluster health and management
//! - [`bulk`] - Bulk operations for efficient data processing
//! - [`dsl`] - Re-export of opensearch-dsl for query building
//!
//! ## Feature Flags
//!
//! Enable only the features you need:
//!
//! ```toml
//! [dependencies]
//! opensearch-client = { version = "0.3", features = [
//!     "search",
//!     "indices",
//!     "cluster"
//! ] }
//! ```

#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

pub extern crate opensearch_dsl;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;
use std::sync::OnceLock;

/// Re-export of opensearch-dsl for convenient access to query building types.
///
/// This allows users to access DSL types like `Query`, `Search`, and `Aggregation`
/// without needing a separate import.
pub use opensearch_dsl as dsl;

mod bulk;
mod bulker;

pub use bulk::*;
pub use bulker::*;

/// Asynchronous search operations for long-running queries.
#[cfg(feature = "asynchronous_search")]
pub mod asynchronous_search;

/// Cluster and index information in a compact, tabular format.
#[cfg(feature = "cat")]
pub mod cat;

/// Cluster-level operations including health monitoring and settings management.
#[cfg(feature = "cluster")]
pub mod cluster;

/// Common types and utilities used across the client.
pub mod common;

/// Core client functionality and configuration.
pub mod core;

/// Operations for managing dangling indices.
#[cfg(feature = "dangling_indices")]
pub mod dangling_indices;
#[cfg(feature = "indices")]
pub mod indices;
#[cfg(feature = "ingest")]
pub mod ingest;
#[cfg(feature = "insights")]
pub mod insights;
#[cfg(feature = "ism")]
pub mod ism;
#[cfg(feature = "knn")]
pub mod knn;
#[cfg(feature = "ml")]
pub mod ml;
#[cfg(feature = "nodes")]
pub mod nodes;
#[cfg(feature = "notifications")]
pub mod notifications;
#[cfg(feature = "observability")]
pub mod observability;
#[cfg(feature = "ppl")]
pub mod ppl;
// #[cfg(feature = "query")]
pub mod query;
#[cfg(feature = "remote_store")]
pub mod remote_store;
#[cfg(feature = "replication")]
pub mod replication;
#[cfg(feature = "rollups")]
pub mod rollups;
pub mod search;
#[cfg(feature = "security")]
pub mod security;
#[cfg(feature = "snapshot")]
pub mod snapshot;
#[cfg(feature = "sql")]
pub mod sql;
// #[cfg(feature = "tasks")]
pub mod tasks;
#[cfg(feature = "transforms")]
pub mod transforms;

mod client;
#[cfg(feature = "tools")]
pub mod tools;

pub use client::configuration::Configuration;
pub use client::configuration::ConfigurationBuilder;
pub use client::Error;
pub use client::OsClient;
pub use client::ResponseContent;

mod document;
pub use document::*;
pub use opensearch_macro::OpenSearch;
static OPENSEARCH: OnceLock<OsClient> = OnceLock::new();

pub fn set_opensearch(client: OsClient) -> Result<(), OsClient> {
    OPENSEARCH
        .set(client)
        .map_err(|_| OsClient::from_environment().unwrap())
}

pub fn get_opensearch() -> &'static OsClient {
    OPENSEARCH.get_or_init(|| {
        OsClient::from_environment().expect("Failed to load OPENSEARCH configuration")
    })
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use serde::de::DeserializeOwned;

    use super::*;
    fn load_entity<T: DeserializeOwned>(name: &str) -> T {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/base/{name}"));
        let text = std::fs::read_to_string(filename).unwrap();
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn test_document_delete_response() {
        let decoded: crate::common::DocumentDeleteResponse =
            load_entity("document_delete.response.json");
        assert_eq!(decoded.id, String::from("MzcIJX8BA7mbufL6DOwl"));
    }
}
