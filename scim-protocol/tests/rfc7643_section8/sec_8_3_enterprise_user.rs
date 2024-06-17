use scim_protocol::endpoint::{EnterpriseUserExtensions, EnterpriseUserResponse};
use scim_protocol::protocol::Meta;
use scim_protocol::resource::enterprise_user::{EnterpriseUser, Manager};
use scim_protocol::resource::user::{
    Address, Email, GroupMembership, InstantMessaging, Name, PhoneNumber, Photo, User,
    X509Certificate,
};

#[test]
fn test_response() {
    let actual: EnterpriseUserResponse =
        serde_json::from_str(include_str!("./sec_8_3_enterprise_user.json")).unwrap();
    assert_eq!(expected(), actual);
}

fn expected() -> EnterpriseUserResponse {
    EnterpriseUserResponse {
        schemas: vec![
            "urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
            "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User".to_string(),
        ],
        id: "2819c223-7f76-453a-919d-413861904646".to_string(),
        external_id: Some("701984".to_string()),
        meta: Meta {
            resource_type: Some("User".to_string()),
            created: Some("2010-01-23T04:56:22Z".to_string()),
            last_modified: Some("2011-05-13T04:42:34Z".to_string()),
            location: Some(
                "https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646".to_string(),
            ),
            version: Some(r#"W/"3694e05e9dff591""#.to_string()),
        },
        resource: user(),
        extensions: EnterpriseUserExtensions {
            enterprise_user: enterprise_user(),
        },
    }
}

fn user() -> User {
    User {
        user_name: "bjensen@example.com".to_string(),
        name: Some(Name {
            formatted: Some("Ms. Barbara J Jensen, III".to_string()),
            family_name: Some("Jensen".to_string()),
            given_name: Some("Barbara".to_string()),
            middle_name: Some("Jane".to_string()),
            honorific_prefix: Some("Ms.".to_string()),
            honorific_suffix: Some("III".to_string()),
        }),
        display_name: Some("Babs Jensen".to_string()),
        nick_name: Some("Babs".to_string()),
        profile_url: Some("https://login.example.com/bjensen".to_string()),
        title: Some("Tour Guide".to_string()),
        user_type: Some("Employee".to_string()),
        preferred_language: Some("en-US".to_string()),
        locale: Some("en-US".to_string()),
        timezone: Some("America/Los_Angeles".to_string()),
        active: Some(true),
        password: None,
        emails: vec![
            Email {
                value_type: Some("work".to_string()),
                primary: true,
                display: None,
                value: "bjensen@example.com".to_string(),
                reference: None,
            },
            Email {
                value_type: Some("home".to_string()),
                primary: false,
                display: None,
                value: "babs@jensen.org".to_string(),
                reference: None,
            },
        ],
        phone_numbers: vec![
            PhoneNumber {
                value_type: Some("work".to_string()),
                primary: false,
                display: None,
                value: "555-555-5555".to_string(),
                reference: None,
            },
            PhoneNumber {
                value_type: Some("mobile".to_string()),
                primary: false,
                display: None,
                value: "555-555-4444".to_string(),
                reference: None,
            },
        ],
        ims: vec![InstantMessaging {
            value_type: Some("aim".to_string()),
            primary: false,
            display: None,
            value: "someaimhandle".to_string(),
            reference: None,
        }],
        photos: vec![
            Photo {
                value_type: Some("photo".to_string()),
                primary: false,
                display: None,
                value: "https://photos.example.com/profilephoto/72930000000Ccne/F".to_string(),
                reference: None,
            },
            Photo {
                value_type: Some("thumbnail".to_string()),
                primary: false,
                display: None,
                value: "https://photos.example.com/profilephoto/72930000000Ccne/T".to_string(),
                reference: None,
            },
        ],
        addresses: vec![
            Address {
                value_type: Some("work".to_string()),
                primary: true,
                display: None,
                value: None,
                reference: None,
                formatted: Some(
                    "100 Universal City Plaza\nHollywood, CA 91608 USA".to_string(),
                ),
                street_address: Some("100 Universal City Plaza".to_string()),
                locality: Some("Hollywood".to_string()),
                region: Some("CA".to_string()),
                postal_code: Some("91608".to_string()),
                country: Some("USA".to_string()),
            },
            Address {
                value_type: Some("home".to_string()),
                primary: false,
                display: None,
                value: None,
                reference: None,
                formatted: Some("456 Hollywood Blvd\nHollywood, CA 91608 USA".to_string()),
                street_address: Some("456 Hollywood Blvd".to_string()),
                locality: Some("Hollywood".to_string()),
                region: Some("CA".to_string()),
                postal_code: Some("91608".to_string()),
                country: Some("USA".to_string()),
            },
        ],
        groups: vec![
            GroupMembership { value_type: None, primary: false, display: Some("Tour Guides".to_string()), value: "e9e30dba-f08f-4109-8486-d5c6a331660a".to_string(), reference: Some("../Groups/e9e30dba-f08f-4109-8486-d5c6a331660a".to_string()) },
            GroupMembership { value_type: None, primary: false, display: Some("Employees".to_string()), value: "fc348aa8-3835-40eb-a20b-c726e15c55b5".to_string(), reference: Some("../Groups/fc348aa8-3835-40eb-a20b-c726e15c55b5".to_string()) },
            GroupMembership { value_type: None, primary: false, display: Some("US Employees".to_string()), value: "71ddacd2-a8e7-49b8-a5db-ae50d0a5bfd7".to_string(), reference: Some("../Groups/71ddacd2-a8e7-49b8-a5db-ae50d0a5bfd7".to_string()) },
        ],
        entitlements: vec![],
        roles: vec![],
        x509_certificates: vec![X509Certificate {
            value_type: None,
            primary: false,
            display: None,
            value: "MIIDQzCCAqygAwIBAgICEAAwDQYJKoZIhvcNAQEFBQAwTjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFDASBgNVBAoMC2V4YW1wbGUuY29tMRQwEgYDVQQDDAtleGFtcGxlLmNvbTAeFw0xMTEwMjIwNjI0MzFaFw0xMjEwMDQwNjI0MzFaMH8xCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRQwEgYDVQQKDAtleGFtcGxlLmNvbTEhMB8GA1UEAwwYTXMuIEJhcmJhcmEgSiBKZW5zZW4gSUlJMSIwIAYJKoZIhvcNAQkBFhNiamVuc2VuQGV4YW1wbGUuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA7Kr+Dcds/JQ5GwejJFcBIP682X3xpjis56AK02bc1FLgzdLI8auoR+cC9/Vrh5t66HkQIOdA4unHh0AaZ4xL5PhVbXIPMB5vAPKpzz5iPSi8xO8SL7I7SDhcBVJhqVqr3HgllEG6UClDdHO7nkLuwXq8HcISKkbT5WFTVfFZzidPl8HZ7DhXkZIRtJwBweq4bvm3hM1Os7UQH05ZS6cVDgweKNwdLLrT51ikSQG3DYrl+ft781UQRIqxgwqCfXEuDiinPh0kkvIi5jivVu1Z9QiwlYEdRbLJ4zJQBmDrSGTMYn4lRc2HgHO4DqB/bnMVorHB0CC6AV1QoFK4GPe1LwIDAQABo3sweTAJBgNVHRMEAjAAMCwGCWCGSAGG+EIBDQQfFh1PcGVuU1NMIEdlbmVyYXRlZCBDZXJ0aWZpY2F0ZTAdBgNVHQ4EFgQU8pD0U0vsZIsaA16lL8En8bx0F/gwHwYDVR0jBBgwFoAUdGeKitcaF7gnzsNwDx708kqaVt0wDQYJKoZIhvcNAQEFBQADgYEAA81SsFnOdYJtNg5Tcq+/ByEDrBgnusx0jloUhByPMEVkoMZ3J7j1ZgI8rAbOkNngX8+pKfTiDz1RC4+dx8oU6Za+4NJXUjlL5CvV6BEYb1+QAEJwitTVvxB/A67g42/vzgAtoRUeDov1+GFiBZ+GNF/cAYKcMtGcrs2i97ZkJMo=".to_string(),
            reference: None,
        }],
    }
}

fn enterprise_user() -> EnterpriseUser {
    EnterpriseUser {
        employee_number: Some("701984".to_string()),
        cost_center: Some("4130".to_string()),
        organization: Some("Universal Studios".to_string()),
        division: Some("Theme Park".to_string()),
        department: Some("Tour Operations".to_string()),
        manager: Some(Manager {
            value: Some("26118915-6090-4610-87e4-49d8ca9f808d".to_string()),
            reference: Some("../Users/26118915-6090-4610-87e4-49d8ca9f808d".to_string()),
            display_name: Some("John Smith".to_string()),
        }),
    }
}
