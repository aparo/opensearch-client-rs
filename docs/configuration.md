# Configuration Guide

This guide covers all configuration options for the OpenSearch Client for Rust.

## Client Configuration

### Basic Configuration

```rust
use opensearch_client::{ConfigurationBuilder, OsClient};
use url::Url;
use std::time::Duration;

let url = Url::parse("http://localhost:9200")?;
let config = ConfigurationBuilder::new()
    .base_url(url)
    .basic_auth("admin".to_string(), "admin".to_string())
    .build();

let client = OsClient::new(config);
```

### Advanced Configuration

```rust
let config = ConfigurationBuilder::new()
    .base_url(url)
    .basic_auth(username, password)
    .timeout(Duration::from_secs(30))
    .retry_attempts(3)
    .connection_pool_size(10)
    .build();
```

## Authentication Methods

### Basic Authentication

```rust
let config = ConfigurationBuilder::new()
    .base_url(url)
    .basic_auth("username".to_string(), "password".to_string())
    .build();
```

### API Key Authentication

```rust
let config = ConfigurationBuilder::new()
    .base_url(url)
    .api_key("your-api-key".to_string())
    .build();
```

### Custom Headers

```rust
use std::collections::HashMap;

let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer token".to_string());
headers.insert("X-Custom-Header".to_string(), "value".to_string());

let config = ConfigurationBuilder::new()
    .base_url(url)
    .custom_headers(headers)
    .build();
```

## Connection Settings

### Timeout Configuration

```rust
let config = ConfigurationBuilder::new()
    .base_url(url)
    .timeout(Duration::from_secs(30))          // Request timeout
    .connect_timeout(Duration::from_secs(10))  // Connection timeout
    .build();
```

### Retry Configuration

```rust
let config = ConfigurationBuilder::new()
    .base_url(url)
    .retry_attempts(3)                         // Number of retries
    .retry_delay(Duration::from_millis(500))   // Delay between retries
    .build();
```

### Connection Pool

```rust
let config = ConfigurationBuilder::new()
    .base_url(url)
    .connection_pool_size(20)                  // Max connections
    .connection_pool_idle_timeout(Duration::from_secs(30))
    .build();
```

## TLS/SSL Configuration

### Basic TLS

```rust
let config = ConfigurationBuilder::new()
    .base_url(Url::parse("https://localhost:9200")?)
    .tls_verify(true)                          // Verify certificates
    .build();
```

### Custom Certificate

```rust
let config = ConfigurationBuilder::new()
    .base_url(url)
    .tls_cert_path("/path/to/cert.pem")
    .tls_key_path("/path/to/key.pem")
    .tls_ca_cert_path("/path/to/ca.pem")
    .build();
```

### Disable Certificate Verification (Development Only)

```rust
let config = ConfigurationBuilder::new()
    .base_url(url)
    .tls_verify(false)                         // ⚠️ Only for development!
    .build();
```

## Feature Flags

### Available Features

Enable only the features you need to reduce binary size:

```toml
[dependencies]
opensearch-client = { version = "0.3", features = [
    "search",      # Search APIs (default)
    "indices",     # Index management (default)
    "cluster",     # Cluster APIs (default)
    "ml",          # Machine learning APIs (default)
    "security",    # Security APIs
    "tools"        # Utility tools
] }
```

### Feature Descriptions

| Feature | Description | Default |
|---------|-------------|---------|
| `search` | Search and query APIs | ✅ |
| `indices` | Index management operations | ✅ |
| `cluster` | Cluster health and management | ✅ |
| `ml` | Machine learning features | ✅ |
| `security` | Security and authentication APIs | ❌ |
| `tools` | Additional utility tools | ❌ |

### Minimal Configuration

For applications that only need basic search:

```toml
[dependencies]
opensearch-client = { version = "0.3", features = ["search"], default-features = false }
```

### Full Feature Set

For comprehensive OpenSearch integration:

```toml
[dependencies]
opensearch-client = { version = "0.3", features = [
    "search",
    "indices",
    "cluster",
    "ml",
    "security",
    "tools"
] }
```

## Environment Configuration

### Environment Variables

Set configuration through environment variables:

```bash
export OPENSEARCH_URL="https://my-cluster.example.com:9200"
export OPENSEARCH_USERNAME="admin"
export OPENSEARCH_PASSWORD="password"
export OPENSEARCH_TIMEOUT="30"
export OPENSEARCH_RETRY_ATTEMPTS="3"
```

### Loading from Environment

