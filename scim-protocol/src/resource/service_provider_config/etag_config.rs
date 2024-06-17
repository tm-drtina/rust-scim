#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ETagConfig {
    /// A Boolean value specifying whether or not the operation is supported.
    /// REQUIRED.
    pub supported: bool,
}
