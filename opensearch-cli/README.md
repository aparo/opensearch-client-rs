# OpenSearch CLI Tool

A powerful command-line interface for managing OpenSearch clusters, indices, and data operations.

## Installation

### From Crates.io

```bash
cargo install opensearch-cli
```

### From Source

```bash
git clone https://github.com/aparo/opensearch-client-rs.git
cd opensearch-client-rs
cargo install --path opensearch-cli
```

## Configuration

### Environment Variables

Set these environment variables to avoid passing credentials with each command:

```bash
export OPENSEARCH_URL="http://localhost:9200"
export OPENSEARCH_USER="admin"
export OPENSEARCH_PASSWORD="admin"

# For remote cluster operations
export OPENSEARCH_REMOTE_URL="http://remote-cluster:9200"
export OPENSEARCH_REMOTE_USER="admin"
export OPENSEARCH_REMOTE_PASSWORD="admin"
```

### Command Line Options

All commands support these global options:

- `--server, -s`: OpenSearch server URL (default: http://localhost:9200)
- `--user, -u`: Username for authentication (default: admin)
- `--password, -p`: Password for authentication (default: admin)
- `--verbose, -v`: Increase verbosity (can be used multiple times: -v, -vv, -vvv)

For remote operations:
- `--remote-server`: Remote OpenSearch server URL
- `--remote-user`: Remote username
- `--remote-password`: Remote password

## Commands

### 📊 Cluster Metadata Management

#### Dump Metadata

Export cluster metadata for backup or migration:

```bash
# Dump all metadata
opensearch-cli dump-metadata --output ./backup

# Dump specific components
opensearch-cli dump-metadata \
    --output ./backup \
    --ingest-pipelines true \
    --index-templates true \
    --index-components true
```

**Options:**
- `--output`: Output directory (default: "output")
- `--ingest-pipelines`: Include ingest pipelines (default: true)
- `--index-templates`: Include index templates (default: true)
- `--index-components`: Include index components (default: true)

#### Restore Metadata

Restore cluster metadata from backup:

```bash
# Restore all metadata
opensearch-cli restore-metadata --input ./backup

# Restore specific components
opensearch-cli restore-metadata \
    --input ./backup \
    --ingest-pipelines true \
    --index-templates false \
    --index-components true
```

**Options:**
- `--input`: Input directory containing metadata (default: "output")
- `--ingest-pipelines`: Restore ingest pipelines (default: true)
- `--index-templates`: Restore index templates (default: true)
- `--index-components`: Restore index components (default: true)

#### Fix Metadata

Fix and validate cluster metadata:

```bash
# Fix all metadata issues
opensearch-cli fix-metadata --input ./backup

# Fix specific components
opensearch-cli fix-metadata \
    --input ./backup \
    --ingest-pipelines true \
    --index-templates true
```

### 📋 Index Management

#### List Indices

Display all indices in the cluster:

```bash
# List all indices
opensearch-cli list-indices

# Show hidden indices (starting with .)
opensearch-cli list-indices --show-hidden true

# Filter indices by name pattern
opensearch-cli list-indices --contains "log"
```

**Options:**
- `--show-hidden`: Show hidden indices (default: false)
- `--contains`: Filter indices containing this string

**Examples:**
```bash
# List all visible indices
opensearch-cli list-indices

# Find all log indices
opensearch-cli list-indices --contains "log"

# Show system indices
opensearch-cli list-indices --show-hidden true
```

### 💾 Data Operations

#### Dump Index Data

Export index data and mappings:

```bash
# Dump single index
opensearch-cli dump my_index --output ./data

# Dump multiple indices (comma-separated)
opensearch-cli dump "index1,index2,logs-*" --output ./data

# Customize bulk read size
opensearch-cli dump my_index \
    --output ./data \
    --read-bulk 1000
```

**Options:**
- `--output`: Output directory (default: "output")
- `--read-bulk`: Bulk read size for data export (default: 500)
- `indices`: Index names or patterns (required)

**Output Structure:**
```
output/
├── my_index/
│   ├── mapping.json
│   └── data.jsonl
└── logs-2023-10/
    ├── mapping.json
    └── data.jsonl
```

#### Restore Index Data

Import index data and mappings:

```bash
# Restore index with data and mappings
opensearch-cli restore --input ./data

# Skip data, restore only mappings
opensearch-cli restore \
    --input ./data \
    --skip-data true

# Restore specific index with rename
opensearch-cli restore \
    --input ./data \
    --index "my_index" \
    --rename-index "new_index_name"

# Customize bulk size for faster imports
opensearch-cli restore \
    --input ./data \
    --bulk-size 2000
```

**Options:**
- `--input`: Input directory containing data (default: "output")
- `--skip-data`: Skip data restoration (default: false)
- `--skip-mappings`: Skip mapping restoration (default: false)
- `--bulk-size`: Bulk import size (default: 1000)
- `--mode`: Restoration mode (default: "index")
- `--index`: Specific index to restore
- `--rename-index`: New name for the restored index

**Restore Modes:**
- `index`: Restore complete index
- `data`: Restore only data
- `mapping`: Restore only mappings

### 🔄 Index Copy Operations

#### Copy Index

Copy an index within the same cluster or between clusters:

```bash
# Copy index locally (same cluster)
opensearch-cli copy-index my_index \
    --target-index new_index

# Copy to remote cluster
opensearch-cli copy-index my_index \
    --remote true \
    --target-index remote_index \
    --copy-mapping true \
    --size 1000

# Copy with mapping and delete existing target
opensearch-cli copy-index source_index \
    --remote true \
    --target-index target_index \
    --copy-mapping true \
    --delete-existing true \
    --size 500
```

**Options:**
- `--remote`: Copy to remote cluster (default: false)
- `--copy-mapping`: Copy index mapping (default: false)
- `--delete-existing`: Delete target index if exists (default: false)
- `--size`: Bulk copy size (default: 1000)
- `--target-index`: Name of target index
- `index`: Source index name (required)

## Usage Examples

### Complete Cluster Migration

```bash
# 1. Set up environment
export OPENSEARCH_URL="http://source-cluster:9200"
export OPENSEARCH_REMOTE_URL="http://target-cluster:9200"

# 2. Dump source cluster metadata
opensearch-cli dump-metadata --output ./migration

# 3. Dump all indices data
opensearch-cli list-indices | while read index; do
    opensearch-cli dump "$index" --output "./migration/data"
done

# 4. Restore metadata to target cluster
opensearch-cli restore-metadata --input ./migration

# 5. Restore all indices to target cluster
opensearch-cli restore --input ./migration/data
```

### Selective Data Migration

```bash
# Copy only log indices to remote cluster
opensearch-cli list-indices --contains "log" | while read index; do
    opensearch-cli copy-index "$index" \
        --remote true \
        --target-index "$index" \
        --copy-mapping true \
        --size 2000
done
```

### Development Environment Setup

```bash
# Create development indices from production backup
opensearch-cli restore --input ./prod-backup \
    --index "users" \
    --rename-index "users-dev"

opensearch-cli restore --input ./prod-backup \
    --index "products" \
    --rename-index "products-dev"
```

### Index Maintenance

```bash
# Backup critical indices
for index in users products orders; do
    opensearch-cli dump "$index" --output "./backup/$(date +%Y%m%d)"
done

# Restore from specific date
opensearch-cli restore --input "./backup/20231008" \
    --index "users" \
    --rename-index "users-restored"
```

## Logging and Debugging

Control output verbosity with the `--verbose` flag:

```bash
# Basic output
opensearch-cli list-indices

# Info level logging
opensearch-cli --verbose list-indices

# Debug level logging
opensearch-cli -vv list-indices

# Trace level logging (very verbose)
opensearch-cli -vvv list-indices
```

## Error Handling

The CLI provides detailed error messages and proper exit codes:

- `0`: Success
- `1`: General error
- `2`: Connection error
- `3`: Authentication error
- `4`: Not found error

## Configuration File Support

Create a `.env` file in your working directory:

```bash
# .env file
OPENSEARCH_URL=https://my-cluster.example.com:9200
OPENSEARCH_USER=my-username
OPENSEARCH_PASSWORD=my-password
OPENSEARCH_REMOTE_URL=https://backup-cluster.example.com:9200
OPENSEARCH_REMOTE_USER=backup-user
OPENSEARCH_REMOTE_PASSWORD=backup-password
```

## Performance Tips

1. **Bulk Size Tuning**: Adjust `--bulk-size` and `--read-bulk` based on your cluster capacity
2. **Parallel Operations**: Use shell scripting for parallel index operations
3. **Network Optimization**: Use compression and keep connections alive
4. **Memory Management**: Monitor memory usage for large data operations

## Security Considerations

1. **Credentials**: Use environment variables instead of command-line arguments
2. **Network**: Use HTTPS in production environments
3. **Access Control**: Ensure proper permissions for backup/restore operations
4. **Audit**: Enable logging for compliance requirements

## Troubleshooting

### Common Issues

1. **Connection Refused**:
   ```bash
   # Check if OpenSearch is running
   curl -k http://localhost:9200/_cluster/health
   ```

2. **Authentication Failed**:
   ```bash
   # Verify credentials
   opensearch-cli --server http://localhost:9200 --user admin --password admin list-indices
   ```

3. **Permission Denied**:
   ```bash
   # Check user permissions
   opensearch-cli --verbose list-indices
   ```

### Debug Mode

Enable debug logging to troubleshoot issues:

```bash
opensearch-cli -vvv dump my_index --output ./debug
```

This will show:
- HTTP requests and responses
- Authentication details
- Detailed error messages
- Performance metrics