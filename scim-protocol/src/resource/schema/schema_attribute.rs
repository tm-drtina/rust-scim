use crate::serde_macros::{enum_deserializer, struct_deserializer};

use super::{
    SchemaAttributeMutability, SchemaAttributeReturned, SchemaAttributeType,
    SchemaAttributeUniqueness,
};

// TODO: Handle case-insensitive values: Unless otherwise specified, all schema attributes are case insensitive.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "SchemaAttributeScimFormat")]
pub struct SchemaAttribute {
    /// The attribute's name.
    pub name: String,

    /// The attribute's data type.
    /// Valid values are "string", "boolean", "decimal", "integer", "dateTime", "reference", and "complex".
    /// When an attribute is of type "complex", there SHOULD be a corresponding schema attribute "subAttributes" defined, listing the sub-attributes of the attribute.
    pub attribute_type: SchemaAttributeType,

    /// A Boolean value indicating the attribute's plurality.
    pub multi_valued: bool,

    /// The attribute's human-readable description.
    /// When applicable, service providers MUST specify the description.
    pub description: String,

    /// A Boolean value that specifies whether or not the attribute is required.
    pub required: bool,

    /// A collection of suggested canonical values that MAY be used (e.g., "work" and "home").
    /// In some cases, service providers MAY choose to ignore unsupported values.
    /// OPTIONAL.
    pub canonical_values: Vec<String>,

    /// A Boolean value that specifies whether or not a string attribute is case sensitive.
    /// The server SHALL use case sensitivity when evaluating filters.
    /// For attributes that are case exact, the server SHALL preserve case for any value submitted.
    /// If the attribute is case insensitive, the server MAY alter case for a submitted value.
    /// Case sensitivity also impacts how attribute values MAY be compared against filter values (see Section 3.4.2.2 of [RFC7644]).
    pub case_exact: bool,

    /// A single keyword indicating the circumstances under which the value of the attribute can be (re)defined
    pub mutability: SchemaAttributeMutability,

    /// A single keyword that indicates when an attribute and associated values are returned in response to a GET request or in response to a PUT, POST, or PATCH request.
    pub returned: SchemaAttributeReturned,

    /// A single keyword value that specifies how the service provider enforces uniqueness of attribute values.
    /// A server MAY reject an invalid value based on uniqueness by returning HTTP response code 400 (Bad Request).
    /// A client MAY enforce uniqueness on the client side to a greater degree than the service provider enforces.
    /// For example, a client could make a value unique while the server has uniqueness of "none".
    pub uniqueness: SchemaAttributeUniqueness,
}

impl TryFrom<SchemaAttributeScimFormat> for SchemaAttribute {
    type Error = &'static str;

    fn try_from(mut value: SchemaAttributeScimFormat) -> Result<Self, Self::Error> {
        let attribute_type = match value.attribute_type_variant {
            AttributeTypeVariant::String => SchemaAttributeType::String,
            AttributeTypeVariant::Boolean => SchemaAttributeType::Boolean,
            AttributeTypeVariant::Decimal => SchemaAttributeType::Decimal,
            AttributeTypeVariant::Integer => SchemaAttributeType::Integer,
            AttributeTypeVariant::DateTime => SchemaAttributeType::DateTime,
            AttributeTypeVariant::Binary => SchemaAttributeType::Binary,
            AttributeTypeVariant::Reference => SchemaAttributeType::Reference {
                reference_types: value
                    .reference_types
                    .take()
                    .ok_or("Missing field `referenceTypes`")?,
            },
            AttributeTypeVariant::Complex => SchemaAttributeType::Complex {
                sub_attributes: value
                    .sub_attributes
                    .take()
                    .ok_or("Missing field `subAttributes`")?,
            },
        };

        // TODO: are these check correct? Do we need to return error, or can we ignore them?
        if value.reference_types.is_some() {
            return Err("`referenceTypes` field set, but type is not `reference`");
        }
        if value.sub_attributes.is_some() {
            return Err("`subAttributes` field set, but type is not `complex`");
        }

        Ok(SchemaAttribute {
            name: value.name,
            attribute_type,
            multi_valued: value.multi_valued,
            description: value.description,
            required: value.required,
            canonical_values: value.canonical_values,
            case_exact: value.case_exact,
            mutability: value.mutability,
            returned: value.returned,
            uniqueness: value.uniqueness,
        })
    }
}

enum_deserializer!(AttributeTypeVariant {
    String,
    Boolean,
    Decimal,
    Integer,
    DateTime,
    Reference,
    Complex,
    Binary,
});

struct_deserializer!(SchemaAttributeScimFormat {
    [required] Name: String,
    [required] MultiValued: bool,
    [required] Description: String,
    [default] Required: bool,
    [default] CanonicalValues: Vec<String>,
    [default] CaseExact: bool,
    [default] Mutability: SchemaAttributeMutability,
    [default] Returned: SchemaAttributeReturned,
    [default] Uniqueness: SchemaAttributeUniqueness,
    // TODO: use default (String) or return error?
    [required] AttributeTypeVariant: AttributeTypeVariant = "type",
    SubAttributes: Vec<SchemaAttribute>,
    ReferenceTypes: Vec<String>,
});
