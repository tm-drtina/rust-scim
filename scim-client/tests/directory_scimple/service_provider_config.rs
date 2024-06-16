use super::scim_client;
use rust_scim::error::Result;
use rust_scim::resource::service_provider_config::ServiceProviderConfig;

#[tokio::test(flavor = "current_thread")]
async fn sp_config_works() -> Result<()> {
    let client = scim_client();

    let service_provider_config = client.get_single::<ServiceProviderConfig>().await?;
    eprintln!("{service_provider_config:?}");

    Ok(())
}
