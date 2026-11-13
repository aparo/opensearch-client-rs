# Bulk Operations Examples

This guide provides comprehensive examples of using bulk operations for efficient data processing.

The bulk API is the streaming `Bulker`: you create it from an `OsClient` with
`client.bulker()`, which spawns a background flusher. You then enqueue
`index` / `create` / `update` / `delete` actions — each call returns once the
action is **queued** (not once it is indexed) — and finally call `flush().await`
to wait for every queued action to be sent. The Bulker batches internally
according to its `bulk_size`, so you never build batches yourself.

## Basic Bulk Operations

### Bulk Insert

```rust
use opensearch_client::OsClient;

// Basic bulk insert
async fn bulk_insert_posts(client: &OsClient, posts: Vec<BlogPost>) -> Result<(), opensearch_client::Error> {
    let bulker = client.bulker().bulk_size(1000).build();

    for post in &posts {
        // index(index, &body, id) — enqueues the action, returns once queued.
        bulker.index("blog_posts", post, Some(post.id.clone())).await?;
    }

    bulker.flush().await;                 // wait for all queued actions to be sent

    let stats = bulker.statistics();
    println!(
        "Bulk insert completed: {} indexed, {} errors",
        stats.index_actions, stats.error_actions
    );
    Ok(())
}

// Bulk insert with explicit IDs
async fn bulk_insert_with_ids(client: &OsClient) -> Result<(), opensearch_client::Error> {
    let posts = vec![
        BlogPost::new("Post 1".to_string(), "Content 1".to_string(), "author1".to_string(), vec![]),
        BlogPost::new("Post 2".to_string(), "Content 2".to_string(), "author1".to_string(), vec![]),
        BlogPost::new("Post 3".to_string(), "Content 3".to_string(), "author2".to_string(), vec![]),
    ];

    let bulker = client.bulker().build();
    for (i, post) in posts.iter().enumerate() {
        bulker.index("blog_posts", post, Some(format!("custom_id_{}", i + 1))).await?;
    }
    bulker.flush().await;

    Ok(())
}
```

### Bulk Update

```rust
use opensearch_client::{OsClient, UpdateActionBody};

// Bulk update existing documents with partial `doc` updates.
async fn bulk_update_posts(
    client: &OsClient,
    updates: Vec<(String, serde_json::Value)>,
) -> Result<(), opensearch_client::Error> {
    let bulker = client.bulker().build();

    for (id, doc) in updates {
        // `UpdateActionBody::new` wraps a partial `doc`.
        bulker.update("blog_posts", &id, &UpdateActionBody::new(doc)).await?;
    }

    bulker.flush().await;
    Ok(())
}

// Bulk upsert (update or insert) using doc-as-upsert.
async fn bulk_upsert_products(
    client: &OsClient,
    products: Vec<Product>,
) -> Result<(), opensearch_client::Error> {
    let bulker = client.bulker().build();

    for product in &products {
        let doc = serde_json::to_value(product)?;
        // `with_doc_upsert` sets `doc_as_upsert = true`.
        bulker
            .update("products", &product.sku, &UpdateActionBody::with_doc_upsert(&doc, &doc))
            .await?;
    }

    bulker.flush().await;
    Ok(())
}
```

### Bulk Delete

```rust
use opensearch_client::OsClient;

// Bulk delete by IDs
async fn bulk_delete_posts(client: &OsClient, ids: Vec<String>) -> Result<(), opensearch_client::Error> {
    let bulker = client.bulker().build();

    for id in &ids {
        // `delete` is generic over the document type (only used for inference).
        bulker.delete::<BlogPost>("blog_posts", id).await?;
    }

    bulker.flush().await;

    let stats = bulker.statistics();
    println!("Delete actions queued: {}", stats.delete_actions);
    Ok(())
}
```

## Advanced Bulk Patterns

### Batch Processing with Error Handling

