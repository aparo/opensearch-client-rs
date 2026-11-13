# Search Queries Examples

This guide provides comprehensive examples of building and executing search queries using the OpenSearch DSL.

## Basic Search Patterns

### Simple Text Search

```rust
use opensearch_dsl::{Search, Query};

// Simple term search
let search = Search::new()
    .query(Query::term("status", "published"))
    .size(10);

let results = BlogPost::find(search).await?;

// Match query for full-text search
let search = Search::new()
    .query(Query::r#match("content", "rust programming"))
    .size(20);

let results = BlogPost::find(search).await?;

// Multi-match across multiple fields
let search = Search::new()
    .query(
        Query::multi_match(["title", "content"], "opensearch tutorial")
            .ty("best_fields")
            .fuzziness("AUTO")
    );

let results = BlogPost::find(search).await?;
```

### Range Queries

```rust
// Date range
let search = Search::new()
    .query(
        Query::range("created_at")
            .gte("2024-01-01")
            .lte("2024-12-31")
    );

// Numeric range
let search = Search::new()
    .query(
        Query::range("price")
            .gte(100.0)
            .lt(500.0)
    );

// Age range with relative dates
let one_week_ago = (Utc::now() - chrono::Duration::weeks(1)).to_rfc3339();
let search = Search::new()
    .query(
        Query::range("created_at")
            .gte(one_week_ago)
    );
```

## Boolean Queries

### Complex Boolean Logic

```rust
// Must, should, must_not combinations
let search = Search::new()
    .query(
        Query::bool()
            .must([
                Query::term("published", true),
                Query::range("created_at").gte("2024-01-01")
            ])
            .should([
                Query::r#match("title", "rust"),
                Query::r#match("content", "tutorial")
            ])
            .must_not([
                Query::term("tags", "draft")
            ])
            .minimum_should_match(1)
    );

// Nested boolean queries
let search = Search::new()
    .query(
        Query::bool()
            .must([
                Query::bool()
                    .should([
                        Query::term("category", "tech"),
                        Query::term("category", "programming")
                    ]),
                Query::bool()
                    .must([
                        Query::range("view_count").gte(1000),
                        Query::term("featured", true)
                    ])
            ])
    );
```

### Filter Context vs Query Context

```rust
// Using filter for exact matches (no scoring)
let search = Search::new()
    .query(Query::r#match("content", "search tutorial"))  // Scored
    .post_filter(                                          // Not scored
        Query::bool()
            .filter([
                Query::term("published", true),
                Query::range("created_at").gte("2024-01-01")
            ])
    );

// Using constant_score for filtered queries
let search = Search::new()
    .query(
        Query::constant_score(
            Query::bool()
                .filter([
                    Query::term("status", "active"),
                    Query::range("price").gte(10.0)
                ])
        )
        .boost(1.2)
    );
```

## Aggregations

### Metrics Aggregations

```rust
use opensearch_dsl::{Aggregation, CalendarInterval};

// Simple aggregations — chain `.aggregate(name, agg)`, one per call.
// NOTE: `value_count`/`stats`/`range` aggregations are not implemented; use
// `avg`/`max`/`min`/`sum`/`cardinality`.
let search = Search::new()
    .query(Query::match_all())
    .aggregate("avg_price", Aggregation::avg("price"))
    .aggregate("max_views", Aggregation::max("view_count"))
    .aggregate("min_price", Aggregation::min("price"))
    .aggregate("unique_authors", Aggregation::cardinality("author_id"))
    .size(0);  // Don't return documents, just aggregations

// Date histogram — the interval is a `CalendarInterval` enum (not a string).
let search = Search::new()
    .aggregate(
        "posts_per_month",
        Aggregation::date_histogram("created_at").calendar_interval(CalendarInterval::Month),
    )
    .size(0);
```

### Bucket Aggregations

```rust
use opensearch_dsl::{Aggregation, FieldSort, TermsOrder};

// Terms aggregation (top categories) — order via `TermsOrder`.
let search = Search::new()
    .aggregate(
        "top_categories",
        Aggregation::terms("category").size(10).order(TermsOrder::count_descending()),
    )
    .size(0);

// Nested aggregations — sub-aggregations are chained with `.aggregate(..)` too.
let search = Search::new()
    .aggregate(
        "categories",
        Aggregation::terms("category")
            .size(10)
            .aggregate("avg_price_in_category", Aggregation::avg("price"))
            .aggregate(
                "top_products",
                Aggregation::top_hits().size(3).sort([FieldSort::descending("price")]),
            ),
    )
    .size(0);

// Bucketing a numeric field into ranges: the `range` aggregation is not
// implemented, so use one `filter` aggregation per range.
let search = Search::new()
    .aggregate("cheap", Aggregation::filter(Query::range("price").lt(50.0)))
    .aggregate("medium", Aggregation::filter(Query::range("price").gte(50.0).lt(200.0)))
    .aggregate("expensive", Aggregation::filter(Query::range("price").gte(200.0)))
    .size(0);
```

