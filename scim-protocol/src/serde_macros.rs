macro_rules! field_value {
    ($key:ident, $value:expr) => {
        $value
    };
    ($key:ident) => {
        // Note: camel is not camelCase, but rather PascalCase
        // RFC specifies default casing as camelCase, but we need to have PascalCase until this is fixed
        // See: https://github.com/dtolnay/paste/issues/72
        // See: https://github.com/dtolnay/paste/pull/84
        paste::paste! { stringify!([<$key:camel>]) }
    };
}

macro_rules! enum_deserializer {
    ($enum_name:ident {$($key:ident$(: $value:expr)?),+$(,)?}) => {
        paste::paste!{
            enum $enum_name {
                $($key),+
            }
            const [< $enum_name:snake:upper _VARIANTS >]: &[&str] = &[
                $($crate::serde_macros::field_value!($key$(, $value)?)),+
            ];
            struct  [< $enum_name Visitor >];
            impl<'de> serde::de::Visitor<'de> for  [< $enum_name Visitor >] {
                type Value = $enum_name;
                fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    formatter.write_str("field identifier")
                }
                fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                    match value {
                        $(_ if value.eq_ignore_ascii_case($crate::serde_macros::field_value!($key$(, $value)?)) => Ok($enum_name::$key)),+,
                        _ => Err(serde::de::Error::unknown_variant(value, [< $enum_name:snake:upper _VARIANTS >])),
                    }
                }
                fn visit_bytes<E: serde::de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
                    match value {
                        $(_ if value.eq_ignore_ascii_case($crate::serde_macros::field_value!($key$(, $value)?).as_bytes()) => Ok($enum_name::$key)),+,
                        _ => {
                            let value = &String::from_utf8_lossy(value);
                            Err(serde::de::Error::unknown_variant(value, [< $enum_name:snake:upper _VARIANTS >]))
                        }
                    }
                }
                fn visit_borrowed_str<E: serde::de::Error>(self, value: &'de str) -> Result<Self::Value, E> {
                    match value {
                        $(_ if value.eq_ignore_ascii_case($crate::serde_macros::field_value!($key$(, $value)?)) => Ok($enum_name::$key)),+,
                        _ => Err(serde::de::Error::unknown_variant(value, [< $enum_name:snake:upper _VARIANTS >])),
                    }
                }
                fn visit_borrowed_bytes<E: serde::de::Error>(
                    self,
                    value: &'de [u8],
                ) -> Result<Self::Value, E> {
                    match value {
                        $(_ if value.eq_ignore_ascii_case($crate::serde_macros::field_value!($key$(, $value)?).as_bytes()) => Ok($enum_name::$key)),+,
                        _ => {
                            let value = &String::from_utf8_lossy(value);
                            Err(serde::de::Error::unknown_variant(value, [< $enum_name:snake:upper _VARIANTS >]))
                        }
                    }
                }
            }
            impl<'de> serde::de::Deserialize<'de> for $enum_name {
                #[inline]
                fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    deserializer.deserialize_identifier( [< $enum_name Visitor >])
                }
            }
        }
    };
}

macro_rules! value {
    ([required] $key:ident, $field_value:expr) => {
        $key.ok_or(serde::de::Error::missing_field($field_value))?
    };
    ([default] $key:ident, $field_value:expr) => {
        $key.unwrap_or_default()
    };
    ($key:ident, $field_value:expr) => {
        $key
    };
}

macro_rules! value_type {
    ([required] $value_type:ty) => {
        $value_type
    };
    ([default] $value_type:ty) => {
        $value_type
    };
    ($value_type:ty) => {
        Option<$value_type>
    };
}

macro_rules! struct_deserializer {
    ($type:ident {$($([$modifier:tt])? $key:ident: $value_type:ty$(= $value:literal)?),+$(,)?}) => {
        paste::paste!{
            struct $type {
                $(
                    [< $key:snake >]: $crate::serde_macros::value_type!($([$modifier])? $value_type)
                ),+
            }
        }
        impl<'de> serde::Deserialize<'de> for $type {
            #[allow(clippy::too_many_lines)]
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                $crate::serde_macros::enum_deserializer!(Field {$($key: $crate::serde_macros::field_value!($key$(, $value)?)),+});

                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $type;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($type)))
                    }
                    #[inline]
                    fn visit_map<A: serde::de::MapAccess<'de>>(
                        self,
                        mut map: A,
                    ) -> Result<Self::Value, A::Error> {
                        $(
                            paste::paste!{ let mut [< $key:snake >]: Option<$value_type> = None; }
                        )+
                        while let Some(key) = map.next_key()? {
                            match key {
                                $(
                                    Field::$key => {
                                        paste::paste!{
                                            if [< $key:snake >].is_some() {
                                                return Err(<A::Error as serde::de::Error>::duplicate_field($crate::serde_macros::field_value!($key$(, $value)?)));
                                            }
                                            [< $key:snake >] = Some(map.next_value()?);
                                        }
                                    }
                                )+
                            }
                        }

                        paste::paste!{
                            #[allow(clippy::redundant_field_names)]
                            Ok($type {
                                $(
                                    [< $key:snake >]: $crate::serde_macros::value!($([$modifier])? [< $key:snake >], $crate::serde_macros::field_value!($key$(, $value)?))
                                ),+
                            })
                        }
                    }
                }

                deserializer.deserialize_map(Visitor)
            }
        }
    };
}

pub(crate) use field_value;
pub(crate) use value;
pub(crate) use value_type;

pub(crate) use enum_deserializer;
pub(crate) use struct_deserializer;
