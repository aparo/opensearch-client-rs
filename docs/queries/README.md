# Query DSL Guide

`opensearch-dsl` provides a fully type-safe Rust API for building OpenSearch queries and aggregations. The crate is independent of the HTTP client so it can be used standalone for serialization or testing.

**Required feature:** `search` (default) pulls in `opensearch-dsl` automatically.

## Quick Start

```rust
use opensearch_dsl::*;

let search = Search::new()
    .query(
        Query::bool()
            .must(Query::match_("title", "rust async"))
            .filter(Query::term("published", true))
    )
    .sort([Sort::field("created_at").order(SortOrder::Desc)])
    .size(10);

let results = BlogPost::find(search).await?;
```

## The `Search` Builder

`Search` is the top-level builder for a search request body.

```rust
let search = Search::new()
    .query(query)             // Main query
    .from(20)                 // Skip first N results (pagination)
    .size(10)                 // Return N results
    .source(false)            // Disable _source entirely
    .source(["id", "title"]) // Return only specific fields
    .sort([...])              // Sorting (see Sorting section)
    .aggregations([...])      // Aggregations (see Aggregations section)
    .highlight(...)           // Highlighting
    .search_after([...])      // Cursor-based deep pagination
    .min_score(0.5);          // Filter by relevance score
```

## Full-Text Queries

### `match`

Standard full-text search. The query string is analyzed before matching.

```rust
// Basic match
Query::match_("content", "opensearch tutorial")

// With options
Query::match_("content", "opensearch tutorial")
    .operator("and")       // All terms must match (default: "or")
    .fuzziness("AUTO")     // Allow typos
    .minimum_should_match("75%")
```

### `match_phrase`

Match exact phrase in order.

```rust
Query::match_phrase("content", "getting started with opensearch")
    .slop(1)  // Allow 1 word between phrase terms
```

### `multi_match`

Search across multiple fields at once.

```rust
// Best fields (default): score from the best matching field
Query::multi_match(["title^3", "content", "tags"], "opensearch rust")
    .ty("best_fields")
    .fuzziness("AUTO")

// Cross fields: all terms must appear across all fields
Query::multi_match(["first_name", "last_name"], "John Smith")
    .ty("cross_fields")
    .operator("and")

// Most fields: like best_fields but adds scores from all fields
Query::multi_match(["title", "title.english"], "quick brown fox")
    .ty("most_fields")
```

### `match_all` / `match_none`

```rust
Query::match_all()           // All documents, score 1.0
Query::match_all().boost(2.0)
Query::match_none()          // No documents
```

### `query_string`

Supports Lucene syntax directly. Use carefully with user input.

```rust
Query::query_string("title:(rust OR golang) AND published:true")
    .default_field("content")
```

### `simple_query_string`

Safer version of `query_string` — invalid syntax is silently ignored.

```rust
Query::simple_query_string("rust async programming")
    .fields(["title^2", "content"])
    .default_operator("and")
```

## Term-Level Queries

These operate on exact values and do **not** analyze the input.

### `term` / `terms`

```rust
// Exact value match
Query::term("status", "published")
Query::term("price", 42.0)
Query::term("active", true)

// Match any of several values
Query::terms("tags", ["rust", "async", "opensearch"])
```

### `range`

```rust
// Numeric range
Query::range("price").gte(10.0).lt(100.0)

// Date range
Query::range("created_at")
    .gte("2024-01-01")
    .lte("2024-12-31")
    .format("yyyy-MM-dd")

// Relative date (using OpenSearch date math)
Query::range("created_at").gte("now-7d/d")
```

### `exists`

```rust
// Field must be present and non-null
Query::exists("description")

// Combine with bool to find documents without a field
Query::bool().must_not(Query::exists("deleted_at"))
```

### `wildcard` / `regexp` / `prefix`

```rust
// Wildcard (* matches any sequence, ? matches one character)
Query::wildcard("email", "*@example.com")

// Regular expression
Query::regexp("email", "[a-z]+@(gmail|yahoo)\\.com")

// Prefix match (efficient — uses index)
Query::prefix("username", "john")
```

### `fuzzy`

Match terms similar to the query value (useful for typo tolerance).

```rust
Query::fuzzy("title", "opensarch")  // matches "opensearch"
    .fuzziness("AUTO")              // AUTO:3,6 by default
    .max_expansions(50)
    .prefix_length(2)               // First 2 chars must match exactly
```

### `ids`

```rust
Query::ids(["1", "2", "3"])
```

## Compound Queries

### `bool`

The workhorse of compound queries. Combines multiple clauses:

- **`must`**: All clauses must match; contribute to the score.
- **`filter`**: All clauses must match; do **not** contribute to score (and are cached).
- **`should`**: At least one (or `minimum_should_match`) must match; boost score.
- **`must_not`**: Clauses must **not** match; excluded from score.

```rust
Query::bool()
    .must([
        Query::match_("title", "opensearch"),
        Query::range("view_count").gte(100),
    ])
    .filter([
        Query::term("published", true),
        Query::term("language", "en"),
    ])
    .should([
        Query::term("featured", true),
        Query::match_("tags", "tutorial"),
    ])
    .must_not([
        Query::term("archived", true),
    ])
    .minimum_should_match(1)
```

**Performance tip:** Put exact-value filters in `filter` context, not `must`. Filter results are cached and don't affect scoring.

### `boosting`

Return documents matching `positive`, downgrade documents that also match `negative`:

```rust
Query::boosting()
    .positive(Query::match_("content", "opensearch tutorial"))
    .negative(Query::term("quality", "low"))
    .negative_boost(0.2)
```

### `constant_score`

Wrap a filter query and assign a flat score (avoids scoring overhead):

