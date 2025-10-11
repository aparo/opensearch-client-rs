# Testing Guide

This guide covers testing strategies and patterns for applications using the OpenSearch client.

## Test Setup

### Basic Test Configuration

```rust
// tests/common.rs
use opensearch_client::{OpenSearch, ClientBuilder};
use serde_json::json;

pub async fn setup_test_client() -> OpenSearch {
    let client = ClientBuilder::new()
        .host("http://localhost:9200")
        .build()
        .expect("Failed to create test client");

    // Wait for OpenSearch to be ready
    for _ in 0..30 {
        if client.ping().await.is_ok() {
            return client;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    panic!("OpenSearch not available for testing");
}

pub async fn cleanup_test_indices(client: &OpenSearch) {
    let indices = ["test_posts", "test_users", "test_*"];
    for index in &indices {
        let _ = client.indices().delete(index).await;
    }
}

pub fn test_blog_post(id: &str) -> BlogPost {
    BlogPost {
        id: id.to_string(),
        title: format!("Test Post {}", id),
        content: "This is test content".to_string(),
        author_id: "test_author".to_string(),
        tags: vec!["test".to_string(), "rust".to_string()],
        published: true,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        view_count: 0,
        metadata: PostMetadata {
            word_count: 4,
            reading_time_minutes: 1,
            featured_image: None,
            seo_description: None,
        },
    }
}
```

### Docker Test Environment

Create a `docker-compose.test.yml` for integration tests:

```yaml
version: '3.8'
services:
  opensearch:
    image: opensearchproject/opensearch:3.2.0
    environment:
      - cluster.name=opensearch-cluster
      - node.name=opensearch-node1
      - discovery.type=single-node
      - bootstrap.memory_lock=true
      - "OPENSEARCH_JAVA_OPTS=-Xms512m -Xmx512m"
      - "DISABLE_INSTALL_DEMO_CONFIG=true"
      - "DISABLE_SECURITY_PLUGIN=true"
    ulimits:
      memlock:
        soft: -1
        hard: -1
      nofile:
        soft: 65536
        hard: 65536
    ports:
      - "9200:9200"
    volumes:
      - opensearch-data:/usr/share/opensearch/data
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9200"]
      interval: 30s
      timeout: 10s
      retries: 5

volumes:
  opensearch-data:
```

## Unit Tests

### Testing Document Models

```rust
// tests/document_tests.rs
use opensearch_client::Document;

#[tokio::test]
async fn test_document_creation() {
    let post = test_blog_post("123");
    
    assert_eq!(post.id(), "123");
    assert_eq!(post.title, "Test Post 123");
    assert_eq!(BlogPost::index_name(), "blog_posts");
}

#[tokio::test]
async fn test_document_serialization() {
    let post = test_blog_post("456");
    
    let json = serde_json::to_value(&post).unwrap();
    assert_eq!(json["title"], "Test Post 456");
    assert_eq!(json["published"], true);
    
    let deserialized: BlogPost = serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.id, post.id);
    assert_eq!(deserialized.title, post.title);
}

#[tokio::test]
async fn test_field_metadata() {
    let fields = BlogPost::columns();
    
    let title_field = fields.iter().find(|f| f.name == "title").unwrap();
    assert_eq!(title_field.field_type, "string");
    assert_eq!(title_field.os_type, "text");
    assert!(title_field.searchable);
    
    let metadata_field = fields.iter().find(|f| f.name == "metadata").unwrap();
    assert_eq!(metadata_field.field_type, "object");
    assert!(!metadata_field.sub_fields.is_empty());
}
```

### Testing Search Queries

```rust
// tests/search_tests.rs
use opensearch_dsl::{Search, Query};

#[tokio::test]
async fn test_query_building() {
    let search = Search::new()
        .query(Query::term("published", true))
        .size(10);
    
    let json = serde_json::to_value(&search).unwrap();
    assert_eq!(json["size"], 10);
    assert_eq!(json["query"]["term"]["published"], true);
}

#[tokio::test]
async fn test_complex_query() {
    let search = Search::new()
        .query(
            Query::bool()
                .must([
                    Query::r#match("title", "rust"),
                    Query::term("published", true)
                ])
                .filter([
                    Query::range("created_at").gte("2024-01-01")
                ])
        );
    
    let json = serde_json::to_value(&search).unwrap();
    let bool_query = &json["query"]["bool"];
    
    assert!(bool_query["must"].is_array());
    assert!(bool_query["filter"].is_array());
}
```

## Integration Tests

### Basic CRUD Operations

```rust
// tests/integration_tests.rs
use crate::common::*;

#[tokio::test]
async fn test_document_save_and_get() {
    let client = setup_test_client().await;
    cleanup_test_indices(&client).await;
    
    let post = test_blog_post("test_save_get");
    
    // Save document
    let save_result = post.save().await;
    assert!(save_result.is_ok());
    
    // Get document
    let retrieved = BlogPost::get("test_save_get").await;
    assert!(retrieved.is_ok());
    
    let retrieved_post = retrieved.unwrap();
    assert_eq!(retrieved_post.id, post.id);
    assert_eq!(retrieved_post.title, post.title);
    
    cleanup_test_indices(&client).await;
}

#[tokio::test]
async fn test_document_update() {
    let client = setup_test_client().await;
    cleanup_test_indices(&client).await;
    
    let mut post = test_blog_post("test_update");
    post.save().await.unwrap();
    
    // Update document
    let update_result = BlogPost::update(
        "test_update",
        &json!({
            "title": "Updated Title",
            "view_count": 42
        })
    ).await;
    
    assert!(update_result.is_ok());
    
    // Verify update
    let updated_post = BlogPost::get("test_update").await.unwrap();
    assert_eq!(updated_post.title, "Updated Title");
    assert_eq!(updated_post.view_count, 42);
    
    cleanup_test_indices(&client).await;
}

#[tokio::test]
async fn test_document_delete() {
    let client = setup_test_client().await;
    cleanup_test_indices(&client).await;
    
    let post = test_blog_post("test_delete");
    post.save().await.unwrap();
    
    // Verify document exists
    assert!(BlogPost::get("test_delete").await.is_ok());
    
    // Delete document
    let delete_result = BlogPost::delete("test_delete").await;
    assert!(delete_result.is_ok());
    
    // Verify document is deleted
    assert!(BlogPost::get("test_delete").await.is_err());
    
    cleanup_test_indices(&client).await;
}
```

