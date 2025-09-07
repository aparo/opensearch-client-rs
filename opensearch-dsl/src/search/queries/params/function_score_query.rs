use std::{fmt, fmt::Debug};

use chrono::{DateTime, Utc};
use serde::{
    de::{self, Deserialize, DeserializeOwned, Deserializer, MapAccess, Visitor},
    ser::{Serialize, SerializeMap, Serializer},
};

use crate::{search::*, util::*};

/// Each document is scored by the defined functions. The parameter `score_mode`
/// specifies how the computed scores are combined
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionScoreMode {
    /// Scores are multiplied (default)
    Multiply,

    /// Scores are summed
    Sum,

    /// Scores are averaged
    Avg,

    /// The first function that has a matching filter is applied
    First,

    /// Maximum score is used
    Max,

    /// Minimum score is used
    Min,
}

impl Default for FunctionScoreMode {
    fn default() -> Self {
        Self::Multiply
    }
}

/// The newly computed score is combined with the score of the query. The
/// parameter `boost_mode` defines how.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionBoostMode {
    /// Query score and function score is multiplied (default)
    Multiply,

    /// Only function score is used, the query score is ignored
    Replace,

    /// Query score and function score are added
    Sum,

    /// Average
    Avg,

    /// Max of query score and function score
    Max,

    /// Min of query score and function score
    Min,
}

impl Default for FunctionBoostMode {
    fn default() -> Self {
        Self::Multiply
    }
}

macro_rules! function {
    ($name:ident { $($variant:ident($query:ty)),+ $(,)? }) => {
        /// Functions available for use in [FunctionScoreQuery](crate::FunctionScoreQuery)
        #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
        #[allow(missing_docs)]
        #[serde(untagged)]
        pub enum $name {
            $(
                $variant($query),
            )*
        }

        $(
            impl From<$query> for $name {
                fn from(q: $query) -> Self {
                    $name::$variant(q)
                }
            }
        )+

        $(
            impl From<$query> for Option<$name> {
                fn from(q: $query) -> Self {
                    Some($name::$variant(q))
                }
            }
        )+
    };
}

function!(Function {
    Weight(Weight),
    RandomScore(RandomScore),
    FieldValueFactor(FieldValueFactor),
    DecayDateTime(Decay<DateTime<Utc>>),
    DecayLocation(Decay<GeoLocation>),
    DecayI8(Decay<i8>),
    DecayI16(Decay<i16>),
    DecayI32(Decay<i32>),
    DecayI64(Decay<i64>),
    DecayU8(Decay<u8>),
    DecayU16(Decay<u16>),
    DecayU32(Decay<u32>),
    DecayU64(Decay<u64>),
    Script(Script),
});

impl Function {
    /// Creates an instance of [Weight](Weight)
    pub fn weight(weight: f32) -> Weight {
        Weight::new(weight)
    }

    /// Creates an instance of [RandomScore](RandomScore)
    pub fn random_score() -> RandomScore {
        RandomScore::new()
    }

    /// Creates an instance of [FieldValueFactor](FieldValueFactor)
    ///
    /// - `field` - Field to be extracted from the document.
    pub fn field_value_factor<T>(field: T) -> FieldValueFactor
    where
        T: ToString,
    {
        FieldValueFactor::new(field)
    }

    /// Creates an instance of [Decay](Decay)
    ///
    /// - `function` - Decay function variant
    /// - `field` - Field to apply function to
    /// - `origin` - The point of origin used for calculating distance. Must be
    ///   given as a number
    /// for numeric field, date for date fields and geo point for geo fields.
    /// Required for geo and numeric field. For date fields the default is
    /// `now`. Date math (for example now-1h) is supported for origin.
    /// - `scale` - Required for all types. Defines the distance from origin +
    ///   offset at which the
    /// computed score will equal `decay` parameter. For geo fields: Can be
    /// defined as number+unit (1km, 12m,…​). Default unit is meters. For date
    /// fields: Can to be defined as a number+unit ("1h", "10d",…​). Default unit
    /// is milliseconds. For numeric field: Any number.
    pub fn decay<T, O>(
        function: DecayFunction,
        field: T,
        origin: O,
        scale: <O as Origin>::Scale,
    ) -> Decay<O>
    where
        T: ToString,
        O: Origin,
    {
        Decay::new(function, field, origin, scale)
    }

