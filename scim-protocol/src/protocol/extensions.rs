use std::fmt::{self, Formatter};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

pub trait Extensions {
    const SCHEMA: &'static [&'static str];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoExtensions;
impl Extensions for NoExtensions {
    const SCHEMA: &'static [&'static str] = &[];
}

impl<'de> Deserialize<'de> for NoExtensions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NoExtensions;
            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("NoExtensions struct")
            }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(NoExtensions)
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                if map.next_entry::<Value, Value>()?.is_some() {
                    Err(serde::de::Error::custom(
                        "Unexpected data, NoExtenstions struct has no data",
                    ))
                } else {
                    Ok(NoExtensions)
                }
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

impl Serialize for NoExtensions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_unit()
    }
}
