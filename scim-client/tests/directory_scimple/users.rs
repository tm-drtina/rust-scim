use super::scim_client;
use reqwest::StatusCode;
use scim_client::error::ScimError;
use scim_protocol::endpoint::UsersEndpoint;
use scim_protocol::protocol::{NoExtensions, ScimRequest, UserRequest};
use scim_protocol::resource::user::User;

#[cfg(test)]
use pretty_assertions::assert_eq;

#[tokio::test(flavor = "current_thread")]
async fn user_works() {
    let client = scim_client();

    let user = client.get_all::<UsersEndpoint>().await.unwrap();
    eprintln!("{user:?}");
}

#[tokio::test(flavor = "current_thread")]
async fn create_delete() {
    let client = scim_client();

    let user = User {
        user_name: "bob".to_string(),
        name: None,
        display_name: Some("Bob IV.".to_string()),
        nick_name: Some("bobo".to_string()),
        profile_url: None,
        title: None,
        user_type: Some("superadmin".to_string()),
        preferred_language: Some("en-us".to_string()),
        locale: None,
        timezone: None,
        active: Some(true),
        password: Some("secret".to_string()),
        emails: vec![],
        phone_numbers: vec![],
        ims: vec![],
        photos: vec![],
        addresses: vec![],
        groups: vec![],
        entitlements: vec![],
        roles: vec![],
        x509_certificates: Vec::new(),
    };

    let expected_user = {
        let mut u = user.clone();
        u.password = None;
        u
    };

    let req = UserRequest::from_parts(None, user, NoExtensions);

    let created_user = client.create::<UsersEndpoint>(&req).await.unwrap();
    assert_eq!(expected_user, created_user.resource);

    let fetched_user = client.get::<UsersEndpoint>(&created_user.id).await.unwrap();
    assert_eq!(expected_user, fetched_user.resource);

    client.delete::<UsersEndpoint>(&created_user).await.unwrap();

    match client.get::<UsersEndpoint>(&created_user.id).await {
        Ok(_) => panic!("Fetching deleted user should fail"),
        Err(ScimError::HttpError(e)) if e.status() == Some(StatusCode::NOT_FOUND) => {}
        Err(e) => panic!("Wrong error returned: {e}"),
    }
}
