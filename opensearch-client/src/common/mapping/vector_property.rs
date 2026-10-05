/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Vector and similarity field type property structs.
 */

use serde::{Deserialize, Serialize};

use super::enums::{KnnCompressionLevel, KnnDataType, KnnEngine, KnnMode, KnnSpaceType};
use super::knn_vector_method::KnnVectorMethod;

/// `knn_vector` field: dense vector for k-NN / ANN similarity search.
///
/// Supports multiple engines (Lucene, Faiss, NMSLIB, JVector), methods (HNSW, IVF, flat,
/// disk_ann), and data types (float, byte, half_float, binary).
///
/// # Example (HNSW with Faiss)
/// ```json
/// {
///   "type": "knn_vector",
///   "dimension": 128,
///   "space_type": "l2",
///   "method": {
///     "name": "hnsw",
///     "engine": "faiss",
///     "parameters": {"ef_construction": 100, "m": 16}
///   }
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KnnVectorProperty {
    /// Number of vector dimensions (required, range 1–16000).
    #[serde(rename = "dimension")]
    pub dimension: u32,
    /// Element data type (default float).
    #[serde(rename = "data_type", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<KnnDataType>,
    /// Distance function (default l2). Overridden by method.space_type if method is set.
    #[serde(rename = "space_type", default, skip_serializing_if = "Option::is_none")]
    pub space_type: Option<KnnSpaceType>,
    /// Workload mode: in_memory (lowest latency) or on_disk (lowest cost).
    #[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<KnnMode>,
    /// Quantization compression level.
    #[serde(rename = "compression_level", default, skip_serializing_if = "Option::is_none")]
    pub compression_level: Option<KnnCompressionLevel>,
    /// Algorithm and engine configuration.
    #[serde(rename = "method", default, skip_serializing_if = "Option::is_none")]
    pub method: Option<KnnVectorMethod>,
    /// Pre-trained model ID for embedding generation.
    #[serde(rename = "model_id", default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl KnnVectorProperty {
    pub fn new(dimension: u32) -> Self {
        Self {
            dimension,
            ..Default::default()
        }
    }

    pub fn with_space_type(mut self, space_type: KnnSpaceType) -> Self {
        self.space_type = Some(space_type);
        self
    }

    pub fn with_method(mut self, method: KnnVectorMethod) -> Self {
        self.method = Some(method);
        self
    }
}

/// `rank_feature` field: single positive float that boosts or penalizes relevance scores.
///
/// Queryable only via the `rank_feature` query. No aggregations or sorting.
/// Top 9 significant bits of precision (~0.4% relative error).
///
/// # Example
/// ```json
/// {
///   "type": "rank_feature",
///   "positive_score_impact": false
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RankFeatureProperty {
    /// When false, higher values decrease the score instead of increasing it (default true).
    #[serde(rename = "positive_score_impact", default, skip_serializing_if = "Option::is_none")]
    pub positive_score_impact: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl RankFeatureProperty {
    pub fn new() -> Self {
        Self::default()
    }
}

/// `rank_features` field: hashmap of feature name → positive float.
///
/// Used for sparse feature sets; accessed as `field.feature_name` in rank_feature queries.
/// No aggregations or sorting.
///
/// # Example
/// ```json
/// {
///   "type": "rank_features"
/// }
/// ```
/// Document value: `{"tf": 12.5, "idf": 0.9}`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RankFeaturesProperty {
    /// When false, higher values decrease scores (applied to all features, default true).
    #[serde(rename = "positive_score_impact", default, skip_serializing_if = "Option::is_none")]
    pub positive_score_impact: Option<bool>,
    /// Arbitrary metadata stored with the mapping.
    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl RankFeaturesProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
