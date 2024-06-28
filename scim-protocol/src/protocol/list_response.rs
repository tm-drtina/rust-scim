use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ListResponse<T> {
    /// Must be singleton list of `["urn:ietf:params:scim:api:messages:2.0:ListResponse"]`
    // TODO: verify in requests
    #[allow(unused)]
    schemas: Vec<String>,

    /// The total number of results returned by the list or query operation. The value may be larger than the number of resources returned, such as when returning a single page (see Section 3.4.2.4) of results where multiple pages are available. REQUIRED.
    pub total_results: usize,

    /// A multi-valued list of complex objects containing the requested resources.
    /// This MAY be a subset of the full set of resources if pagination (Section 3.4.2.4) is requested.
    /// REQUIRED if "totalResults" is non-zero.
    #[serde(default = "Vec::new", rename = "Resources")]
    pub resources: Vec<T>,

    /// The 1-based index of the first result in the current set of list results.
    /// REQUIRED when partial results are returned due to pagination.
    pub start_index: usize,

    /// The number of resources returned in a list response page.
    /// REQUIRED when partial results are returned due to pagination.    
    pub items_per_page: usize,
}
