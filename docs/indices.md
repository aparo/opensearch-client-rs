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
use opensearch_client::indices::IndexTemplateMapping;

client.indices()
    .create()
    .index("blog_posts")
    .body(IndexTemplateMapping::default())
    .call()
    .await?;
```

### With explicit mappings

The `body` is an `IndexTemplateMapping` (fields `settings`, `mappings`,
`aliases`). Build it from JSON with `serde_json::from_value`:

```rust
use opensearch_client::indices::IndexTemplateMapping;
use serde_json::json;

let body: IndexTemplateMapping = serde_json::from_value(json!({
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
}))?;

client.indices()
    .create()
    .index("blog_posts")
    .body(body)
    .call()
    .await?;
```

### Create only if absent

```rust
use opensearch_client::indices::IndexTemplateMapping;

let exists = client.indices().exists().index("blog_posts").call().await?;
if !exists {
    client.indices()
        .create()
        .index("blog_posts")
        .body(IndexTemplateMapping::default())
        .call()
        .await?;
}
```

## Checking Whether an Index Exists

```rust
let exists: bool = client.indices().exists().index("blog_posts").call().await?;
```

## Getting Index Information

```rust
// Mappings + settings + aliases in one call
let info = client.indices().get().index("blog_posts").call().await?;

// Mappings only
let mappings = client.indices().get_mapping().index("blog_posts").call().await?;

// Settings only
let settings = client.indices().get_settings().index("blog_posts").call().await?;
```

## Updating Mappings

You can add new fields to an existing index, but you cannot change existing field types (you'd need to reindex for that).

```rust
use opensearch_client::indices::PutMapping;
use serde_json::json;

let mapping: PutMapping = serde_json::from_value(json!({
    "properties": {
        "author_id": { "type": "keyword" },
        "word_count": { "type": "integer" }
    }
}))?;

client.indices()
    .put_mapping()
    .index("blog_posts")
    .put_mapping(mapping)
    .call()
    .await?;
```

## Updating Settings

Dynamic settings can be changed on a live index. Static settings (like `number_of_shards`) require closing the index first.

```rust
use opensearch_client::indices::PutSettings;
use serde_json::json;

// Dynamic setting — applies immediately
let settings: PutSettings = serde_json::from_value(json!({
    "number_of_replicas": 2,
    "refresh_interval": "30s"
}))?;

client.indices()
    .put_settings()
    .index("blog_posts")
    .put_settings(settings)
    .call()
    .await?;
```

## Aliases

Aliases allow zero-downtime index migrations and provide a stable name for the application.

Alias operations go through `update_aliases`, whose body is an `UpdateAliases`
value (a list of `actions`):

```rust
use opensearch_client::indices::UpdateAliases;
use serde_json::json;

// Add an alias
let actions: UpdateAliases = serde_json::from_value(json!({
    "actions": [
        { "add": { "index": "blog_posts", "alias": "posts" } }
    ]
}))?;
client.indices().update_aliases().update_aliases(actions).call().await?;

// Alias with a filter (filtered alias — documents are pre-filtered)
let actions: UpdateAliases = serde_json::from_value(json!({
    "actions": [
        { "add": {
            "index": "blog_posts",
            "alias": "published_posts",
            "filter": { "term": { "published": true } }
        }}
    ]
}))?;
client.indices().update_aliases().update_aliases(actions).call().await?;

// Atomic alias swap (for zero-downtime reindexing)
let actions: UpdateAliases = serde_json::from_value(json!({
    "actions": [
        { "remove": { "index": "blog_posts_v1", "alias": "posts" } },
        { "add":    { "index": "blog_posts_v2", "alias": "posts" } }
    ]
}))?;
client.indices().update_aliases().update_aliases(actions).call().await?;
```

## Index Templates

Templates apply settings and mappings automatically to new indices whose name matches a pattern.

```rust
use opensearch_client::indices::PutIndexTemplate;
use serde_json::json;

// Index template (OpenSearch 2.x style)
let template: PutIndexTemplate = serde_json::from_value(json!({
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
}))?;

client.indices()
    .put_index_template()
    .name("logs-template")
    .put_index_template(template)
    .call()
    .await?;
```

## Refreshing an Index

Force a refresh to make recently indexed documents immediately searchable:

```rust
// Refresh a specific index
client.indices().refresh().index("blog_posts").call().await?;

// Refresh all indices (avoid in production) — omit the index
client.indices().refresh().call().await?;
```

## Flushing

Force a flush to persist data to disk:

```rust
client.indices().flush().index("blog_posts").call().await?;
```

## Deleting an Index

```rust
client.indices().delete().index("blog_posts").call().await?;
```

## Listing Indices

```rust
let indices: Vec<String> = client.indices().list_indices().await?;
for index in indices {
    println!("{}", index);
}
```

## Reindexing

Copy documents from one index to another (useful for mapping changes):

```rust
use opensearch_client::common::Reindex;
use serde_json::json;

let body: Reindex = serde_json::from_value(json!({
    "source": { "index": "blog_posts_v1" },
    "dest":   { "index": "blog_posts_v2" }
}))?;

client.reindex().reindex(body).call().await?;
```

For large indices, run it asynchronously (`wait_for_completion(false)` returns a
task handle instead of blocking):

```rust
use opensearch_client::common::Reindex;
use serde_json::json;

let body: Reindex = serde_json::from_value(json!({
    "source": { "index": "blog_posts_v1" },
    "dest":   { "index": "blog_posts_v2" }
}))?;

let task = client.reindex()
    .reindex(body)
    .wait_for_completion(false)
    .call()
    .await?;

println!("Reindex submitted: {:?}", task);
```

## Zero-Downtime Index Rotation

The standard pattern for changing mappings without downtime:

1. Create a new index (`blog_posts_v2`) with the updated mappings.
2. Reindex data from `blog_posts_v1` to `blog_posts_v2`.
3. Atomically swap the alias `posts` from `v1` to `v2`.
4. Delete `blog_posts_v1` when confident the migration succeeded.

```rust
use opensearch_client::common::Reindex;
use opensearch_client::indices::{IndexTemplateMapping, UpdateAliases};
use serde_json::json;

// 1. Create new index
client.indices()
    .create()
    .index("blog_posts_v2")
    .body(new_mappings)   // IndexTemplateMapping
    .call().await?;

// 2. Reindex
let reindex: Reindex = serde_json::from_value(json!({
    "source": { "index": "blog_posts_v1" },
    "dest":   { "index": "blog_posts_v2" }
}))?;
client.reindex().reindex(reindex).call().await?;

// 3. Swap alias atomically
let actions: UpdateAliases = serde_json::from_value(json!({
    "actions": [
        { "remove": { "index": "blog_posts_v1", "alias": "posts" } },
        { "add":    { "index": "blog_posts_v2", "alias": "posts" } }
    ]
}))?;
client.indices().update_aliases().update_aliases(actions).call().await?;

// 4. Clean up
client.indices().delete().index("blog_posts_v1").call().await?;
```
