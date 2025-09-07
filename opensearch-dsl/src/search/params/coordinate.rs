use std::{fmt, fmt::Display, str::FromStr};

use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

/// Represents a point in two dimensional space
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coordinate {
    x: f32,
    y: f32,
}

impl Coordinate {
    /// Creates an instance of [`Coordinate`]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("[{}, {}]", &self.x, &self.y).fmt(f)
    }
}

impl Serialize for Coordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        [self.x, self.y].serialize(serializer)
    }
}

struct CoordinateVisitor;

impl<'de> Visitor<'de> for CoordinateVisitor {
    type Value = Coordinate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of two floats")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let x = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let y = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        Ok(Coordinate { x, y })
    }
}

impl<'de> Deserialize<'de> for Coordinate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple(2, CoordinateVisitor)
    }
}

impl From<[f32; 2]> for Coordinate {
    fn from(value: [f32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<(f32, f32)> for Coordinate {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(',');

        let x = values.next().and_then(|x| x.trim().parse().ok());
        let y = values.next().and_then(|x| x.trim().parse().ok());

        match (x, y, values.next()) {
            (Some(x), Some(y), None) => Ok(Self { x, y }),
            _ => Err(format!("Couldn't parse '{s}' as coordinate")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(Coordinate::new(1.1, 2.2), json!([1.1, 2.2]));
        assert_serialize(Coordinate::from([1.1, 2.2]), json!([1.1, 2.2]));
        assert_serialize(Coordinate::from((1.1, 2.2)), json!([1.1, 2.2]));
    }

    #[test]
    fn from_str() {
        assert_eq!(
            Coordinate::from_str("1.1, 2.2").unwrap(),
            Coordinate::new(1.1, 2.2)
        );
        assert_eq!(
            Coordinate::from_str("1,2").unwrap(),
            Coordinate::new(1., 2.)
        );

        assert!(Coordinate::from_str("1.1").is_err());
        assert!(Coordinate::from_str("1.1,2.2,3").is_err());
        assert!(Coordinate::from_str("abc").is_err());
    }
}
