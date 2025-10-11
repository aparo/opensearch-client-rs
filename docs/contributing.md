# Contributing Guide

Thank you for your interest in contributing to the OpenSearch Rust Client! This guide will help you get started with contributing to the project.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- Docker (for running OpenSearch locally)
- Git

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/opensearch-client-rs.git
   cd opensearch-client-rs
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Set up OpenSearch for Testing**
   ```bash
   docker run -d -p 9200:9200 \
     -e "discovery.type=single-node" \
     -e "DISABLE_SECURITY_PLUGIN=true" \
     opensearchproject/opensearch:latest
   ```

4. **Run Tests**
   ```bash
   cargo test
   ```

## 📝 Types of Contributions

We welcome various types of contributions:

### Code Contributions
- Bug fixes
- New features
- Performance improvements
- Code quality improvements
- Documentation updates

### Non-Code Contributions
- Bug reports
- Feature requests
- Documentation improvements
- Examples and tutorials
- Community support

## 🔧 Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 2. Make Changes

Follow these guidelines when making changes:

#### Code Style
- Use `cargo fmt` to format your code
- Run `cargo clippy` to check for common issues
- Follow Rust naming conventions
- Add documentation for public APIs

#### Testing
- Write unit tests for new functionality
- Add integration tests for complex features
- Ensure all tests pass: `cargo test`
- Test with different OpenSearch versions if applicable

#### Documentation
- Update relevant documentation
- Add examples for new features
- Update the changelog for significant changes

### 3. Commit Changes

Use clear, descriptive commit messages:

```bash
git commit -m "feat: add support for geo queries

- Add GeoPoint and GeoBoundingBox query types
- Update DSL to support geo distance queries
- Add comprehensive tests for geo functionality"
```

Follow conventional commit format:
- `feat:` - New features
- `fix:` - Bug fixes  
- `docs:` - Documentation changes
- `style:` - Code style changes
- `refactor:` - Code refactoring
- `test:` - Test additions or changes
- `chore:` - Maintenance tasks

### 4. Push and Create PR

```bash
git push origin your-branch-name
```

Then create a Pull Request with:
- Clear title and description
- Reference any related issues
- Include testing information
- List any breaking changes

## 🏗️ Project Structure

Understanding the codebase structure:

```
opensearch-client/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── client/             # Client configuration and auth
│   ├── bulk.rs             # Bulk operations
│   ├── bulker.rs           # Bulk operation executor
│   ├── core/               # Core OpenSearch API
│   ├── indices/            # Index management
│   ├── search/             # Search operations
│   └── [api_modules]/      # Generated API modules
├── opensearch-dsl/         # Query DSL library
├── opensearch-cli/         # Command-line tools
├── tests/                  # Integration tests
└── docs/                   # Documentation
```

## 🧪 Testing Guidelines

### Unit Tests
```bash
# Run unit tests
cargo test --lib

# Test specific module
cargo test client::auth
```

### Integration Tests
```bash
# Requires OpenSearch running on localhost:9200
cargo test --test integration_tests

# Run with different OpenSearch version
OPENSEARCH_VERSION=2.11.0 cargo test
```

### Test Coverage
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## 📚 Adding New Features

### New API Endpoints

When adding support for new OpenSearch APIs:

1. **Add the endpoint definition**
   ```rust
   // In appropriate module (e.g., src/cluster/mod.rs)
   pub async fn new_endpoint(&self, params: NewEndpointParams) -> Result<Response, Error> {
       let url = format!("/_cluster/new_endpoint");
       self.client.send(Method::GET, &url, None).await
   }
   ```

