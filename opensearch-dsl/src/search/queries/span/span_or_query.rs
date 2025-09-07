use serde::Serialize;

use super::SpanQuery;
use crate::{util::*, Query};

/// Matches the union of its span clauses. The span or query maps to Lucene
/// `SpanOrQuery`.
///
/// The `clauses` element is a list of one or more other span type queries.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-span-or-query.html>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct SpanOrQuery {
    clauses: Vec<SpanQuery>,
}

impl ShouldSkip for SpanOrQuery {}

impl Query {
    /// Creates an instance of [`SpanOrQuery`]
    pub fn span_or<T>(clauses: T) -> SpanOrQuery
    where
        T: IntoIterator,
        T::Item: Into<SpanQuery>,
    {
        SpanOrQuery {
            clauses: clauses.into_iter().map(Into::into).collect(),
        }
    }
}

serialize_with_root!("span_or": SpanOrQuery);
deserialize_with_root!("span_or": SpanOrQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_or([Query::span_term("test", 1234)]),
            json!({
                "span_or": {
                    "clauses": [
                        {
                            "span_term": {
                                "test": {
                                    "value": 1234
                                }
                            }
                        }
                    ]
                }
            }),
        );

        assert_serialize_query(
            Query::span_or([Query::span_term("test", 1234)]),
            json!({
                "span_or": {
                    "clauses": [
                        {
                            "span_term": {
                                "test": {
                                    "value": 1234
                                }
                            }
                        }
                    ]
                }
            }),
        );
    }
}
