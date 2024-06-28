use crate::protocol::NoExtensions;
use crate::resource::enterprise_user::EnterpriseUser;
use crate::resource::user::User;
use crate::{generate_endpoint, generate_extension};

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

generate_extension!(
    extension EnterpriseUserExtensions {
        enterprise_user: EnterpriseUser,
    }
);
