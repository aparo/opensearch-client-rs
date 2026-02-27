# Basic Operations Examples

These examples cover the most common day-to-day operations. Each example is self-contained and can be run with a local OpenSearch instance.

## Setup

All examples assume:

```toml
# Cargo.toml
[dependencies]
opensearch-client = "0.3"
opensearch-dsl = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
url = "2"
```

```rust
use opensearch_client::{ConfigurationBuilder, OsClient};
use url::Url;

fn build_client() -> OsClient {
    let config = ConfigurationBuilder::new()
        .base_url(Url::parse("http://localhost:9200").unwrap())
        .build();
    OsClient::new(config)
}
```

## Cluster Health Check

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = build_client();
    let health = client.cluster().health().send().await?;
    println!("status: {:?}", health.status);
    Ok(())
}
```

## Indexing a Document

```rust
use serde_json::json;

let client = build_client();

let resp = client
    .index("blog_posts")
    .id("post-1")
    .body(json!({
        "title": "Getting Started with OpenSearch",
        "content": "OpenSearch is a fork of Elasticsearch...",
        "published": true,
        "created_at": "2024-01-15T10:00:00Z"
    }))
    .send()
    .await?;

println!("indexed: {} result={}", resp.id_, resp.result);
```

## Getting a Document by ID

```rust
let resp = client
    .get("blog_posts", "post-1")
    .send()
    .await?;

if resp.found {
    let doc: serde_json::Value = resp.source_.unwrap();
    println!("title: {}", doc["title"]);
}
```

## Updating a Document

### Partial update (only change specific fields)

```rust
let resp = client
    .update("blog_posts", "post-1")
    .body(json!({
        "doc": {
            "view_count": 42,
            "updated_at": "2024-01-16T08:00:00Z"
        }
    }))
    .send()
    .await?;

println!("result: {}", resp.result);
```

### Script update (increment a counter)

```rust
let resp = client
    .update("blog_posts", "post-1")
    .body(json!({
        "script": {
            "source": "ctx._source.view_count += params.increment",
            "params": { "increment": 1 }
        }
    }))
    .send()
    .await?;
```

## Deleting a Document

```rust
let resp = client
    .delete("blog_posts", "post-1")
    .send()
    .await?;

println!("result: {}", resp.result);  // "deleted"
```

## Simple Search

```rust
use opensearch_dsl::{Search, Query};

let search = Search::new()
    .query(Query::match_("title", "opensearch"))
    .size(10);

let resp = client
    .search()
    .index("blog_posts")
    .body(&search)
    .send()
    .await?;

println!("total: {}", resp.hits.total.value);
for hit in resp.hits.hits {
    let doc: serde_json::Value = hit.source_.unwrap();
    println!("  - {}", doc["title"]);
}
```

## Upsert (insert or update)

```rust
// Insert the document if it doesn't exist, update it if it does
let resp = client
    .update("blog_posts", "post-99")
    .body(json!({
        "doc": {
            "title": "New or Updated Post",
            "view_count": 0
        },
        "doc_as_upsert": true
    }))
    .send()
    .await?;

println!("result: {}", resp.result); // "created" or "updated"
```

## Delete by Query

```rust
let resp = client
    .delete_by_query("blog_posts")
    .body(json!({
        "query": {
            "term": { "published": false }
        }
    }))
    .send()
    .await?;

println!("deleted: {} documents", resp.deleted);
```

## Count Documents

```rust
let resp = client
    .count()
    .index("blog_posts")
    .body(json!({
        "query": { "term": { "published": true } }
    }))
    .send()
    .await?;

println!("published posts: {}", resp.count);
```

## Using the Document Trait (ORM style)

Define your model once and get all CRUD operations for free:

```rust
use opensearch_client::{Document, OpenSearch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "blog_posts")]
pub struct BlogPost {
    #[os(id)]
    pub id: String,
    pub title: String,
    pub published: bool,
    pub view_count: u32,
}

// Set the global client once at startup
opensearch_client::set_opensearch(build_client());

// Create
let post = BlogPost {
    id: "post-1".to_string(),
    title: "Hello World".to_string(),
    published: true,
    view_count: 0,
};
post.save().await?;

// Read
let found = BlogPost::get("post-1").await?;
println!("{}", found.title);

// Update
BlogPost::update("post-1", &json!({ "view_count": 10 })).await?;

// Delete
BlogPost::delete("post-1").await?;

// Search
let results = BlogPost::find(
    Search::new().query(Query::term("published", true)).size(5)
).await?;
println!("{} results", results.hits.total.value);
```
