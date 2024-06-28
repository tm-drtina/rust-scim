use serde::{Deserialize, Serialize};

/// A URI that is a uniform resource locator (as defined in Section 1.1.3 of [RFC3986]) that points to a resource location representing the user's image.
/// The resource MUST be a file (e.g., a GIF, JPEG, or PNG image file) rather than a web page containing an image.
/// Service providers MAY return the same image in different sizes, although it is recognized that no standard for describing images of various sizes currently exists.
/// Note that this attribute SHOULD NOT be used to send down arbitrary photos taken by this user; instead, profile photos of the user that are suitable for display when describing the user should be sent.
/// Instead of the standard canonical values for type, this attribute defines the following canonical values to represent popular photo sizes: "photo" and "thumbnail".
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Photo {
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
