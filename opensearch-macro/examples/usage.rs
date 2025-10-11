use opensearch_macro::OpenSearch;
use opensearch_client::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "addresses")]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zipcode: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "users")]
pub struct User {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub active: bool,
    pub address: Address,  // Nested document
    pub tags: Vec<String>,
}

fn main() {
    println!("User index name: {}", User::index_name());
    
    // Create a sample user to demonstrate the id() method
    let user = User {
        id: "123".to_string(),
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
        active: true,
        address: Address {
            street: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            zipcode: 12345,
        },
        tags: vec!["developer".to_string(), "rust".to_string()],
    };
    
    println!("User ID: {}", user.id());
    
    println!("User fields:");
    for field in User::columns() {
        println!("  - {}: {}", field.name, field.field_type);
    }
    
    println!("\nAddress fields:");
    for field in Address::columns() {
        println!("  - {}: {}", field.name, field.field_type);
    }
}