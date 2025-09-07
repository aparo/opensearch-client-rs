use super::{Hit, TotalHits};
use crate::util::ShouldSkip;

/// Matched hits
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct HitsMetadata {
    /// Total number of matched documents
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub total: Option<TotalHits>,

    /// Maximum document score. [`None`] when documents are implicitly sorted
    /// by a field other than `_score`
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub max_score: Option<f32>,

    /// Matched hits
    #[serde(default = "Vec::new")]
    pub hits: Vec<Hit>,
}
