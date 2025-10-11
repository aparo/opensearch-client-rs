# Bulk Operations Examples

This guide provides comprehensive examples of using bulk operations for efficient data processing.

## Basic Bulk Operations

### Bulk Insert

```rust
use opensearch_client::{BulkOperation, Bulker};

// Basic bulk insert
async fn bulk_insert_posts(posts: Vec<BlogPost>) -> Result<(), opensearch_client::Error> {
    let operations: Vec<BulkOperation> = posts
        .into_iter()
        .map(|post| BulkOperation::index(post))
        .collect();

    let bulker = Bulker::new();
    let response = bulker.execute(operations).await?;

    // Check for errors
    if response.errors {
        for item in response.items {
            if let Some(error) = item.index.and_then(|i| i.error) {
                eprintln!("Bulk error: {}", error);
            }
        }
    }

    println!("Bulk insert completed: {} operations", response.items.len());
    Ok(())
}

// Bulk insert with custom IDs
async fn bulk_insert_with_ids() -> Result<(), opensearch_client::Error> {
    let posts = vec![
        BlogPost::new("Post 1".to_string(), "Content 1".to_string(), "author1".to_string(), vec![]),
        BlogPost::new("Post 2".to_string(), "Content 2".to_string(), "author1".to_string(), vec![]),
        BlogPost::new("Post 3".to_string(), "Content 3".to_string(), "author2".to_string(), vec![]),
    ];

    let operations: Vec<BulkOperation> = posts
        .into_iter()
        .enumerate()
        .map(|(i, post)| {
            BulkOperation::index(post)
                .id(format!("custom_id_{}", i + 1))
        })
        .collect();

    let bulker = Bulker::new();
    bulker.execute(operations).await?;

    Ok(())
}
```

### Bulk Update

```rust
// Bulk update existing documents
async fn bulk_update_posts(updates: Vec<(String, serde_json::Value)>) -> Result<(), opensearch_client::Error> {
    let operations: Vec<BulkOperation> = updates
        .into_iter()
        .map(|(id, update_doc)| {
            BulkOperation::update(&id, update_doc)
                .doc_as_upsert(false)  // Don't create if doesn't exist
        })
        .collect();

    let bulker = Bulker::new();
    let response = bulker.execute(operations).await?;

    // Process results
    for (i, item) in response.items.iter().enumerate() {
        if let Some(update_result) = &item.update {
            match &update_result.result {
                Some(result) => println!("Operation {}: {}", i, result),
                None => {
                    if let Some(error) = &update_result.error {
                        eprintln!("Update error {}: {}", i, error);
                    }
                }
            }
        }
    }

    Ok(())
}

// Bulk upsert (update or insert)
async fn bulk_upsert_products(products: Vec<Product>) -> Result<(), opensearch_client::Error> {
    let operations: Vec<BulkOperation> = products
        .into_iter()
        .map(|product| {
            let id = product.sku.clone();
            BulkOperation::update(&id, serde_json::to_value(&product).unwrap())
                .doc_as_upsert(true)  // Create if doesn't exist
        })
        .collect();

    let bulker = Bulker::new();
    bulker.execute(operations).await?;

    Ok(())
}
```

### Bulk Delete

```rust
// Bulk delete by IDs
async fn bulk_delete_posts(ids: Vec<String>) -> Result<(), opensearch_client::Error> {
    let operations: Vec<BulkOperation> = ids
        .into_iter()
        .map(|id| BulkOperation::delete(&id))
        .collect();

    let bulker = Bulker::new();
    let response = bulker.execute(operations).await?;

    let deleted_count = response.items
        .iter()
        .filter(|item| {
            item.delete
                .as_ref()
                .map(|d| d.result.as_ref() == Some(&"deleted".to_string()))
                .unwrap_or(false)
        })
        .count();

    println!("Deleted {} documents", deleted_count);
    Ok(())
}
```

## Advanced Bulk Patterns

### Batch Processing with Error Handling

