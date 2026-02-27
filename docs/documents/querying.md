# Querying with the Document Trait

The `Document` trait provides high-level query methods that work directly on your model types. These wrap the lower-level `Search` builder from `opensearch-dsl`.

## `find` — Execute a Full Search

`find` accepts any `Search` value and returns typed results:

```rust
use opensearch_dsl::{Search, Query, Sort, SortOrder};

let results = BlogPost::find(
    Search::new()
        .query(Query::term("published", true))
        .sort([Sort::field("created_at").order(SortOrder::Desc)])
        .size(20)
        .from(0),
)
.await?;

println!("total: {}", results.hits.total.value);
for hit in results.hits.hits {
    if let Some(post) = hit.source_ {
        println!("{}: {}", post.id, post.title);
    }
}
```

## `find_all` — Get All Documents

Returns all documents up to an optional limit:

```rust
// Up to 1000 documents (default limit)
let all = BlogPost::find_all(None).await?;

// At most 50 documents
let first_50 = BlogPost::find_all(Some(50)).await?;
```

## `find_one` — Get the First Match

Returns `Option<T>` — the first document matching the query, or `None`:

```rust
let post = BlogPost::find_one(
    Search::new().query(Query::term("slug", "hello-world"))
)
.await?;

match post {
    Some(p) => println!("found: {}", p.title),
    None => println!("not found"),
}
```

## `count` — Count Matching Documents

Returns the number of documents matching a query:

```rust
// Count all documents
let total = BlogPost::count(None).await?;

// Count with a query
let published_count = BlogPost::count(
    Some(Query::term("published", true))
).await?;

println!("{} published out of {} total", published_count, total);
```

## Common Search Patterns

### Keyword Search with Filters

```rust
let results = BlogPost::find(
    Search::new()
        .query(
            Query::bool()
                .must(Query::multi_match(["title^2", "content"], "async rust"))
                .filter([
                    Query::term("published", true),
                    Query::terms("tags", ["rust", "programming"]),
                ])
        )
        .size(10),
)
.await?;
```

### Date Range Filter

```rust
let recent = BlogPost::find(
    Search::new()
        .query(
            Query::bool()
                .filter([
                    Query::term("published", true),
                    Query::range("created_at").gte("now-30d/d"),
                ])
        )
        .sort([Sort::field("created_at").order(SortOrder::Desc)])
        .size(5),
)
.await?;
```

### Pagination

```rust
async fn paginate(page: usize, per_page: usize) -> anyhow::Result<Vec<BlogPost>> {
    let results = BlogPost::find(
        Search::new()
            .query(Query::term("published", true))
            .from((page.saturating_sub(1)) * per_page)
            .size(per_page)
            .sort([Sort::field("created_at").order(SortOrder::Desc)]),
    )
    .await?;

    Ok(results
        .hits
        .hits
        .into_iter()
        .filter_map(|h| h.source_)
        .collect())
}
```

### Aggregations

The `find` return value carries aggregations alongside hits:

```rust
let results = BlogPost::find(
    Search::new()
        .size(0)
        .aggregations([
            ("by_tag", Aggregation::terms("tags").size(20)),
            ("posts_per_month",
                Aggregation::date_histogram("created_at").calendar_interval("month")),
        ]),
)
.await?;

if let Some(aggs) = results.aggregations {
    // Process aggregation buckets
}
```

## Accessing Scores and Metadata

Hit objects include relevance score and OpenSearch metadata:

```rust
let results = BlogPost::find(
    Search::new()
        .query(Query::match_("title", "rust tutorial"))
        .size(5),
)
.await?;

for hit in results.hits.hits {
    println!(
        "id={} score={:.2}",
        hit.id_,
        hit.score_.unwrap_or(0.0)
    );
    if let Some(post) = hit.source_ {
        println!("  title: {}", post.title);
    }
}
```

## Using `get` for Single Documents

When you know the ID, `get` is more efficient than a search:

```rust
// Returns the document or an error if not found
let post = BlogPost::get("post-123").await?;
println!("{}", post.title);
```

## Refreshing After Write

After a write, newly indexed documents are not immediately visible to search (due to the default 1-second refresh interval). Force a refresh in tests or time-sensitive contexts:

```rust
post.save().await?;
// In production, avoid this — let the refresh happen naturally
client.indices().refresh("blog_posts").send().await?;

let found = BlogPost::find_one(
    Search::new().query(Query::term("id", &post.id))
).await?;
assert!(found.is_some());
```
