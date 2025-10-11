# CLI Tools

The OpenSearch client includes command-line tools for data management and operations.

## Installation

Install the CLI tools using Cargo:

```bash
cargo install opensearch-cli
```

Or build from source:

```bash
git clone https://github.com/aparo/opensearch-client-rs.git
cd opensearch-client-rs/opensearch-cli
cargo build --release
```

## Available Commands

### opensearch-dump

Tool for exporting data from OpenSearch indices.

#### Usage

```bash
opensearch-dump [OPTIONS] --index <INDEX>
```

#### Options

- `--host <HOST>` - OpenSearch host (default: http://localhost:9200)
- `--index <INDEX>` - Index name to dump
- `--output <FILE>` - Output file (default: stdout)
- `--format <FORMAT>` - Output format: json, jsonl, csv (default: jsonl)
- `--query <QUERY>` - Query to filter documents
- `--size <SIZE>` - Batch size for scrolling (default: 1000)
- `--scroll <TIME>` - Scroll timeout (default: 5m)
- `--mapping` - Include index mapping in output
- `--pretty` - Pretty-print JSON output

#### Examples

```bash
# Dump entire index
opensearch-dump --index blog_posts --output posts.jsonl

# Dump with query filter
opensearch-dump --index blog_posts \
  --query '{"term": {"published": true}}' \
  --output published_posts.jsonl

# Dump to CSV format
opensearch-dump --index products \
  --format csv \
  --output products.csv

# Include mapping information
opensearch-dump --index blog_posts \
  --mapping \
  --output posts_with_mapping.json \
  --format json \
  --pretty
```

### opensearch-restore

Tool for importing data into OpenSearch indices.

#### Usage

```bash
opensearch-restore [OPTIONS] --index <INDEX> --input <FILE>
```

#### Options

- `--host <HOST>` - OpenSearch host (default: http://localhost:9200)
- `--index <INDEX>` - Target index name
- `--input <FILE>` - Input file to restore from
- `--format <FORMAT>` - Input format: json, jsonl, csv (default: jsonl)
- `--bulk-size <SIZE>` - Bulk operation size (default: 1000)
- `--create-index` - Create index if it doesn't exist
- `--mapping <FILE>` - Index mapping file (JSON)
- `--settings <FILE>` - Index settings file (JSON)
- `--transform <SCRIPT>` - Transformation script for documents
- `--dry-run` - Show what would be imported without actually doing it

#### Examples

```bash
# Basic restore
opensearch-restore --index blog_posts --input posts.jsonl

# Restore with index creation
opensearch-restore --index new_blog_posts \
  --input posts.jsonl \
  --create-index \
  --mapping mapping.json \
  --settings settings.json

# Restore CSV data
opensearch-restore --index products \
  --input products.csv \
  --format csv

# Dry run to validate data
opensearch-restore --index blog_posts \
  --input posts.jsonl \
  --dry-run
```

### opensearch-migrate

Tool for migrating data between indices or clusters.

#### Usage

```bash
opensearch-migrate [OPTIONS] --source-index <INDEX> --target-index <INDEX>
```

#### Options

- `--source-host <HOST>` - Source OpenSearch host
- `--target-host <HOST>` - Target OpenSearch host (default: same as source)
- `--source-index <INDEX>` - Source index name
- `--target-index <INDEX>` - Target index name
- `--query <QUERY>` - Query to filter documents
- `--transform <SCRIPT>` - Lua script for document transformation
- `--bulk-size <SIZE>` - Bulk operation size
- `--parallel <NUM>` - Number of parallel workers
- `--reindex` - Use reindex API when possible

#### Examples

```bash
# Simple migration
opensearch-migrate --source-index old_posts --target-index new_posts

# Cross-cluster migration
opensearch-migrate \
  --source-host http://old-cluster:9200 \
  --target-host http://new-cluster:9200 \
  --source-index blog_posts \
  --target-index blog_posts

# Migration with transformation
opensearch-migrate \
  --source-index products \
  --target-index products_v2 \
  --transform transform.lua

# Filtered migration
opensearch-migrate \
  --source-index blog_posts \
  --target-index published_posts \
  --query '{"term": {"published": true}}'
```

## Configuration

### Configuration File

Create a configuration file at `~/.opensearch-cli/config.toml`:

```toml
[default]
host = "http://localhost:9200"
username = "admin"
password = "admin"
verify_ssl = false

[production]
host = "https://search.example.com:9200"
username = "user"
password = "password"
verify_ssl = true

[development]
host = "http://dev-search:9200"
```

Use with `--config production` or set `OPENSEARCH_CONFIG=production`.

### Environment Variables

- `OPENSEARCH_HOST` - Default host
- `OPENSEARCH_USERNAME` - Authentication username  
- `OPENSEARCH_PASSWORD` - Authentication password
- `OPENSEARCH_CONFIG` - Configuration profile to use

## Advanced Features

### Document Transformation

Create Lua scripts for transforming documents during migration:

```lua
-- transform.lua
function transform(doc)
    -- Convert old field names
    if doc.old_field then
        doc.new_field = doc.old_field
        doc.old_field = nil
    end
    
    -- Add computed fields
    doc.created_year = string.sub(doc.created_at, 1, 4)
    
    -- Normalize tags
    if doc.tags then
        for i, tag in ipairs(doc.tags) do
            doc.tags[i] = string.lower(tag)
        end
    end
    
    return doc
end
```

### Custom Mapping

Define index mappings for data restoration:

```json
{
  "mappings": {
    "properties": {
      "title": {
        "type": "text",
        "analyzer": "standard",
        "fields": {
          "keyword": {
            "type": "keyword"
          }
        }
      },
      "content": {
        "type": "text",
        "analyzer": "standard"
      },
      "created_at": {
        "type": "date",
        "format": "strict_date_optional_time"
      },
      "tags": {
        "type": "keyword"
      }
    }
  }
}
```

### Index Settings

Configure index settings:

```json
{
  "settings": {
    "number_of_shards": 1,
    "number_of_replicas": 0,
    "analysis": {
      "analyzer": {
        "custom_analyzer": {
          "type": "custom",
          "tokenizer": "standard",
          "filter": ["lowercase", "stop"]
        }
      }
    }
  }
}
```

## Performance Tuning

### Bulk Operations

- Increase `--bulk-size` for faster imports (default: 1000)
- Monitor memory usage with larger batch sizes
- Use `--parallel` for multiple workers

### Memory Usage

- Use streaming mode for large datasets
- Adjust scroll timeout for large indices
- Monitor OpenSearch cluster resources

### Network Optimization

- Use compression for remote clusters
- Adjust timeouts for slow networks
- Consider running tools close to the cluster

## Troubleshooting

### Common Issues

**Connection Errors**
```bash
# Test connectivity
curl -X GET "http://localhost:9200"

# Check authentication
opensearch-dump --index _cluster/health
```

**Memory Issues**
```bash
# Reduce batch size
opensearch-dump --index large_index --size 100

# Use streaming mode
opensearch-migrate --source-index large_index --bulk-size 500
```

**Timeout Errors**
```bash
# Increase scroll timeout
opensearch-dump --index slow_index --scroll 10m

# Reduce batch size for slower operations
opensearch-restore --index target --bulk-size 100
```

### Debug Mode

Enable debug output:

```bash
RUST_LOG=debug opensearch-dump --index blog_posts
```

## Examples and Use Cases

### Daily Backup

```bash
#!/bin/bash
# daily-backup.sh

DATE=$(date +%Y%m%d)
BACKUP_DIR="/backups/opensearch"

mkdir -p "$BACKUP_DIR/$DATE"

# Backup all indices
for index in blog_posts users products; do
    opensearch-dump \
        --index "$index" \
        --output "$BACKUP_DIR/$DATE/$index.jsonl" \
        --mapping > "$BACKUP_DIR/$DATE/$index-mapping.json"
done
```

### Data Migration

```bash
#!/bin/bash
# migrate-to-new-cluster.sh

OLD_HOST="http://old-cluster:9200"
NEW_HOST="http://new-cluster:9200"

for index in blog_posts users products; do
    echo "Migrating $index..."
    
    opensearch-migrate \
        --source-host "$OLD_HOST" \
        --target-host "$NEW_HOST" \
        --source-index "$index" \
        --target-index "$index" \
        --bulk-size 1000 \
        --parallel 4
done
```

### Data Transformation

```bash
# Transform and clean data
opensearch-migrate \
    --source-index raw_data \
    --target-index clean_data \
    --transform clean_transform.lua \
    --query '{"range": {"timestamp": {"gte": "2024-01-01"}}}'
```

The CLI tools provide powerful capabilities for managing OpenSearch data with performance optimization and flexible transformation options.