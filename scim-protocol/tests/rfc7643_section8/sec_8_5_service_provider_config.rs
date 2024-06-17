use scim_protocol::protocol::Meta;
use scim_protocol::resource::service_provider_config::{
    AuthenticationScheme, AuthenticationSchemeType, BulkConfig, ChangePasswordConfig, ETagConfig,
    FilterConfig, PatchConfig, ServiceProviderConfig, SortConfig,
};

#[test]
fn test() {
    let actual: ServiceProviderConfig =
        serde_json::from_str(include_str!("./sec_8_5_service_provider_config.json")).unwrap();
    assert_eq!(expected(), actual);
}

fn expected() -> ServiceProviderConfig {
    ServiceProviderConfig {
        schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig".to_string()],
        documentation_uri: Some("http://example.com/help/scim.html".to_string()),
        patch: PatchConfig { supported: true },
        bulk: BulkConfig {
            supported: true,
            max_operations: 1000,
            max_payload_size: 1048576,
        },
        filter: FilterConfig {
            supported: true,
            max_results: 200,
        },
        change_password: ChangePasswordConfig { supported: true },
        sort: SortConfig { supported: true },
        etag: ETagConfig { supported: true },
        authentication_schemes: vec![
            AuthenticationScheme {
                authentication_scheme_type: AuthenticationSchemeType::OAuthBearerToken,
                name: "OAuth Bearer Token".to_string(),
                description: "Authentication scheme using the OAuth Bearer Token Standard"
                    .to_string(),
                spec_uri: Some("http://www.rfc-editor.org/info/rfc6750".to_string()),
                documentation_uri: Some("http://example.com/help/oauth.html".to_string()),
                primary: true,
            },
            AuthenticationScheme {
                authentication_scheme_type: AuthenticationSchemeType::HttpBasic,
                name: "HTTP Basic".to_string(),
                description: "Authentication scheme using the HTTP Basic Standard".to_string(),
                spec_uri: Some("http://www.rfc-editor.org/info/rfc2617".to_string()),
                documentation_uri: Some("http://example.com/help/httpBasic.html".to_string()),
                primary: false,
            },
        ],
        meta: Meta {
            resource_type: Some("ServiceProviderConfig".to_string()),
            created: Some("2010-01-23T04:56:22Z".to_string()),
            last_modified: Some("2011-05-13T04:42:34Z".to_string()),
            location: Some("https://example.com/v2/ServiceProviderConfig".to_string()),
            version: Some(r#"W/"3694e05e9dff594""#.to_string()),
        },
    }
}
