use std::{collections::HashMap, option::Option, vec::Vec};

use serde::{Deserialize, Serialize};
use serde_json::Map;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pipeline {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(default, rename = "on_failure", skip_serializing_if = "Vec::is_empty")]
  on_failure: Vec<Processor>,
  #[serde(default, rename = "processors", skip_serializing_if = "Vec::is_empty")]
  processors: Vec<Processor>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  version: Option<u32>,
}

impl Pipeline {
  pub fn new() -> Pipeline {
    Pipeline::default()
  }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Processor {
  #[serde(rename = "key_value")]
  KeyValueProcessor(KeyValueProcessor),
  #[serde(rename = "set_security_user")]
  SetSecurityUserProcessor(SetSecurityUserProcessor),
  #[serde(rename = "join")]
  JoinProcessor(JoinProcessor),
  #[serde(rename = "attachment")]
  AttachmentProcessor(AttachmentProcessor),
  #[serde(rename = "foreach")]
  ForeachProcessor(ForeachProcessor),
  #[serde(rename = "csv")]
  CsvProcessor(CsvProcessor),
  #[serde(rename = "pipeline")]
  PipelineProcessor(PipelineProcessor),
  #[serde(rename = "dissect")]
  DissectProcessor(DissectProcessor),
  #[serde(rename = "user_agent")]
  UserAgentProcessor(UserAgentProcessor),
  #[serde(rename = "remove")]
  RemoveProcessor(RemoveProcessor),
  #[serde(rename = "url_decode")]
  UrlDecodeProcessor(UrlDecodeProcessor),
  #[serde(rename = "split")]
  SplitProcessor(SplitProcessor),
  #[serde(rename = "fail")]
  FailProcessor(FailProcessor),
  #[serde(rename = "sort")]
  SortProcessor(SortProcessor),
  // CircleProcessor(CircleProcessor),
  #[serde(rename = "trim")]
  TrimProcessor(TrimProcessor),
  #[serde(rename = "script")]
  ScriptProcessor(ScriptProcessor),
  #[serde(rename = "json")]
  JsonProcessor(JsonProcessor),
  #[serde(rename = "uppercase")]
  UppercaseProcessor(UppercaseProcessor),
  #[serde(rename = "date")]
  DateProcessor(DateProcessor),
  #[serde(rename = "dot_expander")]
  DotExpanderProcessor(DotExpanderProcessor),
  #[serde(rename = "lowercase")]
  LowercaseProcessor(LowercaseProcessor),
  #[serde(rename = "set")]
  SetProcessor(SetProcessor),
  #[serde(rename = "grok")]
  GrokProcessor(GrokProcessor),
  #[serde(rename = "gsub")]
  GsubProcessor(GsubProcessor),
  #[serde(rename = "convert")]
  ConvertProcessor(ConvertProcessor),
  #[serde(rename = "geo_ip")]
  GeoIpProcessor(GeoIpProcessor),
  #[serde(rename = "bytes")]
  BytesProcessor(BytesProcessor),
  #[serde(rename = "inference")]
  InferenceProcessor(InferenceProcessor),
  #[serde(rename = "rename")]
  RenameProcessor(RenameProcessor),
  #[serde(rename = "append")]
  AppendProcessor(AppendProcessor),
  #[serde(rename = "date_index_name")]
  DateIndexNameProcessor(DateIndexNameProcessor),
  #[serde(rename = "drop")]
  DropProcessor(DropProcessor),
  #[serde(rename = "sparse_encoding")]
  SparseEncodingProcessor(SparseEncodingProcessor),
  #[serde(rename = "text_embedding")]
  TextEmbeddingProcessor(TextEmbeddingProcessor),
  #[serde(rename = "text_image_embedding")]
  TextImageEmbeddingProcessor(TextImageEmbeddingProcessor),
  #[serde(untagged)]
  CustomProcessor(CustomProcessor),
}

impl Default for Processor {
  fn default() -> Self {
    Processor::CustomProcessor(CustomProcessor::default())
  }
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct CustomProcessor(Map<String, serde_json::Value>);

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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
#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ForeachProcessor {
  field: String,
  processor: Box<Processor>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

// #[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct DateProcessor {
  field: String,
  formats: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  locale: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  timezone: Option<String>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct DotExpanderProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  path: Option<String>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct LowercaseProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct SetProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  copy_from: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_empty_value: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  media_type: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  override_field: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  value: Option<serde_json::Value>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GrokProcessor {
  field: String,
  patterns: Vec<String>,
  #[serde(default, skip_serializing_if = "HashMap::is_empty")]
  pattern_definitions: HashMap<String, String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  trace_match: Option<bool>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GsubProcessor {
  field: String,
  pattern: String,
  replacement: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConvertProcessor {
  field: String,
  type_field: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GeoIpProcessor {
  field: String,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  properties: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  database_file: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  first_only: Option<bool>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  ignore_missing: Option<bool>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BytesProcessor {
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct InferenceProcessor {
  model_id: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  target_field: Option<String>,
  #[serde(default, skip_serializing_if = "HashMap::is_empty")]
  field_map: HashMap<String, serde_json::Value>,
  #[serde(default, skip_serializing_if = "HashMap::is_empty")]
  inference_config: HashMap<String, serde_json::Value>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RenameProcessor {
  field: String,
  target_field: String,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct AppendProcessor {
  field: String,
  value: Vec<serde_json::Value>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  allow_duplicates: Option<bool>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct DateIndexNameProcessor {
  field: String,
  date_rounding: String,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  date_formats: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  index_name_format: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  index_name_prefix: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  locale: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  timezone: Option<String>,
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct DropProcessor {
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

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct SparseEncodingProcessor {
  model_id: String,
  field_map: HashMap<String, String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TextEmbeddingProcessor {
  model_id: String,
  field_map: HashMap<String, String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TextImageEmbedding {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  text: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  image: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TextImageEmbeddingProcessor {
  model_id: String,
  embedding: String,
  field_map: TextImageEmbedding,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  tag: Option<String>,
}
#[cfg(test)]
mod tests {

  use std::{default, path::PathBuf};

  use serde::de::DeserializeOwned;

  use super::*;
  fn load_entity<T: DeserializeOwned>(name: &str) -> T {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/ingest/pipeline.{name}.json"));
    let text = std::fs::read_to_string(filename).unwrap();
    serde_json::from_str(&text).unwrap()
  }

  #[test]
  fn test_ml_processors() {
    let decoded: Pipeline = load_entity("ml");
    let expected = Pipeline {
      description: Some("A ml pipeline".to_owned()),
      processors: vec![
        Processor::SparseEncodingProcessor(SparseEncodingProcessor {
          model_id: "aP2Q8ooBpBj3wT4HVS8a".to_owned(),
          field_map: vec![("passage_text".to_owned(), "passage_embedding".to_owned())]
            .into_iter()
            .collect(),
          ..default::Default::default()
        }),
        Processor::TextEmbeddingProcessor(TextEmbeddingProcessor {
          model_id: "bQ1J8ooBpBj3wT4HVUsb".to_owned(),
          field_map: vec![("passage_text".to_owned(), "passage_embedding".to_owned())]
            .into_iter()
            .collect(),
          ..default::Default::default()
        }),
        Processor::TextImageEmbeddingProcessor(TextImageEmbeddingProcessor {
          model_id: "bQ1J8ooBpBj3wT4HVUsb".to_owned(),
          embedding: "vector_embedding".to_owned(),
          field_map: TextImageEmbedding {
            text: Some("image_description".to_owned()),
            image: Some("image_binary".to_owned()),
          },
          ..default::Default::default()
        }),
      ],
      ..Default::default()
    };
    // println!("{}", serde_json::to_string(&expected).unwrap());
    assert_eq!(decoded.description, Some("A ml pipeline".to_owned()));
    assert_eq!(decoded, expected);
  }
}
