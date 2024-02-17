# Strongly typed OpenSearch DSL written in Rust

[![Crates.io](https://img.shields.io/crates/v/opensearch-dsl)](https://crates.io/crates/opensearch-dsl)
[![Crates.io](https://img.shields.io/crates/l/opensearch-dsl)](https://crates.io/crates/opensearch-dsl)
[![Crates.io](https://img.shields.io/crates/d/opensearch-dsl)](https://crates.io/crates/opensearch-dsl)
[![Docs.io](https://docs.rs/opensearch-dsl/badge.svg)](https://docs.rs/opensearch-dsl)

A high level library, giving a strongly typed DSL that maps one to one with the official OpenSearch query DSL.
This is a Opensearch fork based on the https://github.com/vinted/elasticsearch-dsl-rs

## Features

- Strongly typed queries
- Strongly typed aggregations
- Strongly typed completions
- Response structures
- Automatically skips empty queries making DSL pleasant to use
- Crate doesn't depend on [opensearch-rs](https://github.com/opensearch/opensearch-rs) and can
  be used as a standalone library with any HTTP client to call OpenSearch

## Installation

Add `opensearch-dsl` crate and version to Cargo.toml

```toml
[dependencies]
opensearch-dsl = "0.4"
```

## Documentation

Documentation for the library is available on [docs.rs](https://docs.rs/opensearch-dsl)

## Quick start

```rust
use opensearch_dsl::*;

fn main() {
    let query = Search::new()
        .source(false)
        .stats("statistics")
        .from(0)
        .size(30)
        .query(
            Query::bool()
                .must(Query::multi_match(
                    ["title", "description"],
                    "you know, for search",
                ))
                .filter(Query::terms("tags", ["opensearch"]))
                .should(Query::term("verified", true).boost(10)),
        )
        .aggregate(
            "country_ids",
            Aggregation::terms("country_id")
                .aggregate("catalog_ids", Aggregation::terms("catalog_id"))
                .aggregate("company_ids", Aggregation::terms("company_id"))
                .aggregate(
                    "top1",
                    Aggregation::top_hits()
                        .size(1)
                        .sort(FieldSort::ascending("user_id")),
                ),
        )
        .rescore(Rescore::new(Query::term("field", 1)).query_weight(1.2));
}
```

```json
{
  "_source": false,
  "stats": ["statistics"],
  "from": 0,
  "size": 30,
  "query": {
    "bool": {
      "must": [
        {
          "multi_match": {
            "fields": ["title", "description"],
            "query": "you know, for search"
          }
        }
      ],
      "filter": [{ "terms": { "tags": ["opensearch"] } }],
      "should": [{ "term": { "verified": { "value": true, "boost": 10.0 } } }]
    }
  },
  "aggs": {
    "country_ids": {
      "terms": { "field": "country_id" },
      "aggs": {
        "catalog_ids": { "terms": { "field": "catalog_id" } },
        "company_ids": { "terms": { "field": "company_id" } },
        "top1": {
          "top_hits": {
            "size": 1,
            "sort": [{ "user_id": { "order": "asc" } }]
          }
        }
      }
    }
  },
  "rescore": [
    {
      "query": {
        "rescore_query": { "term": { "field": { "value": 1 } } },
        "query_weight": 1.2
      }
    }
  ]
}
```

See [examples](examples) for more.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
