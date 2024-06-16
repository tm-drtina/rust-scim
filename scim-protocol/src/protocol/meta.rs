use serde::Deserialize;

type DateTime = String;

/// A complex attribute containing resource metadata.
/// All "meta" sub-attributes are assigned by the service provider (have a "mutability" of "readOnly"), and all of these sub-attributes have a "returned" characteristic of "default".
/// This attribute SHALL be ignored when provided by clients.
#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Meta {
    /// The name of the resource type of the resource.
    /// This attribute has a mutability of "readOnly" and "caseExact" as "true".
    pub resource_type: Option<String>,

    /// The "DateTime" that the resource was added to the service provider.
    /// This attribute MUST be a DateTime.
    pub created: Option<DateTime>,

    /// The most recent DateTime that the details of this resource were updated at the service provider.
    /// If this resource has never been modified since its initial creation, the value MUST be the same as the value of "created".
    pub last_modified: Option<DateTime>,

    /// The URI of the resource being returned.
    /// This value MUST be the same as the "Content-Location" HTTP response header (see Section 3.1.4.2 of [RFC7231]).
    pub location: Option<String>,

    /// The version of the resource being returned.
    /// This value must be the same as the entity-tag (ETag) HTTP response header (see Sections 2.1 and 2.3 of [RFC7232]).
    /// This attribute has "caseExact" as "true".
    /// Service provider support for this attribute is optional and subject to the service provider's support for versioning (see Section 3.14 of [RFC7644]).
    /// If a service provider provides "version" (entity-tag) for a representation and the generation of that entity-tag does not satisfy all of the characteristics of a strong validator (see Section 2.1 of [RFC7232]), then the origin server MUST mark the "version" (entity-tag) as weak by prefixing its opaque value with "W/" (case sensitive).
    pub version: Option<String>,
}
