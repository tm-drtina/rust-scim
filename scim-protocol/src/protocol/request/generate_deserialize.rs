#[macro_export]
macro_rules! generate_request_deserialize {
    ($req:ident) => {
        impl<'de> serde::de::Deserialize<'de> for $req {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct RequestVisitor;
                impl<'de> serde::de::Visitor<'de> for RequestVisitor {
                    type Value = $req;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("request object")
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut external_id = None;
                        let mut resource_map = serde_json::Map::new();
                        let mut extensions_map = serde_json::Map::new();

                        let extensions_schema = <<Self::Value as $crate::protocol::ScimRequest>::Extensions as $crate::protocol::Extensions>::SCHEMA;

                        while let Some(key) = map.next_key::<String>()? {
                            if key.eq_ignore_ascii_case("externalId") {
                                if external_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("externalId"));
                                }
                                external_id = Some(map.next_value()?);
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
                        let resource = match <<Self::Value as $crate::protocol::ScimRequest>::Resource as serde::Deserialize>::deserialize(
                            serde_json::Value::Object(resource_map),
                        ) {
                            Ok(resource) => resource,
                            Err(err) => return Err(serde::de::Error::custom(err.to_string())),
                        };

                        let extensions = match <<Self::Value as $crate::protocol::ScimRequest>::Extensions as serde::Deserialize>::deserialize(
                            serde_json::Value::Object(extensions_map),
                        ) {
                            Ok(extensions) => extensions,
                            Err(err) => return Err(serde::de::Error::custom(err.to_string())),
                        };

                        Ok($req {
                            external_id,
                            resource,
                            extensions,
                        })
                    }
                }

                deserializer.deserialize_map(RequestVisitor)
            }
        }
    };
}
