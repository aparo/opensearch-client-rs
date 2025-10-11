/// This file contains examples from the README documentation
/// to ensure they compile correctly.
use opensearch_client::{Document, OpenSearch};
use serde::{Deserialize, Serialize};

// Example from the README
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub active: bool,
    pub profile: UserProfile, // Nested document
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "user_profiles")]
pub struct UserProfile {
    #[os(id)]
    pub id: String,
    pub bio: String,
    pub website: Option<String>,
    pub location: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "addresses")]
pub struct Address {
    pub street: String,
    pub city: String,
    pub country: String,
    pub zipcode: u32,
}

// Blog example from README
#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "blog_posts")]
pub struct BlogPost {
    #[os(id)]
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: Author,
    pub tags: Vec<String>,
    pub published: bool,
    pub created_at: String, // ISO 8601 datetime
    pub view_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "authors")]
pub struct Author {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub email: String,
    // Using a simple string instead of complex nested type for this example
    pub bio: String,
}

// AuthorProfile as a regular struct (not a Document)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorProfile {
    pub bio: String,
    pub website: Option<String>,
    pub social_links: Vec<String>,
}

fn main() {
    println!("Documentation examples compile successfully!");

    // Test field introspection
    println!("User index: {}", User::index_name());
    println!("BlogPost fields:");
    for field in BlogPost::columns() {
        println!(
            "  - {}: {} ({})",
            field.name, field.field_type, field.os_type
        );
    }

    // Test instance creation
    let user = User {
        id: "test123".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        age: 25,
        active: true,
        profile: UserProfile {
            id: "profile123".to_string(),
            bio: "Test bio".to_string(),
            website: Some("https://example.com".to_string()),
            location: Address {
                street: "123 Test St".to_string(),
                city: "Test City".to_string(),
                country: "Test Country".to_string(),
                zipcode: 12345,
            },
        },
        tags: vec!["test".to_string()],
    };

    println!("Created user with ID: {}", user.id());
}
