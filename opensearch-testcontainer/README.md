# OpenSearch Testcontainer

A Rust testcontainer implementation for OpenSearch, enabling easy integration testing with OpenSearch instances in Docker containers.

## Overview

This crate provides a `testcontainers` implementation for OpenSearch, allowing you to spin up OpenSearch instances programmatically for integration tests. It's built on top of the [testcontainers](https://github.com/testcontainers/testcontainers-rs) library.

## Features

- 🐳 **Docker Integration** - Seamlessly runs OpenSearch in Docker containers
- ⚡ **Fast Setup** - Quick container startup for testing
- 🔧 **Configurable** - Customizable cluster settings and environment variables  
- 🔒 **Security Ready** - Built-in authentication support
- 🎯 **Ready Conditions** - Waits for OpenSearch to be fully initialized
- 📦 **Multiple Versions** - Support for different OpenSearch versions

## Installation

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
opensearch-testcontainer = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Basic Usage

```rust
use opensearch_testcontainer::OpenSearch;
use testcontainers::runners::AsyncRunner;

#[tokio::test]
async fn test_opensearch_integration() {
    // Start OpenSearch container with default settings
    let opensearch = OpenSearch::default().start().await.unwrap();
    
    // Get the host port
    let host_port = opensearch.get_host_port_ipv4(9200).await.unwrap();
    let base_url = format!("https://127.0.0.1:{}", host_port);
    
    // Your test code here
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let response = client
        .get(&base_url)
        .basic_auth("admin", Some("?qbr9:6Y7nk6"))
        .send()
        .await
        .unwrap();
    
    assert!(response.status().is_success());
}
```

### Custom Configuration

```rust
use opensearch_testcontainer::OpenSearch;
use testcontainers::runners::AsyncRunner;

#[tokio::test]
async fn test_custom_opensearch() {
    let opensearch = OpenSearch::default()
        .with_tag("2.11.0")  // Use specific version
        .with_cluster_name("test-cluster")
        .with_env_var("OPENSEARCH_JAVA_OPTS", "-Xms1g -Xmx1g")
        .start()
        .await
        .unwrap();
    
    // Container is ready for use
    let host_port = opensearch.get_host_port_ipv4(9200).await.unwrap();
    // ... your test code
}
```

## Configuration Options

### Builder Methods

The `OpenSearch` struct provides several builder methods for customization:

| Method | Description | Example |
|--------|-------------|---------|
| `with_name(name)` | Set custom Docker image name | `.with_name("my-registry/opensearch")` |
| `with_tag(tag)` | Set OpenSearch version | `.with_tag("2.11.0")` |
| `with_cluster_name(name)` | Set cluster name | `.with_cluster_name("test-cluster")` |
| `with_env_var(key, value)` | Add environment variable | `.with_env_var("OPENSEARCH_JAVA_OPTS", "-Xms512m")` |

### Default Configuration

The default OpenSearch container comes preconfigured with:

- **Image**: `opensearchproject/opensearch:3.1.0`
- **Discovery**: Single-node cluster
- **Authentication**: Username `admin`, password `?qbr9:6Y7nk6`
- **Ports**: 9200 (HTTP), 9300 (Transport), 9600 (Performance Analyzer)
- **Ready Conditions**: Waits for cluster state to be GREEN and ML configuration

### Authentication

```rust
let opensearch = OpenSearch::default();

// Get credentials
let username = opensearch.username(); // "admin"
let password = opensearch.password(); // "?qbr9:6Y7nk6"

// Use in HTTP client
let client = reqwest::Client::builder()
    .danger_accept_invalid_certs(true)
    .build()
    .unwrap();

let response = client
    .get("https://127.0.0.1:9200")
    .basic_auth(username, Some(password))
    .send()
    .await
    .unwrap();
```

## Common Patterns

### Testing with opensearch-client

```rust
use opensearch_client::{ClientBuilder, OpenSearch as OSClient};
use opensearch_testcontainer::OpenSearch;
use testcontainers::runners::AsyncRunner;

#[tokio::test]
async fn test_with_opensearch_client() {
    // Start container
    let container = OpenSearch::default().start().await.unwrap();
    let host_port = container.get_host_port_ipv4(9200).await.unwrap();
    
    // Create client
    let client = ClientBuilder::new()
        .host(&format!("https://127.0.0.1:{}", host_port))
        .basic_auth("admin", "?qbr9:6Y7nk6")
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    // Test operations
    let info = client.info().await.unwrap();
    assert_eq!(info.version.number, "3.1.0");
}
```