```rust
Query::constant_score(
    Query::bool().filter([
        Query::term("status", "active"),
        Query::range("stock").gt(0),
    ])
)
.boost(1.5)
```

### `dis_max`

Return the highest score from any matching sub-query (useful for multi-field search):

```rust
Query::dis_max([
    Query::match_("title", "quick brown fox"),
    Query::match_("content", "quick brown fox"),
])
.tie_breaker(0.3)  // Add fraction of other matches' scores
```

## Geo Queries

Requires a `geo_point` or `geo_shape` field mapping.

```rust
// Documents within a radius
Query::geo_distance("location")
    .point(GeoLocation::new(40.7128, -74.0060))
    .distance("10km")

// Documents within a bounding box
Query::geo_bounding_box("location")
    .top_left(GeoLocation::new(40.8, -74.1))
    .bottom_right(GeoLocation::new(40.6, -73.9))

// Documents within a polygon
Query::geo_polygon("location")
    .points([
        GeoLocation::new(40.73, -74.1),
        GeoLocation::new(40.01, -71.12),
        GeoLocation::new(50.56, -90.58),
    ])
```

## Nested and Join Queries

### `nested`

Query nested objects (mapped as `"type": "nested"`):

```rust
Query::nested("comments")
    .query(
        Query::bool()
            .must([
                Query::match_("comments.body", "great article"),
                Query::term("comments.approved", true),
            ])
    )
    .score_mode("avg")  // "avg", "sum", "min", "max", "none"
```

### `has_child` / `has_parent`

For parent-join field mappings:

```rust
// Documents that have a child matching the query
Query::has_child("comment")
    .query(Query::term("approved", true))
    .min_children(1)
    .score_mode("sum")

// Documents whose parent matches the query
Query::has_parent("blog")
    .query(Query::term("featured", true))
    .score(true)
```

## Sorting

```rust
use opensearch_dsl::{Sort, SortOrder};

Search::new()
    .sort([
        Sort::score(),                                    // _score desc (default)
        Sort::field("created_at").order(SortOrder::Desc),
        Sort::field("title.keyword").order(SortOrder::Asc),
    ])
```

Sort by nested field:

```rust
Sort::field("ratings.value")
    .order(SortOrder::Desc)
    .nested(NestedSort::new("ratings").filter(Query::term("ratings.verified", true)))
```

## Source Filtering

```rust
// Disable _source (saves bandwidth when you don't need the document body)
Search::new().source(false)

// Include only specific fields
Search::new().source(["id", "title", "created_at"])

// Exclude specific fields
Search::new().source_excludes(["internal_notes", "raw_html"])
```

## Highlighting

```rust
use opensearch_dsl::{Highlight, HighlightField};

Search::new()
    .query(Query::match_("content", "rust async"))
    .highlight(
        Highlight::new()
            .fields([
                ("title", HighlightField::new()),
                ("content", HighlightField::new()
                    .fragment_size(150)
                    .number_of_fragments(3)
                ),
            ])
            .pre_tags(["<mark>"])
            .post_tags(["</mark>"])
    )
```

Access highlights in results:

```rust
for hit in results.hits.hits {
    if let Some(highlights) = &hit.highlight {
        if let Some(fragments) = highlights.get("content") {
            println!("{}", fragments.join(" ... "));
        }
    }
}
```

## Aggregations

See the dedicated sections in [`search-queries.md`](../examples/search-queries.md) for extensive aggregation examples. A quick reference:

```rust
use opensearch_dsl::Aggregation;

Search::new()
    .size(0)  // Don't return documents when only aggregations are needed
    .aggregations([
        // Metrics
        ("avg_price",   Aggregation::avg("price")),
        ("max_views",   Aggregation::max("view_count")),
        ("total",       Aggregation::value_count("id")),
        ("price_stats", Aggregation::stats("price")),

        // Bucket
        ("by_status",   Aggregation::terms("status").size(10)),
        ("by_month",    Aggregation::date_histogram("created_at")
                            .calendar_interval("month")),

        // Nested bucket + metric
        ("by_category", Aggregation::terms("category").size(20)
                            .aggregations([
                                ("avg_price", Aggregation::avg("price")),
                            ])),
    ])
```

## Pagination Patterns

### Offset pagination (simple, max ~10k results)

```rust
Search::new()
    .query(Query::match_all())
    .from(page * per_page)
    .size(per_page)
```

### Search After (cursor-based, unlimited depth)

Requires a stable sort with a unique tiebreaker field:

```rust
// First page
let search = Search::new()
    .sort([
        Sort::field("created_at").order(SortOrder::Desc),
        Sort::field("id").order(SortOrder::Asc),  // tiebreaker
    ])
    .size(20);

let page1 = BlogPost::find(search).await?;
let last_hit = page1.hits.hits.last().unwrap();

// Next page — use sort values from last hit
let search = Search::new()
    .sort([
        Sort::field("created_at").order(SortOrder::Desc),
        Sort::field("id").order(SortOrder::Asc),
    ])
    .search_after(last_hit.sort.clone().unwrap())
    .size(20);
```

## Serializing Queries

Since `opensearch-dsl` is pure `serde`, you can serialize any query to JSON for debugging or logging:

```rust
let query = Query::bool()
    .must(Query::match_("title", "rust"))
    .filter(Query::term("published", true));

let json = serde_json::to_string_pretty(&Search::new().query(query))?;
println!("{}", json);
```

## Testing Queries

Unit-test query structure without a live cluster:

```rust
#[test]
fn test_search_query() {
    let search = Search::new()
        .query(Query::term("status", "active"))
        .size(5);

    let value = serde_json::to_value(&search).unwrap();
    assert_eq!(value["size"], 5);
    assert_eq!(value["query"]["term"]["status"], "active");
}
```
