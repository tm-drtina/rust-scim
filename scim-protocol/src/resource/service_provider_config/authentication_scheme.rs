#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AuthenticationScheme {
    /// The authentication scheme.
    /// This specification defines the values "oauth", "oauth2", "oauthbearertoken", "httpbasic", and "httpdigest".
    /// REQUIRED.
    #[serde(rename = "type")]
    pub authentication_scheme_type: AuthenticationSchemeType,

    /// The common authentication scheme name, e.g., HTTP Basic.
    /// REQUIRED.
    pub name: String,

    /// A description of the authentication scheme.
    /// REQUIRED.
    pub description: String,

    /// An HTTP-addressable URL pointing to the authentication scheme's specification.
    /// OPTIONAL.
    pub spec_uri: Option<String>, // TODO: change to URI/URL

    /// An HTTP-addressable URL pointing to the authentication scheme's usage documentation.
    /// OPTIONAL.
    pub documentation_uri: Option<String>,

    /// A Boolean value indicating the 'primary' or preferred attribute value for this attribute, e.g., the preferred mailing address or the primary email address.
    /// The primary attribute value "true" MUST appear no more than once.
    /// If not specified, the value of "primary" SHALL be assumed to be "false".
    #[serde(default)]
    pub primary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthenticationSchemeType {
    OAuth,
    OAuth2,
    OAuthBearerToken,
    HttpBasic,
    HttpDigest,
}
