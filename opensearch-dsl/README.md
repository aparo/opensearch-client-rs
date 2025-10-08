# OpenSearch DSL for Rust

[![Crates.io](https://img.shields.io/crates/v/opensearch-dsl)](https://crates.io/crates/opensearch-dsl)
[![Crates.io](https://img.shields.io/crates/l/opensearch-dsl)](https://crates.io/crates/opensearch-dsl)
[![Crates.io](https://img.shields.io/crates/d/opensearch-dsl)](https://crates.io/crates/opensearch-dsl)
[![Docs.io](https://docs.rs/opensearch-dsl/badge.svg)](https://docs.rs/opensearch-dsl)

A high-level, strongly typed Domain Specific Language (DSL) for building OpenSearch queries in Rust. This library provides a complete mapping to the OpenSearch Query DSL with compile-time type safety.

*Based on the excellent [elasticsearch-dsl-rs](https://github.com/vinted/elasticsearch-dsl-rs) project, adapted for OpenSearch.*

## 🚀 Features

- **🔒 Type Safety**: Strongly typed queries, aggregations, and responses with compile-time validation
- **🎯 Complete Coverage**: Full support for OpenSearch Query DSL including complex nested queries
- **📊 Rich Aggregations**: Support for all aggregation types with proper result parsing
- **🧩 Composable**: Build complex queries by composing smaller query components
- **⚡ Zero-Cost Abstractions**: Compiles to efficient JSON with no runtime overhead
- **🔌 Client Agnostic**: Works with any HTTP client, not tied to specific OpenSearch client libraries
- **📝 Auto-Generated JSON**: Automatically produces valid OpenSearch JSON from Rust code
- **🎨 Fluent API**: Chainable method calls for intuitive query building

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
opensearch-dsl = "0.3"
```

For integration with the opensearch-client:

```toml
[dependencies]
opensearch-dsl = "0.3"
opensearch-client = "0.3"
```

## 📚 Table of Contents

- [Quick Start](#quick-start)
- [Query Types](#query-types)
- [Aggregations](#aggregations)
- [Search Response Parsing](#search-response-parsing)
- [Advanced Usage](#advanced-usage)
- [Examples](#examples)

## 🚀 Quick Start

### Basic Search Query

```rust
use opensearch_dsl::*;

let search = Search::new()
    .source(false)
    .from(0)
    .size(10)
    .query(Query::match_all())
    .sort(vec![Sort::field("timestamp").desc()]);

// Generates:
// {
//   "_source": false,
//   "from": 0,
//   "size": 10,
//   "query": { "match_all": {} },
//   "sort": [{ "timestamp": { "order": "desc" } }]
// }
```

### Complex Boolean Query

```rust
let search = Search::new()
    .query(
        Query::bool()
            .must(vec![
                Query::match_("title", "OpenSearch"),
                Query::range("date").gte("2023-01-01")
            ])
            .should(vec![
                Query::term("category", "tutorial"),
                Query::term("featured", true)
            ])
            .filter(vec![
                Query::term("status", "published")
            ])
            .minimum_should_match(1)
    );
```

## 🔍 Query Types

### Full-Text Queries

#### Match Query
```rust
// Simple match
Query::match_("title", "OpenSearch tutorial")

// Match with options
Query::match_("content", "search engine")
    .fuzziness("AUTO")
    .operator("and")
    .analyzer("english")
```

#### Multi-Match Query
```rust
Query::multi_match(
    vec!["title^2", "content", "tags"],
    "OpenSearch"
)
.type_("best_fields")
.tie_breaker(0.3)
```

#### Match Phrase
```rust
Query::match_phrase("content", "machine learning")
    .slop(2)
```

#### Query String
```rust
Query::query_string("title:OpenSearch AND status:active")
    .default_field("content")
    .allow_leading_wildcard(false)
```

### Term-Level Queries

#### Term Query
```rust
Query::term("status", "published")
Query::term("user_id", 12345)
Query::term("active", true)
```

#### Terms Query
```rust
Query::terms("tags", vec!["rust", "opensearch", "search"])
```

#### Range Query
```rust
Query::range("timestamp")
    .gte("2023-01-01")
    .lte("2023-12-31")
    .format("yyyy-MM-dd")

Query::range("price")
    .gte(10.0)
    .lt(100.0)
```

#### Exists Query
```rust
Query::exists("optional_field")
```

#### Wildcard and Regex
```rust
Query::wildcard("name", "John*")
Query::regexp("email", ".*@company\\.com")
```

### Compound Queries

#### Boolean Query
```rust
Query::bool()
    .must(vec![
        Query::match_("title", "search"),
        Query::range("date").gte("2023-01-01")
    ])
    .must_not(vec![
        Query::term("status", "deleted")
    ])
    .should(vec![
        Query::term("featured", true).boost(2.0),
        Query::match_("tags", "trending")
    ])
    .filter(vec![
        Query::term("published", true)
    ])
    .minimum_should_match(1)
```

#### Boosting Query
```rust
Query::boosting()
    .positive(Query::match_("content", "opensearch"))
    .negative(Query::match_("content", "deprecated"))
    .negative_boost(0.2)
```

#### Constant Score Query
```rust
Query::constant_score()
    .filter(Query::term("category", "electronics"))
    .boost(1.2)
```

### Nested and Join Queries

#### Nested Query
```rust
Query::nested("comments")
    .query(
        Query::bool()
            .must(vec![
                Query::match_("comments.author", "John"),
                Query::range("comments.date").gte("2023-01-01")
            ])
    )
    .score_mode("avg")
```

#### Has Child Query
```rust
Query::has_child("comment")
    .query(Query::match_("text", "great article"))
    .score_mode("sum")
```

#### Has Parent Query
```rust
Query::has_parent("article")
    .query(Query::term("category", "technology"))
    .score(true)
```

## 📊 Aggregations

### Bucket Aggregations

#### Terms Aggregation
```rust
Aggregation::terms("category")
    .size(10)
    .order(vec![("_count", "desc")])
    .min_doc_count(1)
```

#### Date Histogram
```rust
Aggregation::date_histogram("timestamp", "month")
    .format("yyyy-MM")
    .min_doc_count(0)
    .extended_bounds(DateHistogramBounds {
        min: "2023-01-01".to_string(),
        max: "2023-12-31".to_string(),
    })
```

#### Range Aggregation
```rust
Aggregation::range("price")
    .ranges(vec![
        AggregationRange::to(50.0),
        AggregationRange::from_to(50.0, 100.0),
        AggregationRange::from(100.0),
    ])
```

#### Histogram
```rust
Aggregation::histogram("price", 10.0)
    .min_doc_count(1)
    .extended_bounds(HistogramBounds {
        min: 0.0,
        max: 1000.0,
    })
```

### Metric Aggregations

#### Basic Metrics
```rust
Aggregation::avg("price")
Aggregation::sum("revenue")
Aggregation::min("date")
Aggregation::max("score")
Aggregation::cardinality("user_id")
```

#### Stats Aggregations
```rust
Aggregation::stats("response_time")
Aggregation::extended_stats("latency")
```

#### Percentiles
```rust
Aggregation::percentiles("response_time")
    .percents(vec![50.0, 95.0, 99.0])

Aggregation::percentile_ranks("response_time")
    .values(vec![100.0, 500.0, 1000.0])
```

#### Top Hits
```rust
Aggregation::top_hits()
    .size(3)
    .sort(vec![Sort::field("timestamp").desc()])
    .source(SourceFilter::includes(vec!["title", "author"]))
```

### Pipeline Aggregations

#### Bucket Script
```rust
Aggregation::bucket_script("sales_bucket_sort")
    .buckets_path(hashmap! {
        "sales" => "sales".to_string(),
        "returns" => "returns".to_string(),
    })
    .script("params.sales - params.returns")
```

#### Cumulative Sum
```rust
Aggregation::cumulative_sum("cumulative_sales")
    .buckets_path("sales")
```

### Nested Aggregations

```rust
Aggregation::terms("category")
    .size(10)
    .sub_aggregation("monthly_sales",
        Aggregation::date_histogram("date", "month")
            .sub_aggregation("total_revenue", Aggregation::sum("price"))
            .sub_aggregation("avg_rating", Aggregation::avg("rating"))
    )
    .sub_aggregation("top_products",
        Aggregation::terms("product_id")
            .size(5)
            .sub_aggregation("product_stats", Aggregation::stats("price"))
    )
```

## 📈 Search Response Parsing

The DSL includes strongly typed response structures for parsing OpenSearch results:

### Basic Response Handling

```rust
use opensearch_dsl::search::response::*;

// Assuming you have a search response JSON
let response: SearchResponse = serde_json::from_str(&response_json)?;

// Access hits
for hit in response.hits.hits {
    println!("Document ID: {}", hit.id);
    println!("Score: {:?}", hit.score);
    println!("Source: {:?}", hit.source);
    
    // Access highlights
    if let Some(highlight) = hit.highlight {
        for (field, fragments) in highlight {
            println!("Highlighted {}: {:?}", field, fragments);
        }
    }
}

// Access aggregations
if let Some(aggregations) = response.aggregations {
    // Handle bucket aggregations
    if let Some(category_agg) = aggregations.get("categories") {
        if let AggregationResponse::Bucket(bucket) = category_agg {
            for bucket_item in &bucket.buckets {
                println!("Category: {}, Count: {}", bucket_item.key, bucket_item.doc_count);
                
                // Access sub-aggregations
                if let Some(sub_aggs) = &bucket_item.aggregations {
                    // Process nested aggregations
                }
            }
        }
    }
    
    // Handle metric aggregations
    if let Some(avg_agg) = aggregations.get("average_price") {
        if let AggregationResponse::Simple(simple) = avg_agg {
            if let Some(value) = simple.value {
                println!("Average price: {}", value);
            }
        }
    }
}
```

### Working with Aggregation Results

```rust
// Example: Processing complex aggregation hierarchy
fn process_sales_analysis(response: &SearchResponse) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(aggregations) = &response.aggregations {
        // Monthly sales data
        if let Some(AggregationResponse::DateHistogram(monthly)) = aggregations.get("monthly_sales") {
            for bucket in &monthly.buckets {
                println!("Month: {}", bucket.key_as_string.as_ref().unwrap_or(&bucket.key));
                println!("Orders: {}", bucket.doc_count);
                
                // Revenue for this month
                if let Some(sub_aggs) = &bucket.aggregations {
                    if let Some(AggregationResponse::Simple(revenue)) = sub_aggs.get("total_revenue") {
                        println!("Revenue: ${:.2}", revenue.value.unwrap_or(0.0));
                    }
                }
            }
        }
        
        // Top categories
        if let Some(AggregationResponse::Bucket(categories)) = aggregations.get("top_categories") {
            println!("\nTop Categories:");
            for bucket in &categories.buckets {
                println!("- {}: {} sales", bucket.key, bucket.doc_count);
            }
        }
    }
    
    Ok(())
}
```

## ⚙️ Advanced Usage

### Custom Source Filtering

```rust
let search = Search::new()
    .source(SourceFilter::includes(vec!["title", "author", "date"]))
    .query(Query::match_all());

// Or exclude specific fields
let search = Search::new()
    .source(SourceFilter::excludes(vec!["internal_data", "temp_field"]))
    .query(Query::match_all());
```

### Highlighting

```rust
let search = Search::new()
    .query(Query::match_("content", "search terms"))
    .highlight(
        Highlight::new()
            .field("title", HighlightField::new().fragment_size(150))
            .field("content", HighlightField::new().number_of_fragments(3))
            .pre_tags(vec!["<mark>"])
            .post_tags(vec!["</mark>"])
    );
```

### Sorting

```rust
let search = Search::new()
    .sort(vec![
        Sort::field("timestamp").desc(),
        Sort::field("score").desc(),
        Sort::field("title.keyword").asc().missing("_last"),
        Sort::geo_distance("location", GeoLocation::LatLon { lat: 40.7128, lon: -74.0060 })
            .unit("km")
            .order("asc")
    ]);
```

### Script Fields

```rust
let search = Search::new()
    .script_fields(hashmap! {
        "calculated_field" => ScriptField::new()
            .script("Math.log(2 + doc['views'].value)")
    });
```

### Rescoring

```rust
let search = Search::new()
    .query(Query::match_("content", "opensearch"))
    .rescore(vec![
        Rescore::new(
            Query::match_phrase("content", "opensearch tutorial")
                .boost(2.0)
        )
        .query_weight(0.7)
        .rescore_query_weight(1.2)
        .window_size(100)
    ]);
```

### Search Templates

```rust
// Define a reusable search template
fn create_user_search_template(query: &str, filters: &HashMap<String, Value>) -> Search {
    Search::new()
        .query(
            Query::bool()
                .must(vec![Query::multi_match(vec!["name", "email"], query)])
                .filter(
                    filters.iter()
                        .map(|(field, value)| Query::term(field, value.clone()))
                        .collect()
                )
        )
        .sort(vec![Sort::field("created_at").desc()])
        .size(50)
}
```

## 🎯 Complete Examples

### E-commerce Product Search

```rust
use opensearch_dsl::*;
use std::collections::HashMap;

fn build_product_search(
    query: &str,
    category: Option<&str>,
    price_range: Option<(f64, f64)>,
    in_stock: bool,
    page: usize,
    size: usize,
) -> Search {
    let mut bool_query = Query::bool()
        .must(vec![
            Query::multi_match(
                vec!["name^3", "description", "brand^2"],
                query
            ).fuzziness("AUTO")
        ])
        .filter(vec![
            Query::term("in_stock", in_stock)
        ]);
    
    // Add category filter if specified
    if let Some(cat) = category {
        bool_query = bool_query.filter(vec![Query::term("category", cat)]);
    }
    
    // Add price range filter if specified
    if let Some((min, max)) = price_range {
        bool_query = bool_query.filter(vec![
            Query::range("price").gte(min).lte(max)
        ]);
    }
    
    Search::new()
        .from(page * size)
        .size(size)
        .query(bool_query)
        .sort(vec![
            Sort::field("_score").desc(),
            Sort::field("popularity_score").desc()
        ])
        .aggregations(vec![
            ("categories", 
                Aggregation::terms("category")
                    .size(20)
            ),
            ("brands",
                Aggregation::terms("brand")
                    .size(15)
            ),
            ("price_ranges",
                Aggregation::range("price")
                    .ranges(vec![
                        AggregationRange::to(25.0),
                        AggregationRange::from_to(25.0, 50.0),
                        AggregationRange::from_to(50.0, 100.0),
                        AggregationRange::from(100.0),
                    ])
            ),
            ("rating_stats", Aggregation::stats("rating"))
        ])
        .highlight(
            Highlight::new()
                .field("name", HighlightField::new())
                .field("description", HighlightField::new().fragment_size(200))
        )
}
```

### Log Analytics Dashboard

```rust
fn build_log_analytics(
    time_range: (&str, &str),
    log_levels: Vec<&str>,
    services: Vec<&str>,
) -> Search {
    Search::new()
        .size(0)  // We only want aggregations
        .query(
            Query::bool()
                .filter(vec![
                    Query::range("timestamp")
                        .gte(time_range.0)
                        .lte(time_range.1),
                    Query::terms("level", log_levels),
                    Query::terms("service", services),
                ])
        )
        .aggregations(vec![
            // Time series of log levels
            ("logs_over_time",
                Aggregation::date_histogram("timestamp", "1h")
                    .sub_aggregation("by_level",
                        Aggregation::terms("level")
                    )
            ),
            
            // Error analysis
            ("error_analysis",
                Aggregation::filter(Query::term("level", "ERROR"))
                    .sub_aggregation("top_errors",
                        Aggregation::terms("message.keyword")
                            .size(10)
                    )
                    .sub_aggregation("affected_services",
                        Aggregation::terms("service")
                    )
            ),
            
            // Performance metrics
            ("response_time_stats",
                Aggregation::filter(Query::exists("response_time"))
                    .sub_aggregation("stats", Aggregation::extended_stats("response_time"))
                    .sub_aggregation("percentiles",
                        Aggregation::percentiles("response_time")
                            .percents(vec![50.0, 90.0, 95.0, 99.0])
                    )
            ),
            
            // Service health overview
            ("service_health",
                Aggregation::terms("service")
                    .sub_aggregation("error_rate",
                        Aggregation::filter(Query::term("level", "ERROR"))
                    )
                    .sub_aggregation("avg_response_time",
                        Aggregation::avg("response_time")
                    )
            )
        ])
}
```

### Time Series Analysis

```rust
fn build_metrics_analysis(metric_name: &str, interval: &str) -> Search {
    Search::new()
        .size(0)
        .query(Query::exists(metric_name))
        .aggregations(vec![
            // Main time series
            ("time_series",
                Aggregation::date_histogram("timestamp", interval)
                    .sub_aggregation("value_stats", Aggregation::extended_stats(metric_name))
                    .sub_aggregation("value_percentiles",
                        Aggregation::percentiles(metric_name)
                            .percents(vec![25.0, 50.0, 75.0, 90.0, 95.0])
                    )
            ),
            
            // Moving average (pipeline aggregation)
            ("moving_avg",
                Aggregation::date_histogram("timestamp", interval)
                    .sub_aggregation("avg_value", Aggregation::avg(metric_name))
                    .sub_aggregation("moving_avg",
                        Aggregation::moving_avg("avg_value")
                            .window(5)
                            .model("linear")
                    )
            ),
            
            // Anomaly detection buckets
            ("anomalies",
                Aggregation::date_histogram("timestamp", interval)
                    .sub_aggregation("value", Aggregation::avg(metric_name))
                    .sub_aggregation("anomaly_score",
                        Aggregation::bucket_script("anomaly_detection")
                            .buckets_path(hashmap! {
                                "current" => "value".to_string()
                            })
                            .script("Math.abs(params.current - 100) > 50 ? 1 : 0")
                    )
            )
        ])
}
```

## 🧪 Testing with Real Data

The DSL includes comprehensive tests using real OpenSearch response data:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complex_search_with_aggregations() {
        let search = Search::new()
            .query(
                Query::bool()
                    .must(vec![Query::match_("content", "opensearch")])
                    .filter(vec![Query::range("date").gte("2023-01-01")])
            )
            .aggregations(vec![
                ("categories", Aggregation::terms("category")),
                ("monthly_stats", 
                    Aggregation::date_histogram("date", "month")
                        .sub_aggregation("avg_score", Aggregation::avg("score"))
                )
            ]);
        
        let json = serde_json::to_string_pretty(&search).unwrap();
        
        // Verify the generated JSON matches expected structure
        assert!(json.contains("\"bool\""));
        assert!(json.contains("\"aggregations\""));
    }
}
```

## 🛠 Development and Contributing

### Building from Source

```bash
git clone https://github.com/aparo/opensearch-client-rs.git
cd opensearch-client-rs/opensearch-dsl
cargo build
```

### Running Tests

```bash
cargo test
```

### Generating Documentation

```bash
cargo doc --open
```

## 🔗 Integration with OpenSearch Client

The DSL works seamlessly with the opensearch-client:

```rust
use opensearch_client::*;
use opensearch_dsl::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OsClient::new(/* configuration */);
    
    let search = Search::new()
        .query(Query::match_("title", "rust"))
        .aggregations(vec![
            ("tags", Aggregation::terms("tags"))
        ]);
    
    let response = client
        .search(&search)
        .index("articles")
        .await?;
    
    // Process strongly typed response
    for hit in response.hits.hits {
        println!("Article: {:?}", hit.source);
    }
    
    if let Some(aggregations) = response.aggregations {
        // Process aggregation results with type safety
    }
    
    Ok(())
}
```

## 📋 Query DSL Reference

### Available Query Types

| Query Type | Description | Example |
|------------|-------------|---------|
| `match_` | Full-text search | `Query::match_("title", "search")` |
| `multi_match` | Multi-field search | `Query::multi_match(vec!["title", "content"], "query")` |
| `term` | Exact term match | `Query::term("status", "published")` |
| `terms` | Multiple exact terms | `Query::terms("tags", vec!["rust", "search"])` |
| `range` | Range queries | `Query::range("price").gte(10).lte(100)` |
| `bool` | Boolean combinations | `Query::bool().must(vec![...])` |
| `exists` | Field existence | `Query::exists("optional_field")` |
| `prefix` | Prefix matching | `Query::prefix("name", "john")` |
| `wildcard` | Wildcard patterns | `Query::wildcard("name", "j*n")` |
| `regexp` | Regular expressions | `Query::regexp("email", ".*@domain\\.com")` |
| `fuzzy` | Fuzzy matching | `Query::fuzzy("name", "john").fuzziness(2)` |
| `nested` | Nested object queries | `Query::nested("comments").query(...)` |

### Aggregation Types Reference

| Aggregation Type | Description | Example |
|------------------|-------------|---------|
| `terms` | Group by field values | `Aggregation::terms("category")` |
| `date_histogram` | Time-based grouping | `Aggregation::date_histogram("date", "month")` |
| `histogram` | Numeric histogram | `Aggregation::histogram("price", 10.0)` |
| `range` | Custom ranges | `Aggregation::range("price").ranges(...)` |
| `avg` | Average values | `Aggregation::avg("price")` |
| `sum` | Sum of values | `Aggregation::sum("quantity")` |
| `min`/`max` | Min/max values | `Aggregation::min("date")` |
| `cardinality` | Unique value count | `Aggregation::cardinality("user_id")` |
| `stats` | Statistical summary | `Aggregation::stats("response_time")` |
| `percentiles` | Percentile analysis | `Aggregation::percentiles("latency")` |
| `top_hits` | Top matching docs | `Aggregation::top_hits().size(3)` |

## 🎯 Best Practices

### 1. Query Optimization

```rust
// Use filters for exact matches (cached)
Query::bool()
    .must(vec![Query::match_("content", "search")])
    .filter(vec![Query::term("status", "published")])

// Use boosting for relevance tuning
Query::match_("title", "important").boost(2.0)
```

### 2. Aggregation Performance

```rust
// Limit aggregation size
Aggregation::terms("category").size(100)

// Use composite aggregations for large datasets
Aggregation::composite()
    .sources(vec![
        CompositeSource::terms("category", "category"),
        CompositeSource::date_histogram("date", "date", "month")
    ])
    .size(1000)
```

### 3. Memory Management

```rust
// Use source filtering to reduce payload
Search::new()
    .source(SourceFilter::includes(vec!["id", "title"]))
    .query(...)

// Limit result size
Search::new()
    .size(20)
    .from(0)
    .query(...)
```

## 🚨 Error Handling

The DSL provides compile-time safety, but runtime errors can still occur:

```rust
use opensearch_dsl::*;

// This will compile but may fail at runtime if the field doesn't exist
let search = Search::new()
    .query(Query::term("non_existent_field", "value"));

// Better: Use exists query first
let search = Search::new()
    .query(
        Query::bool()
            .must(vec![Query::exists("field")])
            .filter(vec![Query::term("field", "value")])
    );
```

## 📚 Additional Resources

- [OpenSearch Documentation](https://opensearch.org/docs/)
- [OpenSearch Query DSL Reference](https://opensearch.org/docs/latest/opensearch/query-dsl/)
- [API Documentation](https://docs.rs/opensearch-dsl)
- [GitHub Repository](https://github.com/aparo/opensearch-client-rs)
- [Examples Directory](examples/)

## 🤝 Contributing

Contributions are welcome! Please see our [Contributing Guide](../CONTRIBUTING.md) for details.

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

*This library is based on the excellent [elasticsearch-dsl-rs](https://github.com/vinted/elasticsearch-dsl-rs) project and adapted for OpenSearch.*
