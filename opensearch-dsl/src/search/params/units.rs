use std::fmt;

use serde::{
    de::{self, Unexpected, Visitor},
    ser::{Serialize, Serializer},
    Deserialize, Deserializer,
};

/// Whenever durations need to be specified, e.g. for a `timeout` parameter,
/// the duration must specify the unit, like `2d` for 2 days.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/common-options.html#time-units>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Time {
    Days(u64),
    Hours(u64),
    Minutes(u64),
    Seconds(u64),
    Milliseconds(u64),
    Microseconds(u64),
    Nanoseconds(u64),
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Days(u) => format!("{u}d"),
            Self::Hours(u) => format!("{u}h"),
            Self::Minutes(u) => format!("{u}m"),
            Self::Seconds(u) => format!("{u}s"),
            Self::Milliseconds(u) => format!("{u}ms"),
            Self::Microseconds(u) => format!("{u}micros"),
            Self::Nanoseconds(u) => format!("{u}nanos"),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeVisitor;

        impl<'de> Visitor<'de> for TimeVisitor {
            type Value = Time;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a duration")
            }

            fn visit_str<E>(self, value: &str) -> Result<Time, E>
            where
                E: de::Error,
            {
                let len = value.len();
                let number = &value[..len - 1];
                let unit = &value[len - 1..];

                let number = number.parse::<u64>().map_err(E::custom)?;

                match unit {
                    "d" => Ok(Time::Days(number)),
                    "h" => Ok(Time::Hours(number)),
                    "m" => Ok(Time::Minutes(number)),
                    "s" => Ok(Time::Seconds(number)),
                    "ms" => Ok(Time::Milliseconds(number)),
                    "micros" => Ok(Time::Microseconds(number)),
                    "nanos" => Ok(Time::Nanoseconds(number)),
                    _ => Err(E::custom(format!("unknown time unit: {}", unit))),
                }
            }
        }

        deserializer.deserialize_str(TimeVisitor)
    }
}
/// Calendar-aware intervals are configured with the `calendar_interval`
/// parameter. You can specify calendar intervals using the unit name, such as
/// `month`, or as a single unit quantity, such as `1M`. For example,`day` and
/// `1d` are equivalent. Multiple quantities, such as `2d`, are not supported.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/search-aggregations-bucket-datehistogram-aggregation.html#calendar_intervals>
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarInterval {
    /// All minutes begin at 00 seconds. One minute is the interval between 00
    /// seconds of the first minute and 00 seconds of the following minute in
    /// the specified time zone, compensating for any intervening leap seconds,
    /// so that the number of minutes and seconds past the hour is the
    /// same at the start and end.
    Minute,
    /// All hours begin at 00 minutes and 00 seconds. One hour (1h) is the
    /// interval between 00:00 minutes of the first hour and 00:00 minutes of
    /// the following hour in the specified time zone, compensating for any
    /// intervening leap seconds, so that the number of minutes and seconds past
    /// the hour is the same at the start and end.
    Hour,
    /// All days begin at the earliest possible time, which is usually 00:00:00
    /// (midnight). One day (1d) is the interval between the start of the day
    /// and the start of the following day in the specified time zone,
    /// compensating for any intervening time changes.
    Day,
    /// One week is the interval between the start day_of_week:hour:minute:second
    /// and the same day of the week and time of the following week in the
    /// specified time zone.
    Week,
    /// One month is the interval between the start day of the month and time of
    /// day and the same day of the month and time of the following month in the
    /// specified time zone, so that the day of the month and time of day are
    /// the same at the start and end.
    Month,
    /// One quarter is the interval between the start day of the month and time of
    /// day and the same day of the month and time of day three months later, so
    /// that the day of the month and time of day are the same at the start and
    /// end.
    Quarter,
    /// One year is the interval between the start day of the month and time of
    /// day and the same day of the month and time of day the following year in
    /// the specified time zone, so that the date and time are the same at the
    /// start and end.
    Year,
}

