use std::collections::BTreeSet;
use std::iter::once;

use crate::protocol::{Extensions, Meta, NoExtensions};
use crate::resource::user::User;
use crate::resource::ScimSchema;

pub struct SchemaMismatch<'a> {
    pub missing: Vec<&'a str>,
    pub extra: Vec<&'a str>,
}

pub trait ScimResponse {
    type Resource: ScimSchema;
    type Extensions: Extensions;

    fn from_parts(
        id: String,
        external_id: Option<String>,
        meta: Meta,
        resource: Self::Resource,
        extensions: Self::Extensions,
    ) -> Self;
    fn into_parts(
        self,
    ) -> (
        String,
        Option<String>,
        Meta,
        Self::Resource,
        Self::Extensions,
    );

    fn schemas(&self) -> &[String];
    fn id(&self) -> &str;
    fn external_id(&self) -> Option<&str>;
    fn meta(&self) -> &Meta;

    fn defined_schemas() -> impl Iterator<Item = &'static str> {
        once(<Self::Resource as ScimSchema>::SCHEMA)
            .chain(<Self::Extensions as Extensions>::SCHEMA.iter().copied())
    }

    fn validate_schema<'a>(&'a self) -> Result<(), SchemaMismatch<'a>> {
        let actual_schemas: BTreeSet<&str> = self.schemas().iter().map(String::as_str).collect();
        let expected_schemas: BTreeSet<&str> = Self::defined_schemas().collect();
        if actual_schemas == expected_schemas {
            Ok(())
        } else {
            let missing = expected_schemas
                .difference(&actual_schemas)
                .copied()
                .collect();
            let extra = actual_schemas
                .difference(&expected_schemas)
                .copied()
                .collect();
            Err(SchemaMismatch { missing, extra })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserResponse {
    pub schemas: Vec<String>,

    /// A unique identifier for a SCIM resource as defined by the service provider.
    /// Each representation of the resource MUST include a non-empty "id" value.
    /// This identifier MUST be unique across the SCIM service provider's entire set of resources.
    /// It MUST be a stable, non-reassignable identifier that does not change when the same resource is returned in subsequent requests.
    /// The value of the "id" attribute is always issued by the service provider and MUST NOT be specified by the client.
    /// The string "bulkId" is a reserved keyword and MUST NOT be used within any unique identifier value.
    /// The attribute characteristics are "caseExact" as "true", a mutability of "readOnly", and a "returned" characteristic of "always".
    /// See Section 9 for additional considerations regarding privacy.
    pub id: String,
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
    /// A complex attribute containing resource metadata.
    /// All "meta" sub-attributes are assigned by the service provider (have a "mutability" of "readOnly"), and all of these sub-attributes have a "returned" characteristic of "default".
    /// This attribute SHALL be ignored when provided by clients.
    pub meta: Meta,

    pub resource: <Self as ScimResponse>::Resource,
    pub extensions: <Self as ScimResponse>::Extensions,
}

impl ScimResponse for UserResponse {
    type Resource = User;
    type Extensions = NoExtensions;

    fn from_parts(
        id: String,
        external_id: Option<String>,
        meta: Meta,
        resource: Self::Resource,
        extensions: Self::Extensions,
    ) -> Self {
        Self {
            schemas: Self::defined_schemas().map(String::from).collect(),
            id,
            external_id,
            meta,
            resource,
            extensions,
        }
    }

    fn into_parts(
        self,
    ) -> (
        String,
        Option<String>,
        Meta,
        Self::Resource,
        Self::Extensions,
    ) {
        (
            self.id,
            self.external_id,
            self.meta,
            self.resource,
            self.extensions,
        )
    }

    fn schemas(&self) -> &[String] {
        &self.schemas
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn external_id(&self) -> Option<&str> {
        self.external_id.as_ref().map(|id| id.as_str())
    }

    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'de> serde::de::Deserialize<'de> for UserResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResponseVisitor;
        impl<'de> serde::de::Visitor<'de> for ResponseVisitor {
            type Value = UserResponse;

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
                    <<Self::Value as ScimResponse>::Extensions as Extensions>::SCHEMA;

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

                let schemas = match schemas {
                    Some(s) => s,
                    None => return Err(serde::de::Error::missing_field("schemas")),
                };
                let id = match id {
                    Some(id) => id,
                    None => return Err(serde::de::Error::missing_field("id")),
                };
                let meta = match meta {
                    Some(s) => s,
                    None => return Err(serde::de::Error::missing_field("meta")),
                };

                let resource = match <<Self::Value as ScimResponse>::Resource as serde::Deserialize>::deserialize(
                    serde_json::Value::Object(resource_map),
                ) {
                    Ok(user) => user,
                    Err(err) => return Err(serde::de::Error::custom(err.to_string())),
                };

                let extensions = match <<Self::Value as ScimResponse>::Extensions as serde::Deserialize>::deserialize(
                    serde_json::Value::Object(extensions_map),
                ) {
                    Ok(user) => user,
                    Err(err) => return Err(serde::de::Error::custom(err.to_string())),
                };

                Ok(UserResponse {
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
