use serde::{Deserialize, Serialize};

/// A list of entitlements for the user that represent a thing the user has.
/// An entitlement may be an additional right to a thing, object, or service.
/// No vocabulary or syntax is specified; service providers and clients are expected to encode sufficient information in the value so as to accurately and without ambiguity determine what the user has access to.
/// This value has no canonical types, although a type may be useful as a means to scope entitlements.
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Entitlement {
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
    pub value: String,

    /// The reference URI of a target resource, if the attribute is a reference.
    /// URIs are canonicalized per Section 6.2 of [RFC3986].
    /// While the representation of a resource may vary in different SCIM protocol API versions (see Section 3.13 of [RFC7644]), URIs for SCIM resources with an API version SHALL be considered comparable to URIs without a version or with a different version.
    /// For example, "<https://example.com/Users/12345>" is equivalent to "<https://example.com/v2/Users/12345>".
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
}
