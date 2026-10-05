/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Shared enums for OpenSearch mapping parameters.
 */

use serde::{Deserialize, Serialize};

/// Controls how new fields are dynamically handled in mappings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DynamicMapping {
    /// Auto-map new fields (default).
    #[default]
    #[serde(rename = "true")]
    True,
    /// Store in _source but don't index unknown fields.
    #[serde(rename = "false")]
    False,
    /// Reject documents with unknown fields.
    Strict,
    /// Strict but dynamic templates still apply.
    StrictAllowTemplates,
    /// Like false but dynamic templates still apply.
    FalseAllowTemplates,
    /// Map new fields as runtime fields.
    Runtime,
}

/// Term vector storage options for text fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TermVector {
    /// No term vectors stored (default).
    #[default]
    No,
    /// Store term vectors.
    Yes,
    /// Store term vectors with positions.
    WithPositions,
    /// Store term vectors with character offsets.
    WithOffsets,
    /// Store term vectors with positions and offsets.
    WithPositionsOffsets,
    /// Store term vectors with positions and payloads.
    WithPositionsPayloads,
    /// Store term vectors with positions, offsets, and payloads.
    WithPositionsOffsetsPayloads,
}

/// Controls what information is stored in the inverted index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum IndexOptions {
    /// Store only document numbers.
    Docs,
    /// Store document numbers and term frequencies.
    Freqs,
    /// Store document numbers, frequencies, and positions (default for text).
    #[default]
    Positions,
    /// Store document numbers, frequencies, positions, and character offsets.
    Offsets,
}

/// Relevance scoring algorithm.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Similarity {
    /// BM25 probabilistic similarity (default).
    #[default]
    #[serde(rename = "BM25")]
    Bm25,
    /// Boolean similarity (1 if matched, 0 if not).
    Boolean,
}

/// Pattern type for dynamic template matching.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MatchPattern {
    /// Simple wildcard matching (default).
    #[default]
    Simple,
    /// Java regex matching.
    Regex,
}

/// k-NN index engine.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum KnnEngine {
    /// NMSLIB engine (deprecated).
    Nmslib,
    /// Facebook AI Similarity Search engine.
    #[default]
    Faiss,
    /// Apache Lucene engine.
    Lucene,
}

/// Vector space type (distance function) for k-NN search.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum KnnSpaceType {
    /// L1 / Manhattan distance.
    L1,
    /// L2 / Euclidean squared distance (default).
    #[default]
    L2,
    /// Chebyshev / L-infinity distance.
    Linf,
    /// Cosine similarity.
    Cosinesimil,
    /// Inner product / dot product.
    Innerproduct,
    /// Hamming distance (for binary/byte vectors).
    Hamming,
    /// Hamming distance on long/binary values (exact only).
    Hammingbit,
}

/// Data type for k-NN vector elements.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum KnnDataType {
    /// 32-bit float (default).
    #[default]
    Float,
    /// Signed 8-bit integer.
    Byte,
    /// 16-bit float.
    HalfFloat,
    /// Binary (each dimension is a single bit, stored as int8).
    Binary,
}

/// Workload mode for k-NN vector fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum KnnMode {
    /// In-memory mode with lowest latency (default).
    #[default]
    InMemory,
    /// On-disk mode using quantization for lowest cost.
    OnDisk,
}

/// Compression level for k-NN vector quantization.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum KnnCompressionLevel {
    /// No quantization (1x memory vs base).
    #[serde(rename = "1x")]
    OneX,
    /// 16-bit quantization (2x memory reduction, faiss only).
    #[serde(rename = "2x")]
    TwoX,
    /// 7-bit quantization (4x memory reduction, lucene only).
    #[serde(rename = "4x")]
    FourX,
    /// 4-bit quantization (8x memory reduction).
    #[serde(rename = "8x")]
    EightX,
    /// 2-bit quantization (16x memory reduction).
    #[serde(rename = "16x")]
    SixteenX,
    /// 1-bit quantization (32x memory reduction).
    #[serde(rename = "32x")]
    ThirtyTwoX,
}

/// Polygon orientation for geo/cartesian shape fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum Orientation {
    /// Right-hand rule / counterclockwise exterior (default).
    #[default]
    #[serde(rename = "RIGHT")]
    Right,
    /// Left-hand rule / clockwise exterior.
    #[serde(rename = "LEFT")]
    Left,
    /// Counterclockwise (alias for Right).
    #[serde(rename = "ccw")]
    Counterclockwise,
    /// Clockwise (alias for Left).
    #[serde(rename = "cw")]
    Clockwise,
}

/// Shingle size for search-as-you-type fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum MaxShingleSize {
    /// Maximum shingle size of 2.
    #[serde(rename = "2")]
    Two,
    /// Maximum shingle size of 3 (default).
    #[default]
    #[serde(rename = "3")]
    Three,
    /// Maximum shingle size of 4.
    #[serde(rename = "4")]
    Four,
}
