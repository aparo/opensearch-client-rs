# CLI Tool

`opensearch-cli` is a single command-line binary for managing OpenSearch
clusters: dumping and restoring cluster metadata, listing indices, and dumping,
restoring, or copying index data (including to a remote cluster).

## Installation

Build from source (the CLI is a crate in this workspace):

```bash
git clone https://github.com/aparo/opensearch-client-rs.git
cd opensearch-client-rs
cargo build --release -p opensearch-cli
# binary at target/release/opensearch-cli
```

Or install it from the checkout:

```bash
cargo install --path opensearch-cli
```

## Global Options

These apply to every subcommand and can also be supplied via environment
variables (a `.env` file is loaded automatically):

| Flag | Env var | Default | Description |
|------|---------|---------|-------------|
| `-s`, `--server <URL>` | `OPENSEARCH_URL` | `http://localhost:9200` | Cluster URL |
| `-u`, `--user <USER>` | `OPENSEARCH_USER` | `admin` | Username |
| `-p`, `--password <PASS>` | `OPENSEARCH_PASSWORD` | `admin` | Password |
| `--remote-server <URL>` | `OPENSEARCH_REMOTE_URL` | `http://localhost:9200` | Remote cluster URL (for `copy-index`) |
| `--remote-user <USER>` | `OPENSEARCH_REMOTE_USER` | `admin` | Remote username |
| `--remote-password <PASS>` | `OPENSEARCH_REMOTE_PASSWORD` | `admin` | Remote password |
| `-v`, `--verbose` | | | Increase logging (`-v` info, `-vv` debug, `-vvv` trace) |

```bash
export OPENSEARCH_URL="http://localhost:9200"
export OPENSEARCH_USER="admin"
export OPENSEARCH_PASSWORD="admin"
```

## Available Commands

### list-indices

List the indices in the cluster.

```bash
opensearch-cli list-indices [--show-hidden] [--contains <PATTERN>]
```

- `--show-hidden` — include indices whose name starts with a dot (default: false)
- `--contains <PATTERN>` — only list indices whose name contains the pattern

```bash
# All non-hidden indices
opensearch-cli list-indices

# Only indices containing "blog"
opensearch-cli list-indices --contains blog
```

### dump-metadata

Export cluster metadata (ingest pipelines, index templates, index components)
to a directory.

```bash
opensearch-cli dump-metadata [OPTIONS] [FILE]
```

- `--ingest-pipelines` — dump ingest pipelines (default: true)
- `--index-templates` — dump index templates (default: true)
- `--index-components` — dump index components (default: true)
- `[FILE]` — output path (default: `output`)

```bash
opensearch-cli dump-metadata ./backup
```

### restore-metadata

Restore cluster metadata previously saved with `dump-metadata`.

```bash
opensearch-cli restore-metadata [OPTIONS] [FILE]
```

- `--ingest-pipelines` / `--index-templates` / `--index-components` (default: true)
- `[FILE]` — input path (default: `output`)

```bash
opensearch-cli restore-metadata ./backup
```

### fix-metadata

Fix (re-apply/repair) cluster metadata from a saved dump. Same options as
`restore-metadata`.

```bash
opensearch-cli fix-metadata [OPTIONS] [FILE]
```

### dump

Dump index data and mappings for one or more indices to a directory.

```bash
opensearch-cli dump [--read-bulk <SIZE>] <INDICES> [FILE]
```

- `--read-bulk <SIZE>` — scroll/read batch size (default: 500)
- `<INDICES>` — index name(s), comma-separated or wildcard patterns
- `[FILE]` — output path (default: `output`)

```bash
opensearch-cli dump "blog_posts,users" ./export
opensearch-cli dump "logs-*" ./export
```

### restore

Restore index data and mappings from a dump.

```bash
opensearch-cli restore [OPTIONS] [FILE]
```

- `--skip-data` — restore only mappings, not data (default: false)
- `--skip-mappings` — restore only data, not mappings (default: false)
- `--bulk-size <SIZE>` — bulk write size (default: 1000)
- `--mode=<MODE>` — `index` (overwrite) or `create` (default: `index`)
- `--index <FILES>` — restrict to specific index files
- `--rename-index <NAME>` — restore under a different index name
- `[FILE]` — input path (default: `output`)

```bash
opensearch-cli restore ./export
opensearch-cli restore --mode=create --bulk-size 2000 ./export
```

### copy-index

Copy an index, optionally to a remote cluster.

```bash
opensearch-cli copy-index [OPTIONS] <INDEX>
```

- `--remote` — copy to the remote cluster (configured with the `--remote-*`
  flags / `OPENSEARCH_REMOTE_*` env vars) (default: false)
- `--copy-mapping` — copy the source mapping to the target (default: false)
- `--delete-existing` — delete the target index before copying (default: false)
- `--size <N>` — scroll batch size (default: 1000)
- `--target-index <NAME>` — target index name (defaults to the source name)
- `<INDEX>` — source index name

```bash
# Local copy with a new name
opensearch-cli copy-index blog_posts --target-index blog_posts_copy --copy-mapping

# Copy to a remote cluster (global --remote-* flags go before the subcommand,
# or set OPENSEARCH_REMOTE_URL / OPENSEARCH_REMOTE_USER / OPENSEARCH_REMOTE_PASSWORD)
opensearch-cli --remote-server https://new-cluster:9200 \
  copy-index blog_posts --remote --target-index blog_posts
```

## Troubleshooting

Enable verbose logging to see what the CLI is doing:

```bash
opensearch-cli -vv list-indices
```

The CLI reuses the same connection settings as the library, so the environment
variables above (or a `.env` file) are the quickest way to point it at a cluster.