```rust
use std::collections::HashMap;

struct BulkProcessor {
    batch_size: usize,
    retry_count: usize,
}

impl BulkProcessor {
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size,
            retry_count: 3,
        }
    }

    pub async fn process_documents<T>(&self, documents: Vec<T>) -> Result<BulkResult, opensearch_client::Error>
    where
        T: serde::Serialize + Clone,
    {
        let mut total_success = 0;
        let mut total_errors = 0;
        let mut failed_operations = Vec::new();

        // Process in batches
        for chunk in documents.chunks(self.batch_size) {
            let operations: Vec<BulkOperation> = chunk
                .iter()
                .map(|doc| BulkOperation::index(doc.clone()))
                .collect();

            match self.execute_with_retry(operations).await {
                Ok(result) => {
                    total_success += result.successful;
                    total_errors += result.failed;
                    failed_operations.extend(result.failed_operations);
                }
                Err(e) => {
                    eprintln!("Batch failed completely: {}", e);
                    total_errors += chunk.len();
                }
            }
        }

        Ok(BulkResult {
            successful: total_success,
            failed: total_errors,
            failed_operations,
        })
    }

    async fn execute_with_retry(&self, operations: Vec<BulkOperation>) -> Result<BulkResult, opensearch_client::Error> {
        let mut attempts = 0;
        let mut current_operations = operations;

        while attempts < self.retry_count {
            let bulker = Bulker::new();
            let response = bulker.execute(current_operations.clone()).await?;

            let mut successful = 0;
            let mut failed_ops = Vec::new();

            for (i, item) in response.items.iter().enumerate() {
                let has_error = item.index
                    .as_ref()
                    .and_then(|idx| idx.error.as_ref())
                    .is_some() ||
                    item.update
                        .as_ref()
                        .and_then(|upd| upd.error.as_ref())
                        .is_some();

                if has_error {
                    failed_ops.push(current_operations[i].clone());
                } else {
                    successful += 1;
                }
            }

            if failed_ops.is_empty() {
                return Ok(BulkResult {
                    successful,
                    failed: 0,
                    failed_operations: Vec::new(),
                });
            }

            // Retry failed operations
            current_operations = failed_ops;
            attempts += 1;

            // Exponential backoff
            if attempts < self.retry_count {
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    100 * (2_u64.pow(attempts as u32))
                )).await;
            }
        }

        Ok(BulkResult {
            successful: 0,
            failed: current_operations.len(),
            failed_operations: current_operations,
        })
    }
}

struct BulkResult {
    successful: usize,
    failed: usize,
    failed_operations: Vec<BulkOperation>,
}
```

### Streaming Bulk Operations

```rust
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

struct StreamingBulker {
    batch_size: usize,
    flush_interval: tokio::time::Duration,
}

impl StreamingBulker {
    pub fn new(batch_size: usize, flush_interval_ms: u64) -> Self {
        Self {
            batch_size,
            flush_interval: tokio::time::Duration::from_millis(flush_interval_ms),
        }
    }

    pub async fn start_processing<T>(&self) -> mpsc::Sender<T>
    where
        T: serde::Serialize + Send + 'static,
    {
        let (tx, rx) = mpsc::channel::<T>(1000);
        let batch_size = self.batch_size;
        let flush_interval = self.flush_interval;

        tokio::spawn(async move {
            let mut stream = ReceiverStream::new(rx);
            let mut batch = Vec::new();
            let mut flush_timer = tokio::time::interval(flush_interval);

            loop {
                tokio::select! {
                    // Receive new document
                    doc = stream.next() => {
                        match doc {
                            Some(document) => {
                                batch.push(BulkOperation::index(document));
                                
                                if batch.len() >= batch_size {
                                    Self::flush_batch(&mut batch).await;
                                }
                            }
                            None => {
                                // Channel closed, flush remaining and exit
                                if !batch.is_empty() {
                                    Self::flush_batch(&mut batch).await;
                                }
                                break;
                            }
                        }
                    }
                    
                    // Periodic flush
                    _ = flush_timer.tick() => {
                        if !batch.is_empty() {
                            Self::flush_batch(&mut batch).await;
                        }
                    }
                }
            }
        });

        tx
    }

    async fn flush_batch(batch: &mut Vec<BulkOperation>) {
        if batch.is_empty() {
            return;
        }

        let operations = std::mem::take(batch);
        let bulker = Bulker::new();
        
        match bulker.execute(operations).await {
            Ok(response) => {
                if response.errors {
                    eprintln!("Bulk operation had errors");
                    for item in &response.items {
                        if let Some(error) = item.index.as_ref().and_then(|i| i.error.as_ref()) {
                            eprintln!("Error: {}", error);
                        }
                    }
                } else {
                    println!("Successfully processed {} operations", response.items.len());
                }
            }
            Err(e) => eprintln!("Bulk operation failed: {}", e),
        }
    }
}

// Usage example
async fn streaming_example() -> Result<(), Box<dyn std::error::Error>> {
    let bulker = StreamingBulker::new(100, 5000); // Batch size 100, flush every 5 seconds
    let sender = bulker.start_processing::<BlogPost>().await;

    // Simulate streaming data
    for i in 0..1000 {
        let post = BlogPost::new(
            format!("Post {}", i),
            format!("Content for post {}", i),
            "author1".to_string(),
            vec!["streaming".to_string()],
        );

        if sender.send(post).await.is_err() {
            break; // Channel closed
        }

        // Simulate some delay between documents
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // Close the channel to flush remaining documents
    drop(sender);
    
    // Wait a bit for final flush
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(())
}
```

### Conditional Bulk Operations

