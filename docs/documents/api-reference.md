# Document Trait API Reference

The `Document` trait provides a complete ORM-like interface for working with OpenSearch documents. All methods are automatically implemented when you use the `#[derive(OpenSearch)]` macro.

## Static Methods

### Index and Metadata Methods

#### `index_name() -> &'static str`

Returns the index name configured with `#[os(index = "...")]`.

```rust
let index = User::index_name();
println!("Users are stored in: {}", index); // "users"
```

#### `columns() -> Vec<Field>`

Returns field metadata for introspection and mapping.

```rust
let fields = User::columns();
for field in fields {
    println!("Field: {} ({})", field.name, field.field_type);
}
```

### Document Retrieval Methods

#### `get(id: &str) -> Result<Self, Error>`

Fetch a document by its ID.

```rust
// Get a user by ID
let user = User::get("user123").await?;
println!("User: {}", user.name);

// Handle not found
match User::get("nonexistent").await {
    Ok(user) => println!("Found: {}", user.name),
    Err(Error::DocumentNotFoundError(_, _)) => println!("Not found"),
    Err(e) => return Err(e),
}
```

#### `find(search: Search) -> Result<TypedSearchResult<Self>, Error>`

Search for documents using a complex query.

```rust
let results = User::find(
    Search::new()
        .query(Query::term("active", true))
        .sort([("name.keyword", "asc")])
        .from(0)
        .size(10)
).await?;

for hit in results.hits.hits {
    if let Some(user) = hit.source {
        println!("User: {}", user.name);
    }
}
```

#### `find_all(limit: Option<usize>) -> Result<TypedSearchResult<Self>, Error>`

Find all documents with an optional limit.

```rust
// Get all users (limited)
let all_users = User::find_all(Some(100)).await?;

// Get all users (no limit - be careful!)
let all_users = User::find_all(None).await?;
```

#### `find_one(search: Search) -> Result<Option<Self>, Error>`

Find a single document matching the search criteria.

```rust
let maybe_user = User::find_one(
    Search::new().query(Query::term("email", "john@example.com"))
).await?;

if let Some(user) = maybe_user {
    println!("Found user: {}", user.name);
}
```

#### `count(query: Option<Query>) -> Result<u32, Error>`

Count documents matching a query.

```rust
// Count all users
let total = User::count(None).await?;

// Count active users
let active = User::count(Some(Query::term("active", true))).await?;

println!("Active users: {}/{}", active, total);
```

### Document Modification Methods

#### `delete(id: &str) -> Result<DocumentDeleteResponse, Error>`

Delete a document by its ID.

```rust
let response = User::delete("user123").await?;
println!("Delete result: {}", response.result);
```

#### `update(id: &str, partial_doc: &Value) -> Result<IndexResponse, Error>`

Update specific fields of a document without replacing the entire document.

```rust
// Update specific fields
User::update("user123", &json!({
    "age": 31,
    "last_login": "2023-10-08T12:00:00Z"
})).await?;

// Update nested fields
User::update("user123", &json!({
    "profile.bio": "Updated biography"
})).await?;
```

## Instance Methods

### Document Identification

#### `id(&self) -> &str`

Get the document's ID from the field marked with `#[os(id)]`.

```rust
let user = User::get("user123").await?;
let user_id = user.id();
println!("Document ID: {}", user_id); // "user123"
```

### Document Persistence

#### `save(&self) -> Result<IndexResponse, Error>`

Create or update this document in OpenSearch.

```rust
let user = User {
    id: "user123".to_string(),
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
    // ... other fields
};

let response = user.save().await?;
println!("Saved with version: {}", response.version);
```

**Behavior:**
- If document with this ID exists, it will be updated
- If document doesn't exist, it will be created
- Uses the document's ID field as the OpenSearch document ID

#### `refresh(&mut self) -> Result<(), Error>`

Reload this document instance with the latest data from OpenSearch.

