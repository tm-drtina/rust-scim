use rust_scim::common::Meta;
use rust_scim::resource::resource_type::{ResourceType, ResourceTypeResource, SchemaExtension};

#[test]
fn test() {
    let actual: Vec<ResourceTypeResource> =
        serde_json::from_str(include_str!("./sec_8_6_resource_type.json")).unwrap();
    assert_eq!(expected(), actual);
}

fn expected() -> Vec<ResourceTypeResource> {
    vec![
        ResourceTypeResource {
            schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:ResourceType".to_string()],
            meta: Meta {
                resource_type: Some("ResourceType".to_string()),
                location: Some("https://example.com/v2/ResourceTypes/User".to_string()),
                created: None,
                last_modified: None,
                version: None,
            },
            resource_type: ResourceType {
                id: Some("User".to_string()),
                name: "User".to_string(),
                description: Some("User Account".to_string()),
                endpoint: "/Users".to_string(),
                schema: "urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
                schema_extensions: vec![SchemaExtension {
                    schema: "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User"
                        .to_string(),
                    required: true,
                }],
            },
        },
        ResourceTypeResource {
            schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:ResourceType".to_string()],
            meta: Meta {
                resource_type: Some("ResourceType".to_string()),
                location: Some("https://example.com/v2/ResourceTypes/Group".to_string()),
                created: None,
                last_modified: None,
                version: None,
            },
            resource_type: ResourceType {
                id: Some("Group".to_string()),
                name: "Group".to_string(),
                description: Some("Group".to_string()),
                endpoint: "/Groups".to_string(),
                schema: "urn:ietf:params:scim:schemas:core:2.0:Group".to_string(),
                schema_extensions: Vec::new(),
            },
        },
    ]
}
