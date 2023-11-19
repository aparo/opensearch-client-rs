use serde::{Deserialize, Deserializer, Serialize};

use crate::{search::*, util::*};

/// Filter documents indexed using the `geo_shape` or `geo_point` type.
///
/// Requires the
/// [`geo_shape` mapping](https://www.elastic.co/guide/en/opensearch/reference/current/geo-shape.html)
/// or the
/// [`geo_point` mapping](https://www.elastic.co/guide/en/opensearch/reference/current/geo-point.html).
///
/// The `geo_shape` query uses the same grid square representation as the
/// `geo_shape` mapping to find documents that have a shape that intersects
/// with the query shape. It will also use the same Prefix Tree configuration
/// as defined for the field mapping.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-geo-shape-query.html>
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct GeoShapeLookupQuery {
  #[serde(skip)]
  field: String,

  #[serde(skip)]
  shape: Shape,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  ignore_unmapped: Option<bool>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  boost: Option<f32>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  _name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Shape {
  indexed_shape: IndexedShape,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  relation: Option<SpatialRelation>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
struct IndexedShape {
  id: String,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  index: Option<String>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  path: Option<String>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  routing: Option<String>,
}

impl Query {
  /// Creates an instance of [`GeoShapeLookupQuery`]
  ///
  /// - `field` - Field you wish to search
  /// - `id` - The ID of the document that containing the pre-indexed shape
  pub fn geo_shape_lookup<S, T>(field: S, id: T) -> GeoShapeLookupQuery
  where
    S: ToString,
    T: ToString, {
    GeoShapeLookupQuery {
      field: field.to_string(),
      shape: Shape {
        indexed_shape: IndexedShape {
          id: id.to_string(),
          index: None,
          path: None,
          routing: None,
        },
        relation: None,
      },
      ignore_unmapped: None,
      boost: None,
      _name: None,
    }
  }
}

impl GeoShapeLookupQuery {
  add_boost_and_name!();

  /// Name of the index where the pre-indexed shape is. Defaults to shapes
  pub fn index<S>(mut self, index: S) -> Self
  where
    S: ToString, {
    self.shape.indexed_shape.index = Some(index.to_string());
    self
  }

  /// The field specified as path containing the pre-indexed shape. Defaults to
  /// shape
  pub fn path<S>(mut self, path: S) -> Self
  where
    S: ToString, {
    self.shape.indexed_shape.path = Some(path.to_string());
    self
  }

  /// The routing of the shape document
  pub fn routing<S>(mut self, routing: S) -> Self
  where
    S: ToString, {
    self.shape.indexed_shape.routing = Some(routing.to_string());
    self
  }

  /// The [geo_shape strategy](https://www.elastic.co/guide/en/opensearch/reference/current/geo-shape.html#spatial-strategy)
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

impl ShouldSkip for GeoShapeLookupQuery {}

serialize_with_root_key_value_pair!("geo_shape": GeoShapeLookupQuery, field, shape);
impl<'de> Deserialize<'de> for GeoShapeLookupQuery {
  fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
  where
    D: Deserializer<'de>, {
    use std::fmt;
    struct WrapperVisitor;

    impl<'de> serde::de::Visitor<'de> for WrapperVisitor {
      type Value = GeoShapeLookupQuery;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct geo_shape")
      }

      fn visit_map<A>(self, mut map: A) -> Result<GeoShapeLookupQuery, A::Error>
      where
        A: serde::de::MapAccess<'de>, {
        let mut query: GeoShapeLookupQuery = GeoShapeLookupQuery::default();
        let mut found_value = false;

        while let Some(key) = map.next_key::<String>()? {
          if key == "geo_shape" {
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
                  let shape = serde_json::from_value::<Shape>(v.clone());
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
                  let value = serde_json::from_value::<Shape>(v.clone());
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

    deserializer.deserialize_struct("Wrapper", &["geo_shape"], WrapperVisitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_serialization() {
    assert_serialize_query(
      Query::geo_shape_lookup("pin.location", "id"),
      json!({
          "geo_shape": {
              "pin.location": {
                  "indexed_shape": {
                      "id": "id",
                  }
              },
          }
      }),
    );

    assert_serialize_query(
      Query::geo_shape_lookup("pin.location", "id")
        .boost(2)
        .name("test")
        .ignore_unmapped(true)
        .relation(SpatialRelation::Within)
        .routing("routing")
        .index("index")
        .path("path"),
      json!({
          "geo_shape": {
              "_name": "test",
              "boost": 2.0,
              "ignore_unmapped": true,
              "pin.location": {
                  "indexed_shape": {
                      "id": "id",
                      "index": "index",
                      "path": "path",
                      "routing": "routing"
                  },
                  "relation": "WITHIN"
              },
          }
      }),
    );
  }
}