```rust
use std::env;

let url = env::var("OPENSEARCH_URL")
    .unwrap_or_else(|_| "http://localhost:9200".to_string());
let username = env::var("OPENSEARCH_USERNAME").ok();
let password = env::var("OPENSEARCH_PASSWORD").ok();

let mut builder = ConfigurationBuilder::new()
    .base_url(Url::parse(&url)?);

if let (Some(user), Some(pass)) = (username, password) {
    builder = builder.basic_auth(user, pass);
}

let config = builder.build();
```

## Configuration Files

### TOML Configuration

Create a `opensearch.toml` file:

```toml
[opensearch]
url = "https://my-cluster.example.com:9200"
username = "admin"
password = "password"
timeout_seconds = 30
retry_attempts = 3

[tls]
verify = true
cert_path = "/path/to/cert.pem"
key_path = "/path/to/key.pem"
ca_cert_path = "/path/to/ca.pem"
```

Load configuration from file:

```rust
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct OpenSearchConfig {
    opensearch: ClientConfig,
    tls: Option<TlsConfig>,
}

#[derive(Deserialize)]
struct ClientConfig {
    url: String,
    username: Option<String>,
    password: Option<String>,
    timeout_seconds: Option<u64>,
    retry_attempts: Option<usize>,
}

#[derive(Deserialize)]
struct TlsConfig {
    verify: Option<bool>,
    cert_path: Option<String>,
    key_path: Option<String>,
    ca_cert_path: Option<String>,
}

let config_str = fs::read_to_string("opensearch.toml")?;
let config: OpenSearchConfig = toml::from_str(&config_str)?;

let mut builder = ConfigurationBuilder::new()
    .base_url(Url::parse(&config.opensearch.url)?);

if let (Some(user), Some(pass)) = (&config.opensearch.username, &config.opensearch.password) {
    builder = builder.basic_auth(user.clone(), pass.clone());
}

if let Some(timeout) = config.opensearch.timeout_seconds {
    builder = builder.timeout(Duration::from_secs(timeout));
}

let client_config = builder.build();
```

## Logging Configuration

### Enable Logging

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
use tracing_subscriber;

// Initialize logging
tracing_subscriber::fmt::init();

// Client operations will now be logged
let response = client.search(&query).await?;
```

### Log Levels

```rust
use tracing_subscriber::filter::LevelFilter;

tracing_subscriber::fmt()
    .with_max_level(LevelFilter::DEBUG)
    .init();
```

### Request/Response Logging

Enable detailed HTTP logging:

```rust
let config = ConfigurationBuilder::new()
    .base_url(url)
    .log_requests(true)
    .log_responses(true)
    .build();
```

## Production Configuration

### Recommended Production Settings

```rust
let config = ConfigurationBuilder::new()
    .base_url(production_url)
    .basic_auth(username, password)
    .timeout(Duration::from_secs(30))          // Reasonable timeout
    .connect_timeout(Duration::from_secs(10))  // Connection timeout
    .retry_attempts(3)                         // Retry failed requests
    .retry_delay(Duration::from_millis(1000))  // Backoff between retries
    .connection_pool_size(20)                  // Connection pooling
    .tls_verify(true)                          // Always verify in production
    .build();
```

### Health Monitoring

```rust
use std::time::Instant;

async fn health_check(client: &OsClient) -> Result<(), Error> {
    let start = Instant::now();
    let health = client.cluster().health().await?;
    let duration = start.elapsed();
    
    println!("Cluster health: {:?} ({}ms)", health.status, duration.as_millis());
    
    if health.status == "red" {
        return Err(Error::ClusterHealthError("Cluster is unhealthy".to_string()));
    }
    
    Ok(())
}
```

### Circuit Breaker Pattern

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct CircuitBreaker {
    failure_count: Arc<AtomicUsize>,
    threshold: usize,
}

impl CircuitBreaker {
    fn new(threshold: usize) -> Self {
        Self {
            failure_count: Arc::new(AtomicUsize::new(0)),
            threshold,
        }
    }
    
    async fn call<F, T>(&self, operation: F) -> Result<T, Error>
    where
        F: std::future::Future<Output = Result<T, Error>>,
    {
        let current_failures = self.failure_count.load(Ordering::Relaxed);
        
        if current_failures >= self.threshold {
            return Err(Error::CircuitBreakerOpen);
        }
        
        match operation.await {
            Ok(result) => {
                self.failure_count.store(0, Ordering::Relaxed);
                Ok(result)
            }
            Err(e) => {
                self.failure_count.fetch_add(1, Ordering::Relaxed);
                Err(e)
            }
        }
    }
}
```