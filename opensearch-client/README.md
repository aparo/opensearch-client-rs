# OpenSearch Client Library API Documentation

A comprehensive Rust client library for OpenSearch with strongly typed APIs and async support.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Client Creation](#client-creation)
- [Core APIs](#core-apis)
- [Search Operations](#search-operations)
- [Index Management](#index-management)
- [Cluster Operations](#cluster-operations)
- [Bulk Operations](#bulk-operations)
- [Advanced Features](#advanced-features)
- [Error Handling](#error-handling)
- [Examples](#examples)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
opensearch-client = { version = "0.3", features = [
    "search",
    "indices",
    "cluster",
    "tools"
] }
opensearch-dsl = "0.3"
tokio = { version = "1.0", features = ["full"] }
```

## Configuration

### Basic Configuration

```rust
use opensearch_client::{ConfigurationBuilder, OsClient};
use url::Url;

let config = ConfigurationBuilder::new()
    .base_url(Url::parse("http://localhost:9200")?)
    .build();

let client = OsClient::new(config);
```

### Authentication

#### Basic Authentication

```rust
let config = ConfigurationBuilder::new()
    .base_url(Url::parse("https://localhost:9200")?)
    .basic_auth("username".to_string(), "password".to_string())
    .build();
```

#### API Key Authentication

```rust
let config = ConfigurationBuilder::new()
    .base_url(Url::parse("https://localhost:9200")?)
    .api_key("your-api-key".to_string())
    .build();
```

#### Custom Headers

```rust
use std::collections::HashMap;

let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer token".to_string());

let config = ConfigurationBuilder::new()
    .base_url(Url::parse("https://localhost:9200")?)
    .default_headers(headers)
    .build();
```

### Advanced Configuration

```rust
use std::time::Duration;

let config = ConfigurationBuilder::new()
    .base_url(Url::parse("https://localhost:9200")?)
    .timeout(Duration::from_secs(30))
    .retry_attempts(3)
    .retry_delay(Duration::from_millis(500))
    .connection_pool_size(10)
    .build();
```

## Client Creation

### Standard Client

```rust
use opensearch_client::OsClient;

let client = OsClient::new(config);
```

### Shared Client (Arc)

```rust
use std::sync::Arc;

let client = Arc::new(OsClient::new(config));
```

## Core APIs

### Health Check

```rust
// Cluster health
let health = client.cluster().health().await?;
println!("Status: {:?}", health.status);

// Ping the cluster
let ping_result = client.ping().await?;
```

### Information

```rust
// Get cluster info
let info = client.info().await?;
println!("Version: {}", info.version.number);
```

## Search Operations

### Basic Search

```rust
use opensearch_dsl::*;
use serde_json::json;

// Simple search
let response = client
    .search()
    .index("my_index")
    .query(Query::match_all())
    .await?;

for hit in response.hits.hits {
    println!("Document: {:?}", hit.source);
}
```

### Advanced Search with DSL

```rust
let search = Search::new()
    .source(vec!["title", "content"])
    .from(0)
    .size(10)
    .query(
        Query::bool()
            .must(vec![
                Query::match_("title", "opensearch"),
                Query::range("timestamp").gte("2023-01-01")
            ])
            .should(vec![
                Query::term("category", "tech")
            ])
            .minimum_should_match(0)
    )
    .sort(vec![
        Sort::field("timestamp").desc(),
        Sort::field("_score")
    ])
    .aggregations(vec![
        ("categories", Aggregation::terms("category")),
        ("monthly_counts", 
            Aggregation::date_histogram("timestamp", "month")
                .sub_aggregation("avg_score", Aggregation::avg("score"))
        )
    ]);

let response = client.search(&search).index("articles").await?;
```

### Search Templates

```rust
// Put search template
client.put_search_template()
    .id("my_template")
    .body(json!({
        "template": {
            "query": {
                "match": {
                    "{{field}}": "{{value}}"
                }
            }
        }
    }))
    .await?;

// Use search template
let response = client.search_template()
    .index("my_index")
    .body(json!({
        "id": "my_template",
        "params": {
            "field": "title",
            "value": "opensearch"
        }
    }))
    .await?;
```

### Scroll Search

```rust
use opensearch_client::search::*;

// Initial scroll request
let mut response = client
    .search()
    .index("large_index")
    .scroll("1m")
    .size(1000)
    .query(Query::match_all())
    .await?;

// Process first batch
for hit in &response.hits.hits {
    println!("Document: {:?}", hit.source);
}

// Continue scrolling
while let Some(scroll_id) = response.scroll_id.clone() {
    response = client
        .scroll()
        .scroll_id(scroll_id)
        .scroll("1m")
        .await?;
    
    if response.hits.hits.is_empty() {
        break;
    }
    
    for hit in &response.hits.hits {
        println!("Document: {:?}", hit.source);
    }
}
```

### Multi Search

```rust
let searches = vec![
    MultiSearchItem::new().index("index1").query(Query::term("status", "active")),
    MultiSearchItem::new().index("index2").query(Query::range("date").gte("2023-01-01")),
];

let response = client.multi_search(searches).await?;

for (i, search_response) in response.responses.iter().enumerate() {
    match search_response {
        Ok(resp) => println!("Search {}: {} hits", i, resp.hits.total.value),
        Err(err) => println!("Search {} failed: {:?}", i, err),
    }
}
```

## Index Management

### Create Index

```rust
use serde_json::json;

// Simple index creation
client.indices()
    .create("my_index")
    .await?;

// Index with mappings and settings
client.indices()
    .create("complex_index")
    .mappings(json!({
        "properties": {
            "title": {
                "type": "text",
                "analyzer": "standard"
            },
            "timestamp": {
                "type": "date",
                "format": "yyyy-MM-dd'T'HH:mm:ss"
            },
            "tags": {
                "type": "keyword"
            },
            "content": {
                "type": "text",
                "analyzer": "english"
            }
        }
    }))
    .settings(json!({
        "number_of_shards": 2,
        "number_of_replicas": 1,
        "refresh_interval": "30s"
    }))
    .await?;
```

### Index Operations

```rust
// Check if index exists
let exists = client.indices().exists("my_index").await?;

// Get index info
let info = client.indices().get("my_index").await?;

// Delete index
client.indices().delete("my_index").await?;

// Close/Open index
client.indices().close("my_index").await?;
client.indices().open("my_index").await?;

// Refresh index
client.indices().refresh("my_index").await?;

// Flush index
client.indices().flush("my_index").await?;
```

### Index Templates

```rust
// Create index template
client.indices()
    .put_template("logs_template")
    .index_patterns(vec!["logs-*"])
    .template(json!({
        "settings": {
            "number_of_shards": 1,
            "number_of_replicas": 0
        },
        "mappings": {
            "properties": {
                "timestamp": {"type": "date"},
                "level": {"type": "keyword"},
                "message": {"type": "text"}
            }
        }
    }))
    .await?;

// Get template
let template = client.indices().get_template("logs_template").await?;

// Delete template
client.indices().delete_template("logs_template").await?;
```

### Aliases

```rust
// Add alias
client.indices()
    .put_alias("my_index", "my_alias")
    .await?;

// Add alias with filter
client.indices()
    .put_alias("logs-2023-10", "current_logs")
    .filter(json!({
        "range": {
            "timestamp": {
                "gte": "now-7d"
            }
        }
    }))
    .await?;

// Atomic alias operations
client.indices()
    .update_aliases()
    .actions(vec![
        AliasAction::remove("old_index", "my_alias"),
        AliasAction::add("new_index", "my_alias")
    ])
    .await?;
```

## Document Operations

### Index Document

```rust
use serde_json::json;

// Index with auto-generated ID
let response = client
    .index("my_index")
    .body(json!({
        "title": "My Document",
        "content": "This is the content",
        "timestamp": "2023-10-08T12:00:00Z"
    }))
    .await?;

println!("Document ID: {}", response.id);

// Index with specific ID
client
    .index("my_index")
    .id("doc_1")
    .body(json!({
        "title": "Document 1",
        "content": "Content for document 1"
    }))
    .await?;
```

### Get Document

```rust
// Get document by ID
let document = client
    .get("my_index", "doc_1")
    .await?;

if document.found {
    println!("Document: {:?}", document.source);
}

// Get specific fields
let document = client
    .get("my_index", "doc_1")
    .source_includes(vec!["title", "timestamp"])
    .await?;
```

### Update Document

```rust
// Partial update
client
    .update("my_index", "doc_1")
    .doc(json!({
        "title": "Updated Title"
    }))
    .await?;

// Update with script
client
    .update("my_index", "doc_1")
    .script(json!({
        "source": "ctx._source.views += params.increment",
        "params": {
            "increment": 1
        }
    }))
    .await?;

// Upsert
client
    .update("my_index", "doc_1")
    .doc(json!({"title": "New Title"}))
    .upsert(json!({"title": "Default Title", "views": 0}))
    .await?;
```

### Delete Document

```rust
// Delete by ID
client.delete("my_index", "doc_1").await?;

// Delete by query
client
    .delete_by_query("my_index")
    .query(Query::range("timestamp").lt("2023-01-01"))
    .await?;
```

## Bulk Operations

### Basic Bulk Operations

```rust
use opensearch_client::bulk::*;

let mut bulk = BulkOperation::new();

// Add documents
bulk.index("my_index", "1", json!({"title": "Doc 1"}));
bulk.index("my_index", "2", json!({"title": "Doc 2"}));

// Update documents
bulk.update("my_index", "1", json!({"title": "Updated Doc 1"}));

// Delete documents
bulk.delete("my_index", "3");

// Execute bulk operation
let response = client.bulk(bulk).await?;

// Check for errors
for item in response.items {
    match item {
        BulkResponseItem::Index(result) => {
            if let Some(error) = result.error {
                println!("Index error: {:?}", error);
            }
        }
        BulkResponseItem::Update(result) => {
            if let Some(error) = result.error {
                println!("Update error: {:?}", error);
            }
        }
        BulkResponseItem::Delete(result) => {
            if let Some(error) = result.error {
                println!("Delete error: {:?}", error);
            }
        }
    }
}
```

### Bulk Helper

```rust
use opensearch_client::BulkHelper;

let bulk_helper = BulkHelper::new(client)
    .chunk_size(1000)
    .refresh_policy(RefreshPolicy::WaitFor);

// Stream documents
let documents = vec![
    json!({"id": 1, "title": "Document 1"}),
    json!({"id": 2, "title": "Document 2"}),
    // ... more documents
];

bulk_helper
    .index("my_index", documents)
    .await?;
```

## Cluster Operations

### Cluster Health

```rust
// Basic health
let health = client.cluster().health().await?;
println!("Status: {:?}", health.status);
println!("Active shards: {}", health.active_shards);

// Detailed health for specific indices
let health = client.cluster()
    .health()
    .index(vec!["index1", "index2"])
    .level("indices")
    .await?;
```

### Cluster Settings

```rust
// Get cluster settings
let settings = client.cluster().get_settings().await?;

// Update cluster settings
client.cluster()
    .put_settings()
    .persistent(json!({
        "cluster.routing.allocation.disk.watermark.low": "85%"
    }))
    .transient(json!({
        "cluster.routing.allocation.enable": "all"
    }))
    .await?;
```

### Node Information

```rust
// Get all nodes
let nodes = client.nodes().info().await?;

// Get specific node info
let node = client.nodes()
    .info()
    .node_id("node_1")
    .metric(vec!["os", "jvm"])
    .await?;
```

### Cluster Stats

```rust
let stats = client.cluster().stats().await?;
println!("Total nodes: {}", stats.nodes.count.total);
println!("Total indices: {}", stats.indices.count);
```

## Advanced Features

### Search After

```rust
let mut search_after: Option<Vec<serde_json::Value>> = None;

loop {
    let mut search = Search::new()
        .size(1000)
        .sort(vec![Sort::field("timestamp").asc(), Sort::field("_id").asc()])
        .query(Query::match_all());
    
    if let Some(after) = search_after {
        search = search.search_after(after);
    }
    
    let response = client.search(&search).index("my_index").await?;
    
    if response.hits.hits.is_empty() {
        break;
    }
    
    // Process documents
    for hit in &response.hits.hits {
        println!("Document: {:?}", hit.source);
    }
    
    // Get search_after for next page
    if let Some(last_hit) = response.hits.hits.last() {
        search_after = Some(last_hit.sort.clone().unwrap_or_default());
    }
}
```

### Point in Time (PIT)

```rust
// Create PIT
let pit = client.open_point_in_time("my_index", "1m").await?;

// Search with PIT
let response = client
    .search()
    .pit(pit.id.clone(), "1m")
    .size(100)
    .sort(vec![Sort::field("_shard_doc").asc()])
    .query(Query::match_all())
    .await?;

// Close PIT when done
client.close_point_in_time(pit.id).await?;
```

### Async Search

```rust
// Submit async search
let submit_response = client
    .async_search()
    .submit()
    .index("large_index")
    .keep_alive("1h")
    .query(Query::match_all())
    .await?;

// Get results
let search_id = submit_response.id;
let response = client
    .async_search()
    .get(search_id.clone())
    .await?;

// Delete async search
client.async_search().delete(search_id).await?;
```

## Error Handling

### Error Types

```rust
use opensearch_client::error::*;

match client.get("my_index", "doc_1").await {
    Ok(response) => {
        if response.found {
            println!("Document: {:?}", response.source);
        } else {
            println!("Document not found");
        }
    }
    Err(OpenSearchError::NotFound(_)) => {
        println!("Index or document not found");
    }
    Err(OpenSearchError::Unauthorized(_)) => {
        println!("Authentication failed");
    }
    Err(OpenSearchError::RequestTimeout(_)) => {
        println!("Request timed out");
    }
    Err(OpenSearchError::ServerError { status_code, message }) => {
        println!("Server error {}: {}", status_code, message);
    }
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

### Retry Logic

```rust
use opensearch_client::retry::*;

let config = ConfigurationBuilder::new()
    .base_url(Url::parse("http://localhost:9200")?)
    .retry_policy(RetryPolicy::exponential(3, Duration::from_millis(100)))
    .build();
```

## Examples

### Full-Text Search Application

```rust
use opensearch_client::*;
use opensearch_dsl::*;
use serde_json::json;

pub struct SearchService {
    client: OsClient,
}

impl SearchService {
    pub fn new(client: OsClient) -> Self {
        Self { client }
    }
    
    pub async fn search_articles(
        &self, 
        query: &str, 
        page: usize, 
        size: usize
    ) -> Result<SearchResponse, Box<dyn std::error::Error>> {
        let search = Search::new()
            .from(page * size)
            .size(size)
            .query(
                Query::bool()
                    .must(vec![
                        Query::multi_match(
                            vec!["title^2", "content", "tags"],
                            query
                        )
                    ])
                    .filter(vec![
                        Query::term("status", "published")
                    ])
            )
            .highlight(
                Highlight::new()
                    .field("title", HighlightField::default())
                    .field("content", HighlightField::default())
            )
            .aggregations(vec![
                ("categories", Aggregation::terms("category")),
                ("tags", Aggregation::terms("tags").size(20))
            ]);
        
        let response = self.client
            .search(&search)
            .index("articles")
            .await?;
        
        Ok(response)
    }
    
    pub async fn get_article(&self, id: &str) -> Result<Option<Article>, Box<dyn std::error::Error>> {
        let response = self.client
            .get("articles", id)
            .await?;
        
        if response.found {
            let article: Article = serde_json::from_value(response.source)?;
            Ok(Some(article))
        } else {
            Ok(None)
        }
    }
    
    pub async fn index_article(&self, article: &Article) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client
            .index("articles")
            .id(&article.id)
            .body(serde_json::to_value(article)?)
            .refresh(RefreshPolicy::WaitFor)
            .await?;
        
        Ok(response.id)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: String,
    pub category: String,
    pub tags: Vec<String>,
    pub status: String,
    pub created_at: String,
}
```

### Log Analytics

```rust
use opensearch_client::*;
use opensearch_dsl::*;

pub struct LogAnalytics {
    client: OsClient,
}

impl LogAnalytics {
    pub async fn analyze_errors(&self, time_range: &str) -> Result<(), Box<dyn std::error::Error>> {
        let search = Search::new()
            .size(0)
            .query(
                Query::bool()
                    .must(vec![
                        Query::term("level", "ERROR"),
                        Query::range("timestamp").gte(time_range)
                    ])
            )
            .aggregations(vec![
                ("error_timeline", 
                    Aggregation::date_histogram("timestamp", "1h")
                        .sub_aggregation("unique_errors", 
                            Aggregation::cardinality("message.keyword")
                        )
                ),
                ("top_errors",
                    Aggregation::terms("message.keyword")
                        .size(10)
                        .sub_aggregation("affected_services",
                            Aggregation::terms("service")
                        )
                ),
                ("error_by_service",
                    Aggregation::terms("service")
                        .sub_aggregation("error_rate",
                            Aggregation::date_histogram("timestamp", "1h")
                        )
                )
            ]);
        
        let response = self.client
            .search(&search)
            .index("logs-*")
            .await?;
        
        // Process aggregation results
        if let Some(aggregations) = response.aggregations {
            // Handle timeline data
            if let Some(timeline) = aggregations.get("error_timeline") {
                // Process timeline aggregation
            }
            
            // Handle top errors
            if let Some(top_errors) = aggregations.get("top_errors") {
                // Process top errors aggregation
            }
        }
        
        Ok(())
    }
}
```

This documentation provides comprehensive coverage of the OpenSearch client library API. For more specific use cases and advanced features, refer to the inline documentation and examples in the codebase.