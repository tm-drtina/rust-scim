use scim_protocol::protocol::{Meta, NoExtensions, UserResponse};
use scim_protocol::resource::user::User;

#[test]
fn test_response() {
    let actual: UserResponse =
        serde_json::from_str(include_str!("./sec_8_1_minimal_user.json")).unwrap();
    let expected = UserResponse {
        schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".to_string()],
        id: "2819c223-7f76-453a-919d-413861904646".to_string(),
        external_id: None,
        meta: meta(),
        resource: user(),
        extensions: NoExtensions,
    };
    assert_eq!(expected, actual);
}

fn meta() -> Meta {
    Meta {
        resource_type: Some("User".to_string()),
        created: Some("2010-01-23T04:56:22Z".to_string()),
        last_modified: Some("2011-05-13T04:42:34Z".to_string()),
        location: Some(
            "https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646".to_string(),
        ),
        version: Some(r#"W/"3694e05e9dff590""#.to_string()),
    }
}

fn user() -> User {
    User {
        user_name: "bjensen@example.com".to_string(),
        name: None,
        display_name: None,
        nick_name: None,
        profile_url: None,
        title: None,
        user_type: None,
        preferred_language: None,
        locale: None,
        timezone: None,
        active: None,
        password: None,
        emails: Vec::new(),
        phone_numbers: Vec::new(),
        ims: Vec::new(),
        photos: Vec::new(),
        addresses: Vec::new(),
        groups: Vec::new(),
        entitlements: Vec::new(),
        roles: Vec::new(),
        x509_certificates: Vec::new(),
    }
}