```rust
let mut user = User::get("user123").await?;
println!("Current age: {}", user.age);

// Someone else updates the user...
// Refresh to get the latest data
user.refresh().await?;
println!("Updated age: {}", user.age);
```

## Field Metadata Structure

The `Field` struct returned by `columns()` provides detailed information about each field:

```rust
pub struct Field {
    pub name: String,           // Field name as it appears in the struct
    pub field_type: String,     // Human-readable type (string, number, boolean, object)
    pub os_type: String,        // OpenSearch mapping type (text, long, boolean, object)
    pub aggregatable: bool,     // Whether field can be used in aggregations
    pub searchable: bool,       // Whether field can be searched/filtered
    pub sub_fields: Vec<Box<Field>>, // Nested fields (for object types)
}
```

### Field Type Examples

```rust
let fields = User::columns();
for field in fields {
    match field.field_type.as_str() {
        "string" => println!("{}: Text field, searchable: {}", field.name, field.searchable),
        "number" => println!("{}: Numeric field, aggregatable: {}", field.name, field.aggregatable),
        "boolean" => println!("{}: Boolean field", field.name),
        "object" => {
            println!("{}: Nested object with {} sub-fields", field.name, field.sub_fields.len());
            for sub_field in &field.sub_fields {
                println!("  - {}: {}", sub_field.name, sub_field.field_type);
            }
        }
        _ => println!("{}: Other type ({})", field.name, field.field_type),
    }
}
```

## Error Handling

All Document trait methods return `Result<T, Error>`. Common error types:

### `DocumentNotFoundError`

```rust
match User::get("nonexistent").await {
    Ok(user) => { /* handle success */ }
    Err(Error::DocumentNotFoundError(index, id)) => {
        println!("Document {} not found in index {}", id, index);
    }
    Err(e) => { /* handle other errors */ }
}
```

### `ConnectionError`

```rust
match user.save().await {
    Ok(response) => { /* success */ }
    Err(Error::ConnectionError(msg)) => {
        eprintln!("Failed to connect to OpenSearch: {}", msg);
    }
    Err(e) => { /* other errors */ }
}
```

### `SerializationError`

```rust
match User::update("123", &invalid_json).await {
    Ok(response) => { /* success */ }
    Err(Error::SerializationError(msg)) => {
        eprintln!("Invalid JSON data: {}", msg);
    }
    Err(e) => { /* other errors */ }
}
```

## Performance Considerations

### Batch Operations

For multiple document operations, prefer batch methods:

```rust
// ❌ Inefficient - multiple individual operations
for user in users {
    user.save().await?;
}

// ✅ Efficient - use bulk operations
let mut bulk = BulkOperation::new();
for user in users {
    bulk.index(User::index_name(), &user.id(), serde_json::to_value(user)?);
}
client.bulk(bulk).await?;
```

### Projection with Source Filtering

Use source filtering to retrieve only needed fields:

```rust
let query = Search::new()
    .source(["name", "email"])  // Only these fields
    .query(Query::term("active", true));

let results = User::find(query).await?;
```

### Pagination

For large result sets:

```rust
let page_size = 20;
let page_number = 2;
let from = (page_number - 1) * page_size;

let results = User::find(
    Search::new()
        .from(from)
        .size(page_size)
        .query(Query::match_all())
).await?;
```

## Usage Examples

### Complete CRUD Workflow

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create
    let user = User {
        id: "user123".to_string(),
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
        active: true,
    };
    
    let response = user.save().await?;
    println!("Created user: {}", response.id);
    
    // Read
    let retrieved = User::get("user123").await?;
    println!("Retrieved: {}", retrieved.name);
    
    // Update
    User::update("user123", &json!({"age": 31})).await?;
    
    // Search
    let active_users = User::find(
        Search::new().query(Query::term("active", true))
    ).await?;
    
    println!("Found {} active users", active_users.hits.total.value);
    
    // Delete
    User::delete("user123").await?;
    println!("User deleted");
    
    Ok(())
}
```