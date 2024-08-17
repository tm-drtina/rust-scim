use crate::resource::schema::SchemaAttribute;

/// The attribute's data type.
/// Valid values are "string", "boolean", "decimal", "integer", "dateTime", "reference", and "complex".
/// When an attribute is of type "complex", there SHOULD be a corresponding schema attribute "subAttributes" defined, listing the sub-attributes of the attribute.
///
/// Note: RFC is missing binary type, but uses it in examples.
/// See: <https://www.rfc-editor.org/errata/eid5606>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaAttributeType {
    String,
    Boolean,
    Decimal,
    Integer,
    DateTime,
    Reference {
        /// A multi-valued array of JSON strings that indicate the SCIM resource types that may be referenced.
        /// Valid values are as follows:
        ///  +  A SCIM resource type (e.g., "User" or "Group"),
        ///  +  "external" - indicating that the resource is an external resource (e.g., a photo), or
        ///  +  "uri" - indicating that the reference is to a service endpoint or an identifier (e.g., a schema URN).
        ///
        /// This attribute is only applicable for attributes that are of type "reference" (Section 2.3.7).
        // TODO: Convert into enum?
        reference_types: Vec<String>,
    },
    Complex {
        /// When an attribute is of type "complex", "subAttributes" defines a set of sub-attributes.
        /// "subAttributes" has the same schema sub-attributes as "attributes".
        sub_attributes: Vec<SchemaAttribute>,
    },
    Binary,
}
impl Default for SchemaAttributeType {
    fn default() -> Self {
        Self::String
    }
}
