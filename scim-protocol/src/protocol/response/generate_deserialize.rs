#[macro_export]
macro_rules! generate_response_deserialize {
    ($resp:ident) => {
        impl<'de> serde::de::Deserialize<'de> for $resp {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct ResponseVisitor;
                impl<'de> serde::de::Visitor<'de> for ResponseVisitor {
                    type Value = $resp;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("response object")
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut schemas = None;
                        let mut id = None;
                        let mut external_id = None;
                        let mut meta = None;
                        let mut resource_map = serde_json::Map::new();
                        let mut extensions_map = serde_json::Map::new();

                        let extensions_schema =
                            <<Self::Value as $crate::protocol::ScimResponse>::Extensions as $crate::protocol::Extensions>::SCHEMA;

                        while let Some(key) = map.next_key::<String>()? {
                            if key.eq_ignore_ascii_case("schemas") {
                                if schemas.is_some() {
                                    return Err(serde::de::Error::duplicate_field("schemas"));
                                }
                                schemas = Some(map.next_value()?);
                            } else if key.eq_ignore_ascii_case("id") {
                                if id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                id = Some(map.next_value()?);
                            } else if key.eq_ignore_ascii_case("externalId") {
                                if external_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("external_id"));
                                }
                                external_id = Some(map.next_value()?);
                            } else if key.eq_ignore_ascii_case("meta") {
                                if meta.is_some() {
                                    return Err(serde::de::Error::duplicate_field("meta"));
                                }
                                meta = Some(map.next_value()?);
                            } else if extensions_schema.contains(&key.as_str()) {
                                if let Some(_old_value) = extensions_map.insert(key, map.next_value()?) {
                                    return Err(serde::de::Error::custom(format!("duplicate value")));
                                }
                            } else {
                                if let Some(_old_value) = resource_map.insert(key, map.next_value()?) {
                                    return Err(serde::de::Error::custom(format!("duplicate value")));
                                }
                            }
                        }

                        let Some(schemas) = schemas else {
                            return Err(serde::de::Error::missing_field("schemas"));
                        };
                        let Some(id) = id else {
                            return Err(serde::de::Error::missing_field("id"));
                        };
                        let Some(meta) = meta else {
                            return Err(serde::de::Error::missing_field("meta"));
                        };

                        let resource = match <<Self::Value as $crate::protocol::ScimResponse>::Resource as serde::Deserialize>::deserialize(
                            serde_json::Value::Object(resource_map),
                        ) {
                            Ok(resource) => resource,
                            Err(err) => return Err(serde::de::Error::custom(err.to_string())),
                        };

                        let extensions = match <<Self::Value as $crate::protocol::ScimResponse>::Extensions as serde::Deserialize>::deserialize(
                            serde_json::Value::Object(extensions_map),
                        ) {
                            Ok(extensions) => extensions,
                            Err(err) => return Err(serde::de::Error::custom(err.to_string())),
                        };

                        Ok($resp {
                            schemas,
                            id,
                            external_id,
                            meta,
                            resource,
                            extensions,
                        })
                    }
                }

                deserializer.deserialize_map(ResponseVisitor)
            }
        }
    };
}
