/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * k-NN vector method configuration.
 */

use serde::{Deserialize, Serialize};

use super::enums::{KnnEngine, KnnSpaceType};

/// Algorithm method configuration for a `knn_vector` field.
///
/// # Example (HNSW with Faiss)
/// ```json
/// {
///   "name": "hnsw",
///   "engine": "faiss",
///   "space_type": "l2",
///   "parameters": {
///     "ef_construction": 100,
///     "m": 16
///   }
/// }
/// ```
///
/// # Example (IVF with product quantization)
/// ```json
/// {
///   "name": "ivf",
///   "engine": "faiss",
///   "space_type": "l2",
///   "parameters": {
///     "nlist": 4,
///     "encoder": {"name": "pq", "parameters": {"m": 8, "code_size": 8}}
///   }
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KnnVectorMethod {
    /// Algorithm name: `hnsw`, `ivf`, `flat`, `disk_ann`.
    #[serde(rename = "name")]
    pub name: String,
    /// Backend engine (default faiss).
    #[serde(rename = "engine", default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<KnnEngine>,
    /// Distance function (overrides field-level space_type when set).
    #[serde(rename = "space_type", default, skip_serializing_if = "Option::is_none")]
    pub space_type: Option<KnnSpaceType>,
    /// Algorithm-specific parameters (e.g. ef_construction, m, nlist, nprobes).
    #[serde(rename = "parameters", default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl KnnVectorMethod {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            engine: None,
            space_type: None,
            parameters: None,
        }
    }

    /// Create an HNSW method with the given engine.
    pub fn hnsw(engine: KnnEngine) -> Self {
        Self {
            name: "hnsw".into(),
            engine: Some(engine),
            space_type: None,
            parameters: None,
        }
    }

    /// Create an IVF method (faiss only).
    pub fn ivf() -> Self {
        Self {
            name: "ivf".into(),
            engine: Some(KnnEngine::Faiss),
            space_type: None,
            parameters: None,
        }
    }

    /// Create a flat (exact search) method.
    pub fn flat(engine: KnnEngine) -> Self {
        Self {
            name: "flat".into(),
            engine: Some(engine),
            space_type: None,
            parameters: None,
        }
    }

    pub fn with_space_type(mut self, space_type: KnnSpaceType) -> Self {
        self.space_type = Some(space_type);
        self
    }

    pub fn with_parameters(mut self, parameters: serde_json::Value) -> Self {
        self.parameters = Some(parameters);
        self
    }
}
