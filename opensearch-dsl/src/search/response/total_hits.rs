use super::TotalHitsRelation;

/// Total number of matched documents
#[derive(Default, Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct TotalHits {
    /// Number of total documents
    pub value: u64,

    /// Relation to total number of matched documents
    pub relation: TotalHitsRelation,
}

impl TotalHits {
    /// Create default Total instance
    pub fn new(value: Option<u64>) -> Self {
        Self {
            value: value.unwrap_or(0),
            relation: TotalHitsRelation::Equal,
        }
    }
}
