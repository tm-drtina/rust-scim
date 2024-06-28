use crate::protocol::{Extensions, ScimRequest, ScimResponse};
use crate::resource::ScimSchema;

pub trait ScimEndpoint {
    const ENDPOINT: &'static str;
    const SCHEMA: &'static str = <Self::Resource as ScimSchema>::SCHEMA;
    const EXTENSIONS: &'static [&'static str] = <Self::Extensions as Extensions>::SCHEMA;

    type Resource: ScimSchema;
    type Extensions: Extensions;

    type Request: ScimRequest<Resource = Self::Resource, Extensions = Self::Extensions>;
    type Response: ScimResponse<Resource = Self::Resource, Extensions = Self::Extensions>;

    #[must_use]
    fn resource_uri(base_url: &str) -> String {
        format!("{}{}", base_url, Self::ENDPOINT)
    }

    #[must_use]
    fn single_resource_uri(base_url: &str, id: &str) -> String {
        format!("{}/{}", Self::resource_uri(base_url), id)
    }
}
