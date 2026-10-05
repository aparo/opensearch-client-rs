/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Discriminated union of all OpenSearch field types.
 */

use serde::{Deserialize, Serialize};

use super::{
    autocomplete_property::{CompletionProperty, SearchAsYouTypeProperty},
    boolean_property::BooleanProperty,
    cartesian_property::{XyPointProperty, XyShapeProperty},
    date_property::{DateNanosProperty, DateProperty},
    geo_property::{GeoPointProperty, GeoShapeProperty},
    ip_property::IpProperty,
    numeric_property::{NumericProperty, ScaledFloatProperty},
    object_property::{FlatObjectProperty, JoinProperty, NestedProperty, ObjectProperty},
    range_property::{
        DateRangeProperty, DoubleRangeProperty, FloatRangeProperty, IntegerRangeProperty,
        IpRangeProperty, LongRangeProperty,
    },
    specialized_property::{
        AggregateMetricDoubleProperty, AliasProperty, BinaryProperty, HistogramProperty,
        Murmur3HashProperty, PercolatorProperty,
    },
    text_property::{
        ConstantKeywordProperty, IcuCollationKeywordProperty, KeywordProperty,
        MatchOnlyTextPropertyValue, TextProperty, TokenCountProperty, VersionProperty,
        WildcardProperty,
    },
    vector_property::{KnnVectorProperty, RankFeatureProperty, RankFeaturesProperty},
};

