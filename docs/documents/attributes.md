# Macro Attributes

The OpenSearch derive macro uses attributes to configure document behavior. These attributes control index mapping, field behavior, and document identification.

## Index Configuration

### `#[os(index = "...")]`

Every document struct must specify its target index using the `index` attribute:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]           // Target index name
pub struct User {
    #[os(id)]
    pub id: String,
    pub name: String,
}
```

**Requirements:**
- Must be present on every struct that derives `OpenSearch`
- Index name must be a string literal
- Index name should follow OpenSearch naming conventions

**Index Naming Best Practices:**

```rust
// ✅ Good examples
#[os(index = "users")]              // Simple, descriptive
#[os(index = "blog_posts")]         // Snake case for multi-word
#[os(index = "product-reviews")]    // Kebab case alternative
#[os(index = "events-2023")]        // Time-based naming

// ❌ Avoid these
#[os(index = "Users")]              // Uppercase not recommended
#[os(index = "user data")]          // Spaces not allowed
#[os(index = "user@data")]          // Special chars not allowed
```

**Runtime Access:**

```rust
let index_name = User::index_name();
println!("Users are stored in: {}", index_name); // "users"
```

## ID Field Configuration

### `#[os(id)]`

One field in each document must be marked as the unique identifier:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "products")]
pub struct Product {
    #[os(id)]                       // This field is the document ID
    pub sku: String,                // Must be String type
    pub name: String,
    pub price: f64,
}
```

**Requirements:**
- Exactly one field must have `#[os(id)]`
- ID field must be of type `String`
- ID field provides the unique identifier for OpenSearch documents

**ID Field Examples:**

```rust
// ✅ Valid ID field configurations
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,                 // Simple string ID
    // ...
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "products")]
pub struct Product {
    #[os(id)]
    pub sku: String,                // Business identifier as ID
    // ...
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "sessions")]
pub struct Session {
    #[os(id)]
    pub session_token: String,      // Token as ID
    // ...
}
```

**Runtime Access:**

```rust
let user = User::get("user123").await?;
let user_id = user.id();           // Returns &str
println!("User ID: {}", user_id);  // "user123"
```

## Advanced Attribute Usage

### Multiple Documents, Multiple Indices

Each document type maps to exactly one index:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "user_profiles")]      // Different index
pub struct UserProfile {
    #[os(id)]
    pub user_id: String,            // Can reference User ID
    pub bio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "user_sessions")]      // Another different index
pub struct UserSession {
    #[os(id)]
    pub session_id: String,
    pub user_id: String,            // Foreign key reference
}
```

### ID Generation Strategies

While the macro requires a `String` ID field, you can implement various ID generation strategies:

```rust
use uuid::Uuid;

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),    // UUID v4
            name,
            email,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

// Usage
let user = User::new("John Doe".to_string(), "john@example.com".to_string());
user.save().await?;
```

## Validation and Error Handling

### Compile-Time Validation

The macro performs several compile-time checks:

```rust
// ❌ This won't compile - missing index attribute
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
pub struct InvalidDoc {
    pub id: String,
}

// ❌ This won't compile - missing ID field
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "invalid")]
pub struct InvalidDoc2 {
    pub name: String,
}

// ❌ This won't compile - wrong ID field type
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "invalid")]
pub struct InvalidDoc3 {
    #[os(id)]
    pub id: u64,                    // Must be String!
    pub name: String,
}
```

### Runtime Behavior

**ID Field Access:**
```rust
let user = User {
    id: "user123".to_string(),
    name: "John".to_string(),
};

// The ID field is used for document operations
assert_eq!(user.id(), "user123");

// Save uses the ID for document creation
user.save().await?;  // Creates document with ID "user123"
```

**Index Access:**
```rust
// Index name is used for all operations
assert_eq!(User::index_name(), "users");

// All operations target the configured index
let user = User::get("123").await?;      // Searches in "users" index
User::delete("123").await?;              // Deletes from "users" index
```

## Common Patterns

### Hierarchical Document Structure

```rust
// Base document
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "organizations")]
pub struct Organization {
    #[os(id)]
    pub id: String,
    pub name: String,
}

// Child document with parent reference
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "departments")]
pub struct Department {
    #[os(id)]
    pub id: String,
    pub org_id: String,             // Reference to parent
    pub name: String,
}

// Leaf document
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "employees")]
pub struct Employee {
    #[os(id)]
    pub id: String,
    pub dept_id: String,            // Reference to department
    pub name: String,
}
```

### Time-Series Documents

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "metrics-2023")]       // Time-based index naming
pub struct Metric {
    #[os(id)]
    pub id: String,                 // timestamp + metric_name
    pub timestamp: String,
    pub metric_name: String,
    pub value: f64,
}

impl Metric {
    pub fn new(metric_name: String, value: f64) -> Self {
        let timestamp = chrono::Utc::now();
        Self {
            id: format!("{}_{}", timestamp.timestamp(), metric_name),
            timestamp: timestamp.to_rfc3339(),
            metric_name,
            value,
        }
    }
}
```

### Multi-Tenant Documents

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "tenant_data")]
pub struct TenantDocument {
    #[os(id)]
    pub id: String,
    pub tenant_id: String,          // Tenant isolation
    pub data: String,
}

impl TenantDocument {
    pub fn new_for_tenant(tenant_id: String, data: String) -> Self {
        Self {
            id: format!("{}_{}", tenant_id, Uuid::new_v4()),
            tenant_id,
            data,
        }
    }
}
```