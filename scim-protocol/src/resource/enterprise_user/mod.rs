mod enterprise_user;
mod manager;

pub use enterprise_user::EnterpriseUser;
pub use manager::Manager;

use crate::resource::ScimSchema;

impl ScimSchema for EnterpriseUser {
    const SCHEMA: &'static str = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User";
}
