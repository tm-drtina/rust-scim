#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BulkConfig {
    /// A Boolean value specifying whether or not the operation is supported.
    /// REQUIRED.
    pub supported: bool,

    /// An integer value specifying the maximum number of operations.
    /// REQUIRED.
    pub max_operations: usize,

    /// An integer value specifying the maximum payload size in bytes.
    /// REQUIRED.
    pub max_payload_size: usize,
}
