use serde_json::Value;

use crate::{util::ShouldSkip, Map};

/// Error cause
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ErrorCause {
    /// Deeper error cause
    pub caused_by: Option<Box<ErrorCause>>,

    /// Error cause reason
    pub reason: Option<String>,

    /// Root error cause
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub root_cause: Vec<ErrorCause>,

    /// Exception stack trace
    pub stack_trace: Option<String>,

    /// Suppressed error causes
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub suppressed: Vec<ErrorCause>,

    /// Type of error cause
    #[serde(rename = "type")]
    pub ty: Option<String>,

    /// Additional fields that are not part of the strongly typed error cause
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip", flatten)]
    pub additional_details: Map<String, Value>,
}
