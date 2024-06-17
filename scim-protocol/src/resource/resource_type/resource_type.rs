use super::SchemaExtension;
use crate::protocol::Meta;

/// The `ResourceType` schema specifies the metadata about a resource type.
/// Resource type resources are READ-ONLY and identified using the following schema URI: "urn:ietf:params:scim:schemas:core:2.0:ResourceType".
/// Unlike other core resources, all attributes are REQUIRED unless otherwise specified.
/// The "id" attribute is not required for the resource type resource.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceType {
    pub schemas: Vec<String>,

    /// The resource type's server unique id.
    /// This is often the same value as the "name" attribute.
    /// OPTIONAL.
    pub id: Option<String>,

    /// A complex attribute containing resource metadata.
    /// All "meta" sub-attributes are assigned by the service provider (have a "mutability" of "readOnly"), and all of these sub-attributes have a "returned" characteristic of "default".
    /// This attribute SHALL be ignored when provided by clients.
    pub meta: Meta,

    /// The resource type name.  When applicable, service providers MUST specify the name, e.g., "User" or "Group".
    /// This name is referenced by the "meta.resourceType" attribute in all resources.
    /// REQUIRED.
    pub name: String,

    /// The resource type's human-readable description.
    /// When applicable, service providers MUST specify the description.
    /// OPTIONAL.
    pub description: Option<String>,

    /// The resource type's HTTP-addressable endpoint relative to the Base URL of the service provider, e.g., "Users".
    /// REQUIRED.
    pub endpoint: String,

    /// The resource type's primary/base schema URI, e.g., "urn:ietf:params:scim:schemas:core:2.0:User".
    /// This MUST be equal to the "id" attribute of the associated "Schema" resource.
    /// REQUIRED.
    pub schema: String,

    /// A list of URIs of the resource type's schema extensions.
    /// OPTIONAL.
    #[serde(default)]
    pub schema_extensions: Vec<SchemaExtension>,
}
