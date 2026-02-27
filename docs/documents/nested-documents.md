# Nested Documents

OpenSearch supports two ways to model nested data: **objects** (default) and **nested objects**. Understanding the difference is important for correct query behaviour.

## Objects vs Nested Objects

### Object Fields (default)

By default, nested Rust structs are flattened into the parent document. This is efficient but means that object boundaries are lost — queries across an array of objects may match on different array elements.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "blog_posts")]
pub struct BlogPost {
    #[os(id)]
    pub id: String,
    pub title: String,
    pub metadata: PostMetadata,  // Embedded object — flattened in OpenSearch
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMetadata {
    pub word_count: u32,
    pub reading_time_minutes: u8,
    pub featured: bool,
}
```

Query an object field with dot notation:

```rust
let results = BlogPost::find(
    Search::new()
        .query(
            Query::bool()
                .filter([
                    Query::term("metadata.featured", true),
                    Query::range("metadata.word_count").gte(1000),
                ])
        )
).await?;
```

### Nested Objects (explicit mapping required)

For arrays of objects where you need to query multiple fields on the same element, use `"type": "nested"` in the mapping and `Query::nested()` in queries.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "blog_posts")]
pub struct BlogPost {
    #[os(id)]
    pub id: String,
    pub title: String,
    pub comments: Vec<Comment>,  // Will need "nested" mapping
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub author: String,
    pub body: String,
    pub approved: bool,
    pub rating: u8,
}
```

Create the index with the nested mapping:

```rust
client.indices()
    .create("blog_posts")
    .body(serde_json::json!({
        "mappings": {
            "properties": {
                "title": { "type": "text" },
                "comments": {
                    "type": "nested",
                    "properties": {
                        "author":   { "type": "keyword" },
                        "body":     { "type": "text" },
                        "approved": { "type": "boolean" },
                        "rating":   { "type": "byte" }
                    }
                }
            }
        }
    }))
    .send()
    .await?;
```

Query nested objects — each clause applies to the same array element:

```rust
let results = BlogPost::find(
    Search::new()
        .query(
            Query::nested("comments")
                .query(
                    Query::bool()
                        .must(Query::match_("comments.body", "great article"))
                        .filter(Query::term("comments.approved", true))
                )
                .score_mode("avg")
        )
).await?;
```

## Common Patterns

### Optional Nested Object

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "products")]
pub struct Product {
    #[os(id)]
    pub sku: String,
    pub name: String,
    pub dimensions: Option<Dimensions>,  // Optional embedded object
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width_cm: f32,
    pub height_cm: f32,
    pub depth_cm: f32,
    pub weight_kg: f32,
}
```

Query only documents where the optional field exists:

```rust
let has_dimensions = Product::find(
    Search::new()
        .query(Query::exists("dimensions"))
).await?;
```

### Deeply Nested Hierarchy

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "organizations")]
pub struct Organization {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub departments: Vec<Department>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub name: String,
    pub budget_usd: u64,
    pub employees: Vec<Employee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub role: String,
    pub salary_usd: u64,
}
```

For deeply nested queries, each level that requires cross-field accuracy needs `"type": "nested"` in the mapping. Object fields work for simple embedded structs.

### Self-Referential Documents

OpenSearch does not support true recursive nesting. Use separate indices with ID references instead:

```rust
// Parent
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "categories")]
pub struct Category {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,  // Reference to another Category
}

// Child
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "products")]
pub struct Product {
    #[os(id)]
    pub sku: String,
    pub name: String,
    pub category_id: String,        // Reference to Category
}
```

Resolve the relationship in application code by fetching both types.

## Aggregations on Nested Fields

Nested aggregations require a `nested` aggregation wrapper:

```rust
let search = Search::new()
    .size(0)
    .aggregations([
        ("comments_agg",
            Aggregation::nested("comments")
                .aggregations([
                    ("avg_rating", Aggregation::avg("comments.rating")),
                    ("approved_count",
                        Aggregation::filter(Query::term("comments.approved", true))
                    ),
                ])
        ),
    ]);

let results = BlogPost::find(search).await?;
```

## Updating Nested Objects

Because OpenSearch stores nested objects as hidden documents, updating a single nested element requires updating the entire parent document:

```rust
// Fetch, modify in Rust, save back
let mut post = BlogPost::get("post-1").await?;
if let Some(comment) = post.comments.iter_mut().find(|c| c.author == "alice") {
    comment.approved = true;
}
post.save().await?;
```

For frequently updated nested arrays, consider whether a separate index (with a `post_id` foreign key) would be more efficient.
