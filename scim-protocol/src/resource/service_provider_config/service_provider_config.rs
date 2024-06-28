use crate::protocol::Meta;

use super::{
    AuthenticationScheme, BulkConfig, ChangePasswordConfig, ETagConfig, FilterConfig, PatchConfig,
    SortConfig,
};

/// SCIM provides a schema for representing the service provider's configuration, identified using the following schema URI: "urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig".
/// The service provider configuration resource enables a service provider to discover SCIM specification features in a standardized form as well as provide additional implementation details to clients.
/// All attributes have a mutability of "readOnly".
/// Unlike other core resources, the "id" attribute is not required for the service provider configuration resource.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceProviderConfig {
    /// Must be singleton list of `["urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig"]`
    // TODO: verify in requests
    pub schemas: Vec<String>,

    /// An HTTP-addressable URL pointing to the service provider's human-consumable help documentation.
    /// OPTIONAL.
    pub documentation_uri: Option<String>,

    /// A complex type that specifies PATCH configuration options.
    /// REQUIRED.
    /// See Section 3.5.2 of [RFC7644].
    pub patch: PatchConfig,

    /// A complex type that specifies bulk configuration options.
    /// See Section 3.7 of [RFC7644].
    /// REQUIRED.
    pub bulk: BulkConfig,

    /// A complex type that specifies FILTER options.
    /// REQUIRED.
    /// See Section 3.4.2.2 of [RFC7644].
    pub filter: FilterConfig,

    /// A complex type that specifies configuration options related to changing a password.
    /// REQUIRED.
    pub change_password: ChangePasswordConfig,

    /// A complex type that specifies Sort configuration options.
    /// REQUIRED.
    pub sort: SortConfig,

    /// A complex type that specifies `ETag` configuration options.
    /// REQUIRED.
    pub etag: ETagConfig,

    /// A multi-valued complex type that specifies supported authentication scheme properties.
    /// To enable seamless discovery of configurations, the service provider SHOULD, with the appropriate security considerations, make the authenticationSchemes attribute publicly accessible without prior authentication.
    /// REQUIRED.
    pub authentication_schemes: Vec<AuthenticationScheme>,

    pub meta: Meta,
}
