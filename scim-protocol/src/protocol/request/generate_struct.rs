#[macro_export]
macro_rules! generate_request_struct {
    ($req:ident, $resource:ty, $extensions:ty) => {
        pub struct $req {
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

            pub resource: <Self as $crate::protocol::ScimRequest>::Resource,
            pub extensions: <Self as $crate::protocol::ScimRequest>::Extensions,
        }

        impl $crate::protocol::ScimRequest for $req {
            type Resource = $resource;
            type Extensions = $extensions;

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
    };
}
