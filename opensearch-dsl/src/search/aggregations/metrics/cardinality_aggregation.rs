use crate::{search::*, util::*};

/// A `single-value` metrics aggregation that calculates an approximate count of
/// distinct values.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/search-aggregations-metrics-cardinality-aggregation.html>
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CardinalityAggregation {
  cardinality: CardinalityAggregationInner,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct CardinalityAggregationInner {
  field: String,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  precision_threshold: Option<u16>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  missing: Option<String>,
}

impl Aggregation {
  /// Creates an instance of [`CardinalityAggregation`]
  ///
  /// - `field` - field to aggregate
  pub fn cardinality<T>(field: T) -> CardinalityAggregation
  where
    T: ToString, {
    CardinalityAggregation {
      cardinality: CardinalityAggregationInner {
        field: field.to_string(),
        precision_threshold: None,
        missing: None,
      },
    }
  }
}

impl CardinalityAggregation {
  /// The `precision_threshold` options allows to trade memory for accuracy, and
  /// defines a unique count below which counts are expected to be close to
  /// accurate. Above this value, counts might become a bit more fuzzy.
  /// The maximum supported value is 40000, thresholds above this number will
  /// have the same effect as a threshold of 40000. The default value is 3000
  pub fn precision_threshold(mut self, precision_threshold: u16) -> Self {
    self.cardinality.precision_threshold = Some(precision_threshold);
    self
  }

  /// The `missing` parameter defines how documents that are missing a value
  /// should be treated. By default they will be ignored but it is also
  /// possible to treat them as if they had a value.
  pub fn missing<T>(mut self, missing: T) -> Self
  where
    T: ToString, {
    self.cardinality.missing = Some(missing.to_string());
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialization() {
    assert_serialize_aggregation(
      Aggregation::cardinality("test_field"),
      json!({ "cardinality": { "field": "test_field" } }),
    );

    assert_serialize_aggregation(
      Aggregation::cardinality("test_field")
        .precision_threshold(100u16)
        .missing("N/A"),
      json!({
          "cardinality": {
              "field": "test_field",
              "precision_threshold": 100,
              "missing": "N/A"
          }
      }),
    );
  }
}
