use serde::{Deserialize, Deserializer, Serialize};

use crate::{search::*, util::*};

/// Queries documents that contain fields indexed using the `shape` type.
///
/// Requires the [`shape` Mapping](https://www.elastic.co/guide/en/opensearch/reference/current/shape.html).
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-shape-query.html>
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct ShapeQuery {
  #[serde(skip)]
  field: String,

  #[serde(skip)]
  shape: InlineShape,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  ignore_unmapped: Option<bool>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  boost: Option<f32>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  _name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
struct InlineShape {
  shape: Shape,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  relation: Option<SpatialRelation>,
}

impl Query {
  /// Creates an instance of [`ShapeQuery`]
  ///
  /// - `field` - Field you wish to search
  /// - `shape` - Shape you with to search
  pub fn shape<S, T>(field: S, shape: T) -> ShapeQuery
  where
    S: ToString,
    T: Into<Shape>, {
    ShapeQuery {
      field: field.to_string(),
      shape: InlineShape {
        shape: shape.into(),
        relation: None,
      },
      ignore_unmapped: None,
      boost: None,
      _name: None,
    }
  }
}

impl ShapeQuery {
  add_boost_and_name!();

  /// The [shape strategy](https://www.elastic.co/guide/en/opensearch/reference/current/geo-shape.html#spatial-strategy)
  /// mapping parameter determines which spatial relation operators may be
  /// used at search time.
  pub fn relation(mut self, relation: SpatialRelation) -> Self {
    self.shape.relation = Some(relation);
    self
  }

  /// When set to true the `ignore_unmapped` option will ignore an unmapped
  /// field and will not match any documents for this query. This can be
  /// useful when querying multiple indexes which might have different
  /// mappings. When set to `false` (the default value) the query will throw
  /// an exception if the field is not mapped.
  pub fn ignore_unmapped(mut self, ignore_unmapped: bool) -> Self {
    self.ignore_unmapped = Some(ignore_unmapped);
    self
  }
}

impl ShouldSkip for ShapeQuery {}

serialize_with_root_key_value_pair!("shape": ShapeQuery, field, shape);
impl<'de> Deserialize<'de> for ShapeQuery {
  fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
  where
    D: Deserializer<'de>, {
    use std::fmt;
    struct WrapperVisitor;

    impl<'de> serde::de::Visitor<'de> for WrapperVisitor {
      type Value = ShapeQuery;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct shape")
      }

      fn visit_map<A>(self, mut map: A) -> Result<ShapeQuery, A::Error>
      where
        A: serde::de::MapAccess<'de>, {
        let mut query: ShapeQuery = ShapeQuery::default();
        let mut found_value = false;

        while let Some(key) = map.next_key::<String>()? {
          if key == "shape" {
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
                "ignore_unmapped" => {
                  match v.as_bool() {
                    Some(ignore_unmapped) => {
                      query.ignore_unmapped = Some(ignore_unmapped);
                    }
                    None => {
                      return Err(serde::de::Error::invalid_type(
                        serde::de::Unexpected::Other("not a boolean"),
                        &"a boolean",
                      ));
                    }
                  }
                }
                "value" => {
                  let shape = serde_json::from_value::<InlineShape>(v.clone());
                  match shape {
                    Ok(shape) => {
                      query.shape = shape;
                      found_value = true;
                    }
                    Err(e) => {
                      return Err(serde::de::Error::custom(format!("error parsing distance: {}", e)));
                    }
                  }
                }
                _ => {
                  query.field = k.to_owned();
                  let value = serde_json::from_value::<InlineShape>(v.clone());
                  match value {
                    Ok(value) => {
                      query.shape = value;
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

    deserializer.deserialize_struct("Wrapper", &["shape"], WrapperVisitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_serialization() {
    assert_serialize_query(
      Query::shape("pin.location", Shape::point([2.2, 1.1])),
      json!({
          "shape": {
              "pin.location": {
                  "shape": {
                      "type": "point",
                      "coordinates": [2.2, 1.1]
                  }
              },
          }
      }),
    );

    assert_serialize_query(
      Query::shape("pin.location", Shape::point([2.2, 1.1]))
        .boost(2)
        .name("test")
        .ignore_unmapped(true)
        .relation(SpatialRelation::Within),
      json!({
          "shape": {
              "_name": "test",
              "boost": 2.0,
              "ignore_unmapped": true,
              "pin.location": {
                  "shape": {
                      "type": "point",
                      "coordinates": [2.2, 1.1]
                  },
                  "relation": "WITHIN"
              },
          }
      }),
    );
  }
}
