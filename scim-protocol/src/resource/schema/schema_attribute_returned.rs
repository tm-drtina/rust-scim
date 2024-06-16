use serde::Deserialize;

/// A single keyword that indicates when an attribute and associated values are returned in response to a GET request or in response to a PUT, POST, or PATCH request.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SchemaAttributeReturned {
    /// The attribute is always returned, regardless of the contents of the "attributes" parameter.
    /// For example, "id" is always returned to identify a SCIM resource.
    Always,

    /// The attribute is never returned.
    /// This may occur because the original attribute value (e.g., a hashed value) is not retained by the service provider.
    /// A service provider MAY allow attributes to be used in a search filter.
    Never,

    /// The attribute is returned by default in all SCIM operation responses where attribute values are returned.
    /// If the GET request "attributes" parameter is specified, attribute values are only returned if the attribute is named in the "attributes" parameter.
    /// DEFAULT.
    Default,

    /// The attribute is returned in response to any PUT, POST, or PATCH operations if the attribute was specified by the client (for example, the attribute was modified).
    /// The attribute is returned in a SCIM query operation only if specified in the "attributes" parameter.
    Request,
}
impl Default for SchemaAttributeReturned {
    fn default() -> Self {
        Self::Default
    }
}
