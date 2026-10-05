/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 */

// ── Shared enums ──────────────────────────────────────────────────────────────
pub mod enums;
pub use self::enums::{
    DynamicMapping, IndexOptions, KnnCompressionLevel, KnnDataType, KnnEngine, KnnMode,
    KnnSpaceType, MatchPattern, MaxShingleSize, Orientation, Similarity, TermVector,
};

// ── Field type property structs ───────────────────────────────────────────────
pub mod autocomplete_property;
pub use self::autocomplete_property::{CompletionFuzzy, CompletionProperty, SearchAsYouTypeProperty};

pub mod boolean_property;
pub use self::boolean_property::BooleanProperty;

pub mod cartesian_property;
pub use self::cartesian_property::{XyPointProperty, XyShapeProperty};

pub mod date_property;
pub use self::date_property::{DateNanosProperty, DateProperty};

pub mod geo_property;
pub use self::geo_property::{GeoPointProperty, GeoShapeProperty};

pub mod ip_property;
pub use self::ip_property::IpProperty;

pub mod numeric_property;
pub use self::numeric_property::{NumericProperty, ScaledFloatProperty};

pub mod object_property;
pub use self::object_property::{FlatObjectProperty, JoinProperty, NestedProperty, ObjectProperty, PropertiesMap};

pub mod range_property;
pub use self::range_property::{
    DateRangeProperty, DoubleRangeProperty, FloatRangeProperty, IntegerRangeProperty,
    IpRangeProperty, LongRangeProperty, RangeProperty,
};

pub mod specialized_property;
pub use self::specialized_property::{
    AggregateMetricDoubleProperty, AliasProperty, BinaryProperty, HistogramProperty,
    Murmur3HashProperty, PercolatorProperty,
};

pub mod text_property;
pub use self::text_property::{
    ConstantKeywordProperty, FielddataFrequencyFilter, IcuCollationKeywordProperty, KeywordProperty,
    MatchOnlyTextPropertyValue, TextProperty, TokenCountProperty, VersionProperty, WildcardProperty,
};

pub mod vector_property;
pub use self::vector_property::{KnnVectorProperty, RankFeatureProperty, RankFeaturesProperty};

// ── Property discriminated union ──────────────────────────────────────────────
pub mod property;
pub use self::property::Property;

// ── Legacy / retained structs ─────────────────────────────────────────────────
pub mod all_field;
pub use self::all_field::AllField;

pub mod data_stream_timestamp;
pub use self::data_stream_timestamp::DataStreamTimestamp;

pub mod dynamic_template;
pub use self::dynamic_template::DynamicTemplate;

pub mod field_mapping;
pub use self::field_mapping::FieldMapping;

pub mod field_names_field;
pub use self::field_names_field::FieldNamesField;

pub mod index_field;
pub use self::index_field::IndexField;

pub mod knn_vector_method;
pub use self::knn_vector_method::KnnVectorMethod;

pub mod match_only_text_property;
pub use self::match_only_text_property::MatchOnlyTextProperty;

pub mod match_only_text_property_value;
// Re-export MatchOnlyTextPropertyValue from text_property (canonical location).
// The legacy match_only_text_property_value module is kept for backward compatibility.

pub mod property_base;
pub use self::property_base::PropertyBase;

pub mod routing_field;
pub use self::routing_field::RoutingField;

pub mod size_field;
pub use self::size_field::SizeField;

pub mod source_field;
pub use self::source_field::SourceField;

pub mod suggest_context;
pub use self::suggest_context::SuggestContext;

pub mod text_index_prefixes;
pub use self::text_index_prefixes::TextIndexPrefixes;

pub mod type_mapping;
pub use self::type_mapping::TypeMapping;

#[cfg(test)]
pub mod tests;
