use serde::{de, Deserialize, Serialize};

use crate::{search::*, util::*};
/// Returns documents that contain one or more **exact** terms in a provided
/// field. The terms query is the same as the term query, except you can search
/// for multiple values.
///
/// To create a terms query with numeric values:
/// ```
/// # use opensearch_dsl::queries::*;
/// # use opensearch_dsl::queries::params::*;
/// # let query =
/// Query::terms("test", vec![123]);
/// ```
/// To create a terms query with string values and optional fields:
/// ```
/// # use opensearch_dsl::queries::*;
/// # use opensearch_dsl::queries::params::*;
/// # let query =
/// Query::terms("test", vec!["username"]).boost(2).name("test");
/// ```
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-terms-query.html>
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct TermsQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip)]
    terms: Terms,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`TermsQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `values` - An array of terms you wish to find in the provided field. To
    ///   return a
    /// document, one or more terms must exactly match a field value,
    /// including whitespace and capitalization.<br/>
    /// By default, OpenSearch limits the `terms` query to a maximum of
    /// 65,536 terms. You can change this limit using the
    /// [`index.max_terms_count setting`](https://www.elastic.co/guide/en/opensearch/reference/current/index-modules.html#index-max-terms-count).<br/>
    /// > To use the field values of an existing document as search terms,
    /// use the terms lookup parameters.
    pub fn terms<S, I>(field: S, terms: I) -> TermsQuery
    where
        S: ToString,
        I: Into<Terms>,
    {
        TermsQuery {
            field: field.to_string(),
            terms: terms.into(),
            boost: None,
            _name: None,
        }
    }
}

impl TermsQuery {
    add_boost_and_name!();
}

impl ShouldSkip for TermsQuery {
    fn should_skip(&self) -> bool {
        self.terms.should_skip()
    }
}

serialize_with_root_key_value_pair!("terms": TermsQuery, field, terms);

impl<'de> Deserialize<'de> for TermsQuery {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        use std::fmt;
        struct WrapperVisitor;

        impl<'de> de::Visitor<'de> for WrapperVisitor {
            type Value = TermsQuery;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Wrapper")
            }

            fn visit_map<A>(self, mut map: A) -> Result<TermsQuery, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut terms_query: TermsQuery = TermsQuery::default();
                let mut found_terms = false;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "terms" {
                        let inner_map =
                            map.next_value::<serde_json::Map<String, serde_json::Value>>()?;
                        for (k, v) in inner_map.iter() {
                            match k.as_str() {
                                "field" => match v.as_str() {
                                    Some(field) => {
                                        terms_query.field = field.to_string();
                                    }
                                    None => {
                                        return Err(de::Error::invalid_type(
                                            de::Unexpected::Other("not a string"),
                                            &"a string",
                                        ));
                                    }
                                },
                                "boost" => match v.as_f64() {
                                    Some(boost) => {
                                        terms_query.boost = Some(boost as f32);
                                    }
                                    None => {
                                        return Err(de::Error::invalid_type(
                                            de::Unexpected::Other("not a float"),
                                            &"a float",
                                        ));
                                    }
                                },
                                "_name" => match v.as_str() {
                                    Some(_name) => {
                                        terms_query._name = Some(_name.to_string());
                                    }
                                    None => {
                                        return Err(de::Error::invalid_type(
                                            de::Unexpected::Other("not a string"),
                                            &"a string",
                                        ));
                                    }
                                },
                                "values" => {
                                    let values = serde_json::from_value::<Terms>(v.clone());
                                    match values {
                                        Ok(values) => {
                                            terms_query.terms = values;
                                            found_terms = true;
                                        }
                                        Err(e) => {
                                            return Err(de::Error::custom(format!(
                                                "error parsing terms: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                                _ => {
                                    terms_query.field = k.to_owned();
                                    let values = serde_json::from_value::<Terms>(v.clone());
                                    match values {
                                        Ok(values) => {
                                            terms_query.terms = values;
                                            found_terms = true;
                                        }
                                        Err(e) => {
                                            return Err(de::Error::custom(format!(
                                                "error parsing terms: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if found_terms {
                    Ok(terms_query)
                } else {
                    Err(de::Error::missing_field("values or terms array"))
                }
            }
        }

        deserializer.deserialize_struct("Wrapper", &["terms"], WrapperVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::terms("test", [12, 13, 123]),
            json!({"terms": { "test": [12, 13, 123] } }),
        );

        assert_serialize_query(
            Query::terms("test", [123]).boost(2).name("test"),
            json!({
                "terms": {
                    "test": [123],
                    "boost": 2.0,
                    "_name": "test",
                }
            }),
        );
    }

    #[test]
    fn should_skip_when_there_are_no_values() {
        let values: Vec<i32> = Vec::new();
        let query = Query::terms("test", values);

        assert!(query.should_skip())
    }

    #[test]
    fn should_not_skip_when_there_are_no_values() {
        let query = Query::terms("test", [123]);

        assert!(!query.should_skip())
    }
}
