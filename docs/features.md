# Feature Flags

`opensearch-client` uses Cargo feature flags to let you include only the OpenSearch APIs you actually need. Unused features are not compiled, which reduces both compile time and final binary size.

## Why Feature Flags Matter

OpenSearch exposes dozens of API groups. Compiling support for all of them adds:

- **Compile time**: each module adds type definitions, serialization code, and trait impls that rustc must process.
- **Binary size**: unused dead code is eliminated by the linker, but the type infrastructure for serde and reqwest still inflates the binary if features are compiled in.

By disabling features you don't need, a minimal binary (e.g., a microservice that only searches) can be significantly smaller and faster to compile than one compiled with `full`.

## Default Features

When you add `opensearch-client` without specifying features, you get:

```toml
[dependencies]
opensearch-client = "0.3"
```

This compiles:

| Feature | Included APIs |
|---------|---------------|
| `bon` | Builder-pattern derives for request types |
| `search` | Search and query APIs (pulls in `opensearch-dsl`) |
| `indices` | Index creation, deletion, settings, mappings (depends on `search`) |
| `ingest` | Ingest pipeline management |
| `cluster` | Cluster health, settings, stats |
| `ml` | Machine learning model management and inference |

## Opting Out of Defaults

Use `default-features = false` to start from zero and add only what you need:

```toml
[dependencies]
opensearch-client = { version = "0.3", default-features = false, features = ["search"] }
```

This is the recommended approach for production microservices where compile time and binary size are important.

## All Available Features

### Core

| Feature | Description | Depends on |
|---------|-------------|------------|
| `bon` | Builder pattern for request types | — |
| `search` | Search, query, scroll, multi-search APIs | `opensearch-dsl`, `futures` |
| `indices` | Index management (create, delete, settings, mappings, aliases, templates) | `search` |
| `ingest` | Ingest pipeline CRUD | — |
| `cluster` | Cluster health, stats, settings, reroute | — |
| `nodes` | Node info, stats, hot threads | — |
| `tasks` | Task management API | — |
| `cat` | Compact `_cat` APIs (indices, nodes, shards, etc.) | — |
| `snapshot` | Snapshot and restore | — |

### OpenSearch-Specific

| Feature | Description | Depends on |
|---------|-------------|------------|
| `ml` | ML model upload, deployment, inference | — |
| `knn` | k-NN plugin APIs | — |
| `sql` | SQL query interface | — |
| `ppl` | PPL (Piped Processing Language) query interface | — |
| `ism` | Index State Management policies | — |
| `asynchronous_search` | Async search submit/get/delete | — |
| `security` | Users, roles, role mappings, permissions | — |
| `replication` | Cross-cluster replication | — |
| `remote_store` | Remote store restore | — |
| `rollups` | Index rollup jobs | — |
| `transforms` | Index transforms | — |
| `dangling_indices` | Dangling index management | — |
| `notifications` | Notification channels | — |
| `observability` | Observability objects | — |
| `insights` | Query insights | — |

### Composite / Integration

| Feature | Description | Depends on |
|---------|-------------|------------|
| `tools` | High-level utility helpers (index creation from mapping files, etc.) | `indices`, `ingest`, `ml`, `walkdir`, `futures` |
| `loco` | Integration with the [Loco.rs](https://loco.rs) web framework | `loco-rs`, `axum` |
| `full` | All features above | everything |

## Common Configurations

### Search-only service

No index management, no cluster ops — just search:

```toml
[dependencies]
opensearch-client = { version = "0.3", default-features = false, features = ["search"] }
opensearch-dsl = "0.3"
```

### Search and index management

```toml
[dependencies]
# `indices` implies `search`
opensearch-client = { version = "0.3", default-features = false, features = ["indices"] }
opensearch-dsl = "0.3"
```

### Admin / ops tooling

Cluster management, snapshots, security:

```toml
[dependencies]
opensearch-client = { version = "0.3", default-features = false, features = [
    "indices",
    "cluster",
    "nodes",
    "snapshot",
    "security",
    "tasks",
    "cat",
]}
```

### ML inference service

```toml
[dependencies]
opensearch-client = { version = "0.3", default-features = false, features = [
    "search",
    "ml",
    "knn",
]}
```

### Data pipeline (bulk ingestion)

No search needed — just write:

```toml
[dependencies]
# `indices` for index creation, `ingest` for pipelines, `search` is pulled in by `indices`
opensearch-client = { version = "0.3", default-features = false, features = [
    "indices",
    "ingest",
]}
```

### Everything

For tooling or applications that genuinely need all APIs:

```toml
[dependencies]
opensearch-client = { version = "0.3", features = ["full"] }
```

## Compile Time Impact

Compile times are dominated by proc-macros (serde derive, bon) and large dependency trees. Enabling fewer features cuts the amount of code rustc processes in `opensearch-client` itself.

Rough guidance (measured on a 2024-era laptop, `cargo build --release`):

| Configuration | Approx. additional compile time vs. no client |
|--------------|------------------------------------------------|
| `search` only | fastest — only DSL + HTTP core |
| default features | moderate — 6 API groups |
| `full` | slowest — all 25+ API groups |

To benchmark your own build:

```bash
# Clean build with timing
cargo clean && cargo build --timings
# Then open target/cargo-timings/cargo-timing.html
```

## Binary Size Impact

With `default-features = false, features = ["search"]`, the client adds roughly 1–3 MB to a release binary (depending on how much the linker can dead-strip). With `full`, expect 2–5 MB more.

To inspect your binary:

```bash
# Install cargo-bloat
cargo install cargo-bloat

# Show biggest functions
cargo bloat --release --crates
```

## Feature Resolution

Cargo resolves features transitively. For example:

- Adding `indices` automatically enables `search` (because index management uses the search/query types).
- Adding `tools` enables `indices`, `ingest`, `ml`, `walkdir`, and `futures`.

You never need to explicitly list a feature's dependencies — Cargo handles that for you.

## Checking What's Compiled

To see which features are active in your build:

```bash
cargo metadata --format-version 1 | jq '.packages[] | select(.name == "opensearch-client") | .features'
```

Or use `cargo tree` to inspect the dependency graph:

```bash
cargo tree --features search -p opensearch-client
```