/// Whenever the byte size of data needs to be specified, e.g. when setting a
/// buffer size parameter, the value must specify the unit,
/// like `10kb` for 10 kilobytes.
/// Note that these units use powers of 1024, so `1kb` means 1024 bytes.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/common-options.html#byte-units>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Byte {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
    Petabytes(u64),
}

impl Serialize for Byte {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bytes(u) => format!("{u}b"),
            Self::Kilobytes(u) => format!("{u}kb"),
            Self::Megabytes(u) => format!("{u}mb"),
            Self::Gigabytes(u) => format!("{u}gb"),
            Self::Terabytes(u) => format!("{u}tb"),
            Self::Petabytes(u) => format!("{u}pb"),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Byte {
    fn deserialize<D>(deserializer: D) -> Result<Byte, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ByteVisitor;

        impl<'de> Visitor<'de> for ByteVisitor {
            type Value = Byte;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a byte size")
            }

            fn visit_str<E>(self, value: &str) -> Result<Byte, E>
            where
                E: de::Error,
            {
                let len = value.len();
                let number = &value[..len - 2];
                let unit = &value[len - 2..];

                let number = number.parse::<u64>().map_err(E::custom)?;

                match unit {
                    "b" => Ok(Byte::Bytes(number)),
                    "kb" => Ok(Byte::Kilobytes(number)),
                    "mb" => Ok(Byte::Megabytes(number)),
                    "gb" => Ok(Byte::Gigabytes(number)),
                    "tb" => Ok(Byte::Terabytes(number)),
                    "pb" => Ok(Byte::Petabytes(number)),
                    _ => Err(E::custom(format!("unknown byte unit: {}", unit))),
                }
            }
        }

        deserializer.deserialize_str(ByteVisitor)
    }
}

/// Unit-less quantities means that they don’t have a "unit"
/// like "bytes" or "Hertz" or "meter" or "long tonne".
///
/// If one of these quantities is large we’ll print it out like 10m for
/// 10,000,000 or 7k for 7,000. We’ll still print 87 when we mean 87 though.
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/common-options.html#size-units>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Size {
    Kilo(u64),
    Mega(u64),
    Giga(u64),
    Tera(u64),
    Peta(u64),
}

impl Serialize for Size {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Kilo(u) => format!("{u}k"),
            Self::Mega(u) => format!("{u}m"),
            Self::Giga(u) => format!("{u}g"),
            Self::Tera(u) => format!("{u}t"),
            Self::Peta(u) => format!("{u}p"),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Size, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SizeVisitor;

        impl<'de> Visitor<'de> for SizeVisitor {
            type Value = Size;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a size")
            }

            fn visit_str<E>(self, value: &str) -> Result<Size, E>
            where
                E: de::Error,
            {
                let len = value.len();
                let number = &value[..len - 1];
                let unit = &value[len - 1..];

                let number = number.parse::<u64>().map_err(E::custom)?;

                match unit {
                    "k" => Ok(Size::Kilo(number)),
                    "m" => Ok(Size::Mega(number)),
                    "g" => Ok(Size::Giga(number)),
                    "t" => Ok(Size::Tera(number)),
                    "p" => Ok(Size::Peta(number)),
                    _ => Err(E::custom(format!("unknown size unit: {}", unit))),
                }
            }
        }

        deserializer.deserialize_str(SizeVisitor)
    }
}

/// Wherever distances need to be specified, such as the `distance` parameter
/// in the
/// [Geo-distance](https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-geo-distance-query.html)
/// ), the default unit is meters if none is specified.
/// Distances can be specified in other units,
/// such as `"1km"` or `"2mi"` (2 miles).
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-geo-distance-query.html>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Distance {
    Miles(u64),
    Yards(u64),
    Feet(u64),
    Inches(u64),
    Kilometers(u64),
    Meters(u64),
    Centimeter(u64),
    Millimeters(u64),
    NauticalMiles(u64),
}

