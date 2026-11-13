# Configuration Guide

This guide covers all configuration options for the OpenSearch Client for Rust.

## Client Configuration

### Basic Configuration

```rust
use opensearch_client::{ConfigurationBuilder, OsClient};

// `build()` returns an `OsClient` directly — do not wrap it in `OsClient::new`.
// `base_url` must be called before the auth setters.
let client: OsClient = ConfigurationBuilder::new()
    .base_url("http://localhost:9200")
    .basic_auth("admin", "admin")
    .build();
```

### Advanced Configuration

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("http://localhost:9200")
    .basic_auth(username, password)
    .retries(3)                 // transient-error retries (exponential backoff)
    .max_bulk_size(1000)        // default batch size for client.bulker()
    .accept_invalid_certificates(false)
    .build();
```

## Authentication Methods

### Basic Authentication

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("http://localhost:9200")
    .basic_auth("username", "password")
    .build();
```

### Token (Bearer) Authentication

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("http://localhost:9200")
    .token_auth("your-bearer-token")   // sent as `Authorization: Bearer ...`
    .build();
```

### Legacy Authentication

For a pre-encoded `Authorization` header value:

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("http://localhost:9200")
    .legacy_auth("Basic dXNlcjpwYXNz")
    .build();
```

### User Agent

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("http://localhost:9200")
    .user_agent("my-app/1.0")
    .build();
```

## Connection Settings

### Retry Configuration

Transient errors are retried with exponential backoff:

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("http://localhost:9200")
    .retries(3)                                // Number of retries
    .build();
```

### Proxy Configuration

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("http://localhost:9200")
    .proxy_url("http://proxy.internal:8080")?  // returns Result
    .no_proxy_domain("internal.example.com")
    .build();
```

## TLS/SSL Configuration

### Verify Certificates (default)

By default certificates are verified — no setter is required:

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("https://localhost:9200")
    .build();
```

### Disable Certificate Verification (Development Only)

Use `accept_invalid_certificates(true)` for local/dev self-signed certs:

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("https://localhost:9200")
    .accept_invalid_certificates(true)         // ⚠️ Only for development!
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
export OPENSEARCH_USER="admin"
export OPENSEARCH_PASSWORD="password"
export OPENSEARCH_SSL_VERIFY="true"
export OPENSEARCH_MAX_BULK_SIZE="1000"
```

### Loading from Environment

The simplest option reads `OPENSEARCH_URL`, `OPENSEARCH_USER`,
`OPENSEARCH_PASSWORD`, `OPENSEARCH_SSL_VERIFY`, and `OPENSEARCH_MAX_BULK_SIZE`
(defaults: `https://localhost:9200`, `admin`/`admin`):

```rust
let client = OsClient::from_environment()?;
```

Or build it manually from your own variables:

```rust
use std::env;

let url = env::var("OPENSEARCH_URL")
    .unwrap_or_else(|_| "http://localhost:9200".to_string());
let username = env::var("OPENSEARCH_USER").ok();
let password = env::var("OPENSEARCH_PASSWORD").ok();

let mut builder = ConfigurationBuilder::new()
    .base_url(url);

if let (Some(user), Some(pass)) = (username, password) {
    builder = builder.basic_auth(user, pass);
}

let client = builder.build();
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
    .base_url(config.opensearch.url.clone());

if let (Some(user), Some(pass)) = (&config.opensearch.username, &config.opensearch.password) {
    builder = builder.basic_auth(user.clone(), pass.clone());
}

if let Some(retries) = config.opensearch.retry_attempts {
    builder = builder.retries(retries as u32);
}

let client = builder.build();
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
let response = client.search().index("my_index").search(query).call().await?;
```

### Log Levels

```rust
use tracing_subscriber::filter::LevelFilter;

tracing_subscriber::fmt()
    .with_max_level(LevelFilter::DEBUG)
    .init();
```

The client emits `tracing` spans/events for its HTTP operations, so raising the
log level above surfaces request/response detail — there are no `log_requests` /
`log_responses` builder setters.

## Production Configuration

### Recommended Production Settings

```rust
let client: OsClient = ConfigurationBuilder::new()
    .base_url("https://my-cluster.example.com:9200")
    .basic_auth(username, password)
    .retries(3)                                // Retry failed requests
    .max_bulk_size(1000)                       // Default bulk batch size
    .accept_invalid_certificates(false)        // Always verify in production
    .build();
```

### Health Monitoring

```rust
use std::time::Instant;
use opensearch_client::Error;

async fn health_check(client: &OsClient) -> Result<(), Error> {
    let start = Instant::now();
    let health = client.cluster().health().call().await?;
    let duration = start.elapsed();
    
    println!("Cluster health: {:?} ({}ms)", health.status, duration.as_millis());
    
    Ok(())
}
```

### Circuit Breaker Pattern

The client `Error` enum has no circuit-breaker variant, so the wrapper below
uses its own error type (`CbError`) to signal an open circuit:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use opensearch_client::Error;

#[derive(Debug)]
enum CbError {
    Open,
    Client(Error),
}

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
    
    async fn call<F, T>(&self, operation: F) -> Result<T, CbError>
    where
        F: std::future::Future<Output = Result<T, Error>>,
    {
        let current_failures = self.failure_count.load(Ordering::Relaxed);
        
        if current_failures >= self.threshold {
            return Err(CbError::Open);
        }
        
        match operation.await {
            Ok(result) => {
                self.failure_count.store(0, Ordering::Relaxed);
                Ok(result)
            }
            Err(e) => {
                self.failure_count.fetch_add(1, Ordering::Relaxed);
                Err(CbError::Client(e))
            }
        }
    }
}
```