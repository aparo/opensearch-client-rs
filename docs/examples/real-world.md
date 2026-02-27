# Real-World Application Examples

Complete, realistic examples showing common application patterns.

## Blog Platform

A typical blog with full-text search, tag filtering, pagination, and analytics.

### Document Model

```rust
use opensearch_client::{Document, OpenSearch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "posts")]
pub struct Post {
    #[os(id)]
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author_id: String,
    pub tags: Vec<String>,
    pub published: bool,
    pub created_at: String,  // RFC3339
    pub updated_at: String,
    pub view_count: u32,
}
```

### Full-Text Search with Filters

```rust
use opensearch_dsl::{Search, Query, Aggregation, Sort, SortOrder};

pub async fn search_posts(
    text: &str,
    tags: &[String],
    page: usize,
    per_page: usize,
) -> Result<opensearch_dsl::SearchResponse<Post>, opensearch_client::Error> {
    let per_page = per_page.min(100);

    let mut bool_query = Query::bool()
        .filter(Query::term("published", true));

    if !text.is_empty() {
        bool_query = bool_query.must(
            Query::multi_match(["title^3", "content", "tags"], text)
                .ty("best_fields")
                .fuzziness("AUTO"),
        );
    }

    if !tags.is_empty() {
        bool_query = bool_query.filter(Query::terms("tags", tags));
    }

    Post::find(
        Search::new()
            .query(bool_query)
            .sort([
                Sort::score(),
                Sort::field("created_at").order(SortOrder::Desc),
            ])
            .from(page.saturating_sub(1) * per_page)
            .size(per_page)
            .source(["id", "title", "slug", "author_id", "tags", "created_at", "view_count"]),
    )
    .await
}
```

### Tag Aggregation for Sidebar

```rust
pub async fn popular_tags(limit: usize) -> Result<Vec<(String, u64)>, opensearch_client::Error> {
    let search = Search::new()
        .size(0)  // No documents — only aggregations
        .aggregations([
            ("tags", Aggregation::terms("tags").size(limit).order([("_count", "desc")])),
        ]);

    let resp = Post::find(search).await?;
    let buckets = resp
        .aggregations
        .and_then(|a| a.get("tags").cloned())
        .and_then(|a| a.buckets)
        .unwrap_or_default();

    Ok(buckets.into_iter().map(|b| (b.key, b.doc_count)).collect())
}
```

### Increment View Count

```rust
pub async fn increment_views(post_id: &str) -> Result<(), opensearch_client::Error> {
    Post::update(
        post_id,
        &serde_json::json!({
            "script": {
                "source": "ctx._source.view_count += 1",
                "lang": "painless"
            }
        }),
    )
    .await
    .map(|_| ())
}
```

---

## Log Analytics Pipeline

Ingest, store, and query structured log events.

### Document Model

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "logs")]
pub struct LogEvent {
    #[os(id)]
    pub id: String,
    pub timestamp: String,
    pub level: String,       // "debug" | "info" | "warn" | "error"
    pub service: String,
    pub message: String,
    pub trace_id: Option<String>,
    pub duration_ms: Option<u64>,
    pub http_status: Option<u16>,
}
```

### Bulk Ingestion

```rust
use opensearch_client::Bulker;

