pub mod enterprise_user;
pub mod group;
pub mod resource_type;
pub mod schema;
pub mod service_provider_config;
pub mod user;

pub trait ScimSchema {
    const SCHEMA: &'static str;
}
