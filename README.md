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

The macro support is included by default - no additional features needed!

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

### Document Modeling with Macros

The `opensearch-client` provides a powerful macro system for creating strongly-typed document models that automatically implement the `Document` trait.

**Quick Navigation:**
- [Basic Document Definition](#basic-document-definition)
- [Working with Documents](#working-with-document-models)
- [CRUD Operations](#crud-operations)
- [Querying and Search](#querying-and-search)
- [Field Type Mapping](#field-type-mapping)
- [Macro Attributes](#macro-attributes)
- [Field Introspection](#field-introspection)
- [Nested Documents](#nested-documents)
- [Best Practices](#best-practices)
- [API Reference](#-document-trait-api-reference)

#### Basic Document Definition

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

### Working with Document Models

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

    // Update the user
    User::update("user123", &json!({
        "age": 31,
        "active": false
    })).await?;

    // Search for users
    let search_results = User::find(
        Search::new()
            .query(Query::term("active", true))
            .size(10)
    ).await?;

    for hit in search_results.hits.hits {
        if let Some(user) = hit.source {
            println!("Found user: {}", user.name);
        }
    }

    // Count active users
    let count = User::count(Some(Query::term("active", true))).await?;
    println!("Active users: {}", count);

    Ok(())
}
```

### Document Trait Features

The `Document` trait provides a comprehensive API for working with OpenSearch documents:

#### Core Methods

```rust
// Get index name and field metadata
let index = User::index_name();              // "users"
let fields = User::columns();                // Vec<Field> with type info

// Instance methods
let user = User::get("123").await?;
let user_id = user.id();                     // Get document ID
```

#### CRUD Operations

```rust
// Create/Update
let response = user.save().await?;

// Read
let user = User::get("123").await?;
let maybe_user = User::find_one(
    Search::new().query(Query::term("email", "john@example.com"))
).await?;

// Update with partial data
User::update("123", &json!({"age": 31})).await?;

// Refresh instance from database
user.refresh().await?;

// Delete
User::delete("123").await?;
```

#### Querying and Search

```rust
// Find with complex queries
let results = User::find(
    Search::new()
        .query(
            Query::bool()
                .must(Query::term("active", true))
                .filter(Query::range("age").gte(18).lte(65))
        )
        .sort([("name.keyword", "asc")])
        .from(0)
        .size(20)
).await?;

// Find all documents
let all_users = User::find_all(Some(100)).await?;

// Count documents
let active_count = User::count(Some(Query::term("active", true))).await?;
let total_count = User::count(None).await?;
```

### Field Type Mapping

The macro automatically maps Rust types to OpenSearch field types:

| Rust Type | OpenSearch Type | Field Type | Notes |
|-----------|-----------------|------------|-------|
| `String`, `&str` | `text` | `string` | Full-text searchable |
| `u32`, `i32`, `u64`, `i64` | `long` | `number` | Aggregatable, sortable |
| `f32`, `f64` | `double` | `number` | Aggregatable, sortable |
| `bool` | `boolean` | `boolean` | Aggregatable, sortable |
| `Vec<T>` | (inner type) | (inner type) | Arrays of the inner type |
| `Option<T>` | (inner type) | (inner type) | Nullable fields |
| Custom structs | `object` | `object` | Nested documents |

### Macro Attributes

#### Index Configuration

```rust
#[derive(OpenSearch)]
#[os(index = "my_index")]  // Required: specify the index name
pub struct MyDocument {
    // fields...
}
```

#### ID Field

```rust
#[derive(OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]              // Mark this field as the document ID
    pub id: String,
    // other fields...
}
```

The ID field must be of type `String` and will be used as the document's unique identifier in OpenSearch.

### Field Introspection

The generated `columns()` method provides detailed field metadata:

```rust
let fields = User::columns();
for field in fields {
    println!("Field: {}", field.name);
    println!("  Type: {}", field.field_type);
    println!("  OpenSearch Type: {}", field.os_type);
    println!("  Aggregatable: {}", field.aggregatable);
    println!("  Searchable: {}", field.searchable);
}
```

This metadata can be used for:
- Dynamic query building
- Index mapping generation
- API documentation
- Query validation

### Nested Documents

When using custom types as fields, the macro recognizes them as nested documents:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,
    pub profile: UserProfile,  // This becomes a nested object
}

// The UserProfile fields are accessible through its own columns() method
let profile_fields = UserProfile::columns();
```

This enables complex document structures while maintaining type safety and field introspection capabilities.

### Best Practices

#### Required Derives
Always include these derives for full functionality:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
```

- `Debug` - For debugging and logging
- `Clone` - Required by the Document trait
- `Serialize`/`Deserialize` - For JSON conversion with serde
- `OpenSearch` - The derive macro that implements Document

#### ID Field Guidelines

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "my_index")]
pub struct MyDoc {
    #[os(id)]
    pub id: String,  // Must be String type
    // other fields...
}
```

- The ID field must be of type `String`
- Exactly one field must be marked with `#[os(id)]`
- The ID field is used for document identification in OpenSearch

#### Index Naming

```rust
#[os(index = "users")]           // Good: lowercase, descriptive
#[os(index = "blog_posts")]      // Good: snake_case for multi-word
#[os(index = "product-reviews")] // Good: kebab-case alternative
```

- Use lowercase names
- Avoid special characters except hyphens and underscores
- Choose descriptive, consistent naming patterns

#### Type Compatibility

```rust
// Supported primitive types
pub struct MyDoc {
    pub text_field: String,
    pub number_field: u32,
    pub flag_field: bool,
    pub optional_field: Option<String>,
    pub list_field: Vec<String>,
    
    // Nested documents (must also derive OpenSearch)
    pub nested_doc: OtherDoc,
    
    // Custom types for complex data
    pub metadata: serde_json::Value,  // For dynamic content
}
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
- `search` - Search APIs (default)
- `indices` - Index management (default)
- `cluster` - Cluster APIs (default)
- `ml` - Machine learning APIs (default)
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

### Document Modeling Examples

```rust
use opensearch_client::{Document, OpenSearch};
use serde::{Deserialize, Serialize};

// Blog post with tags and metadata
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "blog_posts")]
pub struct BlogPost {
    #[os(id)]
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: Author,
    pub tags: Vec<String>,
    pub published: bool,
    pub created_at: String, // ISO 8601 datetime
    pub view_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "authors")]
pub struct Author {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub bio: String,
}

// Example of a non-Document struct for complex data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorProfile {
    pub bio: String,
    pub website: Option<String>,
    pub social_links: Vec<String>,
}

// Using the models
async fn blog_example() -> Result<(), Box<dyn std::error::Error>> {
    let post = BlogPost {
        id: "post-123".to_string(),
        title: "Getting Started with OpenSearch in Rust".to_string(),
        content: "In this post, we'll explore...".to_string(),
        author: Author {
            id: "author-456".to_string(),
            name: "Jane Developer".to_string(),
            email: "jane@example.com".to_string(),
            bio: "Full-stack developer passionate about Rust".to_string(),
        },
        tags: vec!["rust".to_string(), "opensearch".to_string(), "tutorial".to_string()],
        published: true,
        created_at: "2023-10-08T12:00:00Z".to_string(),
        view_count: 0,
    };

    // Save the blog post
    post.save().await?;

    // Search for published posts by tag
    let rust_posts = BlogPost::find(
        Search::new()
            .query(
                Query::bool()
                    .must(Query::term("published", true))
                    .must(Query::term("tags", "rust"))
            )
            .sort([("created_at", "desc")])
    ).await?;

    Ok(())
}
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

## 📖 Document Trait API Reference

The `Document` trait provides a complete ORM-like interface for working with OpenSearch documents. All methods are automatically implemented when you use the `#[derive(OpenSearch)]` macro.

### Static Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `index_name()` | `fn index_name() -> &'static str` | Returns the index name configured with `#[os(index = "...")]` |
| `columns()` | `fn columns() -> Vec<Field>` | Returns field metadata for introspection and mapping |
| `get(id)` | `async fn get(id: &str) -> Result<Self, Error>` | Fetch a document by ID |
| `delete(id)` | `async fn delete(id: &str) -> Result<DocumentDeleteResponse, Error>` | Delete a document by ID |
| `update(id, doc)` | `async fn update(id: &str, partial_doc: &Value) -> Result<IndexResponse, Error>` | Update document with partial data |
| `find(search)` | `async fn find(search: Search) -> Result<TypedSearchResult<Self>, Error>` | Search with custom query |
| `find_all(limit)` | `async fn find_all(limit: Option<usize>) -> Result<TypedSearchResult<Self>, Error>` | Find all documents with optional limit |
| `find_one(search)` | `async fn find_one(search: Search) -> Result<Option<Self>, Error>` | Find single document matching query |
| `count(query)` | `async fn count(query: Option<Query>) -> Result<u32, Error>` | Count documents matching query |

### Instance Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `id()` | `fn id(&self) -> &str` | Get the document's ID (from field marked with `#[os(id)]`) |
| `save()` | `async fn save(&self) -> Result<IndexResponse, Error>` | Create or update this document |
| `refresh()` | `async fn refresh(&mut self) -> Result<(), Error>` | Reload this instance from OpenSearch |

### Field Metadata

The `Field` struct returned by `columns()` contains:

```rust
pub struct Field {
    pub name: String,           // Field name
    pub field_type: String,     // Human-readable type (string, number, boolean, object)
    pub os_type: String,        // OpenSearch mapping type (text, long, boolean, object)
    pub aggregatable: bool,     // Can be used in aggregations
    pub searchable: bool,       // Can be searched/filtered
    pub sub_fields: Vec<Box<Field>>, // Nested fields (for object types)
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
docker run -d -p 9200:9200 opensearchproject/opensearch:3.2.0
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