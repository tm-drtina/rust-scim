use crate::directory_scimple::users_endpoint::{LuckyNumber, UserExtensions};

use super::users_endpoint::{UserRequest, UsersEndpoint};

use super::scim_client;
use reqwest::StatusCode;
use scim_client::error::ScimError;
use scim_protocol::protocol::ScimRequest;
use scim_protocol::resource::enterprise_user::{EnterpriseUser, Manager};
use scim_protocol::resource::user::User;

#[cfg(test)]
use pretty_assertions::assert_eq;

#[tokio::test(flavor = "current_thread")]
async fn fetchall_works() {
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

    let extensions = UserExtensions {
        enterprise_user: EnterpriseUser {
            employee_number: Some("abc123".to_string()),
            cost_center: None,
            organization: Some("ACME org".to_string()),
            division: Some("IT".to_string()),
            department: Some("IT Support".to_string()),
            manager: Some(Manager {
                value: Some("Big Boss".to_string()),
                reference: None,
                display_name: None,
            }),
        },
        lucky_number: LuckyNumber { lucky_number: 42 },
    };
    let expected_extensions = extensions.clone();

    let req = UserRequest::from_parts(None, user, extensions);

    let created_user = client.create::<UsersEndpoint>(&req).await.unwrap();
    assert_eq!(expected_user, created_user.resource);
    assert_eq!(expected_extensions, created_user.extensions);

    let fetched_user = client.get::<UsersEndpoint>(&created_user.id).await.unwrap();
    assert_eq!(expected_user, fetched_user.resource);
    assert_eq!(expected_extensions, fetched_user.extensions);

    client.delete::<UsersEndpoint>(&created_user).await.unwrap();

    match client.get::<UsersEndpoint>(&created_user.id).await {
        Ok(_) => panic!("Fetching deleted user should fail"),
        Err(ScimError::HttpError(e)) if e.status() == Some(StatusCode::NOT_FOUND) => {}
        Err(e) => panic!("Wrong error returned: {e}"),
    }
}
