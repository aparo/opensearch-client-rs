# Contributing to OpenSearch Client for Rust

Thank you for your interest in contributing to the OpenSearch Client for Rust! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- **Rust**: Install the latest stable version from [rustup.rs](https://rustup.rs/)
- **OpenSearch**: For integration testing (see [Testing](#testing) section)
- **Git**: For version control

### Setting Up the Development Environment

1. **Clone the repository**:
   ```bash
   git clone https://github.com/aparo/opensearch-client-rs.git
   cd opensearch-client-rs
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Check formatting**:
   ```bash
   cargo fmt --check
   ```

5. **Run linter**:
   ```bash
   cargo clippy -- -D warnings
   ```

## 📁 Project Structure

```
opensearch-client-rs/
├── opensearch-client/     # Core client library
├── opensearch-dsl/        # Query DSL library
├── opensearch-cli/        # Command-line tools
├── opensearch-testcontainer/ # Testing utilities
├── examples/             # Usage examples
└── docs/                 # Additional documentation
```

### Workspace Organization

- **`opensearch-client`**: HTTP client, API bindings, authentication
- **`opensearch-dsl`**: Type-safe query building, aggregations, responses
- **`opensearch-cli`**: CLI tools for cluster management
- **`opensearch-testcontainer`**: Integration testing support

## 🎯 How to Contribute

### Types of Contributions

1. **Bug Fixes**: Report and fix bugs
2. **Feature Development**: Add new OpenSearch API support
3. **Documentation**: Improve docs and examples
4. **Testing**: Add tests and improve coverage
5. **Performance**: Optimize performance and memory usage

### Contribution Workflow

1. **Create an Issue**: For bugs or feature requests
2. **Fork the Repository**: Create your own fork
3. **Create a Branch**: Use descriptive branch names
4. **Make Changes**: Follow our coding standards
5. **Add Tests**: Ensure good test coverage
6. **Update Documentation**: Keep docs current
7. **Submit a Pull Request**: With clear description

### Branch Naming

Use descriptive branch names:
- `feature/add-search-api`
- `bugfix/fix-connection-timeout`
- `docs/update-readme`
- `test/add-integration-tests`

## 🧪 Testing

### Unit Tests

Run unit tests for all crates:

```bash
cargo test
```

Run tests for a specific crate:

```bash
cargo test --package opensearch-dsl
cargo test --package opensearch-client
cargo test --package opensearch-cli
```

### Integration Tests

Integration tests require a running OpenSearch instance.

#### Using Docker

Start OpenSearch with Docker:

```bash
docker run -d --name opensearch-test \
  -p 9200:9200 \
  -e "discovery.type=single-node" \
  -e "DISABLE_SECURITY_PLUGIN=true" \
  opensearchproject/opensearch:latest
```

Run integration tests:

```bash
cargo test --features integration-tests
```

#### Using Docker Compose

```yaml
# docker-compose.yml
version: '3'
services:
  opensearch:
    image: opensearchproject/opensearch:latest
    environment:
      - discovery.type=single-node
      - DISABLE_SECURITY_PLUGIN=true
    ports:
      - "9200:9200"
```

```bash
docker-compose up -d
cargo test --features integration-tests
docker-compose down
```

### Test Organization

- **Unit Tests**: In `src/` directories alongside code
- **Integration Tests**: In `tests/` directories
- **Examples**: In `examples/` directory with tests
- **Benchmarks**: In `benches/` directories

### Writing Tests

#### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let query = Query::match_("title", "opensearch");
        let json = serde_json::to_value(query).unwrap();
        
        assert_eq!(json["match"]["title"]["query"], "opensearch");
    }
}
```

#### Integration Test Example

```rust
#[tokio::test]
#[cfg(feature = "integration-tests")]
async fn test_search_integration() {
    let client = create_test_client().await;
    
    let response = client
        .search()
        .index("test_index")
        .query(Query::match_all())
        .await
        .unwrap();
    
    assert!(response.hits.total.value >= 0);
}
```

## 📝 Coding Standards

### Rust Style Guidelines

Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

1. **Formatting**: Use `cargo fmt`
2. **Linting**: Address `cargo clippy` warnings
3. **Naming**: Use `snake_case` for functions/variables, `PascalCase` for types
4. **Documentation**: Add doc comments for public APIs

### Code Quality

#### Documentation

All public APIs must have documentation:

```rust
/// Performs a search operation against the specified index.
///
/// # Arguments
///
/// * `query` - The search query to execute
/// * `index` - The index to search against
///
/// # Examples
///
/// ```rust
/// use opensearch_dsl::*;
/// 
/// let query = Query::match_("title", "opensearch");
/// let response = client.search(query, "my_index").await?;
/// ```
///
/// # Errors
///
/// Returns an error if the search request fails or if the index doesn't exist.
pub async fn search(&self, query: Query, index: &str) -> Result<SearchResponse> {
    // Implementation
}
```

#### Error Handling

Use proper error types and propagation:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenSearchError {
    #[error("Connection failed: {0}")]
    Connection(String),
    
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    
    #[error("Server error {status}: {message}")]
    ServerError { status: u16, message: String },
}
```

#### Async Code

Use async/await properly:

```rust
// Good
async fn search_documents(&self) -> Result<Vec<Document>> {
    let response = self.client.search().await?;
    let documents = response.parse_documents().await?;
    Ok(documents)
}

// Avoid blocking operations in async context
async fn bad_example(&self) -> Result<()> {
    // Don't do this - blocking the async runtime
    std::thread::sleep(Duration::from_secs(1));
    Ok(())
}
```

### API Design Principles

1. **Type Safety**: Use strong typing to prevent runtime errors
2. **Ergonomics**: Provide fluent, chainable APIs
3. **Performance**: Minimize allocations and copies
4. **Compatibility**: Maintain backward compatibility when possible
5. **Consistency**: Follow established patterns across the codebase

#### Example: Fluent API Design

```rust
// Good: Chainable, type-safe builder pattern
let search = Search::new()
    .query(Query::match_("title", "opensearch"))
    .size(10)
    .from(0)
    .sort(vec![Sort::field("timestamp").desc()])
    .source(SourceFilter::includes(vec!["title", "content"]));

// Good: Clear error types
pub enum ValidationError {
    InvalidFieldName(String),
    InvalidSortOrder(String),
    InvalidSize(usize),
}
```

## 🏗 Development Workflow

### Feature Development

1. **Create Issue**: Describe the feature and get feedback
2. **Design API**: Propose API design in the issue
3. **Implement**: Start with tests, then implementation
4. **Document**: Add documentation and examples
5. **Review**: Submit PR for code review

### Bug Fixes

1. **Reproduce**: Create a test that reproduces the bug
2. **Fix**: Implement the minimal fix
3. **Test**: Ensure the test now passes
4. **Verify**: Check that existing tests still pass

### Release Process

1. **Version Bump**: Update version in `Cargo.toml`
2. **Changelog**: Update `CHANGELOG.md`
3. **Documentation**: Update docs if needed
4. **Tag**: Create git tag for the release
5. **Publish**: Publish to crates.io

## 🎨 Adding New OpenSearch APIs

### API Implementation Checklist

When adding support for new OpenSearch APIs:

1. **Research**: Study the OpenSearch documentation
2. **Types**: Define request/response types
3. **Builder**: Create fluent builder APIs
4. **Client**: Add client methods
5. **Tests**: Write comprehensive tests
6. **Docs**: Add documentation and examples

### Example: Adding a New API

```rust
// 1. Define types
#[derive(Serialize, Deserialize)]
pub struct ExplainRequest {
    pub index: String,
    pub id: String,
    pub query: Query,
}

#[derive(Serialize, Deserialize)]
pub struct ExplainResponse {
    pub matched: bool,
    pub explanation: Value,
}

// 2. Add client method
impl OsClient {
    /// Explains why a document matches or doesn't match a query.
    pub async fn explain(&self, request: ExplainRequest) -> Result<ExplainResponse> {
        let url = format!("{}/_explain", self.base_url);
        let response = self.http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
}

// 3. Add tests
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_explain() {
        let client = create_test_client();
        let request = ExplainRequest {
            index: "test".to_string(),
            id: "1".to_string(),
            query: Query::match_all(),
        };
        
        let response = client.explain(request).await.unwrap();
        assert!(response.matched || !response.matched); // Should not panic
    }
}
```

## 🤝 Code Review Guidelines

### For Authors

1. **Self-Review**: Review your own PR before submission
2. **Description**: Provide clear PR description with context
3. **Tests**: Include relevant tests
4. **Documentation**: Update docs if needed
5. **Scope**: Keep PRs focused and reasonably sized

### For Reviewers

1. **Constructive**: Provide constructive feedback
2. **Standards**: Check coding standards compliance
3. **Logic**: Verify the logic and approach
4. **Performance**: Consider performance implications
5. **Security**: Review for security issues

### Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests are included and comprehensive
- [ ] Documentation is updated
- [ ] No breaking changes (or properly flagged)
- [ ] Performance impact is considered
- [ ] Error handling is appropriate

## 📄 Documentation Guidelines

### API Documentation

Use standard Rust doc comments:

```rust
/// Short description of the function.
///
/// Longer description with more details about the function's purpose,
/// behavior, and any important notes.
///
/// # Arguments
///
/// * `param1` - Description of parameter 1
/// * `param2` - Description of parameter 2
///
/// # Returns
///
/// Description of what the function returns.
///
/// # Examples
///
/// ```rust
/// use opensearch_client::*;
/// 
/// let result = function_name(param1, param2);
/// assert_eq!(result, expected_value);
/// ```
///
/// # Errors
///
/// Description of when and why the function might return an error.
///
/// # Panics
///
/// Description of when the function might panic (if ever).
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // Implementation
}
```

### README Updates

When adding significant features:

1. Update the main README
2. Update crate-specific READMEs
3. Add examples to demonstrate usage
4. Update the features list

## 🏷 Issue and PR Templates

### Issue Template

```markdown
## Description
Brief description of the issue or feature request.

## Type
- [ ] Bug report
- [ ] Feature request
- [ ] Documentation improvement
- [ ] Question

## Current Behavior
What currently happens.

## Expected Behavior
What should happen.

## Steps to Reproduce (for bugs)
1. Step 1
2. Step 2
3. Step 3

## Environment
- Rust version: 
- OpenSearch version:
- Crate version:
- OS:
```

### PR Template

```markdown
## Description
Brief description of changes.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Refactoring

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Added new tests for changes

## Documentation
- [ ] Updated API documentation
- [ ] Updated README if needed
- [ ] Added examples if applicable

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] No breaking changes (or properly documented)
```

## 🎉 Recognition

Contributors will be recognized in:

- The project's README
- Release notes for significant contributions
- The project's documentation

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Discord/Slack**: (if available) For real-time chat

## 📜 License

By contributing to this project, you agree that your contributions will be licensed under the same license as the project (Apache 2.0).

---

Thank you for contributing to OpenSearch Client for Rust! 🦀