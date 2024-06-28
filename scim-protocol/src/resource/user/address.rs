use serde::{Deserialize, Serialize};

/// A physical mailing address for this user.
/// Canonical type values of "work", "home", and "other".
/// This attribute is a complex type with the following sub-attributes.
/// All sub-attributes are OPTIONAL.
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub struct Address {
    /// A label indicating the attribute's function, e.g., "work" or "home".
    #[serde(rename = "type")]
    pub value_type: Option<String>,

    /// A Boolean value indicating the 'primary' or preferred attribute value for this attribute, e.g., the preferred mailing address or the primary email address.
    /// The primary attribute value "true" MUST appear no more than once.
    /// If not specified, the value of "primary" SHALL be assumed to be "false".
    #[serde(default)]
    pub primary: bool,

    /// A human-readable name, primarily used for display purposes and having a mutability of "immutable".
    pub display: Option<String>,

    /// The attribute's significant value, e.g., email address, phone number.
    pub value: Option<String>,

    /// The reference URI of a target resource, if the attribute is a reference.
    /// URIs are canonicalized per Section 6.2 of [RFC3986].
    /// While the representation of a resource may vary in different SCIM protocol API versions (see Section 3.13 of [RFC7644]), URIs for SCIM resources with an API version SHALL be considered comparable to URIs without a version or with a different version.
    /// For example, "<https://example.com/Users/12345>" is equivalent to "<https://example.com/v2/Users/12345>".
    #[serde(rename = "$ref")]
    pub reference: Option<String>,

    /// The full mailing address, formatted for display or use
    pub formatted: Option<String>,

    /// The full street address component, which may include house number, street name, P.O. box, and multi-line extended street address information.
    /// This attribute MAY contain newlines.
    pub street_address: Option<String>,

    /// The city or locality component.
    pub locality: Option<String>,

    /// The state or region component.
    pub region: Option<String>,

    /// The zip code or postal code component.
    pub postal_code: Option<String>,

    /// The country name component.
    /// When specified, the value MUST be in ISO 3166-1 "alpha-2" code format [ISO3166]; e.g., the United States and Sweden are "US" and "SE", respectively.
    pub country: Option<String>,
}
