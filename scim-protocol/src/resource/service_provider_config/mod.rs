mod authentication_scheme;
mod bulk_config;
mod change_password_config;
mod etag_config;
mod filter_config;
mod patch_config;
mod service_provider_config;
mod sort_config;

pub use authentication_scheme::{AuthenticationScheme, AuthenticationSchemeType};
pub use bulk_config::BulkConfig;
pub use change_password_config::ChangePasswordConfig;
pub use etag_config::ETagConfig;
pub use filter_config::FilterConfig;
pub use patch_config::PatchConfig;
pub use service_provider_config::ServiceProviderConfig;
pub use sort_config::SortConfig;

pub const ENDPOINT: &str = "/ServiceProviderConfig";
