use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SchemaExtension {
    /// The URI of an extended schema, e.g., "urn:edu:2.0:Staff".
    /// This MUST be equal to the "id" attribute of a "Schema" resource.
    /// REQUIRED.
    pub schema: String,

    /// A Boolean value that specifies whether or not the schema extension is required for the resource type.
    /// If true, a resource of this type MUST include this schema extension and also include any attributes declared as required in this schema extension.
    /// If false, a resource of this type MAY omit this schema extension.
    /// REQUIRED.
    pub required: bool,
}