    /// Creates an instance of script
    ///
    /// - `source` - script source
    pub fn script<T>(source: T) -> FunctionScoreScript
    where
        T: ToString,
    {
        FunctionScoreScript::new(source)
    }
}

/// The `weight` score allows you to multiply the score by the provided weight.
///
/// This can sometimes be desired since boost value set on specific queries gets
/// normalized, while for this score function it does not
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Weight {
    weight: f32,
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    filter: Option<Query>,
}

impl Weight {
    /// Creates an instance of [Weight](Weight)
    pub fn new(weight: f32) -> Self {
        Self {
            weight,
            filter: None,
        }
    }

    /// Add function filter
    pub fn filter<T>(mut self, filter: T) -> Self
    where
        T: Into<Option<Query>>,
    {
        self.filter = filter.into();
        self
    }
}

/// The `random_score` generates scores that are uniformly distributed from `0`
/// up to but not including `1`.
///
/// By default, it uses the internal Lucene doc ids as a source of randomness,
/// which is very efficient but unfortunately not reproducible since documents
/// might be renumbered by merges.
///
/// In case you want scores to be reproducible, it is possible to provide a
/// `seed` and `field`. The final score will then be computed based on this
/// seed, the minimum value of `field` for the considered document and a salt
/// that is computed based on the index name and shard id so that documents that
/// have the same value but are stored in different indexes get different
/// scores. Note that documents that are within the same shard and have the same
/// value for `field` will however get the same score, so it is usually
/// desirable to use a field that has unique values for all documents. A good
/// default choice might be to use the `_seq_no` field, whose only drawback is
/// that scores will change if the document is updated since update operations
/// also update the value of the `_seq_no` field.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct RandomScore {
    random_score: RandomScoreInner,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    filter: Option<Query>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    weight: Option<f32>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
struct RandomScoreInner {
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    seed: Option<Term>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    field: Option<String>,
}

impl RandomScore {
    /// Creates an instance of [RandomScore](RandomScore)
    pub fn new() -> Self {
        Default::default()
    }

    /// Add function filter
    pub fn filter<T>(mut self, filter: T) -> Self
    where
        T: Into<Option<Query>>,
    {
        self.filter = filter.into();
        self
    }

    /// The `weight` score allows you to multiply the score by the provided
    /// `weight`. This can sometimes be desired since boost value set on
    /// specific queries gets normalized, while for this score function it does
    /// not. The number value is of type float.
    pub fn weight<T>(mut self, weight: T) -> Self
    where
        T: num_traits::AsPrimitive<f32>,
    {
        self.weight = Some(weight.as_());
        self
    }

    /// Sets seed value
    pub fn seed<T>(mut self, seed: T) -> Self
    where
        T: Serialize,
    {
        self.random_score.seed = Term::new(seed);
        self
    }

    /// Sets field value
    pub fn field<T>(mut self, field: T) -> Self
    where
        T: ToString,
    {
        self.random_score.field = Some(field.to_string());
        self
    }
}

