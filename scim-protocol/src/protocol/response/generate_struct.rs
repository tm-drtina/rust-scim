#[macro_export]
macro_rules! generate_response_struct {
    ($resp:ident, $resource:ty, $extensions:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $resp {
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
            pub meta: $crate::protocol::Meta,

            pub resource: <Self as $crate::protocol::ScimResponse>::Resource,
            pub extensions: <Self as $crate::protocol::ScimResponse>::Extensions,
        }

        impl $crate::protocol::ScimResponse for $resp {
            type Resource = $resource;
            type Extensions = $extensions;

            fn from_parts(
                id: String,
                external_id: Option<String>,
                meta: $crate::protocol::Meta,
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
                $crate::protocol::Meta,
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

            fn meta(&self) -> &$crate::protocol::Meta {
                &self.meta
            }
        }
    };
}
