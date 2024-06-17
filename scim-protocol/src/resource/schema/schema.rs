use crate::protocol::Meta;

use super::SchemaAttribute;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    /// The unique URI of the schema.
    /// When applicable, service providers MUST specify the URI, e.g., "urn:ietf:params:scim:schemas:core:2.0:User".
    /// Unlike most other schemas, which use some sort of Globally Unique Identifier (GUID) for the "id", the schema "id" is a URI so that it can be registered and is portable between different service providers andclients.
    /// REQUIRED.
    pub id: String,

    /// A complex attribute containing resource metadata.
    /// All "meta" sub-attributes are assigned by the service provider (have a "mutability" of "readOnly"), and all of these sub-attributes have a "returned" characteristic of "default".
    /// This attribute SHALL be ignored when provided by clients.
    pub meta: Meta,

    /// The schema's human-readable name.
    /// When applicable, service providers MUST specify the name, e.g., "User" or "Group".
    /// OPTIONAL.
    pub name: Option<String>,

    /// The schema's human-readable description.
    /// When applicable, service providers MUST specify the description.
    /// OPTIONAL.
    pub description: Option<String>,

    /// A complex type that defines service provider attributes and their qualities.
    pub attributes: Vec<SchemaAttribute>,
}
