use super::scim_client;
use rust_scim::error::Result;
use rust_scim::resource::resource_type::ResourceTypeResource;

#[tokio::test(flavor = "current_thread")]
async fn resource_type_works() -> Result<()> {
    let client = scim_client();

    let resource_types = client.get_all::<ResourceTypeResource>().await?;
    eprintln!("{resource_types:?}");

    Ok(())
}
