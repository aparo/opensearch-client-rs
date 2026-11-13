# OpenSearch Client for Rust

[![Crates.io](https://img.shields.io/crates/v/opensearch-client)](https://crates.io/crates/opensearch-client)
[![License](https://img.shields.io/crates/l/opensearch-client)](https://crates.io/crates/opensearch-client)
[![Documentation](https://docs.rs/opensearch-client/badge.svg)](https://docs.rs/opensearch-client)

A comprehensive Rust client library for OpenSearch with a strongly typed DSL, CLI tools, and extensive API coverage.

## Features

- **Strongly Typed DSL** — type-safe query building with compile-time guarantees
- **Comprehensive API Coverage** — search, indices, cluster, ML, security, snapshot, and more
- **Document ORM** — `#[derive(OpenSearch)]` macro generates a full `Document` trait on your structs
- **CLI Tools** — `opensearch-cli` for cluster management and data migration
- **Async/Await** — built on tokio; every I/O call is non-blocking
- **Production Ready** — retry logic, connection pooling, and typed error handling
- **Modular** — feature flags let you compile only the API groups you use

## Project Structure

| Crate | Description |
|-------|-------------|
| [`opensearch-client`](opensearch-client/) | Core HTTP client and all OpenSearch API bindings |
| [`opensearch-dsl`](opensearch-dsl/) | Standalone, strongly typed query/aggregation DSL |
| [`opensearch-macro`](opensearch-macro/) | `#[derive(OpenSearch)]` proc-macro |
| [`opensearch-cli`](opensearch-cli/) | CLI tool for cluster management and data operations |
| [`opensearch-testcontainer`](opensearch-testcontainer/) | Docker-based test fixtures via `testcontainers` |

## Installation

```toml
[dependencies]
opensearch-client = "0.3"
opensearch-dsl    = "0.3"
```

Macro support is bundled with `opensearch-client` — no extra dependency needed.

## Quick Start

```rust
use opensearch_client::{ConfigurationBuilder, OsClient};
use opensearch_dsl::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `build()` returns an `OsClient` directly. `base_url` must precede auth.
    let client: OsClient = ConfigurationBuilder::new()
        .base_url("http://localhost:9200")
        .basic_auth("admin", "admin")
        .build();

    // Build a typed query
    let query = Search::new()
        .query(Query::bool()
            .must([Query::term("status", "published")])
            .filter([Query::range("date").gte("2024-01-01")]))
        .size(10);

    let response = client.search().index("my_index").search(query).call().await?;
    println!("Hits: {:?}", response.hits.total.map(|t| t.value));
    Ok(())
}
```

See [docs/getting-started.md](docs/getting-started.md) for the full walkthrough, including environment-variable setup and first-index examples.

## Document Modeling

The `#[derive(OpenSearch)]` macro turns a plain Rust struct into a first-class OpenSearch document:

```rust
use opensearch_client::{Document, OpenSearch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "articles")]
pub struct Article {
    #[os(id)]
    pub id: String,
    pub title: String,
    pub published: bool,
}
```

This generates `save()`, `get()`, `delete()`, `find()`, `find_all()`, and `count()` directly on the type:

```rust
article.save().await?;
let a = Article::get("42").await?;
Article::delete("42").await?;
let results = Article::find(Search::new().query(Query::term("published", true))).await?;
```

Full reference: [docs/documents/](docs/documents/)

## Documentation

| Topic | File |
|-------|------|
| Installation & quick start | [docs/getting-started.md](docs/getting-started.md) |
| Client configuration | [docs/configuration.md](docs/configuration.md) |
| Feature flags | [docs/features.md](docs/features.md) |
| Architecture overview | [docs/architecture.md](docs/architecture.md) |
| Document modeling (macro) | [docs/documents/README.md](docs/documents/README.md) |
| — CRUD operations | [docs/documents/crud-operations.md](docs/documents/crud-operations.md) |
| — Querying & search | [docs/documents/querying.md](docs/documents/querying.md) |
| — Macro attributes | [docs/documents/attributes.md](docs/documents/attributes.md) |
| — Field type mapping | [docs/documents/field-types.md](docs/documents/field-types.md) |
| — Nested documents | [docs/documents/nested-documents.md](docs/documents/nested-documents.md) |
| — Best practices | [docs/documents/best-practices.md](docs/documents/best-practices.md) |
| — API reference | [docs/documents/api-reference.md](docs/documents/api-reference.md) |
| Query DSL guide | [docs/queries/README.md](docs/queries/README.md) |
| Index management | [docs/indices.md](docs/indices.md) |
| CLI tools | [docs/cli.md](docs/cli.md) |
| Testing guide | [docs/testing.md](docs/testing.md) |
| Examples | [docs/examples/README.md](docs/examples/README.md) |
| — Basic operations | [docs/examples/basic-operations.md](docs/examples/basic-operations.md) |
| — Bulk operations | [docs/examples/bulk-operations.md](docs/examples/bulk-operations.md) |
| — Document modeling | [docs/examples/document-modeling.md](docs/examples/document-modeling.md) |
| — Search queries | [docs/examples/search-queries.md](docs/examples/search-queries.md) |
| — Real-world patterns | [docs/examples/real-world.md](docs/examples/real-world.md) |
| Agent skill (Claude & AI agents) | [.claude/skills/opensearch-client/SKILL.md](.claude/skills/opensearch-client/SKILL.md) |
| Contributing | [docs/contributing.md](docs/contributing.md) |
| Changelog | [CHANGELOG.md](CHANGELOG.md) |

API docs for the latest release are published at [docs.rs/opensearch-client](https://docs.rs/opensearch-client).

## Agent Skill (Claude & other AI agents)

This repository ships a ready-made **agent skill** that teaches Claude Code (and
other skill-aware agents) how to integrate these crates correctly — client setup,
the `#[derive(OpenSearch)]` document ORM, the query DSL, the streaming bulker, and
testing — with the verified, non-obvious details baked in.

The skill lives at [`.claude/skills/opensearch-client/SKILL.md`](.claude/skills/opensearch-client/SKILL.md).

### Using it inside this repository

Nothing to do. Claude Code auto-discovers skills under `.claude/skills/`, so the
skill is active whenever you run Claude Code from the repo root.

### Installing it into your own project

Copy the skill directory into your project's (or your user-level) skills folder:

```bash
# Per-project: available when running an agent from that project
mkdir -p /path/to/your-project/.claude/skills
cp -r .claude/skills/opensearch-client /path/to/your-project/.claude/skills/

# Or user-wide: available in every project for your user
mkdir -p ~/.claude/skills
cp -r .claude/skills/opensearch-client ~/.claude/skills/
```

Then verify it is picked up:

```bash
claude                      # start Claude Code in the target project
/skills                     # the list should include "opensearch-client"
```

The agent invokes the skill automatically when a task mentions OpenSearch in Rust
(e.g. "add an OpenSearch client", "model this struct as an OpenSearch document",
"build this search query") — or you can trigger it explicitly with
`/opensearch-client`.

### Other agents

Any agent runtime that reads Markdown skills can use the same file. Point your
runtime at `.claude/skills/opensearch-client/SKILL.md`, or paste its contents into
the agent's system/context prompt. The skill is self-contained and version-aligned
with the `docs/` tree, so it needs no extra files to work.

## CLI Tools

```bash
cargo install opensearch-cli

export OPENSEARCH_URL=http://localhost:9200
export OPENSEARCH_USER=admin
export OPENSEARCH_PASSWORD=admin

opensearch-cli list-indices
opensearch-cli dump-metadata   --output ./backup
opensearch-cli restore-metadata --input ./backup
```

See [docs/cli.md](docs/cli.md) for all subcommands and flags.

## Testing

```bash
# Unit tests
cargo test

# Integration tests also run under plain `cargo test`: they auto-start an
# OpenSearch testcontainer (Docker must be available). To reuse an already
# running cluster instead, set OPENSEARCH_URL before running the tests:
docker run -d --name opensearch-test \
  -p 9200:9200 \
  -e "discovery.type=single-node" \
  -e "DISABLE_SECURITY_PLUGIN=true" \
  opensearchproject/opensearch:latest

export OPENSEARCH_URL=http://localhost:9200
cargo test
```

See [docs/testing.md](docs/testing.md) for the full testing guide.

## Contributing

Contributions are welcome. Please read [docs/contributing.md](docs/contributing.md) and [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## License

Apache 2.0 — see [LICENSE](LICENSE).

## Related Projects

- [OpenSearch](https://opensearch.org/)
- [opensearch-rs](https://github.com/opensearch-project/opensearch-rs) — the official Rust client
- [elasticsearch-dsl-rs](https://github.com/vinted/elasticsearch-dsl-rs) — DSL inspiration
