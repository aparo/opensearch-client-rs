use std::{collections::HashMap, option::Option, vec::Vec};

use serde::{Deserialize, Serialize};
use serde_json::Map;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
    #[serde(rename = "urldecode")]
    UrlDecodeProcessor(UrlDecodeProcessor),
    #[serde(rename = "split")]
    SplitProcessor(SplitProcessor),
    #[serde(rename = "fail")]
    FailProcessor(FailProcessor),
    #[serde(rename = "sort")]
    SortProcessor(SortProcessor),
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
    // New processors
    #[serde(rename = "community_id")]
    CommunityIdProcessor(CommunityIdProcessor),
    #[serde(rename = "copy")]
    CopyProcessor(CopyProcessor),
    #[serde(rename = "fingerprint")]
    FingerprintProcessor(FingerprintProcessor),
    #[serde(rename = "html_strip")]
    HtmlStripProcessor(HtmlStripProcessor),
    #[serde(rename = "ip2geo")]
    Ip2GeoProcessor(Ip2GeoProcessor),
    #[serde(rename = "remove_by_pattern")]
    RemoveByPatternProcessor(RemoveByPatternProcessor),
    #[serde(rename = "text_chunking")]
    TextChunkingProcessor(TextChunkingProcessor),
    #[serde(rename = "ml_inference")]
    MlInferenceProcessor(MlInferenceProcessor),
    #[serde(untagged)]
    CustomProcessor(CustomProcessor),
}