/// All OpenSearch field types as a tagged union.
///
/// Serializes with an internal `"type"` discriminator field matching the
/// OpenSearch mapping type name. Use this in `HashMap<String, Property>` for
/// the `properties` map of a `TypeMapping` or `ObjectProperty`.
///
/// # Example (serialized mapping)
/// ```json
/// {
///   "properties": {
///     "title": {"type": "text", "analyzer": "standard"},
///     "status": {"type": "keyword", "ignore_above": 64},
///     "score": {"type": "float"},
///     "created_at": {"type": "date"}
///   }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Property {
    // ── String types ──────────────────────────────────────────────────────────
    /// Analyzed full-text string.
    #[serde(rename = "text")]
    Text(Box<TextProperty>),

    /// Exact-value string (not analyzed).
    #[serde(rename = "keyword")]
    Keyword(Box<KeywordProperty>),

    /// Storage-optimized text; all matches score 1.0.
    #[serde(rename = "match_only_text")]
    MatchOnlyText(Box<MatchOnlyTextPropertyValue>),

    /// Trigram-indexed string optimized for wildcard/regex queries.
    #[serde(rename = "wildcard")]
    Wildcard(Box<WildcardProperty>),

    /// All documents share the same fixed keyword value.
    #[serde(rename = "constant_keyword")]
    ConstantKeyword(Box<ConstantKeywordProperty>),

    /// Stores the integer count of analyzed tokens.
    #[serde(rename = "token_count")]
    TokenCount(Box<TokenCountProperty>),

    /// Semantic versioning string with correct SemVer ordering.
    #[serde(rename = "version")]
    Version(Box<VersionProperty>),

    /// Language-aware keyword using ICU collation (requires analysis-icu plugin).
    #[serde(rename = "icu_collation_keyword")]
    IcuCollationKeyword(Box<IcuCollationKeywordProperty>),

    // ── Numeric types ─────────────────────────────────────────────────────────
    /// Signed 8-bit integer (-128 to 127).
    #[serde(rename = "byte")]
    Byte(Box<NumericProperty>),

    /// Signed 16-bit integer (-32768 to 32767).
    #[serde(rename = "short")]
    Short(Box<NumericProperty>),

    /// Signed 32-bit integer.
    #[serde(rename = "integer")]
    Integer(Box<NumericProperty>),

    /// Signed 64-bit integer.
    #[serde(rename = "long")]
    Long(Box<NumericProperty>),

    /// Unsigned 64-bit integer (0 to 2^64-1).
    #[serde(rename = "unsigned_long")]
    UnsignedLong(Box<NumericProperty>),

    /// 16-bit IEEE 754 float.
    #[serde(rename = "half_float")]
    HalfFloat(Box<NumericProperty>),

    /// 32-bit IEEE 754 float.
    #[serde(rename = "float")]
    Float(Box<NumericProperty>),

    /// 64-bit IEEE 754 float.
    #[serde(rename = "double")]
    Double(Box<NumericProperty>),

    /// Long scaled by a fixed factor for compact float storage.
    #[serde(rename = "scaled_float")]
    ScaledFloat(Box<ScaledFloatProperty>),

    // ── Boolean ───────────────────────────────────────────────────────────────
    /// True/false values.
    #[serde(rename = "boolean")]
    Boolean(Box<BooleanProperty>),

    // ── Date types ────────────────────────────────────────────────────────────
    /// Date (millisecond precision, stored as epoch ms).
    #[serde(rename = "date")]
    Date(Box<DateProperty>),

    /// Date with nanosecond precision (range approx. 1970–2262).
    #[serde(rename = "date_nanos")]
    DateNanos(Box<DateNanosProperty>),

    // ── IP ────────────────────────────────────────────────────────────────────
    /// IPv4 or IPv6 address; CIDR supported in queries.
    #[serde(rename = "ip")]
    Ip(Box<IpProperty>),

    // ── Geographic types ──────────────────────────────────────────────────────
    /// Latitude/longitude point.
    #[serde(rename = "geo_point")]
    GeoPoint(Box<GeoPointProperty>),

    /// Arbitrary geographic shape (polygon, linestring, etc.).
    #[serde(rename = "geo_shape")]
    GeoShape(Box<GeoShapeProperty>),

    // ── Cartesian types ───────────────────────────────────────────────────────
    /// Single point in 2D Cartesian space (non-geographic).
    #[serde(rename = "xy_point")]
    XyPoint(Box<XyPointProperty>),

    /// Shape in 2D Cartesian space (non-geographic).
    #[serde(rename = "xy_shape")]
    XyShape(Box<XyShapeProperty>),

    // ── Range types ───────────────────────────────────────────────────────────
    /// Range of signed 32-bit integers.
    #[serde(rename = "integer_range")]
    IntegerRange(Box<IntegerRangeProperty>),

    /// Range of signed 64-bit integers.
    #[serde(rename = "long_range")]
    LongRange(Box<LongRangeProperty>),

    /// Range of 32-bit floats.
    #[serde(rename = "float_range")]
    FloatRange(Box<FloatRangeProperty>),

    /// Range of 64-bit doubles.
    #[serde(rename = "double_range")]
    DoubleRange(Box<DoubleRangeProperty>),

    /// Range of date values.
    #[serde(rename = "date_range")]
    DateRange(Box<DateRangeProperty>),

    /// Range of IP addresses.
    #[serde(rename = "ip_range")]
    IpRange(Box<IpRangeProperty>),

    // ── Object types ──────────────────────────────────────────────────────────
    /// JSON object with flat internal representation.
    #[serde(rename = "object")]
    Object(Box<ObjectProperty>),

    /// JSON object stored as separate Lucene documents (preserves array element boundaries).
    #[serde(rename = "nested")]
    Nested(Box<NestedProperty>),

    /// Entire JSON object stored as a string; prevents mapping explosion.
    #[serde(rename = "flat_object")]
    FlatObject(Box<FlatObjectProperty>),

    /// Parent/child relationship within a single index.
    #[serde(rename = "join")]
    Join(Box<JoinProperty>),

    // ── Autocomplete types ────────────────────────────────────────────────────
    /// In-memory prefix suggester (matches from the start only).
    #[serde(rename = "completion")]
    Completion(Box<CompletionProperty>),

    /// Prefix and infix completion with auto-created n-gram subfields.
    #[serde(rename = "search_as_you_type")]
    SearchAsYouType(Box<SearchAsYouTypeProperty>),

    // ── Vector types ──────────────────────────────────────────────────────────
    /// Dense vector for k-NN / ANN similarity search.
    #[serde(rename = "knn_vector")]
    KnnVector(Box<KnnVectorProperty>),

    /// Single positive float that boosts/penalizes relevance scores.
    #[serde(rename = "rank_feature")]
    RankFeature(Box<RankFeatureProperty>),

    /// Sparse map of feature name → positive float for relevance boosting.
    #[serde(rename = "rank_features")]
    RankFeatures(Box<RankFeaturesProperty>),

    // ── Specialized types ─────────────────────────────────────────────────────
    /// Base64-encoded binary values; not searchable.
    #[serde(rename = "binary")]
    Binary(Box<BinaryProperty>),

    /// Alias pointing to another indexed field.
    #[serde(rename = "alias")]
    Alias(Box<AliasProperty>),

    /// Stores a query DSL object for use with the percolate query.
    #[serde(rename = "percolator")]
    Percolator(Box<PercolatorProperty>),

    /// Pre-aggregated numeric histogram (values + counts arrays).
    #[serde(rename = "histogram")]
    Histogram(Box<HistogramProperty>),

    /// Pre-aggregated metric statistics (min, max, sum, value_count).
    #[serde(rename = "aggregate_metric_double")]
    AggregateMetricDouble(Box<AggregateMetricDoubleProperty>),

    /// MurmurHash3 of field values for cardinality estimation.
    #[serde(rename = "murmur3")]
    Murmur3(Box<Murmur3HashProperty>),
}

