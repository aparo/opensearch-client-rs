# Document Modeling with Macros

The `opensearch-client` provides a powerful macro system for creating strongly-typed document models that automatically implement the `Document` trait.

## Table of Contents

- [Basic Document Definition](#basic-document-definition)
- [Working with Documents](#working-with-document-models)
- [CRUD Operations](crud-operations.md)
- [Querying and Search](querying.md)
- [Field Type Mapping](field-types.md)
- [Macro Attributes](attributes.md)
- [Field Introspection](introspection.md)
- [Nested Documents](nested-documents.md)
- [Best Practices](best-practices.md)
- [API Reference](api-reference.md)

## Basic Document Definition

```rust
use opensearch_client::{Document, OpenSearch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub active: bool,
    pub profile: UserProfile,  // Nested document
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "user_profiles")]
pub struct UserProfile {
    #[os(id)]
    pub id: String,
    pub bio: String,
    pub website: Option<String>,
    pub location: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "addresses")]
pub struct Address {
    pub street: String,
    pub city: String,
    pub country: String,
    pub zipcode: u32,
}
```

## Working with Document Models

Once you've defined your models, you can use them with full type safety:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new user
    let user = User {
        id: "user123".to_string(),
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
        active: true,
        profile: UserProfile {
            id: "profile123".to_string(),
            bio: "Software developer".to_string(),
            website: Some("https://johndoe.dev".to_string()),
            location: Address {
                street: "123 Main St".to_string(),
                city: "San Francisco".to_string(),
                country: "USA".to_string(),
                zipcode: 94105,
            },
        },
        tags: vec!["developer".to_string(), "rust".to_string()],
    };

    // Save to OpenSearch
    let response = user.save().await?;
    println!("Saved user with ID: {}", response.id);

    // Retrieve by ID
    let retrieved_user = User::get("user123").await?;
    println!("Retrieved: {}", retrieved_user.name);

    Ok(())
}
```

## Document Trait Features

The `Document` trait provides a comprehensive API for working with OpenSearch documents:

### Core Methods

```rust
// Get index name and field metadata
let index = User::index_name();              // "users"
let fields = User::columns();                // Vec<Field> with type info

// Instance methods
let user = User::get("123").await?;
let user_id = user.id();                     // Get document ID
```

## Next Steps

- [CRUD Operations](crud-operations.md) - Learn about create, read, update, delete operations
- [Field Type Mapping](field-types.md) - Understand how Rust types map to OpenSearch types
- [Macro Attributes](attributes.md) - Learn about `#[os(index)]` and `#[os(id)]` attributes
- [Best Practices](best-practices.md) - Guidelines for effective document modeling