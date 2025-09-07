use crate::{search::*, util::*};

/// Returns documents that contain the words of a provided text, in the **same
/// order** as provided. The last term of the provided text is treated as a
/// [prefix](crate::PrefixQuery), matching any words that begin with that term.
///
/// To create a MatchPhrasePrefix query:
/// ```
/// # use opensearch_dsl::queries::*;
/// # use opensearch_dsl::queries::params::*;
/// # let query =
/// Query::match_phrase_prefix("test", "search text")
///   .boost(2)
///   .name("test");
/// ```
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-match-query-phrase-prefix.html>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct MatchPhrasePrefixQuery {
    #[serde(skip)]
    field: String,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    query: Text,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    max_expansions: Option<u8>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    slop: Option<u8>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    zero_terms_query: Option<ZeroTermsQuery>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`MatchPhrasePrefixQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `query` - Text you wish to find in the provided `<field>`. <br/> The
    ///   `match_phrase_prefix`
    /// query analyzes any provided text into tokens before performing a search.
    /// The last term of this text is treated as a [prefix](crate::PrefixQuery),
    /// matching any words that begin with that term.
    pub fn match_phrase_prefix<T, U>(field: T, query: U) -> MatchPhrasePrefixQuery
    where
        T: ToString,
        U: Into<Text>,
    {
        MatchPhrasePrefixQuery {
            field: field.to_string(),
            query: query.into(),
            analyzer: None,
            max_expansions: None,
            slop: None,
            zero_terms_query: None,
            boost: None,
            _name: None,
        }
    }
}

impl MatchPhrasePrefixQuery {
    add_boost_and_name!();

    /// [Analyzer](https://www.elastic.co/guide/en/opensearch/reference/current/analysis.html)
    /// used to convert the text in the `query` value into tokens. Defaults to the
    /// [index-time analyzer](https://www.elastic.co/guide/en/opensearch/reference/current/specify-analyzer.html#specify-index-time-analyzer)
    /// mapped for the `<field>`. If no analyzer is mapped, the index’s default
    /// analyzer is used.
    pub fn analyzer<T>(mut self, analyzer: T) -> Self
    where
        T: ToString,
    {
        self.analyzer = Some(analyzer.to_string());
        self
    }

    /// Maximum number of terms to which the query will expand.
    /// Defaults to `50`.
    pub fn max_expansions(mut self, max_expansions: u8) -> Self {
        self.max_expansions = Some(max_expansions);
        self
    }

    /// The maximum number of intervening unmatched positions, as well as
    /// whether matches are required to be in-order.
    pub fn slop(mut self, slop: u8) -> Self {
        self.slop = Some(slop);
        self
    }

    /// Indicates whether no documents are returned if the `analyzer` removes
    /// all tokens, such as when using a `stop` filter.
    pub fn zero_terms_query(mut self, zero_terms_query: ZeroTermsQuery) -> Self {
        self.zero_terms_query = Some(zero_terms_query);
        self
    }
}

impl ShouldSkip for MatchPhrasePrefixQuery {
    fn should_skip(&self) -> bool {
        self.query.should_skip()
    }
}

serialize_with_root_keyed!("match_phrase_prefix": MatchPhrasePrefixQuery);
deserialize_with_root_keyed!("match_phrase_prefix": MatchPhrasePrefixQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::match_phrase_prefix("test", "search text"),
            json!({
                "match_phrase_prefix": {
                    "test": {
                        "query": "search text"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::match_phrase_prefix("test", "search text")
                .analyzer("search_time_analyzer")
                .max_expansions(20)
                .slop(5)
                .zero_terms_query(ZeroTermsQuery::None)
                .boost(2)
                .name("test"),
            json!({
                "match_phrase_prefix": {
                    "test": {
                        "query": "search text",
                        "analyzer": "search_time_analyzer",
                        "max_expansions": 20,
                        "slop": 5,
                        "zero_terms_query": "none",
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            }),
        );
    }
}
