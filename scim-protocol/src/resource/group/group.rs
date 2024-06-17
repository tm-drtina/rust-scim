use super::Member;

/// SCIM provides a schema for representing groups, identified using the following schema URI: "urn:ietf:params:scim:schemas:core:2.0:Group".
///
/// "Group" resources are meant to enable expression of common group-based or role-based access control models, although no explicit authorization model is defined.
/// It is intended that the semantics of group membership, and any behavior or authorization granted as a result of membership, are defined by the service provider; these are considered out of scope for this specification.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    // The following singular attribute is defined in addition to the common attributes defined in the SCIM core schema:
    /// A human-readable name for the Group.
    /// REQUIRED.
    pub display_name: String,

    // The following multi-valued attribute is defined in addition to the common attributes defined in the SCIM core schema:
    /// A list of members of the Group.
    /// While values MAY be added or removed, sub-attributes of members are "immutable".
    /// The "value" sub-attribute contains the value of an "id" attribute of a SCIM resource, and the "$ref" sub-attribute must be the URI of a SCIM resource such as a "User", or a "Group".
    /// The intention of the "Group" type is to allow the service provider to support nested groups.
    /// Service providers MAY require clients to provide a non-empty value by setting the "required" attribute characteristic of a sub-attribute of the "members" attribute in the "Group" resource schema.
    #[serde(default)]
    pub members: Vec<Member>,
}