impl Default for Processor {
    fn default() -> Self {
        Processor::CustomProcessor(CustomProcessor::default())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct CustomProcessor(Map<String, serde_json::Value>);

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct UserAgentProcessor {
    field: String,
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct SortProcessor {
    field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    order: Option<String>,
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct JsonProcessor {
    field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    add_to_root: Option<bool>,
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConvertProcessor {
    field: String,
    #[serde(rename = "type")]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct SparseEncodingProcessor {
    model_id: String,
    field_map: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TextEmbeddingProcessor {
    model_id: String,
    field_map: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TextImageEmbedding {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    image: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TextImageEmbeddingProcessor {
    model_id: String,
    embedding: String,
    field_map: TextImageEmbedding,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

/// Generates the Community ID flow hash for network flow tuples using SHA-1.
/// Supports TCP, UDP, SCTP, ICMP, and IPv6-ICMP protocols.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct CommunityIdProcessor {
    source_ip_field: String,
    destination_ip_field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_port_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    destination_port_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    iana_protocol_number_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    protocol_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    icmp_type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    icmp_code_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    seed: Option<u16>,
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

/// Copies an entire object in an existing field to another field.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct CopyProcessor {
    source_field: String,
    target_field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    override_target: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    remove_source: Option<bool>,
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

/// Generates a hash value for either certain specified fields or all fields in a document.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct FingerprintProcessor {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    fields: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    exclude_fields: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hash_method: Option<String>,
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

/// Removes HTML tags from string fields; replaced with newline characters.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct HtmlStripProcessor {
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

/// Adds geographical location information for an IPv4 or IPv6 address using an external GeoIP data source.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Ip2GeoProcessor {
    field: String,
    datasource: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    properties: Vec<String>,
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

/// Removes root-level fields from a document using wildcard patterns.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct RemoveByPatternProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    field_pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    exclude_field_pattern: Option<String>,
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

/// Splits a long document into shorter passages using fixed_token_length, fixed_char_length, or delimiter algorithms.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct TextChunkingProcessor {
    field_map: HashMap<String, serde_json::Value>,
    algorithm: HashMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

/// Invokes ML models registered in the OpenSearch ML Commons plugin.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct MlInferenceProcessor {
    model_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    function_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model_config: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model_input: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    input_map: Vec<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    output_map: Vec<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    full_response_path: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ignore_failure: Option<bool>,
    #[serde(rename = "override", default, skip_serializing_if = "Option::is_none")]
    override_field: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_prediction_tasks: Option<u32>,
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
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("tests/ingest/pipeline.{name}.json"));
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
        assert_eq!(decoded.description, Some("A ml pipeline".to_owned()));
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_community_id_processor() {
        let decoded: Pipeline = load_entity("community_id");
        assert_eq!(
            decoded.description,
            Some("Generate community ID for network flows".to_owned())
        );
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::CommunityIdProcessor(p) = &decoded.processors[0] {
            assert_eq!(p.source_ip_field, "source_ip");
            assert_eq!(p.destination_ip_field, "destination_ip");
            assert_eq!(p.source_port_field, Some("source_port".to_owned()));
            assert_eq!(p.destination_port_field, Some("destination_port".to_owned()));
            assert_eq!(
                p.iana_protocol_number_field,
                Some("iana_protocol_number".to_owned())
            );
            assert_eq!(p.target_field, Some("community_id".to_owned()));
        } else {
            panic!("Expected CommunityIdProcessor");
        }
    }

    #[test]
    fn test_copy_processor() {
        let decoded: Pipeline = load_entity("copy");
        assert_eq!(
            decoded.description,
            Some("Pipeline that copies object.".to_owned())
        );
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::CopyProcessor(p) = &decoded.processors[0] {
            assert_eq!(p.source_field, "message.content");
            assert_eq!(p.target_field, "content");
            assert_eq!(p.ignore_missing, Some(true));
            assert_eq!(p.override_target, Some(true));
            assert_eq!(p.remove_source, Some(true));
        } else {
            panic!("Expected CopyProcessor");
        }
    }

    #[test]
    fn test_fingerprint_processor() {
        let decoded: Pipeline = load_entity("fingerprint");
        assert_eq!(
            decoded.description,
            Some("Generate hash value for some specified fields".to_owned())
        );
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::FingerprintProcessor(p) = &decoded.processors[0] {
            assert_eq!(p.fields, vec!["foo", "bar"]);
            assert_eq!(p.hash_method, Some("SHA-1@2.16.0".to_owned()));
            assert_eq!(p.target_field, Some("fingerprint".to_owned()));
        } else {
            panic!("Expected FingerprintProcessor");
        }
    }

    #[test]
    fn test_html_strip_processor() {
        let decoded: Pipeline = load_entity("html_strip");
        assert_eq!(
            decoded.description,
            Some("A pipeline to strip HTML from description field".to_owned())
        );
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::HtmlStripProcessor(p) = &decoded.processors[0] {
            assert_eq!(p.field, "description");
            assert_eq!(p.target_field, Some("cleaned_description".to_owned()));
        } else {
            panic!("Expected HtmlStripProcessor");
        }
    }

    #[test]
    fn test_ip2geo_processor() {
        let decoded: Pipeline = load_entity("ip2geo");
        assert_eq!(
            decoded.description,
            Some("Convert IP to geo location".to_owned())
        );
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::Ip2GeoProcessor(p) = &decoded.processors[0] {
            assert_eq!(p.field, "ip");
            assert_eq!(p.datasource, "my-datasource");
            assert_eq!(p.target_field, Some("ip2geo".to_owned()));
        } else {
            panic!("Expected Ip2GeoProcessor");
        }
    }

    #[test]
    fn test_remove_by_pattern_processor() {
        let decoded: Pipeline = load_entity("remove_by_pattern");
        assert_eq!(
            decoded.description,
            Some("Pipeline that removes the fields by patterns.".to_owned())
        );
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::RemoveByPatternProcessor(p) = &decoded.processors[0] {
            assert_eq!(p.field_pattern, Some("foo*".to_owned()));
        } else {
            panic!("Expected RemoveByPatternProcessor");
        }
    }

    #[test]
    fn test_text_chunking_processor() {
        let decoded: Pipeline = load_entity("text_chunking");
        assert_eq!(
            decoded.description,
            Some("A text chunking ingest pipeline".to_owned())
        );
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::TextChunkingProcessor(p) = &decoded.processors[0] {
            assert!(p.field_map.contains_key("passage_text"));
            assert!(p.algorithm.contains_key("fixed_token_length"));
        } else {
            panic!("Expected TextChunkingProcessor");
        }
    }

    #[test]
    fn test_ml_inference_processor() {
        let decoded: Pipeline = load_entity("ml_inference");
        assert_eq!(
            decoded.description,
            Some("Generate passage_embedding for ingested documents".to_owned())
        );
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::MlInferenceProcessor(p) = &decoded.processors[0] {
            assert_eq!(p.model_id, "my-model-id");
            assert_eq!(p.input_map.len(), 1);
            assert_eq!(p.output_map.len(), 1);
        } else {
            panic!("Expected MlInferenceProcessor");
        }
    }

    #[test]
    fn test_sort_processor_with_order() {
        let decoded: Pipeline = load_entity("sort");
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::SortProcessor(p) = &decoded.processors[0] {
            assert_eq!(p.field, "tags");
            assert_eq!(p.order, Some("desc".to_owned()));
        } else {
            panic!("Expected SortProcessor");
        }
    }

    #[test]
    fn test_convert_processor_type_field() {
        let decoded: Pipeline = load_entity("convert");
        assert_eq!(decoded.processors.len(), 1);
        if let Processor::ConvertProcessor(p) = &decoded.processors[0] {
            assert_eq!(p.field, "price");
            assert_eq!(p.type_field, "float");
            assert_eq!(p.target_field, Some("price_float".to_owned()));
        } else {
            panic!("Expected ConvertProcessor");
        }
    }
}
