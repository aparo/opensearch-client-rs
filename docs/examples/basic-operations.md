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

fn build_client() -> OsClient {
    // `build()` returns an `OsClient` directly — there is no `OsClient::new` wrapper.
    ConfigurationBuilder::new()
        .base_url("http://localhost:9200")
        .build()
}
```

## Cluster Health Check

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = build_client();
    let health = client.cluster().health().call().await?;
    println!("status: {:?}", health.status);
    Ok(())
}
```

## Indexing a Document

```rust
use serde_json::json;

let client = build_client();

let resp = client
    .index()
    .index("blog_posts")
    .id("post-1")
    .body(json!({
        "title": "Getting Started with OpenSearch",
        "content": "OpenSearch is a fork of Elasticsearch...",
        "published": true,
        "created_at": "2024-01-15T10:00:00Z"
    }))
    .call()
    .await?;

println!("indexed: {} result={}", resp.id, resp.result);
```

## Getting a Document by ID

```rust
let resp = client
    .get()
    .index("blog_posts")
    .id("post-1")
    .call()
    .await?;

if resp.found {
    let doc = resp.source.unwrap();  // source: Option<serde_json::Value>
    println!("title: {}", doc["title"]);
}
```

## Updating a Document

### Partial update (only change specific fields)

```rust
use opensearch_client::common::Update;

// The update body is a typed `common::Update`; build it from JSON (it derives Deserialize).
let body: Update = serde_json::from_value(json!({
    "doc": {
        "view_count": 42,
        "updated_at": "2024-01-16T08:00:00Z"
    }
})).unwrap();

let resp = client
    .update()
    .index("blog_posts")
    .id("post-1")
    .update(body)
    .call()
    .await?;

println!("result: {}", resp.result);
```

### Script update (increment a counter)

```rust
use opensearch_client::common::Update;

let body: Update = serde_json::from_value(json!({
    "script": {
        "source": "ctx._source.view_count += params.increment",
        "params": { "increment": 1 }
    }
})).unwrap();

let resp = client
    .update()
    .index("blog_posts")
    .id("post-1")
    .update(body)
    .call()
    .await?;
```

## Deleting a Document

```rust
let resp = client
    .delete()
    .index("blog_posts")
    .id("post-1")
    .call()
    .await?;

println!("result: {}", resp.result);  // "deleted"
```

## Simple Search

```rust
use opensearch_dsl::{Search, Query};

let search = Search::new()
    .query(Query::r#match("title", "opensearch"))
    .size(10);

let resp = client
    .search()
    .index("blog_posts")
    .search(search)
    .call()
    .await?;

// `resp` is a raw `SearchResponse`: total is `Option<TotalHits>`.
let total = resp.hits.total.map(|t| t.value).unwrap_or(0);
println!("total: {}", total);
for doc in resp.documents::<serde_json::Value>()? {
    println!("  - {}", doc["title"]);
}
```

## Upsert (insert or update)

```rust
use opensearch_client::common::Update;

// Insert the document if it doesn't exist, update it if it does
let body: Update = serde_json::from_value(json!({
    "doc": {
        "title": "New or Updated Post",
        "view_count": 0
    },
    "doc_as_upsert": true
})).unwrap();

let resp = client
    .update()
    .index("blog_posts")
    .id("post-99")
    .update(body)
    .call()
    .await?;

println!("result: {}", resp.result); // "created" or "updated"
```

## Delete by Query

```rust
use opensearch_client::common::DeleteByQuery;

let body: DeleteByQuery = serde_json::from_value(json!({
    "query": {
        "term": { "published": false }
    }
})).unwrap();

let resp = client
    .delete_by_query()
    .index("blog_posts")
    .delete_by_query(body)
    .call()
    .await?;

// `resp` is a `TypedOrJson` and implements `Display`.
println!("delete-by-query response: {resp}");
```

## Count Documents

```rust
let resp = client
    .count()
    .index("blog_posts")
    .query(opensearch_dsl::Query::term("published", true))
    .call()
    .await?;

println!("published posts: {}", resp.count);  // resp.count: u32
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
opensearch_client::set_opensearch(build_client()).ok();

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
println!("{} results", results.hits.get_total_value().unwrap_or(0));
```
