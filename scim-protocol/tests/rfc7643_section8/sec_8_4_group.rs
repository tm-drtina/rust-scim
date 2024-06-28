use scim_protocol::endpoint::GroupResponse;
use scim_protocol::protocol::{Meta, NoExtensions};
use scim_protocol::resource::group::{Group, Member};

#[test]
fn test_response() {
    let actual: GroupResponse = serde_json::from_str(include_str!("./sec_8_4_group.json")).unwrap();
    assert_eq!(expected(), actual);
}

fn expected() -> GroupResponse {
    GroupResponse {
        schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:Group".to_string()],
        id: "e9e30dba-f08f-4109-8486-d5c6a331660a".to_string(),
        external_id: None,
        meta: Meta {
            resource_type: Some("Group".to_string()),
            created: Some("2010-01-23T04:56:22Z".to_string()),
            last_modified: Some("2011-05-13T04:42:34Z".to_string()),
            location: Some(
                "https://example.com/v2/Groups/e9e30dba-f08f-4109-8486-d5c6a331660a".to_string(),
            ),
            version: Some(r#"W/"3694e05e9dff592""#.to_string()),
        },
        resource: Group {
            display_name: "Tour Guides".to_string(),
            members: vec![
                Member {
                    value_type: None,
                    primary: false,
                    display: Some("Babs Jensen".to_string()),
                    value: "2819c223-7f76-453a-919d-413861904646".to_string(),
                    reference: Some(
                        "https://example.com/v2/Users/2819c223-7f76-453a-919d-413861904646"
                            .to_string(),
                    ),
                },
                Member {
                    value_type: None,
                    primary: false,
                    display: Some("Mandy Pepperidge".to_string()),
                    value: "902c246b-6245-4190-8e05-00816be7344a".to_string(),
                    reference: Some(
                        "https://example.com/v2/Users/902c246b-6245-4190-8e05-00816be7344a"
                            .to_string(),
                    ),
                },
            ],
        },
        extensions: NoExtensions {},
    }
}
