use std::{option::Option, vec::Vec};

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Pipeline {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(default, rename = "on_failure", skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, rename = "on_failure", skip_serializing_if = "Vec::is_empty")]
  processors: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  version: Option<u32>,
}

impl Pipeline {
  pub fn new() -> Pipeline {
    Pipeline::default()
  }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Processor {
  KeyValueProcessor(KeyValueProcessor),
  SetSecurityUserProcessor(SetSecurityUserProcessor),
  JoinProcessor(JoinProcessor),
  AttachmentProcessor(AttachmentProcessor),
}

#[derive(Serialize, Deserialize)]
pub struct KeyValueProcessor {
  field: String,
  field_split: String,
  #[serde(rename = "value_split")]
  value_split: String,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  exclude_keys: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  include_keys: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  prefix: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  strip_brackets: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  trim_key: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  trim_value: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetSecurityUserProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  properties: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JoinProcessor {
  field: String,
  separator: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachmentProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  remove_binary: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  indexed_chars: Option<u64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  indexed_chars_field: Option<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  properties: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  resource_name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForeachProcessor {
  field: String,
  processor: Processor,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvProcessor {
  field: String,
  target_fields: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  empty_value: Option<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  quote: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  separator: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  trim: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineProcessor {
  name: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing_pipeline: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DissectProcessor {
  field: String,
  pattern: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  append_separator: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserAgentProcessor {
  field: String,
  // #[serde(default, skip_serializing_if = "Vec::is_empty")]
  // options: Vec<UserAgentProperty>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  regex_file: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveProcessor {
  field: Vec<String>,
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlDecodeProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SplitProcessor {
  field: String,
  separator: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  preserve_trailing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FailProcessor {
  message: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SortProcessor {
  field: String,
  // #[serde(default, skip_serializing_if = "Option::is_none")]
  // order: Option<SortOrder>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub struct CircleProcessor {
//   field: String,
//   error_distance: f64,
//   shape_type: ShapeType,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   target_field: Option<String>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   ignore_missing: Option<bool>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   description: Option<String>,
//   #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
//   if_field: Option<String>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   ignore_failure: Option<bool>,
//   #[serde(default, skip_serializing_if = "Vec::is_empty")]
//   on_failure: Vec<Processor>,
//   #[serde(default, skip_serializing_if = "Option::is_none")]
//   tag: Option<String>,
// }

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TrimProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScriptProcessor {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  id: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  lang: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  params: Option<std::collections::HashMap<String, serde_json::Value>>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  source: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  add_to_root: Option<bool>,
  // #[serde(default, skip_serializing_if = "Option::is_none")]
  // add_to_root_conflict_strategy: Option<JsonProcessorConflictStrategy>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  allow_duplicate_keys: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UppercaseProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
  if_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_failure: Option<bool>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}
