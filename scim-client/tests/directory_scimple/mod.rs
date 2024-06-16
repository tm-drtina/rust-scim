//mod enterprise_user;
//mod group;
//mod resource_type;
//mod schema;
//mod service_provider_config;
mod users;

use reqwest::Client;
use scim_client::{ScimClient, ScimConfig};

fn scim_client() -> ScimClient {
    let config = ScimConfig {
        base_url: "http://localhost:8080/v2".to_string(),
        validate_schema: false,
    };
    let http_client = Client::builder().http1_only().build().unwrap();

    ScimClient::new(config, http_client)
}
