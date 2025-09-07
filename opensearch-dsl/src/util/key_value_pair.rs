use std::fmt;

use serde::{
    de::{self, MapAccess, Visitor},
    ser::{Serialize, SerializeMap, Serializer},
    Deserialize, Deserializer,
};

#[derive(Clone, PartialEq, Eq)]
/// A key-value pair structure used for serialization and deserialization.
pub struct KeyValuePair<K, V> {
    /// The key of the key-value pair.
    pub key: K,
    /// The value associated with the key.
    pub value: V,
}

impl<K, V> KeyValuePair<K, V> {
    /// Creates an instance of [`KeyValuePair`]
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K, V> fmt::Debug for KeyValuePair<K, V>
where
    K: fmt::Debug + AsRef<str>,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyValuePair")
            .field(self.key.as_ref(), &self.value)
            .finish()
    }
}

impl<K, V> Serialize for KeyValuePair<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.key, &self.value)?;
        map.end()
    }
}

impl<'de, K, V> Deserialize<'de> for KeyValuePair<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<KeyValuePair<K, V>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyValuePairVisitor<K, V> {
            marker: std::marker::PhantomData<(K, V)>,
        }

        impl<'de, K, V> Visitor<'de> for KeyValuePairVisitor<K, V>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            type Value = KeyValuePair<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map with a single key/value pair")
            }

            fn visit_map<M>(self, mut access: M) -> Result<KeyValuePair<K, V>, M::Error>
            where
                M: MapAccess<'de>,
            {
                let (key, value) = match access.next_entry()? {
                    Some(entry) => entry,
                    None => return Err(de::Error::invalid_length(0, &self)),
                };

                if access.next_entry::<K, V>()?.is_some() {
                    return Err(de::Error::invalid_length(2, &self));
                }

                Ok(KeyValuePair::new(key, value))
            }
        }

        deserializer.deserialize_map(KeyValuePairVisitor {
            marker: std::marker::PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serializes_as_key_value_pair() {
        assert_serialize(KeyValuePair::new("key", "value"), json!({ "key": "value" }));
    }
}
