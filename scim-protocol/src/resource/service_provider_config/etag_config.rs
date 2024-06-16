use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ETagConfig {
    /// A Boolean value specifying whether or not the operation is supported.
    /// REQUIRED.
    pub supported: bool,
}
