# Examples Collection

This directory contains practical examples demonstrating various features of the OpenSearch Client for Rust.

## Example Categories

- [Basic Operations](basic-operations.md) - CRUD operations and simple queries
- [Document Modeling](document-modeling.md) - Advanced document structures and relationships
- [Bulk Operations](bulk-operations.md) - Efficient batch processing
- [Search and Queries](search-queries.md) - Complex search patterns and aggregations
- [Index Management](index-management.md) - Creating and managing indices
- [Real-World Applications](real-world.md) - Complete application examples

## Quick Examples

### Simple Document CRUD

```rust
use opensearch_client::{Document, OpenSearch, ConfigurationBuilder, OsClient};
use serde::{Deserialize, Serialize};
use url::Url;

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
    // Setup client
    let config = ConfigurationBuilder::new()
        .base_url(Url::parse("http://localhost:9200")?)
        .build();
    let _client = OsClient::new(config);

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
                    .must(Query::range("age").gte(18).lte(65))
                    .filter(Query::term("active", true))
                    .should(Query::match_query("name", "john"))
            )
            .sort([("name.keyword", "asc")])
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
use opensearch_client::bulk::*;

async fn bulk_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut bulk = BulkOperation::new();
    
    // Add multiple operations
    bulk.index("users", "1", serde_json::json!({"name": "Alice", "age": 25}));
    bulk.index("users", "2", serde_json::json!({"name": "Bob", "age": 30}));
    bulk.update("users", "3", serde_json::json!({"age": 31}));
    bulk.delete("users", "4");

    // Execute all operations at once
    let response = client.bulk(bulk).await?;
    
    println!("Processed {} items", response.items.len());
    for item in response.items {
        println!("Operation result: {:?}", item);
    }

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