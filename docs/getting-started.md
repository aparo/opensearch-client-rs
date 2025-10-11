# Installation & Quick Start

This guide will help you get started with the OpenSearch Client for Rust.

## 🛠 Installation

Add the dependencies to your `Cargo.toml`:

```toml
[dependencies]
opensearch-client = "0.3"
opensearch-dsl = "0.3"
```

The macro support is included by default - no additional features needed!

### Feature Flags

You can customize which features to include:

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
- `search` - Search APIs (default)
- `indices` - Index management (default)
- `cluster` - Cluster APIs (default)
- `ml` - Machine learning APIs (default)
- `security` - Security APIs
- `tools` - Utility tools

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

## 🚀 Next Steps

- [Document Modeling Guide](documents/README.md) - Learn about the macro system and Document trait
- [Query Building Guide](queries/README.md) - Deep dive into the DSL query system
- [Configuration Guide](configuration.md) - Advanced client configuration options
- [Examples](examples/README.md) - Practical examples and patterns