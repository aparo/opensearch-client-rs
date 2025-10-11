# Document Modeling Examples

This guide provides comprehensive examples of document modeling using the OpenSearch derive macro.

## Blog System Example

A complete blog system with posts, authors, and comments:

```rust
use opensearch_client::{Document, OpenSearch};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "blog_posts")]
pub struct BlogPost {
    #[os(id)]
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,           // Reference to Author
    pub tags: Vec<String>,
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
    pub view_count: u32,
    pub metadata: PostMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMetadata {
    pub word_count: u32,
    pub reading_time_minutes: u32,
    pub featured_image: Option<String>,
    pub seo_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "authors")]
pub struct Author {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub website: Option<String>,
    pub social_links: Vec<String>,
    pub verified: bool,
    pub post_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "comments")]
pub struct Comment {
    #[os(id)]
    pub id: String,
    pub post_id: String,             // Reference to BlogPost
    pub author_name: String,
    pub author_email: String,
    pub content: String,
    pub approved: bool,
    pub created_at: String,
    pub parent_id: Option<String>,   // For threaded comments
}

impl BlogPost {
    pub fn new(title: String, content: String, author_id: String, tags: Vec<String>) -> Self {
        let word_count = content.split_whitespace().count() as u32;
        let reading_time = (word_count / 200).max(1); // Assume 200 WPM reading speed
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            content,
            author_id,
            tags,
            published: false,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            view_count: 0,
            metadata: PostMetadata {
                word_count,
                reading_time_minutes: reading_time,
                featured_image: None,
                seo_description: None,
            },
        }
    }

    pub async fn publish(&mut self) -> Result<(), opensearch_client::Error> {
        self.published = true;
        self.updated_at = Utc::now().to_rfc3339();
        self.save().await.map(|_| ())
    }

    pub async fn increment_views(&mut self) -> Result<(), opensearch_client::Error> {
        self.view_count += 1;
        BlogPost::update(&self.id, &serde_json::json!({
            "view_count": self.view_count
        })).await.map(|_| ())
    }

    pub async fn get_comments(&self) -> Result<Vec<Comment>, opensearch_client::Error> {
        let results = Comment::find(
            opensearch_dsl::Search::new()
                .query(opensearch_dsl::Query::term("post_id", &self.id))
                .sort([("created_at", "desc")])
        ).await?;

        Ok(results.hits.hits.into_iter()
            .filter_map(|hit| hit.source)
            .collect())
    }

    pub async fn get_author(&self) -> Result<Author, opensearch_client::Error> {
        Author::get(&self.author_id).await
    }
}

impl Author {
    pub fn new(name: String, email: String, bio: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            email,
            bio,
            website: None,
            social_links: Vec::new(),
            verified: false,
            post_count: 0,
        }
    }

    pub async fn get_posts(&self) -> Result<Vec<BlogPost>, opensearch_client::Error> {
        let results = BlogPost::find(
            opensearch_dsl::Search::new()
                .query(opensearch_dsl::Query::term("author_id", &self.id))
                .sort([("created_at", "desc")])
        ).await?;

        Ok(results.hits.hits.into_iter()
            .filter_map(|hit| hit.source)
            .collect())
    }

    pub async fn update_post_count(&mut self) -> Result<(), opensearch_client::Error> {
        let count = BlogPost::count(Some(
            opensearch_dsl::Query::term("author_id", &self.id)
        )).await?;
        
        self.post_count = count;
        Author::update(&self.id, &serde_json::json!({
            "post_count": count
        })).await.map(|_| ())
    }
}

impl Comment {
    pub fn new(post_id: String, author_name: String, author_email: String, content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            post_id,
            author_name,
            author_email,
            content,
            approved: false,
            created_at: Utc::now().to_rfc3339(),
            parent_id: None,
        }
    }

    pub fn reply_to(parent: &Comment, author_name: String, author_email: String, content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            post_id: parent.post_id.clone(),
            author_name,
            author_email,
            content,
            approved: false,
            created_at: Utc::now().to_rfc3339(),
            parent_id: Some(parent.id.clone()),
        }
    }

    pub async fn approve(&mut self) -> Result<(), opensearch_client::Error> {
        self.approved = true;
        self.save().await.map(|_| ())
    }
}

// Usage example
async fn blog_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create an author
    let mut author = Author::new(
        "Jane Doe".to_string(),
        "jane@example.com".to_string(),
        "Tech writer and Rust enthusiast".to_string(),
    );
    author.save().await?;

    // Create a blog post
    let mut post = BlogPost::new(
        "Getting Started with OpenSearch".to_string(),
        "OpenSearch is a powerful search engine...".to_string(),
        author.id.clone(),
        vec!["opensearch".to_string(), "rust".to_string()],
    );
    
    // Add metadata
    post.metadata.seo_description = Some("Learn how to use OpenSearch with Rust".to_string());
    post.save().await?;

    // Publish the post
    post.publish().await?;

    // Add a comment
    let comment = Comment::new(
        post.id.clone(),
        "Reader One".to_string(),
        "reader@example.com".to_string(),
        "Great article! Very helpful.".to_string(),
    );
    comment.save().await?;

    // Update author's post count
    author.update_post_count().await?;

    println!("Blog system example completed!");
    Ok(())
}
```