The `Bulker` already batches internally (by `bulk_size`) and retries transient
failures according to the client configuration (`retries`), so a processor is
just a thin wrapper that enqueues documents and reports the final statistics.

```rust
use opensearch_client::OsClient;

struct BulkProcessor {
    client: OsClient,
    bulk_size: u32,
}

struct BulkResult {
    successful: usize,
    failed: usize,
}

impl BulkProcessor {
    pub fn new(client: OsClient, bulk_size: u32) -> Self {
        Self { client, bulk_size }
    }

    pub async fn process_documents<T>(
        &self,
        index: &str,
        documents: Vec<T>,
    ) -> Result<BulkResult, opensearch_client::Error>
    where
        T: serde::Serialize,
    {
        let bulker = self.client.bulker().bulk_size(self.bulk_size).build();

        for doc in &documents {
            bulker.index(index, doc, None).await?;
        }
        bulker.flush().await;

        let stats = bulker.statistics();
        Ok(BulkResult {
            successful: stats.success_actions,
            failed: stats.error_actions,
        })
    }
}
```

### Streaming Bulk Operations

The real `Bulker` is itself a background, batched indexer, so "streaming" simply
means feeding documents into it as they arrive and flushing at the end.

```rust
use opensearch_client::OsClient;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

async fn streaming_example(client: &OsClient) -> Result<(), Box<dyn std::error::Error>> {
    let bulker = client
        .bulker()
        .bulk_size(100)
        .max_concurrent_connections(4)
        .build();

    let (tx, rx) = mpsc::channel::<BlogPost>(1000);

    // Producer task: simulate a stream of documents.
    tokio::spawn(async move {
        for i in 0..1000 {
            let post = BlogPost::new(
                format!("Post {}", i),
                format!("Content for post {}", i),
                "author1".to_string(),
                vec!["streaming".to_string()],
            );
            if tx.send(post).await.is_err() {
                break; // receiver gone
            }
        }
        // `tx` dropped here closes the channel.
    });

    // Consumer: enqueue each received document into the Bulker.
    let mut stream = ReceiverStream::new(rx);
    while let Some(post) = stream.next().await {
        bulker.index("blog_posts", &post, Some(post.id.clone())).await?;
    }

    bulker.flush().await;
    println!("Streamed {} documents", bulker.statistics().index_actions);
    Ok(())
}
```

### Conditional Bulk Operations

```rust
use opensearch_client::{Error, OsClient, UpdateActionBody};
use serde_json::json;
use chrono::Utc;

// Decide per-document whether to update or insert, then queue the action.
async fn conditional_bulk_operations(client: &OsClient) -> Result<(), opensearch_client::Error> {
    let bulker = client.bulker().build();

    let existing_ids = ["doc1", "doc2", "doc3"];
    for id in existing_ids {
        match BlogPost::get(id).await {
            Ok(_) => {
                // Document exists — queue a partial update.
                bulker
                    .update(
                        "blog_posts",
                        id,
                        &UpdateActionBody::new(json!({
                            "updated_at": Utc::now().to_rfc3339(),
                            "view_count": 0  // Reset view count
                        })),
                    )
                    .await?;
            }
            Err(Error::DocumentNotFoundError(_, _)) => {
                // Document doesn't exist — queue an insert.
                let new_post = BlogPost::new(
                    format!("Generated Post {}", id),
                    "Auto-generated content".to_string(),
                    "system".to_string(),
                    vec!["generated".to_string()],
                );
                bulker.index("blog_posts", &new_post, Some(id.to_string())).await?;
            }
            Err(e) => return Err(e),
        }
    }

    bulker.flush().await;
    Ok(())
}
```

## Data Migration Examples

### Migrating from Another Database

