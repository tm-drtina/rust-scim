/// A single keyword indicating the circumstances under which the value of the attribute can be (re)defined
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SchemaAttributeMutability {
    /// The attribute SHALL NOT be modified.
    ReadOnly,

    /// The attribute MAY be updated and read at any time.
    /// This is the default value.
    ReadWrite,

    /// The attribute MAY be defined at resource creation(e.g., POST) or at record replacement via a request (e.g., a PUT).
    /// The attribute SHALL NOT be updated.
    Immutable,

    /// The attribute MAY be updated at any time.
    /// Attribute values SHALL NOT be returned (e.g., because the value is a stored hash).
    /// Note: An attribute with a mutability of "writeOnly" usually also has a returned setting of "never".
    WriteOnly,
}
impl Default for SchemaAttributeMutability {
    fn default() -> Self {
        Self::ReadWrite
    }
}
