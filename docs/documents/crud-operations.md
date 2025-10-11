# CRUD Operations

The Document trait provides a complete set of CRUD (Create, Read, Update, Delete) operations for working with OpenSearch documents.

## Create/Update Operations

### Save Document

The `save()` method creates a new document or updates an existing one:

```rust
let user = User {
    id: "user123".to_string(),
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
    age: 30,
    active: true,
    // ... other fields
};

// Save to OpenSearch
let response = user.save().await?;
println!("Saved user with ID: {}", response.id);
```

### Bulk Operations

For creating multiple documents efficiently:

```rust
use opensearch_client::bulk::*;

let mut bulk = BulkOperation::new();
bulk.index("users", "1", json!({"name": "Alice", "age": 25}));
bulk.index("users", "2", json!({"name": "Bob", "age": 30}));
bulk.index("users", "3", json!({"name": "Charlie", "age": 35}));

let response = client.bulk(bulk).await?;
```

## Read Operations

### Get by ID

Retrieve a document by its unique identifier:

```rust
// Get a user by ID
let user = User::get("user123").await?;
println!("User name: {}", user.name);

// Handle not found case
match User::get("nonexistent").await {
    Ok(user) => println!("Found: {}", user.name),
    Err(Error::DocumentNotFoundError(index, id)) => {
        println!("User {} not found in index {}", id, index);
    }
    Err(e) => return Err(e),
}
```

### Find One Document

Find a single document matching specific criteria:

```rust
let maybe_user = User::find_one(
    Search::new().query(Query::term("email", "john@example.com"))
).await?;

if let Some(user) = maybe_user {
    println!("Found user: {}", user.name);
} else {
    println!("User not found");
}
```

### Find All Documents

Retrieve multiple documents with optional limits:

```rust
// Get all users (with limit)
let all_users = User::find_all(Some(100)).await?;
for hit in all_users.hits.hits {
    if let Some(user) = hit.source {
        println!("User: {}", user.name);
    }
}

// Get all users without limit (be careful with large datasets!)
let all_users = User::find_all(None).await?;
```

## Update Operations

### Partial Updates

Update specific fields without replacing the entire document:

```rust
// Update specific fields
User::update("user123", &json!({
    "age": 31,
    "active": false
})).await?;

// Update nested fields
User::update("user123", &json!({
    "profile.bio": "Updated bio",
    "tags": ["rust", "developer", "updated"]
})).await?;
```

### Refresh Instance

Reload a document instance with the latest data from OpenSearch:

```rust
let mut user = User::get("user123").await?;
println!("Current age: {}", user.age);

// Someone else updates the user...
// Refresh to get latest data
user.refresh().await?;
println!("Updated age: {}", user.age);
```

## Delete Operations

### Delete by ID

Remove a document from the index:

```rust
let delete_response = User::delete("user123").await?;
println!("Deleted: {}", delete_response.result);
```

### Conditional Deletion

Delete documents matching specific criteria:

```rust
// This requires using the lower-level client
let query = Query::bool()
    .must(Query::term("active", false))
    .must(Query::range("last_login").lt("2023-01-01"));

let response = client
    .delete_by_query()
    .index(User::index_name())
    .query(query)
    .await?;

println!("Deleted {} documents", response.deleted);
```

## Error Handling

All CRUD operations return `Result<T, Error>`. Common error patterns:

```rust
use opensearch_client::Error;

match User::get("user123").await {
    Ok(user) => {
        // Document found
        println!("User: {}", user.name);
    }
    Err(Error::DocumentNotFoundError(index, id)) => {
        // Document doesn't exist
        println!("User {} not found in {}", id, index);
    }
    Err(Error::ConnectionError(msg)) => {
        // Network or connection issue
        eprintln!("Connection error: {}", msg);
    }
    Err(e) => {
        // Other errors
        eprintln!("Error: {:?}", e);
    }
}
```

## Performance Tips

### Batch Operations

Use bulk operations for multiple documents:

```rust
// Instead of multiple individual saves
for user in users {
    user.save().await?;  // ❌ Slow
}

// Use bulk operations
let mut bulk = BulkOperation::new();
for user in users {
    bulk.index(User::index_name(), &user.id(), serde_json::to_value(user)?);
}
let response = client.bulk(bulk).await?;  // ✅ Fast
```

### Projection

Use source filtering to retrieve only needed fields:

```rust
let query = Search::new()
    .source(["name", "email"])  // Only fetch specific fields
    .query(Query::term("active", true));

let results = User::find(query).await?;
```

### Pagination

For large result sets, use pagination:

```rust
let page_size = 20;
let mut from = 0;

loop {
    let query = Search::new()
        .from(from)
        .size(page_size)
        .query(Query::match_all());
    
    let results = User::find(query).await?;
    
    if results.hits.hits.is_empty() {
        break;  // No more results
    }
    
    // Process this page
    for hit in results.hits.hits {
        if let Some(user) = hit.source {
            println!("User: {}", user.name);
        }
    }
    
    from += page_size;
}
```