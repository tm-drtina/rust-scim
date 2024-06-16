use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FilterConfig {
    /// A Boolean value specifying whether or not the operation is supported.
    /// REQUIRED.
    pub supported: bool,

    /// An integer value specifying the maximum number of resources returned in a response.
    /// REQUIRED.
    pub max_results: usize,
}
