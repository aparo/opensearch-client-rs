use crate::{search::*, util::*};

/// A `rate` metrics aggregation can be used only inside a `date_histogram` and
/// calculates a rate of documents or a field in each `date_histogram` bucket.
/// The field values can be generated extracted from specific numeric or [histogram fields](https://www.elastic.co/guide/en/opensearch/reference/current/histogram.html)
/// in the documents.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/search-aggregations-metrics-rate-aggregation.html>
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RateAggregation {
  rate: RateAggregationInner,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct RateAggregationInner {
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  field: Option<String>,
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  unit: Option<CalendarInterval>,
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  mode: Option<RateMode>,
}

impl Aggregation {
  /// Creates an instance of [`RateAggregation`]
  pub fn rate() -> RateAggregation {
    RateAggregation {
      rate: RateAggregationInner {
        field: None,
        unit: None,
        mode: None,
      },
    }
  }
}

impl RateAggregation {
  /// Calculate sum or number of values of the `field`
  pub fn field<T>(mut self, field: T) -> Self
  where
    T: ToString, {
    self.rate.field = Some(field.to_string());
    self
  }

  /// The `rate` aggregation supports all rate that can be used [calendar_intervals parameter](https://www.elastic.co/guide/en/opensearch/reference/current/search-aggregations-bucket-datehistogram-aggregation.html#calendar_intervals)
  /// of `date_histogram` aggregation. The specified rate should compatible with
  /// the date_histogram aggregation interval, i.e. it should be possible to
  /// convert the bucket size into the rate. By default the interval of the
  /// `date_histogram` is used.
  ///
  /// There is also an additional limitations if the date histogram is not a
  /// direct parent of the rate histogram. In this case both rate interval and
  /// histogram interval have to be in the same group: [second, `minute`,
  /// hour, day, week] or [month, quarter, year]. For example, if the date
  /// histogram is month based, only rate intervals of month, quarter or year
  /// are supported. If the date histogram is `day` based, only `second`, `
  /// minute`, `hour`, `day, and `week` rate intervals are supported.
  pub fn unit(mut self, unit: CalendarInterval) -> Self {
    self.rate.unit = Some(unit);
    self
  }

  /// By default sum mode is used.
  ///
  /// By adding the `mode` parameter with the value `value_count`, we can change
  /// the calculation from `sum` to the number of values of the field.
  pub fn mode(mut self, mode: RateMode) -> Self {
    self.rate.mode = Some(mode);
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialization() {
    assert_serialize_aggregation(Aggregation::rate(), json!({ "rate": { } }));

    assert_serialize_aggregation(
      Aggregation::rate()
        .field("price")
        .unit(CalendarInterval::Day)
        .mode(RateMode::ValueCount),
      json!({
          "rate": {
              "field": "price",
              "unit": "day",
              "mode": "value_count"
          }
      }),
    );
  }
}
