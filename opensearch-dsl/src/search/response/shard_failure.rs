use super::ErrorCause;
use crate::util::ShouldSkip;

/// Shard failure details
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ShardFailure {
  /// Index name
  pub index: Option<String>,

  /// Node name
  pub node: Option<String>,

  /// Status
  pub status: Option<String>,

  /// Shard
  pub shard: Option<u32>,

  /// Reason
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  pub reason: Option<ErrorCause>,
}