2. **Create request/response types**
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct NewEndpointParams {
       pub param1: String,
       pub param2: Option<bool>,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct NewEndpointResponse {
       pub result: String,
   }
   ```

3. **Add tests**
   ```rust
   #[tokio::test]
   async fn test_new_endpoint() {
       let client = setup_test_client().await;
       let params = NewEndpointParams {
           param1: "test".to_string(),
           param2: Some(true),
       };
       
       let response = client.cluster().new_endpoint(params).await;
       assert!(response.is_ok());
   }
   ```

4. **Add documentation**
   ```rust
   /// Calls the new endpoint API.
   ///
   /// # Arguments
   /// * `params` - Parameters for the new endpoint
   ///
   /// # Example
   /// ```rust
   /// let response = client.cluster().new_endpoint(params).await?;
   /// ```
   pub async fn new_endpoint(&self, params: NewEndpointParams) -> Result<NewEndpointResponse, Error>
   ```

### New Document Derive Features

When adding new derive macro features:

1. **Update the derive macro**
   ```rust
   // In opensearch-client-derive/src/lib.rs
   match &field.attrs.iter().find(|attr| attr.path.is_ident("os")) {
       Some(attr) => parse_os_attribute(attr),
       None => default_field_config(),
   }
   ```

2. **Add tests for the derive macro**
   ```rust
   #[test]
   fn test_new_attribute() {
       let input = quote! {
           #[derive(OpenSearch)]
           #[os(index = "test")]
           struct TestDoc {
               #[os(new_attribute = "value")]
               field: String,
           }
       };
       
       let output = derive_opensearch(input);
       // Assert expected output
   }
   ```

3. **Update documentation with examples**

## 🐛 Bug Reports

When reporting bugs, include:

1. **Environment Information**
   - Rust version: `rustc --version`
   - Crate version
   - OpenSearch version
   - Operating system

2. **Reproduction Steps**
   ```rust
   // Minimal code example that reproduces the issue
   use opensearch_client::*;
   
   #[tokio::main]
   async fn main() {
       // Steps to reproduce
   }
   ```

3. **Expected vs Actual Behavior**
   - What you expected to happen
   - What actually happened
   - Error messages or logs

4. **Additional Context**
   - Screenshots if applicable
   - Related issues or PRs

## 🚀 Feature Requests

For feature requests, please:

1. **Check existing issues** to avoid duplicates
2. **Describe the use case** - why is this needed?
3. **Propose a solution** - how should it work?
4. **Consider alternatives** - are there other ways to achieve this?
5. **Assess impact** - who would benefit from this feature?

Use this template:

```markdown
## Feature Description
Brief description of the feature

## Use Case
Why is this feature needed? What problem does it solve?

## Proposed Solution
How should this feature work?

## Alternatives Considered
What other approaches have you considered?

## Additional Context
Any other relevant information
```

## 📖 Documentation

### Code Documentation

- Use rustdoc comments (`///`) for public APIs
- Include examples in documentation
- Document error conditions
- Explain complex algorithms or design decisions

Example:
```rust
/// Performs a bulk operation on multiple documents.
///
/// This method is more efficient than individual operations when
/// working with many documents.
///
/// # Arguments
/// * `operations` - Vector of bulk operations to execute
///
/// # Returns
/// * `BulkResponse` - Response containing results for each operation
///
/// # Errors
/// * `Error::Connection` - If connection to OpenSearch fails
/// * `Error::Serialization` - If request serialization fails
///
/// # Example
/// ```rust
/// let operations = vec![
///     BulkOperation::index(doc1),
///     BulkOperation::update("id2", update_doc),
/// ];
/// 
/// let response = bulker.execute(operations).await?;
/// ```
pub async fn execute(&self, operations: Vec<BulkOperation>) -> Result<BulkResponse, Error>
```

### Guide Documentation

When adding guides:
- Use clear, descriptive titles
- Include practical examples
- Explain both simple and advanced use cases
- Add troubleshooting sections
- Cross-reference related topics

## 🔍 Review Process

### Pull Request Review

PRs will be reviewed for:

1. **Functionality** - Does it work as intended?
2. **Code Quality** - Is it well-structured and readable?
3. **Testing** - Are there adequate tests?
4. **Documentation** - Is it properly documented?
5. **Performance** - Does it impact performance?
6. **Compatibility** - Does it maintain backward compatibility?

### Addressing Review Comments

- Address all review comments
- Ask for clarification if needed
- Update tests and documentation as requested
- Be responsive to feedback

## 🎯 Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- Major: Breaking changes
- Minor: New features (backward compatible)
- Patch: Bug fixes (backward compatible)

### Changelog

Update `CHANGELOG.md` for significant changes:

```markdown
## [0.2.0] - 2024-01-15

### Added
- Support for geo queries
- Bulk operation retries
- New authentication methods

### Changed
- Improved error handling
- Updated dependencies

### Fixed
- Connection pool cleanup
- Memory leak in streaming operations
```

## 💬 Community

### Communication Channels

- **GitHub Issues** - Bug reports and feature requests
- **GitHub Discussions** - General questions and community chat
- **Discord** - Real-time community chat (if available)

### Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## 🏆 Recognition

Contributors will be:
- Listed in the project contributors
- Mentioned in release notes for significant contributions
- Invited to join the maintainers team for ongoing contributors

## ❓ Questions?

If you have questions about contributing:
- Check existing documentation
- Search GitHub issues and discussions
- Create a new discussion for general questions
- Create an issue for specific problems

Thank you for contributing to the OpenSearch Rust Client! 🚀