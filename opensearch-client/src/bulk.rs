use std::collections::HashMap;

use opensearch_dsl::ShardStatistics;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateActionBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_as_upsert: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,
}

/// Represents the body of an update action.
///
/// This struct provides methods for creating different types of update actions.
/// An update action can be created with a document, a script, or a script with
/// parameters.
///
/// # Examples
///
/// Creating an update action with a document:
///
/// ```
/// use serde_json::json;
/// use opensearch_client::types::bulk::UpdateActionBody;
///
/// let doc = json!({
///     "name": "John Doe",
///     "age": 30,
/// });
///
/// let update_action = UpdateActionBody::new(doc);
/// ```
///
/// Creating an update action with a script:
///
/// ```
/// use opensearch_client::types::bulk::UpdateActionBody;
///
/// let script = r#"ctx._source.age += params.age"#;
///
/// let update_action = UpdateActionBody::from_script(script);
/// ```
///
/// Creating an update action with a script and parameters:
///
/// ```
/// use serde_json::json;
/// use opensearch_client::types::bulk::UpdateActionBody;
///
/// let script = r#"ctx._source.age += params.age"#;
/// let params = json!({
///     "age": 5,
/// });
///
/// let update_action = UpdateActionBody::from_script_parameters(script, params);
/// ```
///
/// Creating an update action with a pre-defined script:
///
/// ```
/// use opensearch_client::types::bulk::{UpdateActionBody, Script};
///
/// let script = Script {
///     source: r#"ctx._source.age += params.age"#.to_string(),
///     params: Some(json!({
///         "age": 5,
///     })),
/// };
///
/// let update_action = UpdateActionBody::with_script(script);
/// ```
impl UpdateActionBody {
    /// Creates a new update action with a document.
    ///
    /// # Arguments
    ///
    /// * `doc` - The document to be updated.
    ///
    /// # Returns
    ///
    /// The update action body.
    pub fn new(doc: Value) -> Self {
        Self {
            doc: Some(doc),
            doc_as_upsert: None,
            script: None,
        }
    }

    /// Creates a new update action with a script.
    ///
    /// # Arguments
    ///
    /// * `script` - The script to be executed for the update action.
    ///
    /// # Returns
    ///
    /// The update action body.
    pub fn from_script(script: &str) -> Self {
        Self {
            doc: None,
            doc_as_upsert: None,
            script: Some(Script {
                source: script.to_string(),
                params: None,
            }),
        }
    }

    /// Creates a new update action with a script and parameters.
    ///
    /// # Arguments
    ///
    /// * `script` - The script to be executed for the update action.
    /// * `params` - The parameters to be passed to the script.
    ///
    /// # Returns
    ///
    /// The update action body.
    pub fn from_script_parameters(script: &str, params: serde_json::Value) -> Self {
        Self {
            doc: None,
            doc_as_upsert: None,
            script: Some(Script {
                source: script.to_string(),
                params: Some(params),
            }),
        }
    }

    /// Creates a new update action with a pre-defined script.
    ///
    /// # Arguments
    ///
    /// * `script` - The pre-defined script to be executed for the update action.
    ///
    /// # Returns
    ///
    /// The update action body.
    pub fn with_script(script: Script) -> Self {
        Self {
            doc: None,
            doc_as_upsert: None,
            script: Some(script),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexResponse {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: i64,
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<ShardStatistics>,
    #[serde(rename = "_seq_no", default)]
    pub seq_no: i64,
    #[serde(rename = "_primary_term", default)]
    pub primary_term: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BulkAction {
    #[serde(rename = "index")]
    Index(IndexAction),
    #[serde(rename = "create")]
    Create(CreateAction),
    #[serde(rename = "update")]
    Update(UpdateAction),
    #[serde(rename = "delete")]
    Delete(DeleteAction),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IndexAction {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateAction {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateAction {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteAction {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BulkResponse {
    pub items: Vec<HashMap<String, BulkItemResponse>>,
    pub took: u64,
    pub errors: bool,
    #[serde(
        rename = "ingest_took",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ingest_took: Option<u32>,
}

impl BulkResponse {
    pub fn is_ok(&self) -> bool {
        !self.errors
    }

    pub fn count_errors(&self) -> usize {
        self.items
            .iter()
            .filter(|i| i.values().any(|i| i.status >= 400))
            .count()
    }

    pub fn count_ok(&self) -> usize {
        self.items
            .iter()
            .filter(|i| i.values().all(|i| i.status < 400))
            .count()
    }

    pub fn count_create_errors(&self) -> usize {
        self.items
            .iter()
            .filter(|i| i.values().any(|i| i.status == 409))
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkError {
    #[serde(rename = "_index")]
    pub index: Option<String>,
    #[serde(default)]
    pub index_uuid: Option<String>,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub shard: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BulkItemResponse {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(default)]
    pub status: i16,
    #[serde(default)]
    pub found: Option<bool>,
    #[serde(rename = "_shards", default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<ShardStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<BulkError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<crate::common::ErrorCause>,
    #[serde(
        default,
        rename = "_primary_term",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_term: Option<i32>,
    #[serde(default, rename = "_seq_no", skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Script {
    #[serde(default)]
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}