/// The `field_value_factor` function allows you to use a field from a document
/// to influence the score.
/// It’s similar to using the `script_score` function, however, it avoids the
/// overhead of scripting. If used on a multi-valued field, only the first value
/// of the field is used in calculations.
///
/// As an example, imagine you have a document indexed with a numeric `my-int`
/// field and wish to influence the score of a document with this field, an
/// example doing so would look like: ```
/// # use opensearch_dsl::{FieldValueFactor, FieldValueFactorModifier};
/// # fn main() {
/// # let _ =
/// FieldValueFactor::new("my-int")
///   .factor(1.2)
///   .modifier(FieldValueFactorModifier::Sqrt)
///   .missing(1.0)
/// # ;}
/// ```
/// Which will translate into the following formula for scoring:
/// ```text
/// sqrt(1.2 * doc['my-int'].value)
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FieldValueFactor {
    field_value_factor: FieldValueFactorInner,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    filter: Option<Query>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    weight: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct FieldValueFactorInner {
    field: String,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    factor: Option<f32>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    modifier: Option<FieldValueFactorModifier>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<f32>,
}

impl FieldValueFactor {
    /// Creates an instance of [FieldValueFactor](FieldValueFactor)
    ///
    /// - `field` - Field to be extracted from the document.
    pub fn new<T>(field: T) -> Self
    where
        T: ToString,
    {
        Self {
            field_value_factor: FieldValueFactorInner {
                field: field.to_string(),
                factor: None,
                modifier: None,
                missing: None,
            },
            filter: None,
            weight: None,
        }
    }

    /// Add function filter
    pub fn filter<T>(mut self, filter: T) -> Self
    where
        T: Into<Option<Query>>,
    {
        self.filter = filter.into();
        self
    }

    /// The `weight` score allows you to multiply the score by the provided
    /// `weight`. This can sometimes be desired since boost value set on
    /// specific queries gets normalized, while for this score function it does
    /// not. The number value is of type float.
    pub fn weight<T>(mut self, weight: T) -> Self
    where
        T: num_traits::AsPrimitive<f32>,
    {
        self.weight = Some(weight.as_());
        self
    }

    /// Factor to multiply the field value with
    pub fn factor(mut self, factor: f32) -> Self {
        self.field_value_factor.factor = Some(factor);
        self
    }

    /// Modifier to apply to the field value
    pub fn modifier(mut self, modifier: FieldValueFactorModifier) -> Self {
        self.field_value_factor.modifier = Some(modifier);
        self
    }

    /// Value used if the document doesn’t have that field. The modifier and
    /// factor are still applied to it as though it were read from the document
    pub fn missing(mut self, missing: f32) -> Self {
        self.field_value_factor.missing = Some(missing);
        self
    }
}

/// Modifier to apply to the field value
///
/// Defaults to [none](FieldValueFactorModifier::None)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldValueFactorModifier {
    /// Do not apply any multiplier to the field value
    None,

    /// Take the [common logarithm](https://en.wikipedia.org/wiki/Common_logarithm) of the field
    /// value
    ///
    /// Because this function will return a negative value and cause an error if
    /// used on values between `0` and `1`, it is recommended to use
    /// [log1p](FieldValueFactorModifier::Log1P) instead.
    Log,

    /// Add 1 to the field value and take the common logarithm
    Log1P,

    /// Add 2 to the field value and take the common logarithm
    Log2P,

    /// Take the [natural logarithm](https://en.wikipedia.org/wiki/Natural_logarithm) of the field
    /// value.
    ///
    /// Because this function will return a negative value and cause an error if
    /// used on values between `0` and `1`, it is recommended to use
    /// [ln1p](FieldValueFactorModifier::Ln1P) instead.
    Ln,

    /// Add 1 to the field value and take the natural logarithm
    Ln1P,

    /// Add 2 to the field value and take the natural logarithm
    Ln2P,

    /// Square the field value (multiply it by itself)
    Square,

    /// Take the [square root](https://en.wikipedia.org/wiki/Square_root) of the field value
    Sqrt,

    /// [Reciprocate](https://en.wikipedia.org/wiki/Multiplicative_inverse) the field value, same
    /// as `1/x` where `x` is the field’s value
    Reciprocal,
}

#[doc(hidden)]
pub trait Origin: Debug + PartialEq + DeserializeOwned + Serialize + Clone {
    type Scale: Debug + PartialEq + DeserializeOwned + Serialize + Clone;
    type Offset: Debug + PartialEq + DeserializeOwned + Serialize + Clone;
}

impl Origin for DateTime<Utc> {
    type Offset = Time;
    type Scale = Time;
}

impl Origin for GeoLocation {
    type Offset = Distance;
    type Scale = Distance;
}

macro_rules! impl_origin_for_numbers {
    ($($name:ident ),+) => {
        $(
            impl Origin for $name {
                type Scale = Self;
                type Offset = Self;
            }
        )+
    }
}

impl_origin_for_numbers![i8, i16, i32, i64, u8, u16, u32, u64, f32, f64];

