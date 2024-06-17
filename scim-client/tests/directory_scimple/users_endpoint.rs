use scim_protocol::generate_endpoint;
use scim_protocol::protocol::Extensions;
use scim_protocol::resource::enterprise_user::EnterpriseUser;
use scim_protocol::resource::user::User;
use scim_protocol::resource::ScimSchema;

generate_endpoint!(
    path = "/Users",
    endpoint_type = UsersEndpoint,
    request = UserRequest,
    response = UserResponse,
    resource = User,
    extensions = UserExtensions,
);

// TODO: generate even this
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct UserExtensions {
    #[serde(rename = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User")]
    pub enterprise_user: EnterpriseUser,
    #[serde(rename = "urn:mem:params:scim:schemas:extension:LuckyNumberExtension")]
    pub lucky_number: LuckyNumber,
}
impl Extensions for UserExtensions {
    const SCHEMA: &'static [&'static str] = &[EnterpriseUser::SCHEMA, LuckyNumber::SCHEMA];
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LuckyNumber {
    pub lucky_number: i64,
}
impl ScimSchema for LuckyNumber {
    const SCHEMA: &'static str = "urn:mem:params:scim:schemas:extension:LuckyNumberExtension";
}
