use serde::{Deserialize, Deserializer, Serialize};

use crate::{search::*, util::*};

/// Matches [geo_point](https://www.elastic.co/guide/en/opensearch/reference/current/geo-point.html)
/// and [geo_shape](https://www.elastic.co/guide/en/opensearch/reference/current/geo-shape.html)
/// values within a given distance of a geopoint.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-geo-distance-query.html>
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct GeoDistanceQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip)]
    location: GeoLocation,

    distance: Distance,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    distance_type: Option<GeoDistanceType>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    validation_method: Option<ValidationMethod>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`GeoDistanceQuery`]
    ///
    /// - `field` - Field you wish to search
    /// - `origin` - GeoPoint to measure distance to
    /// - `distance` - Distance threshold
    pub fn geo_distance<T, U, V>(field: T, origin: U, distance: V) -> GeoDistanceQuery
    where
        T: ToString,
        U: Into<GeoLocation>,
        V: Into<Distance>,
    {
        GeoDistanceQuery {
            field: field.to_string(),
            location: origin.into(),
            distance: distance.into(),
            distance_type: None,
            validation_method: None,
            boost: None,
            _name: None,
        }
    }
}

impl GeoDistanceQuery {
    add_boost_and_name!();

    /// Set to `IGNORE_MALFORMED` to accept geo points with invalid latitude or
    /// longitude, set to `COERCE` to also try to infer correct latitude or
    /// longitude. (default is `STRICT`).
    pub fn validation_method(mut self, validation_method: ValidationMethod) -> Self {
        self.validation_method = Some(validation_method);
        self
    }

    /// How to compute the distance. Can either be [Arc](GeoDistanceType::Arc)
    /// (default), or [Plane](GeoDistanceType::Plane) (faster, but inaccurate on
    /// long distances and close to the poles).
    pub fn distance_type(mut self, distance_type: GeoDistanceType) -> Self {
        self.distance_type = Some(distance_type);
        self
    }
}

impl ShouldSkip for GeoDistanceQuery {}

serialize_with_root_key_value_pair!("geo_distance": GeoDistanceQuery, field, location);
impl<'de> Deserialize<'de> for GeoDistanceQuery {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;
        struct WrapperVisitor;

        impl<'de> serde::de::Visitor<'de> for WrapperVisitor {
            type Value = GeoDistanceQuery;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct geo_distance")
            }

            fn visit_map<A>(self, mut map: A) -> Result<GeoDistanceQuery, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut query: GeoDistanceQuery = GeoDistanceQuery::default();
                let mut found_value = false;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "geo_distance" {
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
                                "distance" => {
                                    let distance = serde_json::from_value::<Distance>(v.clone());
                                    match distance {
                                        Ok(distance) => {
                                            query.distance = distance;
                                        }
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(format!(
                                                "error parsing distance: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                                "distance_type" => {
                                    let distance_type =
                                        serde_json::from_value::<GeoDistanceType>(v.clone());
                                    match distance_type {
                                        Ok(distance_type) => {
                                            query.distance_type = Some(distance_type);
                                        }
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(format!(
                                                "error parsing distance_type: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
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
                                    let value = serde_json::from_value::<GeoLocation>(v.clone());
                                    match value {
                                        Ok(value) => {
                                            query.location = value;
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

        deserializer.deserialize_struct("Wrapper", &["geo_distance"], WrapperVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::geo_distance(
                "pin.location",
                GeoLocation::new(40.12, -71.34),
                Distance::Kilometers(300),
            ),
            json!({
                "geo_distance": {
                    "distance": "300km",
                    "pin.location": [-71.34, 40.12],
                }
            }),
        );

        assert_serialize_query(
            Query::geo_distance(
                "pin.location",
                GeoLocation::new(40.12, -71.34),
                Distance::Kilometers(300),
            )
            .distance_type(GeoDistanceType::Plane)
            .validation_method(ValidationMethod::Strict)
            .name("test_name")
            .boost(1),
            json!({
                "geo_distance": {
                    "distance": "300km",
                    "distance_type": "plane",
                    "pin.location": [-71.34, 40.12],
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "boost": 1.0,
                }
            }),
        );
    }
}
