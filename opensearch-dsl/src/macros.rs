#[doc(hidden)]
#[macro_export]
macro_rules! add_boost_and_name {
  () => {
    /// Floating point number used to decrease or increase the
    /// [relevance scores](https://www.elastic.co/guide/en/opensearch/reference/current/query-filter-context.html#relevance-scores)
    /// of a query. Defaults to `1.0`.
    ///
    /// You can use the boost parameter to adjust relevance scores for
    /// searches containing two or more queries.
    ///
    /// Boost values are relative to the default value of `1.0`.
    /// A boost value between 0 and `1.0` decreases the relevance score.
    /// A value greater than `1.0` increases the relevance score.
    pub fn boost<T>(mut self, boost: T) -> Self
    where
      T: num_traits::AsPrimitive<f32>, {
      self.boost = Some(boost.as_());
      self
    }

    /// You can use named queries to track which queries matched
    /// returned documents. If named queries are used, the response
    /// includes a `matched_queries` property for each hit.
    ///
    /// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-bool-query.html#named-queries>
    pub fn name<S>(mut self, name: S) -> Self
    where
      S: ToString, {
      self._name = Some(name.to_string());
      self
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! add_aggregate {
  () => {
    /// Pushes aggregation
    pub fn aggregate<N, A>(mut self, aggregation_name: N, aggregation: A) -> Self
    where
      N: Into<AggregationName>,
      A: Into<Aggregation>, {
      let a = aggregation.into();
      let _ = self.aggs.entry(aggregation_name.into()).or_insert(a);
      self
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! serialize_with_root {
  ($root:tt : $inner:ty) => {
    impl $crate::serde::Serialize for $inner {
      fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
      where
        S: $crate::serde::ser::Serializer, {
        use $crate::serde::ser::SerializeStruct;

        struct Wrapper<'a> {
          root: &'a $inner,
        }

        impl<'a> $crate::serde::Serialize for Wrapper<'a> {
          fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
          where
            S: $crate::serde::Serializer, {
            <$inner>::serialize(&self.root, serializer)
          }
        }

        let mut state = serializer.serialize_struct("Wrapper", 1)?;
        state.serialize_field($root, &Wrapper { root: self })?;
        state.end()
      }
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! deserialize_with_root {
  ($root:tt : $inner:ty) => {
    impl<'de> $crate::serde::Deserialize<'de> for $inner {
      fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
      where
        D: $crate::serde::Deserializer<'de>, {
        use std::fmt;
        struct WrapperVisitor;

        impl<'de> $crate::serde::de::Visitor<'de> for WrapperVisitor {
          type Value = $inner;

          fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("struct Wrapper")
          }

          fn visit_map<A>(self, mut map: A) -> Result<$inner, A::Error>
          where
            A: $crate::serde::de::MapAccess<'de>, {
            let mut value = None;

            while let Some(key) = map.next_key::<String>()? {
              if key == $root {
                value = Some(map.next_value()?);
              }
            }

            value.ok_or_else(|| $crate::serde::de::Error::missing_field($root))
          }
        }

        deserializer.deserialize_struct("Wrapper", &[$root], WrapperVisitor)
      }
    }
  };
}

// #[macro_export]
// macro_rules! deserialize_with_root {
//   ($root:tt : $inner:ty) => {
//     impl<'de> $crate::serde::Deserialize<'de> for $inner {
//       fn deserialize<D>(deserializer: D) -> ::std::result::Result<$inner,
// D::Error>       where
//         D: $crate::serde::de::Deserializer<'de>, {
//         use std::fmt;

//         use $crate::serde::de::{Deserialize, Deserializer, Visitor};

//         struct Wrapper<'de> {
//           root: &'de $inner,
//         }

//         impl<'de> Deserialize<'de> for Wrapper<'de> {
//           fn deserialize<D>(deserializer: D) ->
// ::std::result::Result<Wrapper<'de>, D::Error>           where
//             D: Deserializer<'de>, {
//             struct FieldVisitor;

//             impl<'de> Visitor<'de> for FieldVisitor {
//               type Value = ();

//               fn expecting(&self, formatter: &mut fmt::Formatter) ->
// fmt::Result {                 formatter.write_str("struct Wrapper")
//               }

//               fn visit_str<E>(self, value: &str) -> ::std::result::Result<(),
// E>               where
//                 E: $crate::serde::de::Error, {
//                 if value == $root {
//                   Ok(())
//                 } else {
//                   Err(E::unknown_field(value, &[$root]))
//                 }
//               }
//             }

//             deserializer.deserialize_struct("Wrapper", &[$root],
// FieldVisitor)?;

//             Ok(Wrapper {
//               root: &<$inner>::deserialize(deserializer)?,
//             })
//           }
//         }

//         let Wrapper { root } = Wrapper::deserialize(deserializer)?;
//         Ok(root.clone())
//       }
//     }
//   };
// }

#[doc(hidden)]
#[macro_export]
macro_rules! serialize_with_root_keyed {
  ($root:tt : $inner:ty) => {
    impl $crate::serde::Serialize for $inner {
      fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
      where
        S: $crate::serde::ser::Serializer, {
        use $crate::serde::ser::SerializeStruct;

        struct Wrapper<'a> {
          root: &'a $inner,
        }

        impl<'a> $crate::serde::Serialize for Wrapper<'a> {
          fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
          where
            S: $crate::serde::Serializer, {
            <$inner>::serialize(&self.root, serializer)
          }
        }

        let mut state = serializer.serialize_struct("Wrapper", 1)?;
        state.serialize_field($root, &KeyValuePair::new(&self.field, &Wrapper { root: self }))?;
        state.end()
      }
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! deserialize_with_root_keyed {
  ($root:tt : $inner:ty) => {
    impl<'de> $crate::serde::Deserialize<'de> for $inner {
      fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
      where
        D: $crate::serde::Deserializer<'de>, {
        use std::fmt;
        struct WrapperVisitor;

        impl<'de> $crate::serde::de::Visitor<'de> for WrapperVisitor {
          type Value = $inner;

          fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("struct Wrapper")
          }

          fn visit_map<A>(self, mut map: A) -> Result<$inner, A::Error>
          where
            A: $crate::serde::de::MapAccess<'de>, {
            let mut value = None;

            while let Some(key) = map.next_key::<String>()? {
              if key == $root {
                value = Some(map.next_value()?);
              }
            }

            value.ok_or_else(|| $crate::serde::de::Error::missing_field($root))
          }
        }

        deserializer.deserialize_struct("Wrapper", &[$root], WrapperVisitor)
      }
    }
  };
}

// #[doc(hidden)]
// #[macro_export]
// macro_rules! deserialize_with_root_keyed {
//   ($root:tt : $inner:ty) => {
//     impl<'de> $crate::serde::Deserialize<'de> for $inner {
//       fn deserialize<D>(deserializer: D) -> ::std::result::Result<$inner,
// D::Error>       where
//         D: $crate::serde::de::Deserializer<'de>, {
//         use std::fmt;

//         use $crate::serde::de::{Deserialize, Deserializer, Visitor};

//         struct Wrapper {
//           root: $inner,
//         }

//         impl<'de> Deserialize<'de> for Wrapper {
//           fn deserialize<D>(deserializer: D) ->
// ::std::result::Result<Wrapper, D::Error>           where
//             D: Deserializer<'de>, {
//             struct FieldVisitor;

//             impl<'de> Visitor<'de> for FieldVisitor {
//               type Value = ();

//               fn expecting(&self, formatter: &mut fmt::Formatter) ->
// fmt::Result {                 formatter.write_str("struct Wrapper")
//               }

//               fn visit_str<E>(self, value: &str) -> ::std::result::Result<(),
// E>               where
//                 E: $crate::serde::de::Error, {
//                 if value == $root {
//                   Ok(())
//                 } else {
//                   Err(E::unknown_field(value, &[$root]))
//                 }
//               }
//             }

//             deserializer.deserialize_struct("Wrapper", &[$root],
// FieldVisitor)?;

//             Ok(Wrapper {
//               root: <$inner>::deserialize(deserializer)?,
//             })
//           }
//         }

//         let Wrapper { root } = Wrapper::deserialize(deserializer)?;
//         Ok(root)
//       }
//     }
//   };
// }

#[doc(hidden)]
#[macro_export]
macro_rules! serialize_with_root_key_value_pair {
  ($root:tt : $inner:ty, $key:ident, $value:ident) => {
    impl $crate::serde::Serialize for $inner {
      fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
      where
        S: $crate::serde::ser::Serializer, {
        use $crate::serde::ser::SerializeStruct;

        struct Wrapper<'a> {
          root: &'a $inner,
        }

        impl<'a> $crate::serde::Serialize for Wrapper<'a> {
          fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
          where
            S: $crate::serde::Serializer, {
            <$inner>::serialize(&self.root, serializer)
          }
        }

        #[derive(Serialize)]
        struct KeyValueWrapper<'a, K, V> {
          #[serde(flatten)]
          wrapper: &'a Wrapper<'a>,

          #[serde(flatten)]
          pair: KeyValuePair<K, V>,
        }

        let mut state = serializer.serialize_struct("Wrapper", 1)?;
        state.serialize_field(
          $root,
          &KeyValueWrapper {
            pair: KeyValuePair::new(&self.$key, &self.$value),
            wrapper: &Wrapper { root: self },
          },
        )?;
        state.end()
      }
    }
  };
}

// #[doc(hidden)]
// #[macro_export]
// macro_rules! deserialize_with_root_key_value_pair {
//   ($root:tt : $inner:ty) => {
//     impl<'de> $crate::serde::Deserialize<'de> for $inner {
//       fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self,
// D::Error>       where
//         D: $crate::serde::Deserializer<'de>, {
//         use std::fmt;
//         struct WrapperVisitor;

//         #[derive(Deserialize)]
//         struct Wrapper {
//           root: $inner,
//         }

//         #[derive(Deserialize)]
//         struct KeyValueWrapper<K, V> {
//           #[serde(flatten)]
//           wrapper: Wrapper,

//           #[serde(flatten)]
//           pair: KeyValuePair<K, V>,
//         }

//         impl<'de> $crate::serde::de::Visitor<'de> for WrapperVisitor {
//           type Value = $inner;

//           fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
// {             formatter.write_str("struct Wrapper")
//           }

//           fn visit_map<A>(self, mut map: A) -> Result<$inner, A::Error>
//           where
//             A: $crate::serde::de::MapAccess<'de>, {
//             let mut value = None;

//             while let Some(key) = map.next_key()? {
//               if key == $root {
//                 let kv_wrapper: KeyValueWrapper<$key, $value> =
// map.next_value()?;                 value =
// Some(kv_wrapper.wrapper.root.clone());               }
//             }

//             value.ok_or_else(||
// $crate::serde::de::Error::missing_field($root))           }
//         }

//         deserializer.deserialize_struct("Wrapper", &[$root], WrapperVisitor)
//       }
//     }
//   };
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! deserialize_with_root_key_value_pair {
//   ($root:tt : $inner:ty, $key:ident, $value:ident) => {
//     impl<'de> $crate::serde::Deserialize<'de> for $inner {
//       fn deserialize<D>(deserializer: D) -> ::std::result::Result<$inner,
// D::Error>       where
//         D: $crate::serde::de::Deserializer<'de>, {
//         use std::fmt;

//         use $crate::serde::de::{Deserialize, Deserializer, Visitor};

//         struct Wrapper {
//           root: $inner,
//         }

//         impl<'de> Deserialize<'de> for Wrapper {
//           fn deserialize<D>(deserializer: D) ->
// ::std::result::Result<Wrapper, D::Error>           where
//             D: Deserializer<'de>, {
//             struct FieldVisitor;

//             impl<'de> Visitor<'de> for FieldVisitor {
//               type Value = ();

//               fn expecting(&self, formatter: &mut fmt::Formatter) ->
// fmt::Result {                 formatter.write_str("struct Wrapper")
//               }

//               fn visit_str<E>(self, value: &str) -> ::std::result::Result<(),
// E>               where
//                 E: $crate::serde::de::Error, {
//                 if value == $root {
//                   Ok(())
//                 } else {
//                   Err(E::unknown_field(value, &[$root]))
//                 }
//               }
//             }

//             deserializer.deserialize_struct("Wrapper", &[$root],
// FieldVisitor)?;

//             Ok(Wrapper {
//               root: <$inner>::deserialize(deserializer)?,
//             })
//           }
//         }

//         let Wrapper { root } = Wrapper::deserialize(deserializer)?;
//         Ok(root)
//       }
//     }
//   };
// }

#[doc(hidden)]
#[macro_export]
macro_rules! serialize_keyed {
  ($inner:ty : $field:ident) => {
    impl $crate::serde::Serialize for $inner {
      fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
      where
        S: $crate::serde::ser::Serializer, {
        use $crate::serde::ser::SerializeMap;

        struct Wrapper<'a> {
          root: &'a $inner,
        }

        impl<'a> $crate::serde::Serialize for Wrapper<'a> {
          fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
          where
            S: $crate::serde::Serializer, {
            <$inner>::serialize(&self.root, serializer)
          }
        }

        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.$field, &Wrapper { root: self })?;
        state.end()
      }
    }
  };
}

// #[doc(hidden)]
// #[macro_export]
// macro_rules! deserialize_keyed {
//   ($inner:ty : $field:ident) => {
//     impl<'de> $crate::serde::Deserialize<'de> for $inner {
//       fn deserialize<D>(deserializer: D) -> ::std::result::Result<$inner,
// D::Error>       where
//         D: $crate::serde::de::Deserializer<'de>, {
//         use std::fmt;

//         use $crate::serde::de::{Deserialize, Deserializer, Visitor};

//         struct Wrapper {
//           root: $inner,
//         }

//         impl<'de> Deserialize<'de> for Wrapper {
//           fn deserialize<D>(deserializer: D) ->
// ::std::result::Result<Wrapper, D::Error>           where
//             D: Deserializer<'de>, {
//             struct FieldVisitor;

//             impl<'de> Visitor<'de> for FieldVisitor {
//               type Value = ();

//               fn expecting(&self, formatter: &mut fmt::Formatter) ->
// fmt::Result {                 formatter.write_str("struct Wrapper")
//               }

//               fn visit_str<E>(self, value: &str) -> ::std::result::Result<(),
// E>               where
//                 E: $crate::serde::de::Error, {
//                 if value == stringify!($field) {
//                   Ok(())
//                 } else {
//                   Err(E::unknown_field(value, &[stringify!($field)]))
//                 }
//               }
//             }

//             deserializer.deserialize_struct("Wrapper", &[stringify!($field)],
// FieldVisitor)?;

//             Ok(Wrapper {
//               root: <$inner>::deserialize(deserializer)?,
//             })
//           }
//         }

//         let Wrapper { root } = Wrapper::deserialize(deserializer)?;
//         Ok(root)
//       }
//     }
//   };
// }