### Search Integration Tests

```rust
#[tokio::test]
async fn test_search_operations() {
    let client = setup_test_client().await;
    cleanup_test_indices(&client).await;
    
    // Create test documents
    let posts = vec![
        test_blog_post("search_1"),
        test_blog_post("search_2"),
        test_blog_post("search_3"),
    ];
    
    for post in &posts {
        post.save().await.unwrap();
    }
    
    // Wait for indexing
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // Test find_all
    let all_results = BlogPost::find_all(None).await.unwrap();
    assert!(all_results.hits.hits.len() >= 3);
    
    // Test search with query
    let search_results = BlogPost::find(
        Search::new()
            .query(Query::term("author_id", "test_author"))
            .size(10)
    ).await.unwrap();
    
    assert!(search_results.hits.hits.len() >= 3);
    
    // Test count
    let count = BlogPost::count(Some(
        Query::term("published", true)
    )).await.unwrap();
    
    assert!(count >= 3);
    
    cleanup_test_indices(&client).await;
}
```

### Bulk Operations Tests

```rust
#[tokio::test]
async fn test_bulk_operations() {
    let client = setup_test_client().await;
    cleanup_test_indices(&client).await;
    
    let posts: Vec<BlogPost> = (1..=10)
        .map(|i| test_blog_post(&format!("bulk_{}", i)))
        .collect();
    
    let operations: Vec<BulkOperation> = posts
        .into_iter()
        .map(|post| BulkOperation::index(post))
        .collect();
    
    let bulker = Bulker::new();
    let response = bulker.execute(operations).await.unwrap();
    
    assert!(!response.errors);
    assert_eq!(response.items.len(), 10);
    
    // Wait for indexing
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // Verify documents were created
    let count = BlogPost::count(None).await.unwrap();
    assert!(count >= 10);
    
    cleanup_test_indices(&client).await;
}
```

## Test Utilities and Helpers

### Mock Data Generation

```rust
// tests/fixtures.rs
use fake::{Fake, Faker};

pub struct TestDataGenerator;

impl TestDataGenerator {
    pub fn blog_post() -> BlogPost {
        BlogPost::new(
            Faker.fake::<String>(),                    // Random title
            (50..500).fake::<String>(),                // Random content
            format!("author_{}", (1..10).fake::<u8>()), // Random author
            (0..5).fake::<Vec<String>>(),              // Random tags
        )
    }
    
    pub fn blog_posts(count: usize) -> Vec<BlogPost> {
        (0..count).map(|_| Self::blog_post()).collect()
    }
    
    pub fn user() -> User {
        User {
            id: uuid::Uuid::new_v4().to_string(),
            name: Faker.fake(),
            email: format!("{}@example.com", Faker.fake::<String>()),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
```

### Assertion Helpers

```rust
// tests/assertions.rs
use opensearch_client::SearchResults;

pub trait SearchAssertions<T> {
    fn assert_has_hits(&self, min_count: usize);
    fn assert_contains_id(&self, id: &str);
    fn assert_all_match<F>(&self, predicate: F) where F: Fn(&T) -> bool;
}

impl<T> SearchAssertions<T> for SearchResults<T> 
where 
    T: serde::de::DeserializeOwned + Clone 
{
    fn assert_has_hits(&self, min_count: usize) {
        assert!(
            self.hits.hits.len() >= min_count,
            "Expected at least {} hits, got {}",
            min_count,
            self.hits.hits.len()
        );
    }
    
    fn assert_contains_id(&self, id: &str) {
        let found = self.hits.hits
            .iter()
            .any(|hit| hit.id == id);
        
        assert!(found, "Search results should contain document with ID: {}", id);
    }
    
    fn assert_all_match<F>(&self, predicate: F) 
    where 
        F: Fn(&T) -> bool 
    {
        for hit in &self.hits.hits {
            if let Some(ref source) = hit.source {
                assert!(
                    predicate(source),
                    "Document {:?} failed predicate test",
                    hit.id
                );
            }
        }
    }
}

// Usage example
#[tokio::test]
async fn test_with_assertions() {
    let results = BlogPost::find(
        Search::new()
            .query(Query::term("published", true))
    ).await.unwrap();
    
    results.assert_has_hits(1);
    results.assert_all_match(|post| post.published);
}
```

## Running Tests

### Test Commands

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_document_save_and_get

# Run tests in parallel
cargo test --release -- --test-threads=4
```

### Continuous Integration

Example GitHub Actions workflow:

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      opensearch:
        image: opensearchproject/opensearch:3.2.0
        env:
          discovery.type: single-node
          DISABLE_SECURITY_PLUGIN: true
        ports:
          - 9200:9200
        options: >-
          --health-cmd "curl -f http://localhost:9200"
          --health-interval 30s
          --health-timeout 10s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          
      - name: Run tests
        run: cargo test
        env:
          OPENSEARCH_URL: http://localhost:9200
```

These testing patterns help ensure your OpenSearch client integration is robust and reliable across different scenarios.