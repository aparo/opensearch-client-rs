# Field Type Mapping

The OpenSearch macro automatically maps Rust types to appropriate OpenSearch field types. Understanding this mapping is crucial for effective document modeling and querying.

## Type Mapping Table

| Rust Type | OpenSearch Type | Field Type | Aggregatable | Searchable | Notes |
|-----------|-----------------|------------|--------------|------------|-------|
| `String`, `&str` | `text` | `string` | ❌ | ✅ | Full-text searchable, analyzed |
| `u32`, `i32`, `u64`, `i64`, `usize`, `isize` | `long` | `number` | ✅ | ✅ | Integer numbers, sortable |
| `f32`, `f64` | `double` | `number` | ✅ | ✅ | Floating point numbers |
| `bool` | `boolean` | `boolean` | ✅ | ✅ | True/false values |
| `Vec<T>` | (inner type) | (inner type) | Varies | Varies | Arrays of the inner type |
| `Option<T>` | (inner type) | (inner type) | Varies | Varies | Nullable fields |
| Custom structs | `object` | `object` | ❌ | ❌ | Nested documents |

## Detailed Type Explanations

### String Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "documents")]
pub struct Document {
    #[os(id)]
    pub id: String,
    
    pub title: String,        // Mapped to "text" - full-text searchable
    pub description: String,  // Analyzed for search
    pub category: String,     // Consider using keyword mapping for exact matches
}
```

**Notes:**
- Strings are mapped to `text` type by default
- Text fields are analyzed (tokenized) for full-text search
- Use keyword fields for exact matching (see advanced mapping)

### Numeric Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "products")]
pub struct Product {
    #[os(id)]
    pub id: String,
    
    pub price: f64,           // Mapped to "double"
    pub quantity: u32,        // Mapped to "long"
    pub rating: f32,          // Mapped to "double"
    pub views: i64,           // Mapped to "long"
}
```

**Notes:**
- All integer types map to `long` in OpenSearch
- Floating point types map to `double`
- Numeric fields support range queries and aggregations

### Boolean Type

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,
    
    pub active: bool,         // Mapped to "boolean"
    pub verified: bool,       // Perfect for filters
    pub premium: bool,        // Supports aggregations
}
```

**Usage examples:**
```rust
// Boolean queries
let active_users = User::find(
    Search::new().query(Query::term("active", true))
).await?;

// Boolean aggregations
let stats = Search::new()
    .aggregations([
        ("active_count", Aggregation::terms("active")),
        ("premium_ratio", Aggregation::avg("premium"))
    ]);
```

### Array Types (Vec<T>)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "articles")]
pub struct Article {
    #[os(id)]
    pub id: String,
    
    pub tags: Vec<String>,    // Array of text fields
    pub scores: Vec<f64>,     // Array of numbers
    pub flags: Vec<bool>,     // Array of booleans
}
```

**Notes:**
- Arrays inherit the type of their inner element
- OpenSearch automatically handles array indexing
- All elements in the array share the same field mapping

**Querying arrays:**
```rust
// Find articles with specific tags
let tagged_articles = Article::find(
    Search::new().query(Query::term("tags", "rust"))
).await?;

// Range queries on numeric arrays
let scored_articles = Article::find(
    Search::new().query(Query::range("scores").gte(8.0))
).await?;
```

### Optional Types (Option<T>)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "profiles")]
pub struct Profile {
    #[os(id)]
    pub id: String,
    
    pub name: String,
    pub bio: Option<String>,      // Nullable text field
    pub age: Option<u32>,         // Nullable number field
    pub verified: Option<bool>,   // Nullable boolean field
}
```

**Notes:**
- Optional fields can be `null` in the OpenSearch document
- The inner type determines the field mapping
- Use `exists` query to check for field presence

**Querying optional fields:**
```rust
// Find profiles with bio
let profiles_with_bio = Profile::find(
    Search::new().query(Query::exists("bio"))
).await?;

// Find profiles without age
let profiles_without_age = Profile::find(
    Search::new().query(Query::bool().must_not(Query::exists("age")))
).await?;
```

### Custom Struct Types (Nested Objects)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "addresses")]
pub struct Address {
    #[os(id)]
    pub id: String,
    pub street: String,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub address: Address,     // Nested object type
}
```

**Notes:**
- Custom structs that implement `Document` are treated as nested objects
- The nested struct's field information is available via its own `columns()` method
- Nested objects maintain their structure in OpenSearch

## Advanced Type Considerations

### Date and Time Types

While not automatically mapped, you can use string types with proper formatting:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "events")]
pub struct Event {
    #[os(id)]
    pub id: String,
    
    pub created_at: String,   // ISO 8601: "2023-10-08T12:00:00Z"
    pub updated_at: String,   // Will be treated as text by default
}
```

For proper date handling, consider using chrono with serde:

```rust
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "events")]
pub struct Event {
    #[os(id)]
    pub id: String,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,   // Serialized as timestamp
}
```

### Custom Serialization

For complex types, implement custom serde serialization:

```rust
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "flexible")]
pub struct FlexibleDoc {
    #[os(id)]
    pub id: String,
    
    pub metadata: Value,      // Arbitrary JSON structure
}
```

## Type Safety Benefits

The macro system provides compile-time guarantees:

```rust
// ✅ This compiles - matching types
let user = User {
    id: "123".to_string(),
    age: 30u32,                // Correct type
    active: true,              // Correct type
};

// ❌ This fails to compile - type mismatch
let user = User {
    id: "123".to_string(),
    age: "thirty",             // Wrong type!
    active: "yes",             // Wrong type!
};
```

## Field Introspection

You can examine the field mappings at runtime:

```rust
let fields = User::columns();
for field in fields {
    println!("Field: {}", field.name);
    println!("  Type: {}", field.field_type);
    println!("  OpenSearch Type: {}", field.os_type);
    println!("  Aggregatable: {}", field.aggregatable);
    println!("  Searchable: {}", field.searchable);
}
```

This introspection capability enables:
- Dynamic query building
- API documentation generation
- Index mapping validation
- Runtime type checking