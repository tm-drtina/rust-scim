use serde::{Deserialize, Serialize};

/// A list of groups to which the user belongs, either through direct membership, through nested groups, or dynamically calculated.
/// The values are meant to enable expression of common group-based or role-based access control models, although no explicit authorization model is defined.
/// It is intended that the semantics of group membership and any behavior or authorization granted as a result of membership are defined by the service provider.
/// The canonical types "direct" and "indirect" are defined to describe how the group membership was derived.
/// Direct group membership indicates that the user is directly associated with the group and SHOULD indicate that clients may modify membership through the "Group" resource.
/// Indirect membership indicates that user membership is transitive or dynamic and implies that clients cannot modify indirect group membership through the "Group" resource but MAY modify direct group membership through the "Group" resource, which may influence indirect memberships.
/// If the SCIM service provider exposes a "Group" resource, the "value" sub-attribute MUST be the "id", and the "$ref" sub-attribute must be the URI of the corresponding "Group" resources to which the user belongs.
/// Since this attribute has a mutability of "readOnly", group membership changes MUST be applied via the "Group" Resource (Section 4.2).
/// This attribute has a mutability of "readOnly".
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GroupMembership {
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
    /// For example, "https://example.com/Users/12345" is equivalent to "https://example.com/v2/Users/12345".
    #[serde(rename = "$ref", default)]
    pub reference: Option<String>,
}
