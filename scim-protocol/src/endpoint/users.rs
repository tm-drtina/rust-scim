use crate::generate_endpoint;
use crate::protocol::{Extensions, NoExtensions};
use crate::resource::enterprise_user::EnterpriseUser;
use crate::resource::user::User;
use crate::resource::ScimSchema;

generate_endpoint!(
    path = "/Users",
    endpoint_type = UsersEndpoint,
    request = UserRequest,
    response = UserResponse,
    resource = User,
    extensions = NoExtensions,
);

generate_endpoint!(
    path = "/Users",
    endpoint_type = EnterpriseUsersEndpoint,
    request = EnterpriseUserRequest,
    response = EnterpriseUserResponse,
    resource = User,
    extensions = EnterpriseUserExtensions,
);

// TODO: generate even this
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct EnterpriseUserExtensions {
    #[serde(rename = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User")]
    pub enterprise_user: EnterpriseUser,
}
impl Extensions for EnterpriseUserExtensions {
    const SCHEMA: &'static [&'static str] = &[EnterpriseUser::SCHEMA];
}
