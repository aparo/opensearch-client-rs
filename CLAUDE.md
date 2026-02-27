# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build

# Format (check only)
cargo fmt --check

# Format (apply)
cargo fmt

# Lint
cargo clippy -- -D warnings

# Sort workspace dependencies
cargo sort -w

# Unit tests (all crates)
cargo test

# Unit tests (specific crate)
cargo test --package opensearch-dsl
cargo test --package opensearch-client
cargo test --package opensearch-cli

# Single test by name
cargo test --package opensearch-dsl test_name

# Integration tests (requires running OpenSearch)
cargo test --features integration-tests
```

### Running OpenSearch for Integration Tests

```bash
docker run -d --name opensearch-test \
  -p 9200:9200 \
  -e "discovery.type=single-node" \
  -e "DISABLE_SECURITY_PLUGIN=true" \
  opensearchproject/opensearch:latest
```

## Architecture

This is a Rust workspace (Edition 2024) with 5 crates:

### Crate Overview

| Crate | Role |
|-------|------|
| `opensearch-client` | Core HTTP client, all OpenSearch API bindings |
| `opensearch-dsl` | Type-safe query/aggregation DSL, independent of the client |
| `opensearch-macro` | `#[derive(OpenSearch)]` proc-macro for the Document ORM |
| `opensearch-cli` | CLI tool for cluster management and data migration |
| `opensearch-testcontainer` | Docker-based test fixtures via `testcontainers` |

**Dependency flow:** `opensearch-cli` → `opensearch-client` → `opensearch-dsl` + `opensearch-macro`

### `opensearch-client`

The main crate with 25+ feature-gated API modules (default features: `bon`, `search`, `indices`, `ingest`, `cluster`, `ml`). Each API group (search, indices, cluster, bulk, sql, ml, security, snapshot, nodes, tasks, cat, etc.) is behind its own feature flag for minimal binary size.

Key files:
- `src/lib.rs` — module organization, re-exports
- `src/document.rs` — `Document` trait definition (ORM-like interface)
- `src/bulker.rs` — smart bulk indexer with automatic batching
- `src/client/` — HTTP client configuration, auth, middleware

The `Document` trait (generated via `#[derive(OpenSearch)]`) provides methods like `save()`, `get()`, `delete()`, `find()`, `find_all()`, `count()` directly on struct types.

### `opensearch-dsl`

Standalone query building library. Core structure:
- `src/search/queries/` — all query types organized by category:
  - `compound/` (bool, constant_score), `term_level/` (term, range, exists), `full_text/` (match, multi_match), `geo/`, `span/`, `joining/`, `specialized/`
- `src/search/aggregation/` — bucket, metric, and pipeline aggregations
- `src/search/sort/`, `highlight/`, `suggesters/`, `collapse/`, `rescoring/`
- `src/search/response/` — typed response parsing structs
- `src/analyze/` — text analysis API types

### `opensearch-macro`

Proc-macro crate that generates `Document` trait impls. Struct attributes:
- `#[os(index = "index_name")]` — required, sets the index name
- `#[os(id)]` — marks the document ID field
- `#[os(skip)]` — exclude from document
- `#[os(none_value = "...")]` — default for Option fields

### `opensearch-cli`

CLI with subcommands: `dump-metadata`, `restore-metadata`, `list-indices`. Supports `--server`/`--user`/`--password` flags and environment variables `OPENSEARCH_URL`, `OPENSEARCH_USER`, `OPENSEARCH_PASSWORD`. Also supports remote cluster flags for copy/migration operations.

## Key Patterns

- **Feature flags**: API modules are gated; when adding features, check which feature flag they belong to and keep the pattern consistent.
- **Builder pattern**: Queries, search requests, and client configuration all use fluent builder APIs.
- **Async throughout**: All I/O uses `tokio`; avoid blocking calls in async context.
- **Integration tests**: Gated by `#[cfg(feature = "integration-tests")]` and require a live OpenSearch instance.
- **Pre-commit hooks**: `cargo fmt` and `cargo sort -w` are configured as pre-commit steps (see `[workspace.metadata.precommit]` in root `Cargo.toml`).
