use scim_protocol::resource::enterprise_user::EnterpriseUser;
use scim_protocol::resource::user::User;
use scim_protocol::resource::ScimSchema;
use scim_protocol::{generate_endpoint, generate_extension};

generate_endpoint!(
    path = "/Users",
    endpoint_type = UsersEndpoint,
    request = UserRequest,
    response = UserResponse,
    resource = User,
    extensions = UserExtensions,
);

generate_extension!(
    extension UserExtensions {
        enterprise_user: EnterpriseUser,
        lucky_number: LuckyNumber,
    }
);

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LuckyNumber {
    pub lucky_number: i64,
}
impl ScimSchema for LuckyNumber {
    const SCHEMA: &'static str = "urn:mem:params:scim:schemas:extension:LuckyNumberExtension";
}
