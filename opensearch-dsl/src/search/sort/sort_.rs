use std::borrow::Cow;

use super::{FieldSort, GeoDistanceSort, ScriptSort, SortSpecialField};

/// Sorting criterion
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Sort {
    /// Special sort field,
    SpecialField(SortSpecialField),

    /// Sorts by field name
    Field(String),

    /// Sorts by field name with finer control
    FieldSort(FieldSort),

    /// Sorts by a geo distance
    GeoDistanceSort(GeoDistanceSort),

    /// Sort by a script
    ScriptSort(ScriptSort),
}

impl std::fmt::Debug for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpecialField(sort) => sort.fmt(f),
            Self::Field(sort) => sort.fmt(f),
            Self::FieldSort(sort) => sort.fmt(f),
            Self::GeoDistanceSort(sort) => sort.fmt(f),
            Self::ScriptSort(sort) => sort.fmt(f),
        }
    }
}

impl From<SortSpecialField> for Sort {
    fn from(value: SortSpecialField) -> Self {
        Self::SpecialField(value)
    }
}

impl From<&str> for Sort {
    fn from(value: &str) -> Self {
        Self::Field(value.to_string())
    }
}

impl From<Cow<'_, str>> for Sort {
    fn from(value: Cow<'_, str>) -> Self {
        Self::Field(value.to_string())
    }
}

impl From<String> for Sort {
    fn from(value: String) -> Self {
        Self::Field(value)
    }
}

impl From<FieldSort> for Sort {
    fn from(value: FieldSort) -> Self {
        Self::FieldSort(value)
    }
}

impl From<GeoDistanceSort> for Sort {
    fn from(value: GeoDistanceSort) -> Self {
        Self::GeoDistanceSort(value)
    }
}

impl From<ScriptSort> for Sort {
    fn from(value: ScriptSort) -> Self {
        Self::ScriptSort(value)
    }
}

impl IntoIterator for Sort {
    type IntoIter = std::option::IntoIter<Self::Item>;
    type Item = Self;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}
