#[macro_export]
macro_rules! generate_extension {
    (extension $name:ident {
        $($var:ident: $type:ty),*
        $(,)?
    }) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $(pub $var: $type),*
        }
        impl $crate::protocol::Extensions for $name {
            const SCHEMA: &'static [&'static str] = &[$(
                <$type as $crate::resource::ScimSchema>::SCHEMA
            ),*];
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum Field {
                    $($var),*
                    // Ignore,
                }
                #[doc(hidden)]
                struct FieldVisitor;
                impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        std::fmt::Formatter::write_str(formatter, "field identifier")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            // TODO: should this be case insensitive check?
                            $(<$type as $crate::resource::ScimSchema>::SCHEMA => {
                                Ok(Field::$var)
                            })*
                            _ => {
                                return Err(E::unknown_field(value, FIELDS));
                            }
                            //_ => Ok(Field::Ignore),
                        }
                    }
                    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            // TODO: should this be case insensitive check?
                            $(_ if value == <$type as $crate::resource::ScimSchema>::SCHEMA.as_bytes() => {
                                Ok(Field::$var)
                            })*
                            _ => {
                                return Err(E::unknown_field(&String::from_utf8_lossy(value), FIELDS));
                            }
                            //_ => Ok(Field::Ignore),
                        }
                    }
                }
                impl<'de> serde::Deserialize<'de> for Field {
                    #[inline]
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        serde::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
                    }
                }
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        std::fmt::Formatter::write_str(formatter, concat!("struct ", stringify!($name)))
                    }

                    #[inline]
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        $(let mut $var: Option<$type> = None;)*
                        while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                            match key {
                                $(Field::$var => {
                                    if Option::is_some(&$var) {
                                        return Err(<A::Error as serde::de::Error>::duplicate_field(
                                            <$type as $crate::resource::ScimSchema>::SCHEMA,
                                        ));
                                    }
                                    $var = Some(serde::de::MapAccess::next_value::<$type>(&mut map)?);
                                })*
                                /*Field::Ignore => {
                                    let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut map)?;
                                }*/
                            }
                        }
                        $(let $var = match $var {
                            Some(field) => field,
                            None => {
                                return Err(<A::Error as serde::de::Error>::missing_field(
                                    <$type as $crate::resource::ScimSchema>::SCHEMA,
                                ));
                            }
                        };)*

                        Ok($name {
                            $($var),*
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    $(<$type as $crate::resource::ScimSchema>::SCHEMA),*
                ];
                serde::Deserializer::deserialize_struct(deserializer, stringify!($name), FIELDS, Visitor)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #[allow(unused_mut)]
                let mut state = serde::Serializer::serialize_struct(
                    serializer,
                    stringify!($name),
                    <$name as $crate::protocol::Extensions>::SCHEMA.len(),
                )?;
                $(serde::ser::SerializeStruct::serialize_field(
                    &mut state,
                    <$type as $crate::resource::ScimSchema>::SCHEMA,
                    &self.$var,
                )?;)*
                serde::ser::SerializeStruct::end(state)
            }
        }
    };
}
