use serde::{Deserialize, Deserializer, Serialize};

use crate::{search::*, util::*};

/// Matches [geo_point](https://www.elastic.co/guide/en/opensearch/reference/current/geo-point.html)
/// and [geo_shape](https://www.elastic.co/guide/en/opensearch/reference/current/geo-shape.html)
/// values that intersect a bounding box.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-geo-bounding-box-query.html>
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct GeoBoundingBoxQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip)]
    bounding_box: GeoBoundingBox,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    validation_method: Option<ValidationMethod>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}
impl Query {
    /// Creates an instance of [`GeoBoundingBoxQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `bounding_box` - A series of vertex coordinates of a geo bounding box
    pub fn geo_bounding_box<T, U>(field: T, bounding_box: U) -> GeoBoundingBoxQuery
    where
        T: ToString,
        U: Into<GeoBoundingBox>,
    {
        GeoBoundingBoxQuery {
            field: field.to_string(),
            bounding_box: bounding_box.into(),
            validation_method: None,
            boost: None,
            _name: None,
        }
    }
}

impl GeoBoundingBoxQuery {
    add_boost_and_name!();

    /// Set to `IGNORE_MALFORMED` to accept geo points with invalid latitude or
    /// longitude, set to `COERCE` to also try to infer correct latitude or
    /// longitude. (default is `STRICT`).
    pub fn validation_method(mut self, validation_method: ValidationMethod) -> Self {
        self.validation_method = Some(validation_method);
        self
    }
}

impl ShouldSkip for GeoBoundingBoxQuery {}

serialize_with_root_key_value_pair!("geo_bounding_box": GeoBoundingBoxQuery, field, bounding_box);
impl<'de> Deserialize<'de> for GeoBoundingBoxQuery {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;
        struct WrapperVisitor;

        impl<'de> serde::de::Visitor<'de> for WrapperVisitor {
            type Value = GeoBoundingBoxQuery;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct geo_bounding_box")
            }

            fn visit_map<A>(self, mut map: A) -> Result<GeoBoundingBoxQuery, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut query: GeoBoundingBoxQuery = GeoBoundingBoxQuery::default();
                let mut found_value = false;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "geo_bounding_box" {
                        let inner_map =
                            map.next_value::<serde_json::Map<String, serde_json::Value>>()?;
                        for (k, v) in inner_map.iter() {
                            match k.as_str() {
                                "field" => match v.as_str() {
                                    Some(field) => {
                                        query.field = field.to_string();
                                    }
                                    None => {
                                        return Err(serde::de::Error::invalid_type(
                                            serde::de::Unexpected::Other("not a string"),
                                            &"a string",
                                        ));
                                    }
                                },
                                "boost" => match v.as_f64() {
                                    Some(boost) => {
                                        query.boost = Some(boost as f32);
                                    }
                                    None => {
                                        return Err(serde::de::Error::invalid_type(
                                            serde::de::Unexpected::Other("not a float"),
                                            &"a float",
                                        ));
                                    }
                                },
                                "_name" => match v.as_str() {
                                    Some(_name) => {
                                        query._name = Some(_name.to_string());
                                    }
                                    None => {
                                        return Err(serde::de::Error::invalid_type(
                                            serde::de::Unexpected::Other("not a string"),
                                            &"a string",
                                        ));
                                    }
                                },
                                "validation_method" => {
                                    let validation_method =
                                        serde_json::from_value::<ValidationMethod>(v.clone());
                                    match validation_method {
                                        Ok(validation_method) => {
                                            query.validation_method = Some(validation_method);
                                        }
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(format!(
                                                "error parsing validation_method: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                                _ => {
                                    query.field = k.to_owned();
                                    let value = serde_json::from_value::<GeoBoundingBox>(v.clone());
                                    match value {
                                        Ok(value) => {
                                            query.bounding_box = value;
                                            found_value = true;
                                        }
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(format!(
                                                "error parsing {}: {}",
                                                k, e
                                            )));
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

        deserializer.deserialize_struct("Wrapper", &["geo_bounding_box"], WrapperVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::geo_bounding_box(
                "pin.location",
                GeoBoundingBox::WellKnownText {
                    wkt: "BBOX (-74.1, -71.12, 40.73, 40.01)".into(),
                },
            ),
            json!({
                "geo_bounding_box": {
                    "pin.location": {
                        "wkt": "BBOX (-74.1, -71.12, 40.73, 40.01)"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::geo_bounding_box(
                "pin.location",
                GeoBoundingBox::MainDiagonal {
                    top_left: GeoLocation::new(40.73, -74.1),
                    bottom_right: GeoLocation::new(40.01, -71.12),
                },
            )
            .validation_method(ValidationMethod::Strict)
            .name("test_name"),
            json!({
                "geo_bounding_box": {
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "pin.location": {
                        "top_left": [ -74.1, 40.73 ],
                        "bottom_right": [ -71.12, 40.01 ]
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::geo_bounding_box(
                "pin.location",
                GeoBoundingBox::Vertices {
                    top: 40.73,
                    left: -74.1,
                    bottom: 40.01,
                    right: -71.12,
                },
            )
            .validation_method(ValidationMethod::Strict)
            .name("test_name")
            .boost(1),
            json!({
                "geo_bounding_box": {
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "boost": 1.0,
                    "pin.location": {
                        "top": 40.73,
                        "left": -74.1,
                        "bottom": 40.01,
                        "right": -71.12
                    }
                }
            }),
        )
    }
}
