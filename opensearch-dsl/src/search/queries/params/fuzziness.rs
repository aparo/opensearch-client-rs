use std::{fmt, ops::Range};

use serde::{
  de::{Error, Visitor},
  Deserialize, Deserializer, Serialize, Serializer,
};
/// Some queries and APIs support parameters to allow inexact _fuzzy_ matching,
/// using the `fuzziness` parameter.
///
/// When querying `text` or `keyword` fields, `fuzziness` is interpreted as a
/// [Levenshtein Edit Distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
/// — the number of one character changes that need to be made to one string to
/// make it the same as another string.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/common-options.html#fuzziness>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fuzziness {
  /// Generates an edit distance based on the length of the term.
  ///
  /// `AUTO` should generally be the preferred value for `fuzziness`.
  Auto,

  /// Low and high distance arguments may be optionally provided
  /// `AUTO:[low],[high]`. If not specified, the default values are 3 and 6,
  /// equivalent to `AUTO:3,6` that make for lengths:
  ///
  /// **`0..2`**
  ///
  /// &nbsp;&nbsp;&nbsp;&nbsp;Must match exactly
  ///
  /// **`3..5`**
  ///
  /// &nbsp;&nbsp;&nbsp;&nbsp;One edit allowed
  ///
  /// **`>5`**
  ///
  /// &nbsp;&nbsp;&nbsp;&nbsp;Two edits allowed
  Range(u8, u8),

  /// The maximum allowed Levenshtein Edit Distance (or number of edits)
  Distance(u8),
}

impl Serialize for Fuzziness {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer, {
    match self {
      Self::Auto => "AUTO".serialize(serializer),
      Self::Range(start, end) => format!("AUTO:{start},{end}").serialize(serializer),
      Self::Distance(d) => d.serialize(serializer),
    }
  }
}

impl<'de> Deserialize<'de> for Fuzziness {
  fn deserialize<D>(deserializer: D) -> Result<Fuzziness, D::Error>
  where
    D: Deserializer<'de>, {
    struct FuzzinessVisitor;

    impl<'de> Visitor<'de> for FuzzinessVisitor {
      type Value = Fuzziness;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a Fuzziness variant")
      }

      fn visit_str<E>(self, value: &str) -> Result<Fuzziness, E>
      where
        E: Error, {
        if value == "AUTO" {
          return Ok(Fuzziness::Auto);
        }

        if value.starts_with("AUTO:") {
          let parts: Vec<&str> = value[5..].split(',').collect();
          if parts.len() == 2 {
            let start = parts[0].parse::<u8>().map_err(E::custom)?;
            let end = parts[1].parse::<u8>().map_err(E::custom)?;
            return Ok(Fuzziness::Range(start, end));
          }
        }

        let distance = value.parse::<u8>().map_err(E::custom)?;
        Ok(Fuzziness::Distance(distance))
      }
    }

    deserializer.deserialize_str(FuzzinessVisitor)
  }
}

impl From<Range<u8>> for Fuzziness {
  fn from(v: Range<u8>) -> Self {
    Self::Range(v.start, v.end)
  }
}

impl From<[u8; 2]> for Fuzziness {
  fn from(v: [u8; 2]) -> Self {
    Self::Range(v[0], v[1])
  }
}

impl From<u8> for Fuzziness {
  fn from(v: u8) -> Self {
    Self::Distance(v)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::util::assert_serialize;

  #[test]
  fn implements_from_u8() {
    let result = Fuzziness::from(8);

    let expectation = Fuzziness::Distance(8);

    assert_eq!(result, expectation);
  }

  #[test]
  fn implements_from_range_u8() {
    let result = Fuzziness::from(0..2);

    let expectation = Fuzziness::Range(0, 2);

    assert_eq!(result, expectation);
  }

  #[test]
  fn serializes() {
    assert_serialize(
      [Fuzziness::Auto, Fuzziness::Range(0, 2), Fuzziness::Distance(5)],
      json!(["AUTO", "AUTO:0,2", 5,]),
    )
  }
}
