use crate::protocol::{NoExtensions, ScimEndpoint, UserRequest, UserResponse};
use crate::resource::user::User;

// TODO: derive request and response here

pub struct UsersEndpoint;
impl ScimEndpoint for UsersEndpoint {
    const ENDPOINT: &'static str = "/Users";

    type Resource = User;
    type Extensions = NoExtensions;

    type Request = UserRequest;
    type Response = UserResponse;
}

