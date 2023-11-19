use serde::{Deserialize, Deserializer, Serialize};

use super::{SortMode, SortOrder};
use crate::{util::ShouldSkip, Term};

/// Sorts search hits by other field values
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct FieldSort {
  #[serde(skip)]
  field: String,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  order: Option<SortOrder>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  mode: Option<SortMode>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  unmapped_type: Option<String>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  format: Option<String>,

  #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
  missing: Option<Term>,
}

impl FieldSort {
  /// Creates an instance of [FieldSort]
  pub fn new<T>(field: T) -> Self
  where
    T: ToString, {
    Self {
      field: field.to_string(),
      order: None,
      mode: None,
      unmapped_type: None,
      format: None,
      missing: None,
    }
  }

  /// Creates an instance of [FieldSort] by ascending order
  pub fn ascending<T>(field: T) -> Self
  where
    T: ToString, {
    Self::new(field).order(SortOrder::Asc)
  }

  /// Creates an instance of [FieldSort] by descending order
  pub fn descending<T>(field: T) -> Self
  where
    T: ToString, {
    Self::new(field).order(SortOrder::Desc)
  }

  /// Explicit order
  ///
  /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#_sort_order>
  pub fn order(mut self, order: SortOrder) -> Self {
    self.order = Some(order);
    self
  }

  /// Sort mode for numeric fields
  ///
  /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#_sort_mode_option>
  pub fn mode(mut self, mode: SortMode) -> Self {
    self.mode = Some(mode);
    self
  }

  /// Fallback type if mapping is not defined
  ///
  /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#_ignoring_unmapped_fields>
  pub fn unmapped_type<T>(mut self, unmapped_type: T) -> Self
  where
    T: ToString, {
    self.unmapped_type = Some(unmapped_type.to_string());
    self
  }

  /// Optional format for datetime sorts
  ///
  /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#_ignoring_unmapped_fields>
  pub fn format<T>(mut self, format: T) -> Self
  where
    T: ToString, {
    self.format = Some(format.to_string());
    self
  }

  /// The missing parameter specifies how docs which are missing the sort field
  /// should be treated
  ///
  /// <https://www.elastic.co/guide/en/opensearch/reference/current/sort-search-results.html#_missing_values>
  pub fn missing<T>(mut self, missing: T) -> Self
  where
    T: Serialize, {
    self.missing = Term::new(missing);
    self
  }
}

impl IntoIterator for FieldSort {
  type IntoIter = std::option::IntoIter<Self::Item>;
  type Item = Self;

  fn into_iter(self) -> Self::IntoIter {
    Some(self).into_iter()
  }
}

serialize_keyed!(FieldSort: field);
impl<'de> Deserialize<'de> for FieldSort {
  fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
  where
    D: Deserializer<'de>, {
    use std::fmt;
    struct WrapperVisitor;

    impl<'de> serde::de::Visitor<'de> for WrapperVisitor {
      type Value = FieldSort;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct fieldsort")
      }

      fn visit_map<A>(self, mut map: A) -> Result<FieldSort, A::Error>
      where
        A: serde::de::MapAccess<'de>, {
        let mut sorter: FieldSort = FieldSort::default();

        while let Some(key) = map.next_key::<String>()? {
          let inner_map = map.next_value::<serde_json::Map<String, serde_json::Value>>()?;
          sorter.field = key.to_owned();
          for (k, v) in inner_map.iter() {
            match k.as_str() {
              "unmapped_type" => {
                match v.as_str() {
                  Some(value) => {
                    sorter.unmapped_type = Some(value.to_string());
                  }
                  None => {
                    return Err(serde::de::Error::invalid_type(
                      serde::de::Unexpected::Other("not a string"),
                      &"a string",
                    ));
                  }
                }
              }
              "format" => {
                match v.as_str() {
                  Some(value) => {
                    sorter.format = Some(value.to_string());
                  }
                  None => {
                    return Err(serde::de::Error::invalid_type(
                      serde::de::Unexpected::Other("not a string"),
                      &"a string",
                    ));
                  }
                }
              }
              "order" => {
                let value = serde_json::from_value::<SortOrder>(v.clone());
                match value {
                  Ok(value) => {
                    sorter.order = Some(value);
                  }
                  Err(e) => {
                    return Err(serde::de::Error::custom(format!("error parsing {}: {}", k, e)));
                  }
                }
              }
              "mode" => {
                let value = serde_json::from_value::<SortMode>(v.clone());
                match value {
                  Ok(value) => {
                    sorter.mode = Some(value);
                  }
                  Err(e) => {
                    return Err(serde::de::Error::custom(format!("error parsing {}: {}", k, e)));
                  }
                }
              }
              "missing" => {
                let value = serde_json::from_value::<Term>(v.clone());
                match value {
                  Ok(value) => {
                    sorter.missing = Some(value);
                  }
                  Err(e) => {
                    return Err(serde::de::Error::custom(format!("error parsing {}: {}", k, e)));
                  }
                }
              }
              _ => {
                return Err(serde::de::Error::custom(format!("fielderror: error parsing {}", k)));
              }
            }
          }
        }
        if sorter.field != "" {
          Ok(sorter)
        } else {
          Err(serde::de::Error::missing_field("required field"))
        }
      }
    }

    deserializer.deserialize_map(WrapperVisitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{util::assert_serialize, SortSpecialField};

  #[test]
  fn serialization() {
    assert_serialize(FieldSort::new("test"), json!({"test": {}}));

    assert_serialize(FieldSort::new(SortSpecialField::Score), json!({"_score": {}}));

    assert_serialize(FieldSort::ascending("field"), json!({ "field": { "order": "asc" } }));

    assert_serialize(FieldSort::descending("field"), json!({ "field": { "order": "desc" } }));

    assert_serialize(
      FieldSort::ascending("test")
        .order(SortOrder::Asc)
        .mode(SortMode::Max)
        .unmapped_type("long")
        .missing("miss"),
      json!({
          "test": {
              "order": "asc",
              "mode": "max",
              "unmapped_type": "long",
              "missing": "miss",
          }
      }),
    );
  }
}
