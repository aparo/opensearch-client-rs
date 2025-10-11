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
use opensearch_dsl::Aggregation;

// Simple aggregations
let search = Search::new()
    .query(Query::match_all())
    .aggregations([
        ("avg_price", Aggregation::avg("price")),
        ("max_views", Aggregation::max("view_count")),
        ("total_posts", Aggregation::value_count("id")),
        ("price_stats", Aggregation::stats("price"))
    ])
    .size(0);  // Don't return documents, just aggregations

// Date histogram
let search = Search::new()
    .aggregations([
        ("posts_per_month", 
            Aggregation::date_histogram("created_at")
                .calendar_interval("month")
                .format("yyyy-MM")
        )
    ])
    .size(0);
```

### Bucket Aggregations

```rust
// Terms aggregation (top categories)
let search = Search::new()
    .aggregations([
        ("top_categories",
            Aggregation::terms("category")
                .size(10)
                .order([("_count", "desc")])
        )
    ])
    .size(0);

// Nested aggregations
let search = Search::new()
    .aggregations([
        ("categories",
            Aggregation::terms("category")
                .size(10)
                .aggregations([
                    ("avg_price_in_category", Aggregation::avg("price")),
                    ("top_products",
                        Aggregation::top_hits()
                            .size(3)
                            .sort([("price", "desc")])
                    )
                ])
        )
    ])
    .size(0);

// Range aggregation
let search = Search::new()
    .aggregations([
        ("price_ranges",
            Aggregation::range("price")
                .ranges([
                    ("cheap", None, Some(50.0)),
                    ("medium", Some(50.0), Some(200.0)),
                    ("expensive", Some(200.0), None)
                ])
        )
    ])
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
// Nested object query
let search = Search::new()
    .query(
        Query::nested("comments")
            .query(
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
// Geo distance
let search = Search::new()
    .query(
        Query::geo_distance("location")
            .distance("10km")
            .lat_lon(40.7128, -74.0060)  // New York coordinates
    );

// Geo bounding box
let search = Search::new()
    .query(
        Query::geo_bounding_box("location")
            .top_left(40.8, -74.1)
            .bottom_right(40.7, -73.9)
    );
```

## Sorting and Pagination

### Complex Sorting

```rust
// Multiple sort criteria
let search = Search::new()
    .query(Query::match_all())
    .sort([
        ("_score", "desc"),          // Relevance first
        ("created_at", "desc"),      // Then by date
        ("title.keyword", "asc")     // Finally by title
    ]);

// Sort with missing values
let search = Search::new()
    .sort([
        ("price", "asc", Some("_last"))  // Put items with no price at the end
    ]);

// Sort by nested field
let search = Search::new()
    .sort([
        ("comments.created_at", "desc")
    ]);
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
    .sort([("created_at", "desc"), ("id", "desc")])
    .search_after([
        serde_json::Value::String("2024-01-15T10:30:00Z".to_string()),
        serde_json::Value::String("doc_id_123".to_string())
    ])
    .size(20);
```

## Search Templates and Suggestions

### Highlighting

```rust
// Highlight matching terms
let search = Search::new()
    .query(Query::r#match("content", "rust programming"))
    .highlight(
        opensearch_dsl::Highlight::new()
            .fields([
                ("title", opensearch_dsl::HighlightField::new()),
                ("content", opensearch_dsl::HighlightField::new()
                    .fragment_size(150)
                    .number_of_fragments(3)
                )
            ])
            .pre_tags(["<mark>"])
            .post_tags(["</mark>"])
    );
```

### Source Filtering

```rust
// Include only specific fields
let search = Search::new()
    .query(Query::match_all())
    .source(["title", "summary", "created_at"]);

// Exclude sensitive fields
let search = Search::new()
    .query(Query::match_all())
    .source_excludes(["internal_notes", "admin_data"]);
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
) -> Result<SearchResults<BlogPost>, opensearch_client::Error> {
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
        .sort([("_score", "desc"), ("created_at", "desc")])
        .from((page - 1) * per_page)
        .size(per_page)
        .highlight(
            opensearch_dsl::Highlight::new()
                .fields([
                    ("title", opensearch_dsl::HighlightField::new()),
                    ("content", opensearch_dsl::HighlightField::new()
                        .fragment_size(200)
                        .number_of_fragments(2)
                    )
                ])
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
) -> Result<SearchResults<Product>, opensearch_client::Error> {
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
    let sort_criteria = match sort_by {
        "price_asc" => vec![("price", "asc"), ("name.keyword", "asc")],
        "price_desc" => vec![("price", "desc"), ("name.keyword", "asc")],
        "newest" => vec![("created_at", "desc"), ("name.keyword", "asc")],
        _ => vec![("_score", "desc"), ("name.keyword", "asc")],
    };

    let search = Search::new()
        .query(bool_query)
        .sort(sort_criteria)
        .from((page - 1) * per_page)
        .size(per_page)
        .aggregations([
            ("categories",
                Aggregation::terms("category_id")
                    .size(20)
            ),
            ("price_ranges",
                Aggregation::range("price")
                    .ranges([
                        ("under_50", None, Some(50.0)),
                        ("50_to_100", Some(50.0), Some(100.0)),
                        ("100_to_200", Some(100.0), Some(200.0)),
                        ("over_200", Some(200.0), None)
                    ])
            ),
            ("avg_price", Aggregation::avg("price"))
        ]);

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