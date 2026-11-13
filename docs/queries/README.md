# Query DSL Guide

`opensearch-dsl` provides a fully type-safe Rust API for building OpenSearch queries and aggregations. The crate is independent of the HTTP client so it can be used standalone for serialization or testing.

**Required feature:** `search` (default) pulls in `opensearch-dsl` automatically.

## Quick Start

```rust
use opensearch_dsl::*;

let search = Search::new()
    .query(
        Query::bool()
            .must([Query::r#match("title", "rust async")])
            .filter([Query::term("published", true)])
    )
    .sort([FieldSort::descending("created_at")])
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
    .aggregate("name", agg)   // Aggregations, chained (see Aggregations section)
    .highlight(...)           // Highlighting
    .search_after([...])      // Cursor-based deep pagination
    .min_score(0.5);          // Filter by relevance score
```

## Full-Text Queries

### `match`

Standard full-text search. The query string is analyzed before matching.

```rust
// Basic match
Query::r#match("content", "opensearch tutorial")

// With options
Query::r#match("content", "opensearch tutorial")
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
        Query::r#match("title", "opensearch"),
        Query::range("view_count").gte(100),
    ])
    .filter([
        Query::term("published", true),
        Query::term("language", "en"),
    ])
    .should([
        Query::term("featured", true),
        Query::r#match("tags", "tutorial"),
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
Query::boosting(
    Query::r#match("content", "opensearch tutorial"),  // positive
    Query::term("quality", "low"),                     // negative
    0.2,                                               // negative_boost
)
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
Query::dis_max()
    .query([
        Query::r#match("title", "quick brown fox"),
        Query::r#match("content", "quick brown fox"),
    ])
    .tie_breaker(0.3)  // Add fraction of other matches' scores
```

## Geo Queries

Requires a `geo_point` or `geo_shape` field mapping.

```rust
// Documents within a radius (field, origin, distance)
Query::geo_distance(
    "location",
    GeoLocation::new(40.7128, -74.0060),
    "10km",
)

// Documents within a bounding box (field, bounding_box)
Query::geo_bounding_box(
    "location",
    GeoBoundingBox::MainDiagonal {
        top_left: GeoLocation::new(40.8, -74.1),
        bottom_right: GeoLocation::new(40.6, -73.9),
    },
)
```

## Nested and Join Queries

### `nested`

Query nested objects (mapped as `"type": "nested"`):

```rust
Query::nested(
    "comments",
    Query::bool()
        .must([
            Query::r#match("comments.body", "great article"),
            Query::term("comments.approved", true),
        ]),
)
.score_mode(NestedQueryScoreMode::Average)  // Average, Sum, Minimum, Maximum, None
```

### `has_child` / `has_parent`

For parent-join field mappings:

```rust
// Documents that have a child matching the query (type, query)
Query::has_child("comment", Query::term("approved", true))
    .min_children(1)
    .score_mode(HasChildScoreMode::Sum)

// Documents whose parent matches the query (parent_type, query)
Query::has_parent("blog", Query::term("featured", true))
    .score(true)
```

## Sorting

```rust
use opensearch_dsl::FieldSort;

Search::new()
    .sort([
        FieldSort::descending("_score"),        // _score desc (default)
        FieldSort::descending("created_at"),
        FieldSort::ascending("title.keyword"),
    ])
```

`SortCollection` offers the same thing with a fluent chain, and bare field-name
strings also work as a shorthand:

```rust
use opensearch_dsl::SortCollection;

Search::new().sort(SortCollection::new().descending("created_at").ascending("id"));
Search::new().sort(["created_at"]);   // string shorthand
```

Sort by a field inside a nested object using its dotted path:

```rust
FieldSort::descending("ratings.value")
```

## Source Filtering

```rust
// Disable _source (saves bandwidth when you don't need the document body)
Search::new().source(false)

// Include only specific fields
Search::new().source(["id", "title", "created_at"])

// Include some fields and exclude others via an (includes, excludes) tuple
Search::new().source((["*"], ["internal_notes", "raw_html"]))
```

## Highlighting

```rust
use opensearch_dsl::{Highlight, Highlighter};

Search::new()
    .query(Query::r#match("content", "rust async"))
    .highlight(
        Highlight::new()
            .highlighter(
                Highlighter::new()
                    .fragment_size(150)
                    .number_of_fragments(3)
                    .tags((["<mark>"], ["</mark>"]))
            )
            .field("title")
            .field("content")
    )
```

Access highlights in results. On a `TypedHit`, `highlight` is a
`Map<String, Vec<String>>` (not an `Option`):

```rust
for hit in results.hits.hits {
    if let Some(fragments) = hit.highlight.get("content") {
        println!("{}", fragments.join(" ... "));
    }
}
```

## Aggregations

See the dedicated sections in [`search-queries.md`](../examples/search-queries.md) for extensive aggregation examples. A quick reference:

```rust
use opensearch_dsl::{Aggregation, CalendarInterval};

Search::new()
    .size(0)  // Don't return documents when only aggregations are needed
    // Metrics — add one aggregation per `.aggregate(..)` call
    .aggregate("avg_price",     Aggregation::avg("price"))
    .aggregate("max_views",     Aggregation::max("view_count"))
    .aggregate("min_price",     Aggregation::min("price"))
    .aggregate("unique_authors", Aggregation::cardinality("author_id"))
    // Bucket
    .aggregate("by_status",   Aggregation::terms("status").size(10))
    .aggregate("by_month",    Aggregation::date_histogram("created_at")
                                  .calendar_interval(CalendarInterval::Month))
    // Nested bucket + metric (sub-aggregations also use `.aggregate(..)`)
    .aggregate("by_category", Aggregation::terms("category").size(20)
                                  .aggregate("avg_price", Aggregation::avg("price")))
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
        FieldSort::descending("created_at"),
        FieldSort::ascending("id"),  // tiebreaker
    ])
    .size(20);

let page1 = BlogPost::find(search).await?;
let last_hit = page1.hits.hits.last().unwrap();

// Next page — use sort values from last hit (`sort` is a Vec, not an Option)
let search = Search::new()
    .sort([
        FieldSort::descending("created_at"),
        FieldSort::ascending("id"),
    ])
    .search_after(last_hit.sort.clone())
    .size(20);
```

## Serializing Queries

Since `opensearch-dsl` is pure `serde`, you can serialize any query to JSON for debugging or logging:

```rust
let query = Query::bool()
    .must([Query::r#match("title", "rust")])
    .filter([Query::term("published", true)]);

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
