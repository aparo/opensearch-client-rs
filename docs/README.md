# OpenSearch Client for Rust - Documentation

[![Crates.io](https://img.shields.io/crates/v/opensearch-client)](https://crates.io/crates/opensearch-client)
[![License](https://img.shields.io/crates/l/opensearch-client)](https://crates.io/crates/opensearch-client)
[![Documentation](https://docs.rs/opensearch-client/badge.svg)](https://docs.rs/opensearch-client)

A comprehensive Rust client library for OpenSearch with a strongly typed DSL, CLI tools, and extensive API coverage.

## 📚 Documentation Structure

### Getting Started
- [Installation & Quick Start](getting-started.md) - Installation, basic setup, and first steps
- [Client Configuration](configuration.md) - Detailed configuration options and settings
- [Architecture Overview](architecture.md) - Understanding the project structure and components

### Core Features
- [Document Modeling](documents/README.md) - Complete guide to the Document trait and macro system
- [Query Building](queries/README.md) - DSL query building and search operations
- [Index Management](indices.md) - Index creation, management, and operations
- [Examples](examples/README.md) - Practical examples and common patterns

### Advanced Topics
- [CLI Tools](cli.md) - Command-line interface for cluster management
- [API Reference](api-reference.md) - Complete API documentation
- [Testing Guide](testing.md) - Testing strategies and utilities

### Development
- [Contributing](contributing.md) - Guidelines for contributors
- [Development Setup](development.md) - Setting up the development environment

## 🚀 Features

- **Strongly Typed DSL**: Type-safe query building with compile-time guarantees
- **Comprehensive API Coverage**: Support for search, indices, cluster management, and more
- **CLI Tools**: Command-line interface for cluster management and data operations
- **Document Modeling**: Macro-based document models with automatic trait implementation
- **Async/Await Support**: Built on modern async Rust with tokio
- **Production Ready**: Includes retry logic, connection pooling, and error handling
- **Extensible**: Modular design with feature flags for optional functionality

## 📦 Project Structure

This workspace contains several crates:

- **`opensearch-client`**: Core client library with API bindings
- **`opensearch-dsl`**: Strongly typed query DSL
- **`opensearch-cli`**: Command-line tools for cluster management
- **`opensearch-macro`**: Derive macros for document modeling
- **`opensearch-testcontainer`**: Testing utilities with container support

## 🔗 Quick Links

- [Installation & Quick Start](getting-started.md)
- [Document Modeling Guide](documents/README.md)
- [Query Building Guide](queries/README.md)
- [CLI Tools Documentation](cli.md)
- [API Reference](api-reference.md)
- [Contributing Guidelines](contributing.md)

## 📞 Support

- [Documentation](https://docs.rs/opensearch-client)
- [GitHub Issues](https://github.com/aparo/opensearch-client-rs/issues)
- [Discussions](https://github.com/aparo/opensearch-client-rs/discussions)

---

Made with ❤️ by the OpenSearch Rust community