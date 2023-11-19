use serde::de::DeserializeOwned;
use serde_json::Value;

use super::{Explanation, NestedIdentity, Source};
use crate::{util::ShouldSkip, InnerHitsResult, Map};

/// Represents a single matched document
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Hit {
  /// Search explanation
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip", rename = "_explanation")]
  pub explanation: Option<Explanation>,

  /// Document index
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip", rename = "_index")]
  pub index: String,

  /// Document ID
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip", rename = "_id")]
  pub id: String,

  /// Document score. [`None`] when documents are implicitly sorted by a
  /// field other than `_score`
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip", rename = "_score")]
  pub score: Option<f32>,

  /// Nested document identity
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip", rename = "_nested")]
  pub nested: Option<NestedIdentity>,

  /// Document source
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip", rename = "_source")]
  pub source: Source,

  /// Highlighted matches
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  pub highlight: Map<String, Vec<String>>,

  /// Inner hits
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  pub inner_hits: Map<String, InnerHitsResult>,

  /// Matched queries
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  pub matched_queries: Vec<String>,

  /// Values document was sorted by
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  pub sort: Vec<Value>,

  /// Field values for the documents. Need to be specified in the request
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  pub fields: Map<String, Value>,
}

impl Hit {
  /// Parses document source into a concrete type
  pub fn source<T>(&self) -> Result<T, serde_json::Error>
  where
    T: DeserializeOwned, {
    self.source.parse()
  }
}