## E-commerce Example

Product catalog with categories, reviews, and inventory:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "products")]
pub struct Product {
    #[os(id)]
    pub sku: String,                 // Use SKU as ID
    pub name: String,
    pub description: String,
    pub category_id: String,
    pub price: f64,
    pub sale_price: Option<f64>,
    pub currency: String,
    pub inventory: Inventory,
    pub specifications: ProductSpecs,
    pub tags: Vec<String>,
    pub images: Vec<String>,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub quantity: u32,
    pub reserved: u32,
    pub available: u32,
    pub reorder_level: u32,
    pub supplier_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSpecs {
    pub weight: Option<f64>,
    pub dimensions: Option<Dimensions>,
    pub material: Option<String>,
    pub color: Option<String>,
    pub size: Option<String>,
    pub custom_attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "categories")]
pub struct Category {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub level: u32,
    pub product_count: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "reviews")]
pub struct Review {
    #[os(id)]
    pub id: String,
    pub product_sku: String,
    pub customer_id: String,
    pub customer_name: String,
    pub rating: u8,                  // 1-5 stars
    pub title: String,
    pub content: String,
    pub verified_purchase: bool,
    pub helpful_votes: u32,
    pub created_at: String,
    pub moderated: bool,
}

impl Product {
    pub fn new(sku: String, name: String, description: String, category_id: String, price: f64) -> Self {
        Self {
            sku: sku.clone(),
            name,
            description,
            category_id,
            price,
            sale_price: None,
            currency: "USD".to_string(),
            inventory: Inventory {
                quantity: 0,
                reserved: 0,
                available: 0,
                reorder_level: 10,
                supplier_id: None,
            },
            specifications: ProductSpecs {
                weight: None,
                dimensions: None,
                material: None,
                color: None,
                size: None,
                custom_attributes: std::collections::HashMap::new(),
            },
            tags: Vec::new(),
            images: Vec::new(),
            active: true,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        }
    }

    pub async fn update_inventory(&mut self, quantity: u32, reserved: u32) -> Result<(), opensearch_client::Error> {
        self.inventory.quantity = quantity;
        self.inventory.reserved = reserved;
        self.inventory.available = quantity.saturating_sub(reserved);
        self.updated_at = Utc::now().to_rfc3339();
        
        Product::update(&self.sku, &serde_json::json!({
            "inventory": self.inventory,
            "updated_at": self.updated_at
        })).await.map(|_| ())
    }

    pub async fn get_reviews(&self) -> Result<Vec<Review>, opensearch_client::Error> {
        let results = Review::find(
            opensearch_dsl::Search::new()
                .query(opensearch_dsl::Query::term("product_sku", &self.sku))
                .sort([("created_at", "desc")])
        ).await?;

        Ok(results.hits.hits.into_iter()
            .filter_map(|hit| hit.source)
            .collect())
    }

