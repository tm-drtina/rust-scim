mod resource_type;
mod schema_extension;

pub use resource_type::ResourceType;
pub use schema_extension::SchemaExtension;

use crate::resource::ScimSchema;

const _ENDPOINT: &'static str = "/ResourceTypes";

impl ScimSchema for ResourceType {
    const SCHEMA: &'static str = "urn:ietf:params:scim:schemas:core:2.0:ResourceType";
}