## Advanced Search Patterns

### Fuzzy and Wildcard Queries

```rust
// Fuzzy search for typos
let search = Search::new()
    .query(
        Query::fuzzy("title", "opensarch")  // Will match "opensearch"
            .fuzziness("AUTO")
            .max_expansions(50)
    );

// Wildcard search
let search = Search::new()
    .query(Query::wildcard("title", "*search*"));

// Regexp search
let search = Search::new()
    .query(Query::regexp("email", r".*@gmail\.com"));
```

### Nested and Object Queries

```rust
// Nested object query — `Query::nested(path, query)` takes the query as its
// second argument.
let search = Search::new()
    .query(
        Query::nested(
            "comments",
            Query::bool()
                .must([
                    Query::r#match("comments.content", "great article"),
                    Query::term("comments.approved", true)
                ])
        )
    );

// Object field query
let search = Search::new()
    .query(
        Query::bool()
            .must([
                Query::term("metadata.featured", true),
                Query::range("metadata.word_count").gte(1000)
            ])
    );
```

### Geo Queries (if using geo fields)

```rust
use opensearch_dsl::{GeoLocation, GeoBoundingBox};

// Geo distance — `geo_distance(field, origin, distance)`.
let search = Search::new()
    .query(Query::geo_distance(
        "location",
        GeoLocation::new(40.7128, -74.0060),  // (latitude, longitude) — New York
        "10km",
    ));

// Geo bounding box — `geo_bounding_box(field, bounding_box)`.
let search = Search::new()
    .query(Query::geo_bounding_box(
        "location",
        GeoBoundingBox::MainDiagonal {
            top_left: GeoLocation::new(40.8, -74.1),
            bottom_right: GeoLocation::new(40.7, -73.9),
        },
    ));
```

## Sorting and Pagination

### Complex Sorting

```rust
use opensearch_dsl::FieldSort;

// Multiple sort criteria
let search = Search::new()
    .query(Query::match_all())
    .sort([
        FieldSort::descending("_score"),         // Relevance first
        FieldSort::descending("created_at"),     // Then by date
        FieldSort::ascending("title.keyword"),   // Finally by title
    ]);

// Sort with missing values
let search = Search::new()
    .sort([FieldSort::ascending("price").missing("_last")]);  // no-price items last

// Sort by nested field
let search = Search::new()
    .sort([FieldSort::descending("comments.created_at")]);
```

### Pagination Patterns

```rust
// Simple offset pagination
let page = 2;
let per_page = 20;
let search = Search::new()
    .query(Query::match_all())
    .from((page - 1) * per_page)
    .size(per_page);

// Search after for deep pagination
let search = Search::new()
    .query(Query::match_all())
    .sort([
        opensearch_dsl::FieldSort::descending("created_at"),
        opensearch_dsl::FieldSort::descending("id"),
    ])
    .search_after([
        serde_json::Value::String("2024-01-15T10:30:00Z".to_string()),
        serde_json::Value::String("doc_id_123".to_string())
    ])
    .size(20);
```

## Search Templates and Suggestions

### Highlighting

```rust
// Highlight matching terms. `Highlight` builds up fields with `.field(name)`
// (or `.field_highlighter(name, highlighter)` for per-field options).
let search = Search::new()
    .query(Query::r#match("content", "rust programming"))
    .highlight(
        opensearch_dsl::Highlight::new()
            .field("title")
            .field("content")
    );
```

### Source Filtering

```rust
// Include only specific fields
let search = Search::new()
    .query(Query::match_all())
    .source(["title", "summary", "created_at"]);

// Exclude sensitive fields — `.source((includes, excludes))` sets both lists;
// pass an empty includes list to exclude only.
let search = Search::new()
    .query(Query::match_all())
    .source((Vec::<&str>::new(), vec!["internal_notes", "admin_data"]));
```

## Real-world Search Examples

### Blog Search with Filters

