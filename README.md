# Strong Opinionated OpenSearch Client for Rust
[![Crates.io](https://img.shields.io/crates/v/opensearch-client.svg)](https://crates.io/crates/opensearch-client)
[![Documentation](https://docs.rs/opensearch-client/badge.svg)](https://docs.rs/opensearch-client)
[![License](https://img.shields.io/crates/l/opensearch-client.svg)](

A high level library, giving a strongly typed DSL that maps one to one with the official OpenSearch query DSL.
This is a Opensearch fork based on the https://github.com/vinted/elasticsearch-dsl-rs

## Features

- Strong typed Request and Response
- Streaming Bulker
- Streaming Response
- Initial implementation of a backup/restore client
- Full typed query based on the [ElasticSearch query DSL by Vinted](https://github.com/vinted/elasticsearch-dsl-rs)
- Exaustive test that show different code usages

## Installation

Add `opensearch-client` crate and version to Cargo.toml

```toml
[dependencies]
opensearch-client = {version="0.1.0", features=["indices"]}
```

Every feature is optional and can be enabled or disabled as needed.
The following features are available:
- default: no feature enabled
- full: all features enabled
- quickwit: enable quickwit compatibility
- search
- cat
- cluster 
- indices: enables also ["search"]
- ingest
- nodes
- mtermvectors
- ml
- remote
- security
- snapshot
- tasks
- tools: enables "indices", "ingest", "ml"

## Documentation

Documentation for the library is available on [docs.rs](https://docs.rs/opensearch-client)

## Quick start

```rust
mod tests {
  use serde_json::json;
  use testcontainers::clients;
  use opensearch_testcontainer::*;
  use tracing_test::traced_test;

  use crate::{url::Url, OsClientBuilder};

  #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
  #[traced_test]
  async fn bulker_ingester() -> Result<(), Box<dyn std::error::Error>> {
    let docker = clients::Cli::default();
    let os_image = OpenSearch::default();
    let node = docker.run(os_image.clone());
    let host_port = node.get_host_port_ipv4(9200);

    let client = OsClientBuilder::new()
      .accept_invalid_certificates(true)
      .base_url(Url::parse(&format!("https://127.0.0.1:{host_port}")).unwrap())
      .basic_auth(os_image.username(), os_image.password())
      .build();

    let test_size: u32 = 100000;
    let bulker = client.bulker().bulk_size(1000).max_concurrent_connections(10).build();
    for i in 0..test_size {
      bulker
        .index("test", &json!({"id":i}), Some(i.to_string()))
        .await
        .unwrap();
    }
    bulker.flush().await;
    let statitics = bulker.statistics();
    drop(bulker);

    assert_eq!(statitics.index_actions, test_size as u64);
    assert_eq!(statitics.create_actions, 0);
    assert_eq!(statitics.delete_actions, 0);
    assert_eq!(statitics.update_actions, 0);
    client.indices().refresh_post().send().await.unwrap();

    let count = client.count().index("test").send().await.unwrap().into_inner();
    assert_eq!(count.count, test_size);
    Ok(())
  }
}
```

If Available the following environment variables are used:

- OPENSEARCH_URL: url of opensearch server (default: http://localhost:9200)
- OPENSEARCH_USER: default user for OpenSearch server (default: admin)
- OPENSEARCH_PASSWORD: default password for OpenSearch server (default: admin)


See [examples](examples) for more.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