```rust
use opensearch_client::OsClient;
use serde_json::Value;

struct MigrationProcessor {
    client: OsClient,
    bulk_size: u32,
}

impl MigrationProcessor {
    pub async fn migrate_posts_from_json(&self, json_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Read JSON file (could be from MongoDB export, SQL dump, etc.)
        let file_content = tokio::fs::read_to_string(json_file).await?;
        let source_data: Vec<Value> = serde_json::from_str(&file_content)?;

        let bulker = self.client.bulker().bulk_size(self.bulk_size).build();
        let mut transform_errors = 0usize;

        for item in &source_data {
            match self.transform_document(item) {
                Ok(blog_post) => {
                    bulker
                        .index("blog_posts", &blog_post, Some(blog_post.id.clone()))
                        .await?;
                }
                Err(e) => {
                    eprintln!("Failed to transform document: {}", e);
                    transform_errors += 1;
                }
            }
        }

        bulker.flush().await;

        let stats = bulker.statistics();
        println!(
            "Migration completed: {} indexed, {} transform errors, {} index errors",
            stats.success_actions, transform_errors, stats.error_actions
        );
        Ok(())
    }

    fn transform_document(&self, source: &Value) -> Result<BlogPost, Box<dyn std::error::Error>> {
        // Transform source document to BlogPost
        let title = source["title"]
            .as_str()
            .ok_or("Missing title")?
            .to_string();

        let content = source["content"]
            .as_str()
            .ok_or("Missing content")?
            .to_string();

        let author_id = source["author_id"]
            .as_str()
            .or_else(|| source["author"]["id"].as_str())
            .ok_or("Missing author_id")?
            .to_string();

        let tags = source["tags"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let mut post = BlogPost::new(title, content, author_id, tags);

        // Handle optional fields
        if let Some(published) = source["published"].as_bool() {
            post.published = published;
        }

        if let Some(created_at) = source["created_at"].as_str() {
            post.created_at = created_at.to_string();
        }

        Ok(post)
    }
}
```

### Index Reindexing

There is no `Document::scroll` / `clear_scroll`. To reindex, page through the
source index (here with `from`/`size`) and feed the transformed documents into a
`Bulker` targeting the new index.

```rust
use opensearch_client::get_opensearch;
use opensearch_dsl::{Query, Search};

async fn reindex_with_transformation() -> Result<(), opensearch_client::Error> {
    let page_size = 500u64;

    let bulker = get_opensearch().bulker().bulk_size(500).build();
    let mut from = 0u64;
    let mut total_processed = 0u64;

    loop {
        // Fetch the next page from the old index.
        let page = BlogPost::find(
            Search::new().query(Query::match_all()).from(from).size(page_size),
        )
        .await?;

        let hits = page.hits.hits;
        if hits.is_empty() {
            break;
        }

        // Transform and reindex documents into the new index.
        for hit in hits {
            if let Some(post) = hit.source {
                let post = transform_for_new_schema(post);
                bulker
                    .index("blog_posts_v2", &post, Some(post.id.clone()))
                    .await?;
                total_processed += 1;
            }
        }

        from += page_size;
    }

    bulker.flush().await;
    println!("Reindexing completed: {} documents processed", total_processed);
    Ok(())
}

fn transform_for_new_schema(mut post: BlogPost) -> BlogPost {
    // Example transformations for schema changes

    // Normalize tags to lowercase
    post.tags = post.tags
        .into_iter()
        .map(|tag| tag.to_lowercase())
        .collect();

    // Update metadata structure
    post.metadata.word_count = post.content.split_whitespace().count() as u32;
    post.metadata.reading_time_minutes = (post.metadata.word_count / 200).max(1);

    // Add default values for new fields
    if post.view_count == 0 {
        post.view_count = 1; // Set default view count
    }

    post
}
```

These examples demonstrate:
- Basic bulk operations (insert, update, delete) with the streaming `Bulker`
- Automatic batching and background flushing (no manual batch building)
- Streaming documents into the Bulker from a channel
- Conditional operations based on existing data
- Data migration patterns
- Reindexing with transformations
- Performance optimization via `bulk_size` and `max_concurrent_connections`