    pub async fn get_average_rating(&self) -> Result<f64, opensearch_client::Error> {
        use opensearch_dsl::*;

        let search = Search::new()
            .query(Query::term("product_sku", &self.sku))
            .aggregations([
                ("avg_rating", Aggregation::avg("rating"))
            ])
            .size(0);

        // This would require implementing aggregation result parsing
        // For now, calculate manually
        let reviews = self.get_reviews().await?;
        if reviews.is_empty() {
            return Ok(0.0);
        }

        let total: u32 = reviews.iter().map(|r| r.rating as u32).sum();
        Ok(total as f64 / reviews.len() as f64)
    }
}

// Usage example
async fn ecommerce_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create category
    let category = Category {
        id: "electronics".to_string(),
        name: "Electronics".to_string(),
        slug: "electronics".to_string(),
        description: Some("Electronic devices and gadgets".to_string()),
        parent_id: None,
        level: 1,
        product_count: 0,
        active: true,
    };
    category.save().await?;

    // Create product
    let mut product = Product::new(
        "LAPTOP-123".to_string(),
        "Gaming Laptop".to_string(),
        "High-performance gaming laptop".to_string(),
        category.id.clone(),
        1299.99,
    );

    // Set specifications
    product.specifications.weight = Some(2.5);
    product.specifications.color = Some("Black".to_string());
    product.tags = vec!["gaming".to_string(), "laptop".to_string(), "rgb".to_string()];

    product.save().await?;

    // Update inventory
    product.update_inventory(50, 5).await?;

    // Add review
    let review = Review {
        id: uuid::Uuid::new_v4().to_string(),
        product_sku: product.sku.clone(),
        customer_id: "customer123".to_string(),
        customer_name: "John Gamer".to_string(),
        rating: 5,
        title: "Excellent gaming laptop!".to_string(),
        content: "This laptop handles all my games perfectly.".to_string(),
        verified_purchase: true,
        helpful_votes: 0,
        created_at: Utc::now().to_rfc3339(),
        moderated: true,
    };
    review.save().await?;

    println!("E-commerce example completed!");
    Ok(())
}
```

## Event Logging Example

Time-series data for application monitoring:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "application_logs")]
pub struct LogEntry {
    #[os(id)]
    pub id: String,
    pub timestamp: String,
    pub level: LogLevel,
    pub service: String,
    pub message: String,
    pub context: LogContext,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogContext {
    pub file: Option<String>,
    pub line: Option<u32>,
    pub function: Option<String>,
    pub module: String,
    pub thread: Option<String>,
    pub additional_data: std::collections::HashMap<String, serde_json::Value>,
}

impl LogEntry {
    pub fn new(level: LogLevel, service: String, message: String) -> Self {
        Self {
            id: format!("{}_{}", 
                Utc::now().timestamp_nanos(),
                uuid::Uuid::new_v4().simple()
            ),
            timestamp: Utc::now().to_rfc3339(),
            level,
            service,
            message,
            context: LogContext {
                file: None,
                line: None,
                function: None,
                module: "unknown".to_string(),
                thread: None,
                additional_data: std::collections::HashMap::new(),
            },
            trace_id: None,
            span_id: None,
            user_id: None,
            session_id: None,
        }
    }

    pub async fn search_by_service(service: &str, hours: u32) -> Result<Vec<LogEntry>, opensearch_client::Error> {
        let since = Utc::now() - chrono::Duration::hours(hours as i64);
        
        let results = LogEntry::find(
            opensearch_dsl::Search::new()
                .query(
                    opensearch_dsl::Query::bool()
                        .must(opensearch_dsl::Query::term("service", service))
                        .filter(opensearch_dsl::Query::range("timestamp").gte(since.to_rfc3339()))
                )
                .sort([("timestamp", "desc")])
                .size(1000)
        ).await?;

        Ok(results.hits.hits.into_iter()
            .filter_map(|hit| hit.source)
            .collect())
    }
}
```

These examples demonstrate:
- Complex nested structures
- Business logic implementation
- Relationships between documents
- Time-series data patterns
- Search and aggregation patterns
- Real-world data modeling scenarios