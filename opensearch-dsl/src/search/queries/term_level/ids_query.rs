use crate::{search::*, util::*, Set};

/// Returns documents based on their IDs. This query uses document IDs stored in
/// the [`_id`](https://www.elastic.co/guide/en/opensearch/reference/current/mapping-id-field.html)
/// field.
///
/// To create IDs query:
/// ```
/// # use opensearch_dsl::queries::*;
/// # use opensearch_dsl::queries::params::*;
/// # let query =
/// Query::ids(vec!["2"]);
/// ```
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-ids-query.html>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct IdsQuery {
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    values: Set<String>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`IdsQuery`]
    ///
    /// - `values` - An array of
    /// [document IDs](https://www.elastic.co/guide/en/opensearch/reference/current/mapping-id-field.html).
    pub fn ids<I>(values: I) -> IdsQuery
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        IdsQuery {
            values: values.into_iter().map(|value| value.to_string()).collect(),
            boost: None,
            _name: None,
        }
    }
}

impl IdsQuery {
    add_boost_and_name!();
}

impl ShouldSkip for IdsQuery {
    fn should_skip(&self) -> bool {
        self.values.should_skip()
    }
}

serialize_with_root!("ids": IdsQuery);
deserialize_with_root!("ids": IdsQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::ids(vec![1, 3, 2, 5, 4, 6]),
            json!({
                "ids": {
                    "values": ["1", "2", "3", "4", "5", "6"],
                }
            }),
        );

        assert_serialize_query(
            Query::ids(vec![1, 3, 2, 5, 4, 6]).boost(1.3).name("test"),
            json!({
                "ids": {
                    "values": ["1", "2", "3", "4", "5", "6"],
                    "boost": 1.3,
                    "_name": "test"
                }
            }),
        );
    }
}
