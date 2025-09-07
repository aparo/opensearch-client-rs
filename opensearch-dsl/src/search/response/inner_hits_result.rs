use super::HitsMetadata;

/// Represents inner hits
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InnerHitsResult {
    /// The actual inner hits
    pub hits: HitsMetadata,
}
