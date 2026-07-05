# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Upgraded `reqwest` to 0.13 (breaking: new connection/TLS API surface)
- Upgraded `serde_json` to latest
- Upgraded `tokio` to 1.5.0
- Expanded documentation: feature flags guide, query DSL reference, index
  management guide, architecture overview, and additional examples
- Added `CLAUDE.md` developer guide with build commands and architecture overview

---

## [0.3.1] — 2026-01-28

### Added
- `upsert` variants on the `Document` trait for additional update-or-insert
  workflows
- OpenSearch Test Container documentation

### Changed
- **Breaking:** document ID and index name parameters changed from `&str` to
  `String` throughout the `Document` trait and client methods; update call sites
  by passing `String::from("…")` or `.to_string()`
- Migrated workspace to **Rust edition 2024**
- Pinned `tokio` to 1.41.1 in `opensearch-testcontainer` to avoid a regression
- Added `std` feature to `serde` flatten attributes to fix deserialization of
  nested structs
- Dependency upgrades: `serde`, `tokio`, `cargo` toolchain

### Fixed
- Corrected documentation URL in crate metadata
- Fixed typo in public API surface

---

## [0.3.0] — 2025-09-07

### Added
- **`bon` builder pattern** (`bon` feature, enabled by default) — all request
  builders now use the `bon` derive macro, giving typed, compile-time-verified
  setters with no runtime overhead
- **`opensearch-macro` crate** — `#[derive(OpenSearch)]` proc-macro providing
  an ORM-like `Document` trait on user structs; supports attributes:
  - `#[os(index = "name")]` — target index
  - `#[os(id)]` — document ID field
  - `#[os(skip)]` — exclude field
  - `#[os(none_value = "…")]` — default for `Option` fields
- **`Document` trait** with methods `save()`, `get()`, `delete()`, `find()`,
  `find_all()`, `count()` generated automatically per struct
- **`opensearch-testcontainer` crate** restored — Docker-based test fixtures
  via the `testcontainers` crate for integration tests without manual setup
- Remote index copy command in `opensearch-cli` (`--remote-*` flags)
- Support for OpenSearch **2.19.0** API surface

### Changed
- Client now initialises from environment variables
  (`OPENSEARCH_URL`, `OPENSEARCH_USER`, `OPENSEARCH_PASSWORD`)
- Dependency upgrades: reqwest-middleware, tracing, anyhow, thiserror

### Fixed
- Edge case with empty pipeline list returned by OpenSearch server

---

## [0.2.0] — 2025-01 → 2025-08 (pre-release development)

> This period covers active development between the 0.1.x releases and the 0.3.0
> redesign. No formal release tag was cut, but the changes below landed on `main`
> and form the foundation for 0.3.0.

### Added
- `opensearch-cli` tool with subcommands:
  - `list-indices` — list cluster indices (with `--show-hidden` flag)
  - `dump-metadata` — export index templates, component templates, pipelines,
    and index mappings to JSON files
  - `restore-metadata` — re-import previously dumped metadata
  - `restore` command for full metadata restore workflow
- `tools` feature in `opensearch-client` wrapping dump/restore utilities
  (`walkdir` dependency, gated)
- Typed pipeline classes for ingest API
- `ResolveIndex` response type
- Security module extracted as its own feature-gated API group
- `MtermVectors` support
- Neural processors in ingest pipeline (OpenSearch ML integration)

### Changed
- Code split into feature-gated modules (`cat`, `cluster`, `dangling_indices`,
  `indices`, `ingest`, `insights`, `ism`, `knn`, `ml`, `nodes`, `notifications`,
  `observability`, `ppl`, `remote_store`, `replication`, `rollups`, `security`,
  `snapshot`, `sql`, `tasks`, `transforms`)
- Removed duplicated `IndexName`, `IndexNames`, `DocumentId`, `Timeout`,
  `MasterTimeout`, `ClusterManagerTimeout` structs — unified into shared types
- `IngestPutPipeline` builder accepts any `Serialize` type instead of raw JSON
- Simplified package-level method names for consistency

### Fixed
- Credential management in HTTP client
- Concurrent error handling in the bulk indexer

---

## [0.1.1] — 2024-04-19

### Changed
- Upgraded `reqwest` to 0.12.4 (HTTP/2 improvements, unified TLS)
- Upgraded `serde`, `serde_json`, `anyhow`, `thiserror`
- Upgraded `opensearch-testcontainer` dependencies

---

## [0.1.0] — 2024-02-17

First public release on [crates.io](https://crates.io/crates/opensearch-client).

### Added
- **`opensearch-client` crate** — async HTTP client for OpenSearch built on
  `reqwest` and `reqwest-middleware`
  - Authentication middleware (Basic Auth)
  - Retry middleware via `reqwest-retry`
  - Distributed-tracing middleware via `reqwest-tracing`
  - Client construction from URL + credentials or environment variables
- **`opensearch-dsl` crate** — standalone, strongly-typed query DSL
  - All compound queries: `bool`, `constant_score`
  - Term-level queries: `term`, `range`, `exists`, `terms`, `ids`, `prefix`,
    `wildcard`, `regexp`, `fuzzy`
  - Full-text queries: `match`, `match_phrase`, `multi_match`, `query_string`,
    `simple_query_string`
  - Geo, span, joining, and specialised query types
  - Bucket, metric, and pipeline aggregations
  - Sort, highlight, suggesters, collapse, and rescoring support
  - Typed search-response parsing structs (`SearchResponse<T>`, `Hit<T>`, etc.)
  - Full query deserialization (round-trip JSON ↔ Rust types)
- **Bulk API** — typed `BulkRequest`/`BulkResponse`; async `Bulker` with
  automatic batching, statistics, and concurrent error correction
- **`search_stream`** — strongly-typed streaming search using `futures::Stream`
- **Indices API** — create, delete, exists, get, put settings/mappings, aliases,
  templates, component templates, refresh, flush, open, close
- **Ingest API** — get/put/delete/simulate pipelines, test pipeline processor
  serialization
- **Cluster API** — health, stats, settings, allocation, tasks
- **ML API** — model register, get, delete, train, list available models
- **CAT API** — indices, nodes, health, aliases, shards
- **Snapshot API** — create, restore, delete, status
- **Nodes API** — info, stats, hot threads
- **Tasks API** — list, get, cancel
- **SQL API** — execute SQL queries against OpenSearch
- **QuickWit** feature flag for QuickWit-compatible endpoints
- **`opensearch-testcontainer` crate** — `testcontainers`-based Docker fixtures
  for integration tests
- Feature flags for zero-cost API selection (`default`, `full`, and individual
  module flags)
- Pre-commit hooks: `cargo fmt` + `cargo sort -w`

### Architecture
- Workspace with 5 crates: `opensearch-client`, `opensearch-dsl`,
  `opensearch-macro`, `opensearch-cli`, `opensearch-testcontainer`
- All I/O is fully async via `tokio`
- Integration tests gated behind `#[cfg(feature = "integration-tests")]`

---

[Unreleased]: https://github.com/aparo/opensearch-client-rs/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/aparo/opensearch-client-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/aparo/opensearch-client-rs/compare/v0.1.1...v0.3.0
[0.1.1]: https://github.com/aparo/opensearch-client-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/aparo/opensearch-client-rs/releases/tag/v0.1.0