pub async fn ingest_batch(events: Vec<LogEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let client = opensearch_client::get_opensearch();

    let bulker = Bulker::builder(client.clone(), events.len())
        .bulk_size(500)
        .build();

    for event in events {
        bulker.index(event).await?;
    }
    bulker.flush().await?;

    Ok(())
}
```

### Error Rate Over Time

```rust
pub async fn error_rate_by_hour(
    service: &str,
    since: &str,  // RFC3339
) -> Result<Vec<(String, u64, u64)>, opensearch_client::Error> {
    let search = Search::new()
        .size(0)
        .query(
            Query::bool()
                .filter([
                    Query::term("service", service),
                    Query::range("timestamp").gte(since),
                ]),
        )
        .aggregations([
            ("by_hour",
                Aggregation::date_histogram("timestamp")
                    .calendar_interval("hour")
                    .format("yyyy-MM-dd'T'HH:mm:ss'Z'")
                    .aggregations([
                        ("errors",
                            Aggregation::filter(Query::term("level", "error")),
                        ),
                    ]),
            ),
        ]);

    let resp = LogEvent::find(search).await?;
    let buckets = resp
        .aggregations
        .and_then(|a| a.get("by_hour").cloned())
        .and_then(|a| a.buckets)
        .unwrap_or_default();

    Ok(buckets
        .into_iter()
        .map(|b| {
            let errors = b
                .aggregations
                .as_ref()
                .and_then(|a| a.get("errors"))
                .and_then(|a| a.doc_count)
                .unwrap_or(0);
            (b.key_as_string.unwrap_or_default(), b.doc_count, errors)
        })
        .collect())
}
```

### Slow Request Detection

```rust
pub async fn slow_requests(threshold_ms: u64, limit: usize) -> Result<Vec<LogEvent>, opensearch_client::Error> {
    let results = LogEvent::find(
        Search::new()
            .query(
                Query::bool()
                    .filter([
                        Query::exists("duration_ms"),
                        Query::range("duration_ms").gte(threshold_ms),
                    ]),
            )
            .sort([Sort::field("duration_ms").order(SortOrder::Desc)])
            .size(limit),
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

---

## E-Commerce Product Catalog

### Document Model

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "products")]
pub struct Product {
    #[os(id)]
    pub sku: String,
    pub name: String,
    pub description: String,
    pub category_id: String,
    pub price: f64,
    pub currency: String,
    pub tags: Vec<String>,
    pub in_stock: bool,
    pub stock_count: u32,
    pub rating: f32,
    pub created_at: String,
}
```

### Faceted Search

```rust
pub struct SearchFilters {
    pub query: String,
    pub category_id: Option<String>,
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,
    pub in_stock_only: bool,
    pub min_rating: Option<f32>,
    pub tags: Vec<String>,
}

pub async fn search_products(
    filters: &SearchFilters,
    page: usize,
    per_page: usize,
) -> Result<opensearch_dsl::SearchResponse<Product>, opensearch_client::Error> {
    let mut filter_clauses: Vec<Query> = vec![];

    if let Some(cat) = &filters.category_id {
        filter_clauses.push(Query::term("category_id", cat.as_str()));
    }
    if filters.in_stock_only {
        filter_clauses.push(Query::term("in_stock", true));
    }
    if filters.price_min.is_some() || filters.price_max.is_some() {
        let mut r = Query::range("price");
        if let Some(min) = filters.price_min { r = r.gte(min); }
        if let Some(max) = filters.price_max { r = r.lte(max); }
        filter_clauses.push(r);
    }
    if let Some(rating) = filters.min_rating {
        filter_clauses.push(Query::range("rating").gte(rating));
    }
    if !filters.tags.is_empty() {
        filter_clauses.push(Query::terms("tags", &filters.tags));
    }

    let mut bool_query = Query::bool();
    if !filters.query.is_empty() {
        bool_query = bool_query.must(
            Query::multi_match(["name^4", "description", "tags"], filters.query.as_str())
                .ty("cross_fields")
                .operator("and"),
        );
    }
    if !filter_clauses.is_empty() {
        bool_query = bool_query.filter(filter_clauses);
    }

    Product::find(
        Search::new()
            .query(bool_query)
            .sort([Sort::score(), Sort::field("rating").order(SortOrder::Desc)])
            .from((page.saturating_sub(1)) * per_page)
            .size(per_page)
            // Aggregations for facets — returned alongside results
            .aggregations([
                ("categories", Aggregation::terms("category_id").size(50)),
                ("price_ranges", Aggregation::range("price").ranges([
                    (None, Some(25.0)),
                    (Some(25.0), Some(100.0)),
                    (Some(100.0), Some(500.0)),
                    (Some(500.0), None),
                ])),
                ("avg_price", Aggregation::avg("price")),
                ("in_stock_count", Aggregation::filter(Query::term("in_stock", true))),
            ]),
    )
    .await
}
```

---

## Multi-Tenant SaaS Application

Isolate tenant data using filtered aliases so application code never needs to add tenant filters manually.

### Setup Per-Tenant Alias

```rust
pub async fn create_tenant_alias(
    client: &OsClient,
    tenant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .indices()
        .put_alias("documents", &format!("docs_{}", tenant_id))
        .body(serde_json::json!({
            "filter": { "term": { "tenant_id": tenant_id } },
            "is_write_index": false
        }))
        .send()
        .await?;
    Ok(())
}
```

### Document Model with Tenant Field

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "documents")]
pub struct TenantDocument {
    #[os(id)]
    pub id: String,
    pub tenant_id: String,   // Always set at creation, enforced by alias filter
    pub title: String,
    pub body: String,
    pub created_at: String,
}

impl TenantDocument {
    pub fn new(tenant_id: &str, title: &str, body: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            title: title.to_string(),
            body: body.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
```