```rust
// Bulk operations with versioning
async fn versioned_bulk_update(updates: Vec<(String, serde_json::Value, u64)>) -> Result<(), opensearch_client::Error> {
    let operations: Vec<BulkOperation> = updates
        .into_iter()
        .map(|(id, doc, version)| {
            BulkOperation::update(&id, doc)
                .version(version)
                .version_type("external")
        })
        .collect();

    let bulker = Bulker::new();
    let response = bulker.execute(operations).await?;

    // Handle version conflicts
    for item in response.items {
        if let Some(update_result) = item.update {
            if let Some(error) = update_result.error {
                if error.contains("version_conflict") {
                    println!("Version conflict detected, document was modified");
                }
            }
        }
    }

    Ok(())
}

// Conditional bulk operations based on existing data
async fn conditional_bulk_operations() -> Result<(), opensearch_client::Error> {
    let mut operations = Vec::new();

    // Check which documents exist
    let existing_ids = vec!["doc1", "doc2", "doc3"];
    for id in existing_ids {
        match BlogPost::get(id).await {
            Ok(_) => {
                // Document exists, update it
                operations.push(
                    BulkOperation::update(id, serde_json::json!({
                        "updated_at": Utc::now().to_rfc3339(),
                        "view_count": 0  // Reset view count
                    }))
                );
            }
            Err(_) => {
                // Document doesn't exist, create it
                let new_post = BlogPost::new(
                    format!("Generated Post {}", id),
                    "Auto-generated content".to_string(),
                    "system".to_string(),
                    vec!["generated".to_string()],
                );
                operations.push(BulkOperation::index(new_post).id(id));
            }
        }
    }

    let bulker = Bulker::new();
    bulker.execute(operations).await?;

    Ok(())
}
```

## Data Migration Examples

### Migrating from Another Database

```rust
use serde_json::Value;

struct MigrationProcessor {
    source_batch_size: usize,
    target_batch_size: usize,
}

impl MigrationProcessor {
    pub async fn migrate_posts_from_json(&self, json_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Read JSON file (could be from MongoDB export, SQL dump, etc.)
        let file_content = tokio::fs::read_to_string(json_file).await?;
        let source_data: Vec<Value> = serde_json::from_str(&file_content)?;

        let mut migrated = 0;
        let mut errors = 0;

        for chunk in source_data.chunks(self.target_batch_size) {
            let mut operations = Vec::new();

            for item in chunk {
                match self.transform_document(item) {
                    Ok(blog_post) => {
                        operations.push(BulkOperation::index(blog_post));
                    }
                    Err(e) => {
                        eprintln!("Failed to transform document: {}", e);
                        errors += 1;
                    }
                }
            }

            if !operations.is_empty() {
                let bulker = Bulker::new();
                match bulker.execute(operations).await {
                    Ok(response) => {
                        migrated += response.items.len();
                        if response.errors {
                            eprintln!("Some documents failed during bulk insert");
                        }
                    }
                    Err(e) => {
                        eprintln!("Bulk operation failed: {}", e);
                        errors += chunk.len();
                    }
                }
            }
        }

        println!("Migration completed: {} migrated, {} errors", migrated, errors);
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

```rust
async fn reindex_with_transformation() -> Result<(), opensearch_client::Error> {
    let scroll_size = 1000;
    let bulk_size = 500;
    
    // Start scroll search on old index
    let mut search = opensearch_dsl::Search::new()
        .query(opensearch_dsl::Query::match_all())
        .size(scroll_size)
        .scroll("5m");

    let mut scroll_id: Option<String> = None;
    let mut total_processed = 0;

    loop {
        let response = if let Some(ref id) = scroll_id {
            // Continue scrolling
            BlogPost::scroll(id, "5m").await?
        } else {
            // Initial search
            let result = BlogPost::find(search.clone()).await?;
            scroll_id = result.scroll_id.clone();
            result
        };

        if response.hits.hits.is_empty() {
            break;
        }

        // Transform and reindex documents
        let mut operations = Vec::new();

        for hit in response.hits.hits {
            if let Some(mut post) = hit.source {
                // Apply transformations
                post = transform_for_new_schema(post);
                
                operations.push(BulkOperation::index(post));

                if operations.len() >= bulk_size {
                    let bulker = Bulker::new();
                    bulker.execute(operations.clone()).await?;
                    total_processed += operations.len();
                    operations.clear();
                    
                    println!("Processed {} documents", total_processed);
                }
            }
        }

        // Process remaining operations
        if !operations.is_empty() {
            let bulker = Bulker::new();
            bulker.execute(operations).await?;
            total_processed += operations.len();
        }
    }

    // Clear scroll
    if let Some(id) = scroll_id {
        BlogPost::clear_scroll(&id).await?;
    }

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
- Basic bulk operations (insert, update, delete)
- Advanced error handling and retry logic
- Streaming and batched processing
- Conditional operations based on existing data
- Data migration patterns
- Reindexing with transformations
- Performance optimization techniques