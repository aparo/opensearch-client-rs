use serde::{Deserialize, Deserializer, Serialize};

use crate::{search::*, util::*};

/// Terms lookup fetches the field values of an existing document.
/// OpenSearch then uses those values as search terms. This can be
/// helpful when searching for a large set of terms.
///
/// Because terms lookup fetches values from a document, the
/// [`_source`](https://www.elastic.co/guide/en/opensearch/reference/current/mapping-source-field.html)
/// mapping field must be enabled to use terms lookup. The `_source`
/// field is enabled by default.
///
/// > By default, OpenSearch limits the `terms` query to a maximum of
/// 65,536 terms. This includes terms fetched using terms lookup. You can
/// change this limit using the
/// [`index.max_terms_count setting`](https://www.elastic.co/guide/en/opensearch/reference/current/index-modules.html#index-max-terms-count).
///
/// To create a terms lookup query:
/// ```
/// # use opensearch_dsl::queries::*;
/// # use opensearch_dsl::queries::params::*;
/// # let query =
/// Query::terms_lookup("test", "index", "id", "path")
///   .routing("routing")
///   .boost(1.3)
///   .name("lookup");
/// ```
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-terms-query.html>
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct TermsLookupQuery {
  #[serde(skip)]
  field: String,

  #[serde(skip)]
  terms_lookup: TermsLookup,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  boost: Option<f32>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  _name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
struct TermsLookup {
  index: String,
  id: String,
  path: String,
  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  routing: Option<String>,
}

impl Query {
  /// Creates an instance of [`TermsLookupQuery`]
  ///
  /// - `field` - Field you wish to search.
  /// - `index` - Name of the index from which to fetch field values.
  /// - `id` - [ID](https://www.elastic.co/guide/en/opensearch/reference/current/mapping-id-field.html)
  /// of the document from which to fetch field values.
  /// - `path` - Name of the field from which to fetch field values. OpenSearch
  ///   uses
  /// these values as search terms for the query. If the field values
  /// include an array of nested inner objects, you can access those objects
  /// using dot notation syntax.
  pub fn terms_lookup<S, T, U, V>(field: S, index: T, id: U, path: V) -> TermsLookupQuery
  where
    S: ToString,
    T: ToString,
    U: ToString,
    V: ToString, {
    TermsLookupQuery {
      field: field.to_string(),
      terms_lookup: TermsLookup {
        index: index.to_string(),
        id: id.to_string(),
        path: path.to_string(),
        routing: None,
      },
      boost: None,
      _name: None,
    }
  }
}

impl TermsLookupQuery {
  add_boost_and_name!();
}

impl TermsLookupQuery {
  /// Custom [routing value](https://www.elastic.co/guide/en/opensearch/reference/current/mapping-routing-field.html)
  /// of the document from which to fetch term values. If a custom routing
  /// value was provided when the document was indexed, this parameter is
  /// required.
  pub fn routing<S>(mut self, routing: S) -> Self
  where
    S: ToString, {
    self.terms_lookup.routing = Some(routing.to_string());
    self
  }
}

impl ShouldSkip for TermsLookupQuery {}

serialize_with_root_key_value_pair!("terms": TermsLookupQuery, field, terms_lookup);
impl<'de> Deserialize<'de> for TermsLookupQuery {
  fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
  where
    D: Deserializer<'de>, {
    use std::fmt;
    struct WrapperVisitor;

    impl<'de> serde::de::Visitor<'de> for WrapperVisitor {
      type Value = TermsLookupQuery;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct terms")
      }

      fn visit_map<A>(self, mut map: A) -> Result<TermsLookupQuery, A::Error>
      where
        A: serde::de::MapAccess<'de>, {
        let mut query: TermsLookupQuery = TermsLookupQuery::default();
        let mut found_value = false;

        while let Some(key) = map.next_key::<String>()? {
          if key == "terms" {
            let inner_map = map.next_value::<serde_json::Map<String, serde_json::Value>>()?;
            for (k, v) in inner_map.iter() {
              match k.as_str() {
                "field" => {
                  match v.as_str() {
                    Some(field) => {
                      query.field = field.to_string();
                    }
                    None => {
                      return Err(serde::de::Error::invalid_type(
                        serde::de::Unexpected::Other("not a string"),
                        &"a string",
                      ));
                    }
                  }
                }
                "boost" => {
                  match v.as_f64() {
                    Some(boost) => {
                      query.boost = Some(boost as f32);
                    }
                    None => {
                      return Err(serde::de::Error::invalid_type(
                        serde::de::Unexpected::Other("not a float"),
                        &"a float",
                      ));
                    }
                  }
                }
                "_name" => {
                  match v.as_str() {
                    Some(_name) => {
                      query._name = Some(_name.to_string());
                    }
                    None => {
                      return Err(serde::de::Error::invalid_type(
                        serde::de::Unexpected::Other("not a string"),
                        &"a string",
                      ));
                    }
                  }
                }
                "value" => {
                  let terms_lookup = serde_json::from_value::<TermsLookup>(v.clone());
                  match terms_lookup {
                    Ok(terms_lookup) => {
                      query.terms_lookup = terms_lookup;
                      found_value = true;
                    }
                    Err(e) => {
                      return Err(serde::de::Error::custom(format!("error parsing distance: {}", e)));
                    }
                  }
                }
                _ => {
                  query.field = k.to_owned();
                  let terms_lookup = serde_json::from_value::<TermsLookup>(v.clone());
                  match terms_lookup {
                    Ok(terms_lookup) => {
                      query.terms_lookup = terms_lookup;
                      found_value = true;
                    }
                    Err(e) => {
                      return Err(serde::de::Error::custom(format!("error parsing {}: {}", k, e)));
                    }
                  }
                }
              }
            }
          }
        }
        if found_value {
          Ok(query)
        } else {
          Err(serde::de::Error::missing_field("location value"))
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
      Query::terms_lookup("test", "index_value", "id_value", "path_value"),
      json!({
          "terms": {
              "test": {
                  "index": "index_value",
                  "id": "id_value",
                  "path": "path_value",
              }
          }
      }),
    );

    assert_serialize_query(
      Query::terms_lookup("test", "index_value", "id_value", "path_value")
        .routing("routing_value")
        .boost(2)
        .name("test"),
      json!({
          "terms": {
              "test": {
                  "index": "index_value",
                  "id": "id_value",
                  "path": "path_value",
                  "routing": "routing_value"
              },
              "boost": 2.0,
              "_name": "test",
          }
      }),
    );
  }
}