### Multiple Test Containers

```rust
use opensearch_testcontainer::OpenSearch;
use testcontainers::runners::AsyncRunner;

#[tokio::test]
async fn test_multiple_clusters() {
    // Start multiple containers for testing cluster interactions
    let cluster1 = OpenSearch::default()
        .with_cluster_name("cluster1")
        .with_env_var("node.name", "node1")
        .start()
        .await
        .unwrap();
    
    let cluster2 = OpenSearch::default()
        .with_cluster_name("cluster2")
        .with_env_var("node.name", "node2")
        .start()
        .await
        .unwrap();
    
    // Test cross-cluster scenarios
    // ...
}
```

### Resource Constraints

```rust
use opensearch_testcontainer::OpenSearch;
use testcontainers::runners::AsyncRunner;

#[tokio::test]
async fn test_resource_limited() {
    let opensearch = OpenSearch::default()
        .with_env_var("OPENSEARCH_JAVA_OPTS", "-Xms256m -Xmx256m")
        .with_env_var("bootstrap.memory_lock", "false")
        .start()
        .await
        .unwrap();
    
    // Test with limited resources
    // ...
}
```

## Advanced Usage

### Custom Wait Conditions

The container waits for specific log messages to ensure OpenSearch is ready:

- `[YELLOW] to [GREEN]` - Cluster state becomes healthy
- `ML configuration initialized successfully` - Machine learning is ready

### Environment Variables

Common OpenSearch environment variables you might want to set:

```rust
let opensearch = OpenSearch::default()
    .with_env_var("cluster.name", "test-cluster")
    .with_env_var("node.name", "test-node")
    .with_env_var("discovery.type", "single-node")
    .with_env_var("bootstrap.memory_lock", "true")
    .with_env_var("OPENSEARCH_JAVA_OPTS", "-Xms512m -Xmx512m")
    .with_env_var("DISABLE_SECURITY_PLUGIN", "true")  // Disable security
    .start()
    .await
    .unwrap();
```

### Version Testing

Test against multiple OpenSearch versions:

```rust
#[tokio::test]
async fn test_version_compatibility() {
    let versions = ["2.11.0", "2.12.0", "3.0.0", "3.1.0"];
    
    for version in versions {
        let container = OpenSearch::default()
            .with_tag(version)
            .start()
            .await
            .unwrap();
        
        // Test your code against this version
        let port = container.get_host_port_ipv4(9200).await.unwrap();
        // ... version-specific tests
    }
}
```

## Troubleshooting

### Common Issues

**Container fails to start**
```rust
// Check Docker is running and you have the opensearch image
// Pull manually if needed: docker pull opensearchproject/opensearch:3.1.0
```

**Connection refused errors**
```rust
// Ensure you're connecting to the correct port
let host_port = container.get_host_port_ipv4(9200).await.unwrap();
let url = format!("https://127.0.0.1:{}", host_port);  // Note: HTTPS, not HTTP
```

**SSL/TLS errors**
```rust
// OpenSearch uses self-signed certificates by default
let client = reqwest::Client::builder()
    .danger_accept_invalid_certs(true)  // Required for testing
    .build()
    .unwrap();
```

**Authentication errors**
```rust
// Use the correct default credentials
.basic_auth("admin", Some("?qbr9:6Y7nk6"))
```

### Debug Logging

Enable logging to see container startup:

```rust
// In your test
env_logger::init();

// Or set environment variable
// RUST_LOG=debug cargo test
```

## Performance Tips

1. **Reuse containers** - For multiple tests, consider using the same container
2. **Resource limits** - Set appropriate JVM heap sizes for faster startup
3. **Disable unnecessary features** - Turn off security or ML if not needed
4. **Parallel tests** - Use `#[tokio::test(flavor = "multi_thread")]` for parallel execution

## Examples

See the `tests` directory for more comprehensive examples:

- Basic connectivity testing
- Index creation and document operations  
- Search and aggregation testing
- Bulk operations testing
- Authentication scenarios

## Requirements

- Docker must be installed and running
- Rust 1.70+ (async/await support)
- Internet connection (to pull OpenSearch Docker images)

## Related Crates

- [`testcontainers`](https://crates.io/crates/testcontainers) - Core testcontainers functionality
- [`opensearch-client`](https://crates.io/crates/opensearch-client) - OpenSearch Rust client
- [`tokio`](https://crates.io/crates/tokio) - Async runtime

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](../LICENSE) file for details.

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../docs/contributing.md) for details on how to contribute to this project.