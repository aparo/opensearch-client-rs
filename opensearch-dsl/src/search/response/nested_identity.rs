use crate::util::ShouldSkip;

/// Nested document metadata
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct NestedIdentity {
    /// Field
    pub field: String,

    /// Offset
    pub offset: u64,

    /// Nested document metadata
    #[serde(
        default,
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_nested"
    )]
    pub nested: Option<Box<NestedIdentity>>,
}