```rust
async fn search_blog_posts(
    query: &str,
    category: Option<&str>,
    tags: &[String],
    published_only: bool,
    date_from: Option<&str>,
    date_to: Option<&str>,
    page: usize,
    per_page: usize,
) -> Result<opensearch_client::search::TypedSearchResult<BlogPost>, opensearch_client::Error> {
    use opensearch_dsl::{FieldSort, Highlight, Query, Search};

    let mut bool_query = Query::bool();

    // Main search query
    if !query.is_empty() {
        bool_query = bool_query.must([
            Query::multi_match(["title^2", "content"], query)
                .ty("best_fields")
                .fuzziness("AUTO")
        ]);
    }

    // Filters
    let mut filters = Vec::new();

    if published_only {
        filters.push(Query::term("published", true));
    }

    if let Some(cat) = category {
        filters.push(Query::term("category", cat));
    }

    if !tags.is_empty() {
        filters.push(Query::terms("tags", tags));
    }

    if let Some(from) = date_from {
        filters.push(Query::range("created_at").gte(from));
    }

    if let Some(to) = date_to {
        filters.push(Query::range("created_at").lte(to));
    }

    if !filters.is_empty() {
        bool_query = bool_query.filter(filters);
    }

    let search = Search::new()
        .query(bool_query)
        .sort([FieldSort::descending("_score"), FieldSort::descending("created_at")])
        .from((page - 1) * per_page)
        .size(per_page)
        .highlight(
            Highlight::new()
                .field("title")
                .field("content")
        );

    BlogPost::find(search).await
}
```

### E-commerce Product Search

```rust
async fn search_products(
    query: &str,
    category: Option<&str>,
    price_min: Option<f64>,
    price_max: Option<f64>,
    in_stock: bool,
    sort_by: &str,  // "relevance", "price_asc", "price_desc", "newest"
    page: usize,
    per_page: usize,
) -> Result<opensearch_client::search::TypedSearchResult<Product>, opensearch_client::Error> {
    use opensearch_dsl::{Aggregation, FieldSort, Query, Search};

    let mut bool_query = Query::bool();

    // Search query
    if !query.is_empty() {
        bool_query = bool_query.must([
            Query::multi_match(
                ["name^3", "description", "tags", "specifications.color", "specifications.material"],
                query
            )
            .ty("cross_fields")
            .fuzziness("AUTO")
        ]);
    }

    // Filters
    let mut filters = vec![Query::term("active", true)];

    if let Some(cat) = category {
        filters.push(Query::term("category_id", cat));
    }

    if in_stock {
        filters.push(Query::range("inventory.available").gt(0));
    }

    // Price range
    if price_min.is_some() || price_max.is_some() {
        let mut price_query = Query::range("price");
        if let Some(min) = price_min {
            price_query = price_query.gte(min);
        }
        if let Some(max) = price_max {
            price_query = price_query.lte(max);
        }
        filters.push(price_query);
    }

    bool_query = bool_query.filter(filters);

    // Sorting
    let sort_criteria: Vec<FieldSort> = match sort_by {
        "price_asc" => vec![FieldSort::ascending("price"), FieldSort::ascending("name.keyword")],
        "price_desc" => vec![FieldSort::descending("price"), FieldSort::ascending("name.keyword")],
        "newest" => vec![FieldSort::descending("created_at"), FieldSort::ascending("name.keyword")],
        _ => vec![FieldSort::descending("_score"), FieldSort::ascending("name.keyword")],
    };

    // Facets — `range` aggregation is not implemented, so each price bucket is a
    // `filter` aggregation.
    let search = Search::new()
        .query(bool_query)
        .sort(sort_criteria)
        .from((page - 1) * per_page)
        .size(per_page)
        .aggregate("categories", Aggregation::terms("category_id").size(20))
        .aggregate("under_50", Aggregation::filter(Query::range("price").lt(50.0)))
        .aggregate("50_to_100", Aggregation::filter(Query::range("price").gte(50.0).lt(100.0)))
        .aggregate("100_to_200", Aggregation::filter(Query::range("price").gte(100.0).lt(200.0)))
        .aggregate("over_200", Aggregation::filter(Query::range("price").gte(200.0)))
        .aggregate("avg_price", Aggregation::avg("price"));

    Product::find(search).await
}
```

These examples demonstrate:
- Basic to advanced query patterns
- Boolean query combinations
- Aggregations for analytics
- Sorting and pagination strategies
- Real-world search implementations
- Performance considerations
- Error handling patterns