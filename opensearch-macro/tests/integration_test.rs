use opensearch_client::Document;
use opensearch_macro::OpenSearch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "test_users")]
pub struct User {
    #[os(id)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub active: bool,
    pub profile: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, OpenSearch)]
#[os(index = "user_profiles")]
pub struct UserProfile {
    #[os(id)]
    pub id: String,
    pub bio: String,
    pub website: Option<String>,
    pub tags: Vec<String>,
}

#[test]
fn test_document_trait_implementation() {
    // Test that the Document trait is properly implemented
    assert_eq!(User::index_name(), "test_users");
    assert_eq!(UserProfile::index_name(), "user_profiles");

    // Test that columns are generated
    let user_columns = User::columns();
    assert!(!user_columns.is_empty());

    let profile_columns = UserProfile::columns();
    assert!(!profile_columns.is_empty());

    // Test that ID method works
    let user = User {
        id: "user123".to_string(),
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
        active: true,
        profile: UserProfile {
            id: "profile123".to_string(),
            bio: "Software developer".to_string(),
            website: Some("https://johndoe.dev".to_string()),
            tags: vec!["rust".to_string(), "programming".to_string()],
        },
    };

    assert_eq!(user.id(), "user123");
    assert_eq!(user.profile.id(), "profile123");
}

#[test]
fn test_nested_fields() {
    let user_columns = User::columns();

    // Find the profile field
    let profile_field = user_columns
        .iter()
        .find(|field| field.name == "profile")
        .expect("Profile field should exist");

    // Check that it has the correct type
    assert_eq!(profile_field.os_type, "object");
    assert_eq!(profile_field.field_type, "object");

    // Check that it has sub-fields from the UserProfile Document
    assert!(
        !profile_field.sub_fields.is_empty(),
        "Profile field should have sub-fields"
    );

    // Verify some of the sub-fields
    let sub_field_names: Vec<&str> = profile_field
        .sub_fields
        .iter()
        .map(|f| f.name.as_str())
        .collect();

    assert!(sub_field_names.contains(&"id"));
    assert!(sub_field_names.contains(&"bio"));
    assert!(sub_field_names.contains(&"website"));
    assert!(sub_field_names.contains(&"tags"));
}

#[test]
fn test_field_types() {
    let user_columns = User::columns();

    // Check string field
    let name_field = user_columns
        .iter()
        .find(|field| field.name == "name")
        .expect("Name field should exist");
    assert_eq!(name_field.os_type, "text");
    assert_eq!(name_field.field_type, "string");
    assert!(!name_field.aggregatable);
    assert!(name_field.searchable);

    // Check number field
    let age_field = user_columns
        .iter()
        .find(|field| field.name == "age")
        .expect("Age field should exist");
    assert_eq!(age_field.os_type, "long");
    assert_eq!(age_field.field_type, "number");
    assert!(age_field.aggregatable);
    assert!(age_field.searchable);

    // Check boolean field
    let active_field = user_columns
        .iter()
        .find(|field| field.name == "active")
        .expect("Active field should exist");
    assert_eq!(active_field.os_type, "boolean");
    assert_eq!(active_field.field_type, "boolean");
    assert!(active_field.aggregatable);
    assert!(active_field.searchable);
}