/// Decay functions score a document with a function that decays depending on
/// the distance of a numeric field value of the document from a user given
/// origin. This is similar to a range query, but with smooth edges instead of
/// boxes.
///
/// To use distance scoring on a query that has numerical fields, the user has
/// to define an `origin` and a `scale` for each field. The `origin` is needed
/// to define the “central point” from which the distance is calculated, and the
/// `scale` to define the rate of decay.
#[derive(Debug, Clone, PartialEq)]
pub struct Decay<T: Origin + DeserializeOwned> {
    function: DecayFunction,

    inner: DecayFieldInner<T>,

    filter: Option<Query>,

    weight: Option<f32>,
}

impl<T: Origin> Serialize for Decay<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;

        map.serialize_entry(&self.function, &self.inner)?;

        if let Some(filter) = &self.filter {
            map.serialize_entry("filter", filter)?;
        }

        if let Some(weight) = &self.weight {
            map.serialize_entry("weight", weight)?;
        }

        map.end()
    }
}

impl<'de, T: Origin + DeserializeOwned> Deserialize<'de> for Decay<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DecayVisitor<T: Origin + DeserializeOwned> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<'de, T: Origin + DeserializeOwned> Visitor<'de> for DecayVisitor<T> {
            type Value = Decay<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Decay")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Decay<T>, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut function = None;
                let inner = None;
                let mut filter = None;
                let mut weight = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "function" => function = Some(map.next_value()?),
                        // TODO: fix me issues with Deserialize<'de>
                        // "inner" => inner = Some(map.next_value()?),
                        "filter" => filter = Some(map.next_value()?),
                        "weight" => weight = Some(map.next_value()?),
                        _ => {
                            return Err(de::Error::unknown_field(
                                key,
                                &["function", "inner", "filter", "weight"],
                            ))
                        }
                    }
                }

                let function = function.ok_or_else(|| de::Error::missing_field("function"))?;
                let inner = inner.ok_or_else(|| de::Error::missing_field("inner"))?;

                Ok(Decay {
                    function,
                    inner,
                    filter,
                    weight,
                })
            }
        }

        deserializer.deserialize_struct(
            "Decay",
            &["function", "inner", "filter", "weight"],
            DecayVisitor {
                _marker: std::marker::PhantomData,
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct DecayFieldInner<T: Origin + DeserializeOwned> {
    field: String,
    inner: DecayInner<T>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct DecayInner<O>
where
    O: Origin + DeserializeOwned,
{
    origin: O,

    scale: <O as Origin>::Scale,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    offset: Option<<O as Origin>::Offset>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    decay: Option<f32>,
}

impl<T: Origin> Serialize for DecayFieldInner<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;

        map.serialize_entry(&self.field, &self.inner)?;

        map.end()
    }
}

impl<'de, O: Origin + DeserializeOwned> Deserialize<'de> for DecayInner<O> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DecayInnerVisitor<O: Origin + DeserializeOwned> {
            _marker: std::marker::PhantomData<O>,
        }

        impl<'de, O: Origin + DeserializeOwned> Visitor<'de> for DecayInnerVisitor<O> {
            type Value = DecayInner<O>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DecayInner")
            }

            fn visit_map<A>(self, mut map: A) -> Result<DecayInner<O>, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut origin = None;
                let mut scale = None;
                let mut offset = None;
                let mut decay = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "origin" => origin = Some(map.next_value()?),
                        "scale" => scale = Some(map.next_value()?),
                        "offset" => offset = map.next_value()?,
                        "decay" => decay = map.next_value()?,
                        _ => {
                            return Err(de::Error::unknown_field(
                                key,
                                &["origin", "scale", "offset", "decay"],
                            ))
                        }
                    }
                }

                let origin = origin.ok_or_else(|| de::Error::missing_field("origin"))?;
                let scale = scale.ok_or_else(|| de::Error::missing_field("scale"))?;

                Ok(DecayInner {
                    origin,
                    scale,
                    offset,
                    decay,
                })
            }
        }

        deserializer.deserialize_struct(
            "DecayInner",
            &["origin", "scale", "offset", "decay"],
            DecayInnerVisitor {
                _marker: std::marker::PhantomData,
            },
        )
    }
}

