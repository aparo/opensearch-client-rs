#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

pub extern crate opensearch_dsl;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;
pub use opensearch_dsl as dsl;

mod bulk;
mod bulker;

pub use bulk::*;
pub use bulker::*;

#[cfg(feature = "asynchronous_search")]
pub mod asynchronous_search;
#[cfg(feature = "cat")]
pub mod cat;
#[cfg(feature = "cluster")]
pub mod cluster;
pub mod common;
pub mod core;
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
