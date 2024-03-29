use serde::{Deserialize, Serialize};

use crate::{search::*, util::*};

/// A query that matches documents matching boolean combinations of other
/// queries. The bool query maps to Lucene BooleanQuery.
/// It is built using one or more boolean clauses, each clause with a typed
/// occurrence.
///
/// The bool query takes a more-matches-is-better approach, so the score from
/// each matching must or should clause will be added together to provide the
/// final _score for each document.
///
/// To create bool query:
/// ```
/// # use opensearch_dsl::queries::*;
/// # use opensearch_dsl::queries::params::*;
/// # let query =
/// Query::bool()
///   .must(Query::term("test1", 1))
///   .must(Query::term("test2", 2))
///   .should(Query::term("test1", 3))
///   .should(Query::term("test2", 4))
///   .filter(Query::term("test1", 5))
///   .filter(Query::term("test2", 6))
///   .must_not(Query::term("test1", 7))
///   .must_not(Query::term("test2", 8))
///   .minimum_should_match("2")
///   .boost(1.3)
///   .name("test");
/// ```
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-bool-query.html>
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct BoolQuery {
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  must: QueryCollection,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  filter: QueryCollection,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  should: QueryCollection,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  must_not: QueryCollection,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  minimum_should_match: Option<String>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  boost: Option<f32>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  _name: Option<String>,
}

impl Query {
  /// Creates an instance of [`BoolQuery`]
  pub fn bool() -> BoolQuery {
    BoolQuery::default()
  }
}

impl BoolQuery {
  add_boost_and_name!();

  /// The clause (query) must appear in matching documents and will contribute
  /// to the score.
  pub fn must<T>(mut self, query: T) -> Self
  where
    T: IntoIterator,
    T::Item: Into<Query>, {
    self.must.extend(query);
    self
  }

  /// The clause (query) should appear in the matching document.
  pub fn should<T>(mut self, query: T) -> Self
  where
    T: IntoIterator,
    T::Item: Into<Query>, {
    self.should.extend(query);
    self
  }

  /// The clause (query) must appear in matching documents.
  /// However unlike must the score of the query will be ignored.
  /// Filter clauses are executed in [filter context](https://www.elastic.co/guide/en/opensearch/reference/current/query-filter-context.html),
  /// meaning that scoring is ignored and clauses are considered for caching.
  pub fn filter<T>(mut self, query: T) -> Self
  where
    T: IntoIterator,
    T::Item: Into<Query>, {
    self.filter.extend(query);
    self
  }

  /// The clause (query) must not appear in the matching documents.
  /// Clauses are executed in [filter context](https://www.elastic.co/guide/en/opensearch/reference/current/query-filter-context.html)
  /// meaning that scoring is ignored and clauses are considered for caching.
  /// Because scoring is ignored, a score of `0` for all documents is returned.
  pub fn must_not<T>(mut self, query: T) -> Self
  where
    T: IntoIterator,
    T::Item: Into<Query>, {
    self.must_not.extend(query);
    self
  }

  /// You can use the `minimum_should_match` parameter to specify the number
  /// or percentage of should clauses returned documents must match.
  ///
  /// If the `bool` query includes at least one `should` clause and no
  /// `must` or `filter` clauses, the default value is `1`.
  /// Otherwise, the default value is `0`.
  ///
  /// For other valid values, see the
  /// [minimum_should_match parameter](https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-minimum-should-match.html).
  pub fn minimum_should_match<T>(mut self, minimum_should_match: T) -> Self
  where
    T: ToString, {
    self.minimum_should_match = Some(minimum_should_match.to_string());
    self
  }
}

impl ShouldSkip for BoolQuery {
  fn should_skip(&self) -> bool {
    self.must.should_skip() && self.filter.should_skip() && self.should.should_skip() && self.must_not.should_skip()
  }
}

serialize_with_root!("bool": BoolQuery);
deserialize_with_root!("bool": BoolQuery);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialization() {
    assert_serialize_query(Query::bool(), json!({ "bool": {} }));

    assert_serialize_query(
      Query::bool()
        .must([Query::term("test1", 1), Query::term("test2", 2)])
        .should([Query::term("test1", 3), Query::term("test2", 4)])
        .filter([Query::term("test1", 5), Query::term("test2", 6)])
        .must_not([Query::term("test1", 7), Query::term("test2", 8)])
        .minimum_should_match("2")
        .boost(1.3)
        .name("test"),
      json!({
          "bool": {
              "must":[
                  { "term": { "test1": {"value": 1} } },
                  { "term": { "test2": {"value": 2} } },
              ],
              "should":[
                  { "term": { "test1": {"value": 3} } },
                  { "term": { "test2": {"value": 4} } },
              ],
              "filter":[
                  { "term": { "test1": {"value": 5} } },
                  { "term": { "test2": {"value": 6} } },
              ],
              "must_not":[
                  { "term": { "test1": {"value": 7} } },
                  { "term": { "test2": {"value": 8} } },
              ],
              "minimum_should_match": "2",
              "boost": 1.3,
              "_name":"test"
          }
      }),
    );

    assert_serialize_query(
      Query::bool()
        .must(Query::term("test1", 1))
        .must(Query::term("test2", 2))
        .should(Query::term("test1", 3))
        .should(Query::term("test2", 4))
        .filter(Query::term("test1", 5))
        .filter(Query::term("test2", 6))
        .must_not(Query::term("test1", 7))
        .must_not(Query::term("test2", 8))
        .minimum_should_match("2")
        .boost(1.3)
        .name("test"),
      json!({
          "bool": {
              "must":[
                  { "term": { "test1": {"value": 1} } },
                  { "term": { "test2": {"value": 2} } },
              ],
              "should":[
                  { "term": { "test1": {"value": 3} } },
                  { "term": { "test2": {"value": 4} } },
              ],
              "filter":[
                  { "term": { "test1": {"value": 5} } },
                  { "term": { "test2": {"value": 6} } },
              ],
              "must_not":[
                  { "term": { "test1": {"value": 7} } },
                  { "term": { "test2": {"value": 8} } },
              ],
              "minimum_should_match": "2",
              "boost": 1.3,
              "_name":"test"
          }
      }),
    );
  }
}