impl<O> Decay<O>
where
    O: Origin,
{
    /// Creates an instance of [Decay](Decay)
    ///
    /// - `function` - Decay function variant
    /// - `field` - Field to apply function to
    /// - `origin` - The point of origin used for calculating distance. Must be
    ///   given as a number
    /// for numeric field, date for date fields and geo point for geo fields.
    /// Required for geo and numeric field. For date fields the default is
    /// `now`. Date math (for example now-1h) is supported for origin.
    /// - `scale` - Required for all types. Defines the distance from origin +
    ///   offset at which the
    /// computed score will equal `decay` parameter. For geo fields: Can be
    /// defined as number+unit (1km, 12m,…​). Default unit is meters. For date
    /// fields: Can to be defined as a number+unit ("1h", "10d",…​). Default unit
    /// is milliseconds. For numeric field: Any number.
    pub fn new<T>(function: DecayFunction, field: T, origin: O, scale: <O as Origin>::Scale) -> Self
    where
        T: ToString,
    {
        Self {
            function,
            inner: DecayFieldInner {
                field: field.to_string(),
                inner: DecayInner {
                    origin,
                    scale,
                    offset: None,
                    decay: None,
                },
            },
            filter: None,
            weight: None,
        }
    }

    /// Add function filter
    pub fn filter<T>(mut self, filter: T) -> Self
    where
        T: Into<Option<Query>>,
    {
        self.filter = filter.into();
        self
    }

    /// The `weight` score allows you to multiply the score by the provided
    /// `weight`. This can sometimes be desired since boost value set on
    /// specific queries gets normalized, while for this score function it does
    /// not. The number value is of type float.
    pub fn weight<T>(mut self, weight: T) -> Self
    where
        T: num_traits::AsPrimitive<f32>,
    {
        self.weight = Some(weight.as_());
        self
    }

    /// If an `offset` is defined, the decay function will only compute the decay
    /// function for documents with a distance greater than the defined
    /// `offset`.
    ///
    /// The default is `0`.
    pub fn offset(mut self, offset: <O as Origin>::Offset) -> Self {
        self.inner.inner.offset = Some(offset);
        self
    }

    /// The `decay` parameter defines how documents are scored at the distance
    /// given at `scale`. If no `decay` is defined, documents at the distance
    /// `scale` will be scored `0.5`.
    pub fn decay(mut self, decay: f32) -> Self {
        self.inner.inner.decay = Some(decay);
        self
    }
}

/// Decay function variants
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-function-score-query.html#_supported_decay_functions>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DecayFunction {
    /// Linear decay
    Linear,

    /// Exponential decay
    Exp,

    /// Gauss decay
    Gauss,
}

/// The script_score function allows you to wrap another query and customize the
/// scoring of it optionally with a computation derived from other numeric field
/// values in the doc using a script expression
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FunctionScoreScript {
    script_score: ScriptInnerWrapper,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct ScriptInnerWrapper {
    script: ScriptInner,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct ScriptInner {
    source: String,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    params: Option<serde_json::Value>,
}

impl FunctionScoreScript {
    /// Creates an instance of [FunctionScoreScript]
    ///
    /// - `source` - script source
    pub fn new<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self {
            script_score: ScriptInnerWrapper {
                script: ScriptInner {
                    source: source.to_string(),
                    params: None,
                },
            },
        }
    }

    /// Sets params value
    pub fn params(mut self, params: serde_json::Value) -> Self {
        self.script_score.script.params = Some(params);
        self
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Decay::new(
                DecayFunction::Gauss,
                "test",
                Utc.with_ymd_and_hms(2014, 7, 8, 9, 1, 0).single().unwrap(),
                Time::Days(7),
            ),
            json!({
                "gauss": {
                    "test": {
                        "origin": "2014-07-08T09:01:00Z",
                        "scale": "7d",
                    }
                }
            }),
        );

        assert_serialize(
            Decay::new(DecayFunction::Linear, "test", 1, 2),
            json!({
                "linear": {
                    "test": {
                        "origin": 1,
                        "scale": 2,
                    }
                }
            }),
        );
    }
}
