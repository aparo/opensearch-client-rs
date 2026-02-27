# Architecture Overview

This document describes the high-level design of the `opensearch-client-rs` workspace and how the crates fit together.

## Workspace Layout

```
opensearch-client-rs/
├── opensearch-client/       # Core HTTP client and all API bindings
├── opensearch-dsl/          # Standalone type-safe query/aggregation DSL
├── opensearch-macro/        # Procedural macro: #[derive(OpenSearch)]
├── opensearch-cli/          # Command-line tool for cluster management
└── opensearch-testcontainer/# Docker-based OpenSearch instances for tests
```

## Dependency Graph

```
opensearch-cli
  └─► opensearch-client (features: search, indices, tools)
        ├─► opensearch-dsl          (when feature "search" is enabled)
        └─► opensearch-macro        (always — provides #[derive(OpenSearch)])

opensearch-testcontainer
  └─► testcontainers             (Docker container lifecycle)
```

`opensearch-dsl` and `opensearch-macro` have no dependency on `opensearch-client`, so they can be used standalone.

## `opensearch-client`

The central crate. It provides:

- **`OsClient`** — the main HTTP client wrapping `reqwest` via `reqwest-middleware`.
- **`ConfigurationBuilder`** — fluent builder for URL, auth, timeouts, retry policy.
- **API modules** — one module per OpenSearch API group, each gated behind a feature flag (see [Feature Flags](features.md)).
- **`Document` trait** — an ORM-like interface for type-safe CRUD, implemented automatically by `#[derive(OpenSearch)]`.
- **`Bulker`** — a channel-based async bulk indexer with configurable batch size and concurrent connections.

### Request / Response Pattern

Every API operation follows the same shape:

```
OsClient::<group>()::<operation>()   →  request builder
    .send() / .await?                →  typed response struct
```

For example:

```rust
// client.search() returns a SearchRequest builder
// .send() executes the HTTP call
let resp = client.search().index("my_index").body(&search).send().await?;
```

### Global Client

For applications that want a singleton client, `opensearch-client` provides:

```rust
opensearch_client::set_opensearch(client);   // Set global
opensearch_client::get_opensearch();          // Retrieve &OsClient
```

The `Document` trait methods (e.g., `User::get("id")`) use the global client internally.

### Middleware Stack

The HTTP client is built on `reqwest-middleware`, enabling a composable middleware chain:

```
[application code]
       ↓
reqwest-middleware (TracingMiddleware, RetryMiddleware)
       ↓
reqwest (async HTTP)
       ↓
OpenSearch cluster
```

Tracing middleware emits `tracing` spans for every request; retry middleware handles transient failures with configurable backoff.

## `opensearch-dsl`

A pure-Rust query builder with no network I/O. Key modules:

- `search/queries/` — all query types, organized by category:
  - `compound/` — `bool`, `boosting`, `constant_score`, `dis_max`
  - `full_text/` — `match`, `match_phrase`, `multi_match`, `query_string`
  - `term_level/` — `term`, `terms`, `range`, `exists`, `wildcard`, `fuzzy`, `regexp`
  - `geo/` — `geo_distance`, `geo_bounding_box`, `geo_polygon`
  - `joining/` — `nested`, `has_child`, `has_parent`
  - `span/`, `specialized/`, `shape/`
- `search/aggregation/` — bucket (terms, histogram, range, date_histogram), metric (avg, sum, stats, percentiles), pipeline
- `search/sort/` — `Sort` builder with nested sort support
- `search/highlight/` — `Highlight` / `HighlightField` builders
- `search/response/` — typed response structs (`SearchResponse`, `Hit`, etc.)
- `analyze/` — text analysis request/response types

Everything serializes to JSON via `serde`. No runtime reflection — all types are checked at compile time.

## `opensearch-macro`

A `proc-macro` crate that expands `#[derive(OpenSearch)]` at compile time. It reads:

- **Struct-level attributes** — `#[os(index = "name")]`
- **Field-level attributes** — `#[os(id)]`, `#[os(skip)]`, `#[os(skip_sort)]`, `#[os(none_value = "...")]`

And generates:
- `impl Document for MyStruct { ... }` — providing `index_name()`, `columns()`, `save()`, `get()`, `delete()`, `update()`, `find()`, `find_all()`, `find_one()`, `count()`, `upsert_doc()`, and `upsert_value()`.

The macro produces code that calls methods on the global `OsClient` from `opensearch-client`, so the generated trait implementation depends on `opensearch-client` being in scope at the call site.

## `opensearch-cli`

A binary crate built with `clap`. It wraps `OsClient` to provide:

- `dump-metadata` / `restore-metadata` — export and import cluster configuration (index settings, templates, aliases, ingest pipelines, etc.)
- `list-indices` — list indices with stats
- `copy-index` — copy an index, optionally to a remote cluster

The CLI supports both local (`OPENSEARCH_*`) and remote (`OPENSEARCH_REMOTE_*`) cluster configuration via environment variables or CLI flags.

## `opensearch-testcontainer`

Thin wrapper around the `testcontainers` crate. Provides a pre-configured `OpenSearch` image descriptor so integration tests can start an ephemeral OpenSearch instance with a single call:

```rust
let docker = testcontainers::clients::Cli::default();
let node = docker.run(opensearch_testcontainer::OpenSearch::default());
let port = node.get_host_port_ipv4(9200);
```

## Design Principles

- **Feature flags over conditionals** — each API group is a separate feature so unused code is never compiled.
- **Builder pattern everywhere** — `ConfigurationBuilder`, `Search`, `Query::bool()`, etc. all use fluent APIs that produce value types (no `mut` threading).
- **Async-first** — all I/O is `async`; the crate requires a `tokio` runtime.
- **Serde as the only serialization layer** — no custom JSON construction; all types implement `Serialize`/`Deserialize`.
- **Compile-time correctness via macros** — the `#[derive(OpenSearch)]` macro validates index name and ID field at compile time, not runtime.