impl Default for Property {
    fn default() -> Self {
        Property::Object(Box::default())
    }
}

impl Property {
    /// Returns the OpenSearch type name string for this property.
    pub fn type_name(&self) -> &'static str {
        match self {
            Property::Text(_) => "text",
            Property::Keyword(_) => "keyword",
            Property::MatchOnlyText(_) => "match_only_text",
            Property::Wildcard(_) => "wildcard",
            Property::ConstantKeyword(_) => "constant_keyword",
            Property::TokenCount(_) => "token_count",
            Property::Version(_) => "version",
            Property::IcuCollationKeyword(_) => "icu_collation_keyword",
            Property::Byte(_) => "byte",
            Property::Short(_) => "short",
            Property::Integer(_) => "integer",
            Property::Long(_) => "long",
            Property::UnsignedLong(_) => "unsigned_long",
            Property::HalfFloat(_) => "half_float",
            Property::Float(_) => "float",
            Property::Double(_) => "double",
            Property::ScaledFloat(_) => "scaled_float",
            Property::Boolean(_) => "boolean",
            Property::Date(_) => "date",
            Property::DateNanos(_) => "date_nanos",
            Property::Ip(_) => "ip",
            Property::GeoPoint(_) => "geo_point",
            Property::GeoShape(_) => "geo_shape",
            Property::XyPoint(_) => "xy_point",
            Property::XyShape(_) => "xy_shape",
            Property::IntegerRange(_) => "integer_range",
            Property::LongRange(_) => "long_range",
            Property::FloatRange(_) => "float_range",
            Property::DoubleRange(_) => "double_range",
            Property::DateRange(_) => "date_range",
            Property::IpRange(_) => "ip_range",
            Property::Object(_) => "object",
            Property::Nested(_) => "nested",
            Property::FlatObject(_) => "flat_object",
            Property::Join(_) => "join",
            Property::Completion(_) => "completion",
            Property::SearchAsYouType(_) => "search_as_you_type",
            Property::KnnVector(_) => "knn_vector",
            Property::RankFeature(_) => "rank_feature",
            Property::RankFeatures(_) => "rank_features",
            Property::Binary(_) => "binary",
            Property::Alias(_) => "alias",
            Property::Percolator(_) => "percolator",
            Property::Histogram(_) => "histogram",
            Property::AggregateMetricDouble(_) => "aggregate_metric_double",
            Property::Murmur3(_) => "murmur3",
        }
    }

    // ── Convenience constructors ───────────────────────────────────────────────

    pub fn text(p: TextProperty) -> Self {
        Property::Text(Box::new(p))
    }

    pub fn keyword(p: KeywordProperty) -> Self {
        Property::Keyword(Box::new(p))
    }

    pub fn integer() -> Self {
        Property::Integer(Box::default())
    }

    pub fn long() -> Self {
        Property::Long(Box::default())
    }

    pub fn float() -> Self {
        Property::Float(Box::default())
    }

    pub fn double() -> Self {
        Property::Double(Box::default())
    }

    pub fn boolean() -> Self {
        Property::Boolean(Box::default())
    }

    pub fn date() -> Self {
        Property::Date(Box::default())
    }

    pub fn date_nanos() -> Self {
        Property::DateNanos(Box::default())
    }

    pub fn ip() -> Self {
        Property::Ip(Box::default())
    }

    pub fn geo_point() -> Self {
        Property::GeoPoint(Box::default())
    }

    pub fn geo_shape() -> Self {
        Property::GeoShape(Box::default())
    }

    pub fn object(p: ObjectProperty) -> Self {
        Property::Object(Box::new(p))
    }

    pub fn nested(p: NestedProperty) -> Self {
        Property::Nested(Box::new(p))
    }

    pub fn knn_vector(dimension: u32) -> Self {
        Property::KnnVector(Box::new(KnnVectorProperty::new(dimension)))
    }

    pub fn alias(path: impl Into<String>) -> Self {
        Property::Alias(Box::new(AliasProperty::new(path)))
    }
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.type_name())
    }
}
