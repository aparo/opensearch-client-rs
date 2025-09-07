use std::fmt;

use serde::{
    de::{Error, Unexpected, Visitor},
    ser::{Serialize, Serializer},
    Deserialize, Deserializer,
};
/// Method used to rewrite the query.
///
/// > **This parameter is for expert users only. Changing the value of this
/// parameter can impact search performance and relevance.**
///
/// The `rewrite` parameter determines:
/// - How Lucene calculates the relevance scores for each matching document
/// - Whether Lucene changes the original query to a `bool` query or bit set
/// - If changed to a `bool` query, which `term` query clauses are included
///
/// **Performance considerations for the rewrite parameter**
///
/// For most uses, we recommend using the `constant_score`,
/// `constant_score_boolean`, or `top_terms_boost_N` rewrite methods.
///
/// Other methods calculate relevance scores. These score calculations
/// are often expensive and do not improve query results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rewrite {
    /// Uses the constant_score_boolean method for fewer matching terms.
    /// Otherwise, this method finds all matching terms in sequence and returns
    /// matching documents using a bit set.
    ConstantScore,

    /// Assigns each document a relevance score equal to the `boost` parameter.
    ///
    /// This method changes the original query to a
    /// [`bool` query](crate::queries::BoolQuery). This `bool` query contains a
    /// `should` clause and [`term` query](crate::queries::TermQuery)
    /// for each matching term.
    ///
    /// This method can cause the final `bool` query to exceed the clause limit
    /// in the
    /// [`indices.query.bool.max_clause_count`](https://www.elastic.co/guide/en/opensearch/reference/current/search-settings.html#indices-query-bool-max-clause-count)
    /// setting. If the query exceeds this limit, OpenSearch returns an error.
    ConstantScoreBoolean,

    /// Calculates a relevance score for each matching document.
    ///
    /// This method changes the original query to a
    /// [`bool` query](crate::queries::BoolQuery). This `bool` query contains a
    /// `should` clause and [`term` query](crate::queries::TermQuery)
    /// for each matching term.
    ///
    /// This method can cause the final `bool` query to exceed the clause limit
    /// in the
    /// [`indices.query.bool.max_clause_count`](https://www.elastic.co/guide/en/opensearch/reference/current/search-settings.html#indices-query-bool-max-clause-count)
    /// setting. If the query exceeds this limit, OpenSearch returns an error.
    ScoringBoolean,

    /// Calculates a relevance score for each matching document as if all terms
    /// had the same frequency. This frequency is the maximum frequency of all
    /// matching terms.
    ///
    /// This method changes the original query to a
    /// [`bool` query](crate::queries::BoolQuery). This `bool` query contains a
    /// `should` clause and [`term` query](crate::queries::TermQuery)
    /// for each matching term.
    ///
    /// The final `bool` query only includes `term` queries for the top `N`
    /// scoring terms.
    ///
    /// You can use this method to avoid exceeding the clause limit in the
    /// [`indices.query.bool.max_clause_count`](https://www.elastic.co/guide/en/opensearch/reference/current/search-settings.html#indices-query-bool-max-clause-count)
    /// setting.
    TopTermsBlendedFrequencies(u64),

    /// Assigns each matching document a relevance score equal to the boost
    /// parameter.
    ///
    /// This method changes the original query to a
    /// [`bool` query](crate::queries::BoolQuery). This `bool` query contains a
    /// `should` clause and [`term` query](crate::queries::TermQuery)
    /// for each matching term.
    ///
    /// The final `bool` query only includes `term` queries for the top `N`
    /// terms.
    ///
    /// You can use this method to avoid exceeding the clause limit in the
    /// [`indices.query.bool.max_clause_count`](https://www.elastic.co/guide/en/opensearch/reference/current/search-settings.html#indices-query-bool-max-clause-count)
    /// setting.
    TopTermsBoost(u64),

    /// Calculates a relevance score for each matching document.
    ///
    /// This method changes the original query to a
    /// [`bool` query](crate::queries::BoolQuery). This `bool` query contains a
    /// `should` clause and [`term` query](crate::queries::TermQuery)
    /// for each matching term.
    ///
    /// The final `bool` query only includes `term` queries for the top `N`
    /// scoring terms.
    ///
    /// You can use this method to avoid exceeding the clause limit in the
    /// [`indices.query.bool.max_clause_count`](https://www.elastic.co/guide/en/opensearch/reference/current/search-settings.html#indices-query-bool-max-clause-count)
    /// setting.
    TopTerms(u64),
}

impl Serialize for Rewrite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ConstantScore => "constant_score".serialize(serializer),
            Self::ConstantScoreBoolean => "constant_score_boolean".serialize(serializer),
            Self::ScoringBoolean => "scoring_boolean".serialize(serializer),
            Self::TopTermsBlendedFrequencies(n) => {
                format!("top_terms_blended_freqs_{n}").serialize(serializer)
            }
            Self::TopTermsBoost(n) => format!("top_terms_boost_{n}").serialize(serializer),
            Self::TopTerms(n) => format!("top_terms_{n}").serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for Rewrite {
    fn deserialize<D>(deserializer: D) -> Result<Rewrite, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RewriteVisitor;

        impl<'de> Visitor<'de> for RewriteVisitor {
            type Value = Rewrite;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a Rewrite variant")
            }

            fn visit_str<E>(self, value: &str) -> Result<Rewrite, E>
            where
                E: Error,
            {
                match value {
                    "constant_score" => Ok(Rewrite::ConstantScore),
                    "constant_score_boolean" => Ok(Rewrite::ConstantScoreBoolean),
                    "scoring_boolean" => Ok(Rewrite::ScoringBoolean),
                    s if s.starts_with("top_terms_blended_freqs_") => {
                        let n = s[23..].parse::<u64>().map_err(E::custom)?;
                        Ok(Rewrite::TopTermsBlendedFrequencies(n))
                    }
                    s if s.starts_with("top_terms_boost_") => {
                        let n = s[15..].parse::<u64>().map_err(E::custom)?;
                        Ok(Rewrite::TopTermsBoost(n))
                    }
                    s if s.starts_with("top_terms_") => {
                        let n = s[10..].parse::<u64>().map_err(E::custom)?;
                        Ok(Rewrite::TopTerms(n))
                    }
                    _ => Err(E::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(RewriteVisitor)
    }
}
