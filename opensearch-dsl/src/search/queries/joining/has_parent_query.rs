use crate::{search::*, util::*};

/// Returns child documents joined to a specific parent document. You can use a
/// join field mapping to create parent-child relationships between documents in
/// the same index.
///
/// To create has_parent query:
/// ```
/// # use opensearch_dsl::queries::*;
/// # use opensearch_dsl::queries::params::*;
/// # let query =
/// Query::has_parent("parent", Query::term("tag", "opensearch"));
/// ```
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-HasParent-query.html>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct HasParentQuery {
    parent_type: String,

    query: Box<Query>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    score: Option<bool>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    ignore_unmapped: Option<bool>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`HasParentQuery`]
    ///
    /// - `parent-type` - Name of the parent relationship mapped for the join
    ///   field.
    /// - `query` - Query you wish to run on parent documents of the `parent_type`
    ///   field. If a
    /// parent document matches the search, the query returns its child documents.
    pub fn has_parent<T, U>(parent_type: T, query: U) -> HasParentQuery
    where
        T: ToString,
        U: Into<Query>,
    {
        HasParentQuery {
            parent_type: parent_type.to_string(),
            query: Box::new(query.into()),
            score: None,
            ignore_unmapped: None,
            boost: None,
            _name: None,
        }
    }
}

impl HasParentQuery {
    add_boost_and_name!();

    /// Indicates whether the relevance score of a matching parent document is
    /// aggregated into its child documents. Defaults to `false`.
    ///
    /// If `false`, OpenSearch ignores the relevance score of the parent document.
    /// OpenSearch also assigns each child document a relevance score equal to
    /// the `query`'s `boost`, which defaults to `1`.
    ///
    /// If `true`, the relevance score of the matching parent document is
    /// aggregated into its child documents' relevance scores.
    pub fn score(mut self, score: bool) -> Self {
        self.score = Some(score);
        self
    }

    /// Indicates whether to ignore an unmapped `parent_type` and not return any
    /// documents instead of an error. Defaults to `false`.
    ///
    /// If `false`, OpenSearch returns an error if the `parent_type` is unmapped.
    ///
    /// You can use this parameter to query multiple indices that may not contain
    /// the `parent_type`.
    pub fn ignore_unmapped(mut self, ignore_unmapped: bool) -> Self {
        self.ignore_unmapped = Some(ignore_unmapped);
        self
    }
}

impl ShouldSkip for HasParentQuery {
    fn should_skip(&self) -> bool {
        self.parent_type.should_skip() || self.query.should_skip()
    }
}

serialize_with_root!("has_parent": HasParentQuery);
deserialize_with_root!("has_parent": HasParentQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::has_parent("parent", Query::term("tag", "opensearch")),
            json!({
                "has_parent": {
                    "parent_type": "parent",
                    "query": {
                        "term": {
                            "tag": {
                                "value": "opensearch"
                            }
                        }
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::has_parent("parent", Query::term("tag", "opensearch"))
                .boost(2)
                .name("test")
                .ignore_unmapped(true)
                .score(true),
            json!({
                "has_parent": {
                    "parent_type": "parent",
                    "score": true,
                    "ignore_unmapped": true,
                    "query": {
                        "term": {
                            "tag": {
                                "value": "opensearch"
                            }
                        }
                    },
                    "boost": 2.0,
                    "_name": "test"
                }
            }),
        );
    }
}
