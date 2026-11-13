# Examples Collection

This directory contains practical examples demonstrating various features of the OpenSearch Client for Rust.

## Example Categories

- [Basic Operations](basic-operations.md) - CRUD operations and simple queries
- [Document Modeling](document-modeling.md) - Advanced document structures and relationships
- [Bulk Operations](bulk-operations.md) - Efficient batch processing
- [Search and Queries](search-queries.md) - Complex search patterns and aggregations
- [Real-World Applications](real-world.md) - Complete application examples (blog, log analytics, e-commerce)

## Quick Examples

### Simple Document CRUD

```rust
use opensearch_client::{Document, OpenSearch, ConfigurationBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client — `build()` returns an `OsClient` directly.
    let client = ConfigurationBuilder::new()
        .base_url("http://localhost:9200")
        .build();
    // Register the global client so the Document ORM methods can use it.
    opensearch_client::set_opensearch(client).ok();

    // Create user
    let user = User {
        id: "user123".to_string(),
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
    };

    // Save to OpenSearch
    user.save().await?;

    // Retrieve user
    let retrieved = User::get("user123").await?;
    println!("Retrieved: {}", retrieved.name);

    // Update user
    User::update("user123", &serde_json::json!({"age": 31})).await?;

    // Delete user
    User::delete("user123").await?;

    Ok(())
}
```

### Search with Filters

```rust
use opensearch_dsl::*;

async fn search_example() -> Result<(), Box<dyn std::error::Error>> {
    // Complex search query
    let results = User::find(
        Search::new()
            .query(
                Query::bool()
                    .must([Query::range("age").gte(18).lte(65)])
                    .filter([Query::term("active", true)])
                    .should([Query::r#match("name", "john")])
            )
            .sort([FieldSort::ascending("name.keyword")])
            .from(0)
            .size(10)
    ).await?;

    for hit in results.hits.hits {
        if let Some(user) = hit.source {
            println!("Found: {} (age: {})", user.name, user.age);
        }
    }

    Ok(())
}
```

### Bulk Operations

```rust
use opensearch_client::{get_opensearch, UpdateActionBody};
use serde_json::{json, Value};

async fn bulk_example() -> Result<(), Box<dyn std::error::Error>> {
    // The `Bulker` is a background, batched, streaming indexer created from a
    // client. Enqueue actions, then `flush().await` to wait for them to be sent.
    let client = get_opensearch();
    let bulker = client.bulker().bulk_size(1000).build();

    // Queue multiple actions — each returns once QUEUED, not once indexed.
    bulker.index("users", &json!({"name": "Alice", "age": 25}), Some("1".to_string())).await?;
    bulker.index("users", &json!({"name": "Bob", "age": 30}), Some("2".to_string())).await?;
    bulker.update("users", "3", &UpdateActionBody::new(json!({"age": 31}))).await?;
    bulker.delete::<Value>("users", "4").await?;

    bulker.flush().await;              // wait for all queued actions to be sent
    let stats = bulker.statistics();
    println!("Index actions: {}, errors: {}", stats.index_actions, stats.error_actions);

    Ok(())
}
```

## Getting Started

1. Choose an example that matches your use case
2. Copy the code to your project
3. Modify the OpenSearch URL and credentials as needed
4. Run with `cargo run`

## Prerequisites

Most examples assume:
- OpenSearch running on `localhost:9200`
- Default credentials (admin/admin) or no authentication
- Rust async runtime (tokio)

## Common Patterns

Each example demonstrates different patterns:
- **Document Modeling**: Using derive macros for type safety
- **Error Handling**: Proper error handling with Result types
- **Async Operations**: Modern async/await patterns
- **Configuration**: Various client configuration options
- **Performance**: Efficient bulk operations and pagination