impl Default for Distance {
    fn default() -> Self {
        Self::Meters(100)
    }
}

impl Serialize for Distance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Miles(u) => format!("{u}mi"),
            Self::Yards(u) => format!("{u}yd"),
            Self::Feet(u) => format!("{u}ft"),
            Self::Inches(u) => format!("{u}in"),
            Self::Kilometers(u) => format!("{u}km"),
            Self::Meters(u) => format!("{u}m"),
            Self::Centimeter(u) => format!("{u}cm"),
            Self::Millimeters(u) => format!("{u}mm"),
            Self::NauticalMiles(u) => format!("{u}nmi"),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Distance {
    fn deserialize<D>(deserializer: D) -> Result<Distance, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistanceVisitor;

        impl<'de> Visitor<'de> for DistanceVisitor {
            type Value = Distance;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a distance")
            }

            fn visit_str<E>(self, value: &str) -> Result<Distance, E>
            where
                E: de::Error,
            {
                let len = value.len();
                let number = &value[..len - 2];
                let unit = &value[len - 2..];

                let number = number.parse::<u64>().map_err(E::custom)?;

                match unit {
                    "mi" => Ok(Distance::Miles(number)),
                    "yd" => Ok(Distance::Yards(number)),
                    "ft" => Ok(Distance::Feet(number)),
                    "in" => Ok(Distance::Inches(number)),
                    "km" => Ok(Distance::Kilometers(number)),
                    "m" => Ok(Distance::Meters(number)),
                    "cm" => Ok(Distance::Centimeter(number)),
                    "mm" => Ok(Distance::Millimeters(number)),
                    "nmi" => Ok(Distance::NauticalMiles(number)),
                    _ => Err(E::custom(format!("unknown distance unit: {}", unit))),
                }
            }
        }

        deserializer.deserialize_str(DistanceVisitor)
    }
}

/// Wherever distances need to be specified, such as the `distance` parameter
/// in the
/// [Geo-distance](https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-geo-distance-query.html)
/// ), the default unit is meters if none is specified.
/// Distances can be specified in other units,
/// such as `"1km"` or `"2mi"` (2 miles).
///
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-geo-distance-query.html>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum DistanceUnit {
    Miles,
    Yards,
    Feet,
    Inches,
    Kilometers,
    Meters,
    Centimeter,
    Millimeters,
    NauticalMiles,
}

impl Serialize for DistanceUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Miles => "mi",
            Self::Yards => "yd",
            Self::Feet => "ft",
            Self::Inches => "in",
            Self::Kilometers => "km",
            Self::Meters => "m",
            Self::Centimeter => "cm",
            Self::Millimeters => "mm",
            Self::NauticalMiles => "nmi",
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DistanceUnit {
    fn deserialize<D>(deserializer: D) -> Result<DistanceUnit, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistanceUnitVisitor;

        impl<'de> Visitor<'de> for DistanceUnitVisitor {
            type Value = DistanceUnit;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a distance unit")
            }

            fn visit_str<E>(self, value: &str) -> Result<DistanceUnit, E>
            where
                E: de::Error,
            {
                match value {
                    "mi" => Ok(DistanceUnit::Miles),
                    "yd" => Ok(DistanceUnit::Yards),
                    "ft" => Ok(DistanceUnit::Feet),
                    "in" => Ok(DistanceUnit::Inches),
                    "km" => Ok(DistanceUnit::Kilometers),
                    "m" => Ok(DistanceUnit::Meters),
                    "cm" => Ok(DistanceUnit::Centimeter),
                    "mm" => Ok(DistanceUnit::Millimeters),
                    "nmi" => Ok(DistanceUnit::NauticalMiles),
                    _ => Err(E::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(DistanceUnitVisitor)
    }
}
