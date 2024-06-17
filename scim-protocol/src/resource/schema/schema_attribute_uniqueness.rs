/// A single keyword value that specifies how the service provider enforces uniqueness of attribute values.
/// A server MAY reject an invalid value based on uniqueness by returning HTTP response code 400 (Bad Request).
/// A client MAY enforce uniqueness on the client side to a greater degree than the service provider enforces.
/// For example, a client could make a value unique while the server has uniqueness of "none".
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SchemaAttributeUniqueness {
    /// The values are not intended to be unique in any way.
    /// DEFAULT.
    None,

    /// The value SHOULD be unique within the context of the current SCIM endpoint (or tenancy) and MAY be globally unique (e.g., a "username", email address, or other server-generated key or counter).
    /// No two resources on the same server SHOULD possess the same value.
    Server,

    /// The value SHOULD be globally unique (e.g., an email address, a GUID, or other value).
    /// No two resources on any server SHOULD possess the same value.
    Global,
}
impl Default for SchemaAttributeUniqueness {
    fn default() -> Self {
        Self::None
    }
}
