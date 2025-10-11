# Best Practices

This guide covers best practices for effective document modeling with the OpenSearch macro system.

## Required Derives

Always include these derives for full functionality:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "my_index")]
pub struct MyDocument {
    #[os(id)]
    pub id: String,
    // other fields...
}
```

### Essential Derives Explained

- **`Debug`** - For debugging and logging support
- **`Clone`** - Required by the Document trait for operations
- **`Serialize`/`Deserialize`** - For JSON conversion with serde
- **`OpenSearch`** - The derive macro that implements the Document trait

### Optional but Recommended Derives

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, OpenSearch)]
#[os(index = "my_index")]
pub struct MyDocument {
    // fields...
}
```

- **`PartialEq`/`Eq`** - For equality comparisons
- **`Hash`** - For using documents as hash map keys
- **`Default`** - For creating default instances

## ID Field Guidelines

### Basic Requirements

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "my_index")]
pub struct MyDoc {
    #[os(id)]
    pub id: String,  // Must be String type
    // other fields...
}
```

**Rules:**
- The ID field must be of type `String`
- Exactly one field must be marked with `#[os(id)]`
- The ID field is used for document identification in OpenSearch

### ID Generation Strategies

**UUID-based IDs:**
```rust
use uuid::Uuid;

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            email,
            created_at: chrono::Utc::now(),
        }
    }
}
```

**Business Logic IDs:**
```rust
impl Product {
    pub fn new(sku: String, name: String) -> Self {
        Self {
            id: sku.clone(),  // Use SKU as ID
            sku,
            name,
        }
    }
}
```

**Composite IDs:**
```rust
impl UserSession {
    pub fn new(user_id: String, session_start: DateTime<Utc>) -> Self {
        let id = format!("{}_{}", user_id, session_start.timestamp());
        Self {
            id,
            user_id,
            session_start,
        }
    }
}
```

## Index Naming Conventions

### Recommended Patterns

```rust
// ✅ Good examples
#[os(index = "users")]              // Simple, descriptive
#[os(index = "blog_posts")]         // Snake case for multi-word
#[os(index = "product-reviews")]    // Kebab case alternative
#[os(index = "events-2023-10")]     // Time-based partitioning
#[os(index = "metrics_v2")]         // Version suffixes
```

### Patterns to Avoid

```rust
// ❌ Problematic examples
#[os(index = "Users")]              // Uppercase not recommended
#[os(index = "user data")]          // Spaces not allowed
#[os(index = "user@data")]          // Special chars problematic
#[os(index = "a")]                  // Too short, not descriptive
```

### Naming Guidelines

- Use lowercase names consistently
- Prefer descriptive names over abbreviations
- Use hyphens or underscores for multi-word names
- Consider time-based partitioning for large datasets
- Include version numbers for schema evolution

## Type Compatibility Guidelines

### Primitive Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "products")]
pub struct Product {
    #[os(id)]
    pub id: String,
    
    // ✅ Well-supported primitive types
    pub name: String,               // Text field
    pub price: f64,                 // Numeric field
    pub in_stock: bool,             // Boolean field
    pub quantity: u32,              // Integer field
    
    // ✅ Collections
    pub tags: Vec<String>,          // Array of strings
    pub ratings: Vec<f64>,          // Array of numbers
    
    // ✅ Optional fields
    pub description: Option<String>,
    pub sale_price: Option<f64>,
}
```

### Complex Types

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
    
    // ✅ Nested documents (must derive OpenSearch)
    pub address: Address,
    
    // ✅ Custom types for dynamic content
    pub metadata: serde_json::Value,
    
    // ✅ Time types (with proper serialization)
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}
```

## Document Structure Best Practices

### Flat vs Nested Structures

**Prefer flat structures when possible:**
```rust
// ✅ Flat structure - easier to query
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub address_street: String,
    pub address_city: String,
    pub address_country: String,
}
```

**Use nested structures for complex relationships:**
```rust
// ✅ Nested when it makes sense
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "orders")]
pub struct Order {
    #[os(id)]
    pub id: String,
    pub items: Vec<OrderItem>,      // Complex nested objects
    pub shipping: ShippingInfo,     // Logical grouping
}
```

### Field Naming Conventions

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "events")]
pub struct Event {
    #[os(id)]
    pub id: String,
    
    // ✅ Use consistent naming
    pub event_type: String,         // snake_case
    pub created_at: String,         // consistent time suffix
    pub updated_at: String,
    
    // ✅ Clear, descriptive names
    pub user_id: String,            // clear relationship
    pub session_id: String,
    
    // ✅ Boolean naming
    pub is_active: bool,            // clear boolean intent
    pub has_errors: bool,
}
```

## Performance Optimization

### Index Design

**Single responsibility per index:**
```rust
// ✅ Good - each document type has its own index
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User { /* user fields */ }

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "orders")]
pub struct Order { /* order fields */ }
```

**Avoid very wide documents:**
```rust
// ❌ Avoid - too many fields in one document
pub struct EverythingDocument {
    // 50+ fields...
}

// ✅ Better - split into focused documents
pub struct User { /* core user fields */ }
pub struct UserProfile { /* extended profile info */ }
pub struct UserSettings { /* user preferences */ }
```

### Query-Friendly Design

**Design for your query patterns:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "products")]
pub struct Product {
    #[os(id)]
    pub id: String,
    
    // ✅ Fields you'll query by
    pub category: String,           // Filtered frequently
    pub status: String,             // Active/inactive queries
    pub price: f64,                 // Range queries
    pub created_at: String,         // Time-based queries
    
    // ✅ Fields for sorting
    pub popularity_score: f64,
    pub last_updated: String,
}
```

## Error Handling Patterns

### Defensive Programming

```rust
impl User {
    pub async fn safe_get(id: &str) -> Result<Option<User>, Error> {
        match User::get(id).await {
            Ok(user) => Ok(Some(user)),
            Err(Error::DocumentNotFoundError(_, _)) => Ok(None),
            Err(e) => Err(e),
        }
    }
    
    pub async fn get_or_create_default(id: &str) -> Result<User, Error> {
        match User::get(id).await {
            Ok(user) => Ok(user),
            Err(Error::DocumentNotFoundError(_, _)) => {
                let default_user = User::default_with_id(id.to_string());
                default_user.save().await?;
                Ok(default_user)
            }
            Err(e) => Err(e),
        }
    }
}
```

### Validation

```rust
impl User {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        if self.age < 0 || self.age > 150 {
            return Err("Invalid age".to_string());
        }
        
        Ok(())
    }
    
    pub async fn save_validated(&self) -> Result<IndexResponse, Error> {
        self.validate().map_err(|msg| Error::ValidationError(msg))?;
        self.save().await
    }
}
```

## Testing Strategies

### Unit Testing Documents

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User {
            id: "test123".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            age: 25,
        };
        
        assert_eq!(user.id(), "test123");
        assert_eq!(User::index_name(), "users");
    }
    
    #[test]
    fn test_field_introspection() {
        let fields = User::columns();
        assert!(fields.iter().any(|f| f.name == "name"));
        assert!(fields.iter().any(|f| f.name == "email"));
    }
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_user_crud_operations() {
    let user = User::new("Test User".to_string(), "test@example.com".to_string());
    
    // Create
    let response = user.save().await.unwrap();
    assert!(!response.id.is_empty());
    
    // Read
    let retrieved = User::get(&user.id).await.unwrap();
    assert_eq!(retrieved.name, user.name);
    
    // Update
    User::update(&user.id, &json!({"age": 30})).await.unwrap();
    
    // Delete
    User::delete(&user.id).await.unwrap();
}
```