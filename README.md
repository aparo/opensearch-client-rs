# OpenSearch Client for Rust

[![Crates.io](https://img.shields.io/crates/v/opensearch-client)](https://crates.io/crates/opensearch-client)
[![License](https://img.shields.io/crates/l/opensearch-client)](https://crates.io/crates/opensearch-client)
[![Documentation](https://docs.rs/opensearch-client/badge.svg)](https://docs.rs/opensearch-client)

A comprehensive Rust client library for OpenSearch with a strongly typed DSL, CLI tools, and extensive API coverage.

## 🚀 Features

- **Strongly Typed DSL**: Type-safe query building with compile-time guarantees
- **Comprehensive API Coverage**: Support for search, indices, cluster management, and more
- **CLI Tools**: Command-line interface for cluster management and data operations
- **Async/Await Support**: Built on modern async Rust with tokio
- **Production Ready**: Includes retry logic, connection pooling, and error handling
- **Extensible**: Modular design with feature flags for optional functionality

## 📦 Project Structure

This workspace contains several crates:

- **`opensearch-client`**: Core client library with API bindings
- **`opensearch-dsl`**: Strongly typed query DSL
- **`opensearch-cli`**: Command-line tools for cluster management
- **`opensearch-testcontainer`**: Testing utilities with container support

## 🛠 Installation

Add the dependencies to your `Cargo.toml`:

```toml
[dependencies]
opensearch-client = "0.3"
opensearch-dsl = "0.3"
```

## 🔧 Quick Start

### Basic Client Usage

```rust
use opensearch_client::{ConfigurationBuilder, OsClient};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client configuration
    let url = Url::parse("http://localhost:9200")?;
    let config = ConfigurationBuilder::new()
        .base_url(url)
        .basic_auth("admin".to_string(), "admin".to_string())
        .build();
    
    let client = OsClient::new(config);
    
    // Get cluster health
    let health = client.cluster().health().await?;
    println!("Cluster status: {:?}", health);
    
    Ok(())
}
```

### Query Building with DSL

```rust
use opensearch_dsl::*;

let query = Search::new()
    .source(false)
    .from(0)
    .size(10)
    .query(
        Query::bool()
            .must(Query::term("status", "published"))
            .filter(Query::range("date").gte("2023-01-01"))
    )
    .aggregations([
        ("status_count", Aggregation::terms("status")),
        ("avg_score", Aggregation::avg("score"))
    ]);

// Execute the search
let response = client.search(&query).index("my_index").await?;
```

### Index Management

```rust
// Create an index
client.indices()
    .create("my_index")
    .mappings(json!({
        "properties": {
            "title": {"type": "text"},
            "timestamp": {"type": "date"}
        }
    }))
    .await?;

// Index a document
client.index("my_index")
    .id("1")
    .body(json!({
        "title": "Hello OpenSearch",
        "timestamp": "2023-10-08T12:00:00Z"
    }))
    .await?;
```

## 🖥 CLI Tools

The project includes a powerful CLI tool for cluster management:

```bash
# Install the CLI
cargo install opensearch-cli

# List all indices
opensearch-cli list-indices

# Dump cluster metadata
opensearch-cli dump-metadata --output ./backup

# Restore metadata
opensearch-cli restore-metadata --input ./backup

# Copy index between clusters
opensearch-cli copy-index --remote my_index --target-index new_index
```

### CLI Configuration

Set environment variables or use command-line flags:

```bash
export OPENSEARCH_URL="https://my-cluster.example.com:9200"
export OPENSEARCH_USER="admin"
export OPENSEARCH_PASSWORD="password"

# Or use flags
opensearch-cli --server https://my-cluster.example.com:9200 --user admin list-indices
```

## 🏗 Architecture

### Client Library (`opensearch-client`)

The core client provides:
- **HTTP Transport**: Built on reqwest with middleware support
- **Authentication**: Basic auth, API keys, and custom auth
- **API Modules**: Organized by OpenSearch API categories
- **Error Handling**: Comprehensive error types and retry logic
- **Connection Management**: Connection pooling and keep-alive

### DSL Library (`opensearch-dsl`)

The DSL provides type-safe query building:
- **Query Types**: Match, term, bool, range, and more
- **Aggregations**: Bucket, metric, and pipeline aggregations
- **Response Parsing**: Strongly typed response structures
- **Validation**: Compile-time query validation

### CLI Tools (`opensearch-cli`)

Command-line utilities for:
- **Metadata Management**: Index templates, pipelines, components
- **Data Operations**: Dump, restore, and copy indices
- **Cluster Management**: Health checks and monitoring
- **Remote Operations**: Multi-cluster support

## 🔧 Configuration

### Client Configuration

```rust
use opensearch_client::ConfigurationBuilder;

let config = ConfigurationBuilder::new()
    .base_url(url)
    .basic_auth(username, password)
    .timeout(Duration::from_secs(30))
    .retry_attempts(3)
    .build();
```

### Feature Flags

Enable only the features you need:

```toml
[dependencies]
opensearch-client = { version = "0.3", features = [
    "search",
    "indices", 
    "cluster",
    "ml"
] }
```

Available features:
- `search` - Search APIs
- `indices` - Index management
- `cluster` - Cluster APIs
- `ml` - Machine learning APIs
- `security` - Security APIs
- `tools` - Utility tools

## 📚 Examples

### Bulk Operations

```rust
use opensearch_client::bulk::*;

let mut bulk = BulkOperation::new();
bulk.index("my_index", "1", json!({"field": "value1"}));
bulk.index("my_index", "2", json!({"field": "value2"}));
bulk.delete("my_index", "3");

let response = client.bulk(bulk).await?;
```

### Aggregations

```rust
let search = Search::new()
    .aggregations([
        ("sales_over_time", 
            Aggregation::date_histogram("timestamp", "month")
                .sub_aggregation("total_sales", Aggregation::sum("amount"))
        ),
        ("top_products",
            Aggregation::terms("product_id")
                .size(10)
                .order([("total_sales", "desc")])
                .sub_aggregation("total_sales", Aggregation::sum("amount"))
        )
    ]);
```

### Stream Processing

```rust
let mut stream = client.search_stream(query).index("logs").scroll("1m");
while let Some(response) = stream.next().await {
    for hit in response?.hits.hits {
        // Process each document
        println!("{:?}", hit.source);
    }
}
```

## 🧪 Testing

Run the test suite:

```bash
# Unit tests
cargo test

# Integration tests (requires OpenSearch running)
cargo test --features integration-tests

# Test with specific OpenSearch version
docker run -d -p 9200:9200 opensearchproject/opensearch:2.11.0
cargo test
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Clone the repository:
```bash
git clone https://github.com/aparo/opensearch-client-rs.git
cd opensearch-client-rs
```

2. Install dependencies:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Start OpenSearch for integration tests:
```bash
docker run -d -p 9200:9200 \
  -e "discovery.type=single-node" \
  -e "DISABLE_SECURITY_PLUGIN=true" \
  opensearchproject/opensearch:latest
```

## 📄 License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

- [OpenSearch](https://opensearch.org/) - The OpenSearch search engine
- [elasticsearch-dsl-rs](https://github.com/vinted/elasticsearch-dsl-rs) - Original Elasticsearch DSL inspiration
- [opensearch-rs](https://github.com/opensearch-project/opensearch-rs) - Alternative Rust client

## 📞 Support

- [Documentation](https://docs.rs/opensearch-client)
- [GitHub Issues](https://github.com/aparo/opensearch-client-rs/issues)
- [Discussions](https://github.com/aparo/opensearch-client-rs/discussions)

---

Made with ❤️ by the OpenSearch Rust community