use serde::{Deserialize, Deserializer, Serialize};

use super::{SortMode, SortOrder};
use crate::{
    util::{KeyValuePair, ShouldSkip},
    DistanceUnit, GeoDistanceType, GeoLocation,
};

/// Sorts search hits by other field values
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct GeoDistanceSort {
    #[serde(skip)]
    field: String,

    #[serde(skip)]
    points: Vec<GeoLocation>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<SortOrder>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    unit: Option<DistanceUnit>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    mode: Option<SortMode>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    distance_type: Option<GeoDistanceType>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    ignore_unmapped: Option<bool>,
}

impl GeoDistanceSort {
    /// Creates an instance of [GeoDistanceSort]
    pub fn new<T, U>(field: T, points: U) -> Self
    where
        T: ToString,
        U: IntoIterator,
        U::Item: Into<GeoLocation>,
    {
        Self {
            field: field.to_string(),
            points: points.into_iter().map(Into::into).collect(),
            order: None,
            unit: None,
            mode: None,
            distance_type: None,
            ignore_unmapped: None,
        }
    }

    /// Creates an instance of [GeoDistanceSort] by ascending order
    pub fn ascending<T, U>(field: T, points: U) -> Self
    where
        T: ToString,
        U: IntoIterator,
        U::Item: Into<GeoLocation>,
    {
        Self::new(field, points).order(SortOrder::Asc)
    }

    /// Creates an instance of [GeoDistanceSort] by descending order
    pub fn descending<T, U>(field: T, points: U) -> Self
    where
        T: ToString,
        U: IntoIterator,
        U::Item: Into<GeoLocation>,
    {
        Self::new(field, points).order(SortOrder::Desc)
    }

    /// Explicit order
    ///
    /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#_sort_order>
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }

    /// The unit to use when computing sort values
    ///
    /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#geo-sorting>
    pub fn unit(mut self, unit: DistanceUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Sort mode for numeric fields
    ///
    /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#_sort_mode_option>
    pub fn mode(mut self, mode: SortMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// How to compute the distance. Can either be arc (default), or plane
    /// (faster, but inaccurate on long distances and close to the poles).
    ///
    /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#_sort_mode_option>
    pub fn distance_type(mut self, distance_type: GeoDistanceType) -> Self {
        self.distance_type = Some(distance_type);
        self
    }

    /// Indicates if the unmapped field should be treated as a missing value.
    /// Setting it to `true` is equivalent to specifying an `unmapped_type` in
    /// the field sort. The default is `false` (unmapped field cause the search
    /// to fail).
    ///
    /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#_sort_mode_option>
    pub fn ignore_unmapped(mut self, ignore_unmapped: bool) -> Self {
        self.ignore_unmapped = Some(ignore_unmapped);
        self
    }
}

impl IntoIterator for GeoDistanceSort {
    type IntoIter = std::option::IntoIter<Self::Item>;
    type Item = Self;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}

serialize_with_root_key_value_pair!("_geo_distance": GeoDistanceSort, field, points);
impl<'de> Deserialize<'de> for GeoDistanceSort {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;
        struct WrapperVisitor;

        impl<'de> serde::de::Visitor<'de> for WrapperVisitor {
            type Value = GeoDistanceSort;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct shape")
            }

            fn visit_map<A>(self, mut map: A) -> Result<GeoDistanceSort, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut query: GeoDistanceSort = GeoDistanceSort::default();
                let mut found_value = false;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "_geo_distance" {
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
                                "ignore_unmapped" => match v.as_bool() {
                                    Some(ignore_unmapped) => {
                                        query.ignore_unmapped = Some(ignore_unmapped);
                                    }
                                    None => {
                                        return Err(serde::de::Error::invalid_type(
                                            serde::de::Unexpected::Other("not a boolean"),
                                            &"a boolean",
                                        ));
                                    }
                                },
                                "order" => {
                                    let value = serde_json::from_value::<SortOrder>(v.clone());
                                    match value {
                                        Ok(value) => {
                                            query.order = Some(value);
                                        }
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(format!(
                                                "error parsing order: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                                "unit" => {
                                    let value = serde_json::from_value::<DistanceUnit>(v.clone());
                                    match value {
                                        Ok(value) => {
                                            query.unit = Some(value);
                                        }
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(format!(
                                                "error parsing unit: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                                "mode" => {
                                    let value = serde_json::from_value::<SortMode>(v.clone());
                                    match value {
                                        Ok(value) => {
                                            query.mode = Some(value);
                                        }
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(format!(
                                                "error parsing mode: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                                "distance_type" => {
                                    let value =
                                        serde_json::from_value::<GeoDistanceType>(v.clone());
                                    match value {
                                        Ok(value) => {
                                            query.distance_type = Some(value);
                                        }
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(format!(
                                                "error parsing distance_type: {}",
                                                e
                                            )));
                                        }
                                    }
                                }
                                _ => {
                                    query.field = k.to_owned();
                                    let value =
                                        serde_json::from_value::<Vec<GeoLocation>>(v.clone());
                                    match value {
                                        Ok(value) => {
                                            query.points = value;
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
                    Err(serde::de::Error::missing_field("point values"))
                }
            }
        }

        deserializer.deserialize_struct("Wrapper", &["_geo_distance"], WrapperVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_serialize;

    #[test]
    fn serialization() {
        assert_serialize(
            GeoDistanceSort::new("test", GeoLocation::new(1.2, 3.3)),
            json!({
                "_geo_distance": {
                    "test": [ [3.3, 1.2] ]
                }
            }),
        );

        assert_serialize(
            GeoDistanceSort::ascending("test", GeoLocation::new(1.2, 3.3)),
            json!({
                "_geo_distance": {
                    "test": [ [3.3, 1.2] ],
                    "order": "asc",
                }
            }),
        );

        assert_serialize(
            GeoDistanceSort::descending("test", GeoLocation::new(1.2, 3.3))
                .order(SortOrder::Asc)
                .unit(DistanceUnit::Inches)
                .mode(SortMode::Max)
                .distance_type(GeoDistanceType::Arc)
                .ignore_unmapped(true),
            json!({
                "_geo_distance": {
                    "test": [ [3.3, 1.2] ],
                    "unit": "in",
                    "order": "asc",
                    "mode": "max",
                    "distance_type": "arc",
                    "ignore_unmapped": true,
                }
            }),
        );
    }
}
