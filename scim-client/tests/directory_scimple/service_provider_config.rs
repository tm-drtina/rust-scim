use super::scim_client;

#[tokio::test(flavor = "current_thread")]
async fn sp_config_works() {
    let client = scim_client();

    let service_provider_config = client.get_service_provider_config().await.unwrap();
    eprintln!("{service_provider_config:?}");
}
