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
use opensearch_dsl::{Search, Query, FieldSort};

pub async fn search_posts(
    text: &str,
    tags: &[String],
    page: usize,
    per_page: usize,
) -> Result<opensearch_client::search::TypedSearchResult<Post>, opensearch_client::Error> {
    let per_page = per_page.min(100);

    let mut bool_query = Query::bool()
        .filter([Query::term("published", true)]);

    if !text.is_empty() {
        bool_query = bool_query.must([
            Query::multi_match(["title^3", "content", "tags"], text)
                .ty("best_fields")
                .fuzziness("AUTO"),
        ]);
    }

    if !tags.is_empty() {
        bool_query = bool_query.filter([Query::terms("tags", tags)]);
    }

    Post::find(
        Search::new()
            .query(bool_query)
            .sort([
                FieldSort::descending("_score"),
                FieldSort::descending("created_at"),
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
    use opensearch_dsl::{Aggregation, AggregationResponse, Search, TermsOrder};

    // The typed ORM result (`TypedSearchResult`) does not carry aggregations, so
    // run the aggregation through the raw client search, which returns a
    // `SearchResponse` with an `aggregations` map.
    let search = Search::new()
        .size(0)  // No documents — only aggregations
        .aggregate(
            "tags",
            Aggregation::terms("tags").size(limit as u64).order(TermsOrder::count_descending()),
        );

    let resp = opensearch_client::get_opensearch()
        .search()
        .index("posts")
        .search(search)
        .call()
        .await?;

    let mut tags = Vec::new();
    if let Some(AggregationResponse::Bucket(agg)) = resp.aggregations.get("tags") {
        for bucket in &agg.buckets {
            tags.push((bucket.key_to_string(), bucket.doc_count as u64));
        }
    }
    Ok(tags)
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
pub async fn ingest_batch(events: Vec<LogEvent>) -> Result<(), Box<dyn std::error::Error>> {
    // `client.bulker()` returns a builder that spawns a background flusher.
    let client = opensearch_client::get_opensearch();
    let bulker = client.bulker().bulk_size(500).build();

    for event in events {
        // index(index, &body, id) — enqueues the action (returns once queued).
        bulker.index("logs", &event, Some(event.id.clone())).await?;
    }
    bulker.flush().await;   // wait for all queued actions to be sent

    Ok(())
}
```

### Error Rate Over Time

```rust
pub async fn error_rate_by_hour(
    service: &str,
    since: &str,  // RFC3339
) -> Result<Vec<(String, u64, u64)>, opensearch_client::Error> {
    use opensearch_dsl::{Aggregation, AggregationResponse, CalendarInterval, Query, Search};

    let search = Search::new()
        .size(0)
        .query(
            Query::bool()
                .filter([
                    Query::term("service", service),
                    Query::range("timestamp").gte(since),
                ]),
        )
        .aggregate(
            "by_hour",
            Aggregation::date_histogram("timestamp")
                .calendar_interval(CalendarInterval::Hour)
                .aggregate("errors", Aggregation::filter(Query::term("level", "error"))),
        );

    let resp = opensearch_client::get_opensearch()
        .search()
        .index("logs")
        .search(search)
        .call()
        .await?;

    let mut out = Vec::new();
    if let Some(AggregationResponse::Bucket(by_hour)) = resp.aggregations.get("by_hour") {
        for bucket in &by_hour.buckets {
            // A `filter` sub-aggregation deserializes as a doc-count aggregation.
            let errors = match bucket.sub_aggs.get("errors") {
                Some(AggregationResponse::DocCount(dc)) => dc.doc_count as u64,
                _ => 0,
            };
            out.push((bucket.key_to_string(), bucket.doc_count as u64, errors));
        }
    }
    Ok(out)
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
            .sort([opensearch_dsl::FieldSort::descending("duration_ms")])
            .size(limit),
    )
    .await?;

    Ok(results
        .hits
        .hits
        .into_iter()
        .filter_map(|h| h.source)
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
) -> Result<opensearch_dsl::SearchResponse, opensearch_client::Error> {
    use opensearch_dsl::{Aggregation, FieldSort, Query, Search};

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
        bool_query = bool_query.must([
            Query::multi_match(["name^4", "description", "tags"], filters.query.as_str())
                .ty("cross_fields")
                .operator("and"),
        ]);
    }
    if !filter_clauses.is_empty() {
        bool_query = bool_query.filter(filter_clauses);
    }

    // Aggregations (facets) are only exposed on the raw `SearchResponse`, so run
    // the search through the client rather than the typed ORM `find`.
    let search = Search::new()
        .query(bool_query)
        .sort([FieldSort::descending("_score"), FieldSort::descending("rating")])
        .from((page.saturating_sub(1)) * per_page)
        .size(per_page)
        // Facets — the `range` aggregation is not implemented, so each price
        // bucket is expressed as a `filter` aggregation.
        .aggregate("categories", Aggregation::terms("category_id").size(50))
        .aggregate("under_25", Aggregation::filter(Query::range("price").lt(25.0)))
        .aggregate("25_to_100", Aggregation::filter(Query::range("price").gte(25.0).lt(100.0)))
        .aggregate("100_to_500", Aggregation::filter(Query::range("price").gte(100.0).lt(500.0)))
        .aggregate("over_500", Aggregation::filter(Query::range("price").gte(500.0)))
        .aggregate("avg_price", Aggregation::avg("price"))
        .aggregate("in_stock_count", Aggregation::filter(Query::term("in_stock", true)));

    opensearch_client::get_opensearch()
        .search()
        .index("products")
        .search(search)
        .call()
        .await
}
```

---

## Multi-Tenant SaaS Application

Isolate tenant data using filtered aliases so application code never needs to add tenant filters manually.

### Setup Per-Tenant Alias

```rust
pub async fn create_tenant_alias(
    client: &opensearch_client::OsClient,
    tenant_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use opensearch_client::indices::PutAlias;
    use opensearch_dsl::Query;

    client
        .indices()
        .put_alias()
        .index("documents")
        .put_alias(PutAlias {
            alias: Some(format!("docs_{}", tenant_id)),
            filter: Some(Query::term("tenant_id", tenant_id).into()),
            is_write_index: Some(false),
            ..Default::default()
        })
        .call()
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
