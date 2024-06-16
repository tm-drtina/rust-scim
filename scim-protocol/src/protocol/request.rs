use crate::protocol::{Extensions, NoExtensions};
use crate::resource::user::User;
use crate::resource::ScimSchema;

pub trait ScimRequest {
    type Resource: ScimSchema;
    type Extensions: Extensions;

    fn from_parts(
        external_id: Option<String>,
        resource: Self::Resource,
        extensions: Self::Extensions,
    ) -> Self;
    fn into_parts(self) -> (Option<String>, Self::Resource, Self::Extensions);

    fn external_id(&self) -> Option<&str>;
    fn resource(&self) -> &Self::Resource;
    fn extensions(&self) -> &Self::Extensions;
}

pub struct UserRequest {
    /// A String that is an identifier for the resource as defined by the provisioning client.
    /// The "externalId" may simplify identification of a resource between the provisioning client and the service provider by allowing the client to use a filter to locate the resource with an identifier from the provisioning domain, obviating the need to store a local mapping between the provisioning domain's identifier of the resource and the identifier used by the service provider.
    /// Each resource MAY include a non-empty "externalId" value.
    /// The value of the "externalId" attribute is always issued by the provisioning client and MUST NOT be specified by the service provider.
    /// The service provider MUST always interpret the externalId as scoped to the provisioning domain.
    /// While the server does not enforce uniqueness, it is assumed that the value's uniqueness is controlled by the client setting the value.
    /// See Section 9 for additional considerations regarding privacy.
    /// This attribute has "caseExact" as "true" and a mutability of "readWrite".
    /// This attribute is OPTIONAL.
    pub external_id: Option<String>,

    pub resource: User,

    pub extensions: NoExtensions,
}

impl ScimRequest for UserRequest {
    type Resource = User;
    type Extensions = NoExtensions;

    fn from_parts(
        external_id: Option<String>,
        resource: Self::Resource,
        extensions: Self::Extensions,
    ) -> Self {
        Self {
            external_id,
            resource,
            extensions,
        }
    }
    fn into_parts(self) -> (Option<String>, Self::Resource, Self::Extensions) {
        (self.external_id, self.resource, self.extensions)
    }

    fn external_id(&self) -> Option<&str> {
        self.external_id.as_ref().map(|id| id.as_str())
    }
    fn resource(&self) -> &Self::Resource {
        &self.resource
    }
    fn extensions(&self) -> &Self::Extensions {
        &self.extensions
    }
}

impl<'de> serde::de::Deserialize<'de> for UserRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RequestVisitor;
        impl<'de> serde::de::Visitor<'de> for RequestVisitor {
            type Value = UserRequest;

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

                let extensions_schema =
                    <<Self::Value as ScimRequest>::Extensions as Extensions>::SCHEMA;

                while let Some(key) = map.next_key::<String>()? {
                    if key.eq_ignore_ascii_case("externalId") {
                        if external_id.is_some() {
                            return Err(serde::de::Error::duplicate_field("external_id"));
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
                let resource = match <<Self::Value as ScimRequest>::Resource as serde::Deserialize>::deserialize(
                    serde_json::Value::Object(resource_map),
                ) {
                    Ok(user) => user,
                    Err(err) => return Err(serde::de::Error::custom(err.to_string())),
                };

                let extensions = match <<Self::Value as ScimRequest>::Extensions as serde::Deserialize>::deserialize(
                    serde_json::Value::Object(extensions_map),
                ) {
                    Ok(user) => user,
                    Err(err) => return Err(serde::de::Error::custom(err.to_string())),
                };

                Ok(UserRequest {
                    external_id,
                    resource,
                    extensions,
                })
            }
        }

        deserializer.deserialize_map(RequestVisitor)
    }
}

impl serde::Serialize for UserRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        serde::ser::SerializeMap::serialize_entry(&mut map, "externalId", &self.external_id)?;
        serde::Serialize::serialize(
            &self.resource,
            crate::flatmap_serializer::FlatMapSerializer(&mut map),
        )?;
        serde::Serialize::serialize(
            &self.extensions,
            crate::flatmap_serializer::FlatMapSerializer(&mut map),
        )?;

        serde::ser::SerializeMap::end(map)
    }
}
