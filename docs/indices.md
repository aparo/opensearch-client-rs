# Index Management

Index management APIs are available when the `indices` feature is enabled (default). Because `indices` depends on `search`, both are compiled together.

```toml
[dependencies]
opensearch-client = { version = "0.3", features = ["indices"] }
```

All operations are accessed via `client.indices()`.

## Creating an Index

### Minimal index

```rust
client.indices().create("blog_posts").send().await?;
```

### With explicit mappings

```rust
use serde_json::json;

client.indices()
    .create("blog_posts")
    .body(json!({
        "settings": {
            "number_of_shards": 1,
            "number_of_replicas": 1
        },
        "mappings": {
            "properties": {
                "title": {
                    "type": "text",
                    "fields": {
                        "keyword": { "type": "keyword", "ignore_above": 256 }
                    }
                },
                "content":    { "type": "text" },
                "published":  { "type": "boolean" },
                "created_at": { "type": "date" },
                "view_count": { "type": "long" },
                "tags":       { "type": "keyword" }
            }
        }
    }))
    .send()
    .await?;
```

### Create only if absent

```rust
let exists = client.indices().exists("blog_posts").send().await?;
if !exists {
    client.indices().create("blog_posts").send().await?;
}
```

## Checking Whether an Index Exists

```rust
let exists: bool = client.indices().exists("blog_posts").send().await?;
```

## Getting Index Information

```rust
// Mappings + settings + aliases in one call
let info = client.indices().get("blog_posts").send().await?;

// Mappings only
let mappings = client.indices().get_mapping("blog_posts").send().await?;

// Settings only
let settings = client.indices().get_settings("blog_posts").send().await?;
```

## Updating Mappings

You can add new fields to an existing index, but you cannot change existing field types (you'd need to reindex for that).

```rust
client.indices()
    .put_mapping("blog_posts")
    .body(json!({
        "properties": {
            "author_id": { "type": "keyword" },
            "word_count": { "type": "integer" }
        }
    }))
    .send()
    .await?;
```

## Updating Settings

Dynamic settings can be changed on a live index. Static settings (like `number_of_shards`) require closing the index first.

```rust
// Dynamic setting — applies immediately
client.indices()
    .put_settings("blog_posts")
    .body(json!({
        "index": {
            "number_of_replicas": 2,
            "refresh_interval": "30s"
        }
    }))
    .send()
    .await?;
```

## Aliases

Aliases allow zero-downtime index migrations and provide a stable name for the application.

```rust
// Add an alias
client.indices()
    .put_alias("blog_posts", "posts")
    .send()
    .await?;

// Alias with a filter (filtered alias — documents are pre-filtered)
client.indices()
    .put_alias("blog_posts", "published_posts")
    .body(json!({
        "filter": { "term": { "published": true } }
    }))
    .send()
    .await?;

// Atomic alias swap (for zero-downtime reindexing)
client.indices()
    .update_aliases()
    .body(json!({
        "actions": [
            { "remove": { "index": "blog_posts_v1", "alias": "posts" } },
            { "add":    { "index": "blog_posts_v2", "alias": "posts" } }
        ]
    }))
    .send()
    .await?;
```

## Index Templates

Templates apply settings and mappings automatically to new indices whose name matches a pattern.

```rust
// Index template (OpenSearch 2.x style)
client.indices()
    .put_index_template("logs-template")
    .body(json!({
        "index_patterns": ["logs-*"],
        "template": {
            "settings": {
                "number_of_shards": 1,
                "number_of_replicas": 0
            },
            "mappings": {
                "properties": {
                    "timestamp": { "type": "date" },
                    "level":     { "type": "keyword" },
                    "message":   { "type": "text" },
                    "service":   { "type": "keyword" }
                }
            }
        },
        "priority": 100
    }))
    .send()
    .await?;
```

## Refreshing an Index

Force a refresh to make recently indexed documents immediately searchable:

```rust
// Refresh a specific index
client.indices().refresh("blog_posts").send().await?;

// Refresh all indices (avoid in production)
client.indices().refresh("_all").send().await?;
```

## Flushing

Force a flush to persist data to disk:

```rust
client.indices().flush("blog_posts").send().await?;
```

## Deleting an Index

```rust
client.indices().delete("blog_posts").send().await?;
```

## Listing Indices

With the `cat` feature:

```rust
let indices = client.cat().indices().send().await?;
for index in indices {
    println!("{}: {} docs, {} bytes", index.index, index.docs_count, index.store_size);
}
```

## Reindexing

Copy documents from one index to another (useful for mapping changes):

```rust
client.core()
    .reindex()
    .body(json!({
        "source": { "index": "blog_posts_v1" },
        "dest":   { "index": "blog_posts_v2" }
    }))
    .send()
    .await?;
```

For large indices, run it asynchronously and poll the task:

```rust
let task = client.core()
    .reindex()
    .body(json!({
        "source": { "index": "blog_posts_v1" },
        "dest":   { "index": "blog_posts_v2" }
    }))
    .wait_for_completion(false)
    .send()
    .await?;

println!("Reindex task ID: {}", task.task);
```

## Zero-Downtime Index Rotation

The standard pattern for changing mappings without downtime:

1. Create a new index (`blog_posts_v2`) with the updated mappings.
2. Reindex data from `blog_posts_v1` to `blog_posts_v2`.
3. Atomically swap the alias `posts` from `v1` to `v2`.
4. Delete `blog_posts_v1` when confident the migration succeeded.

```rust
// 1. Create new index
client.indices().create("blog_posts_v2")
    .body(new_mappings)
    .send().await?;

// 2. Reindex
client.core().reindex()
    .body(json!({
        "source": { "index": "blog_posts_v1" },
        "dest":   { "index": "blog_posts_v2" }
    }))
    .send().await?;

// 3. Swap alias atomically
client.indices().update_aliases()
    .body(json!({
        "actions": [
            { "remove": { "index": "blog_posts_v1", "alias": "posts" } },
            { "add":    { "index": "blog_posts_v2", "alias": "posts" } }
        ]
    }))
    .send().await?;

// 4. Clean up
client.indices().delete("blog_posts_v1").send().await?;
```
