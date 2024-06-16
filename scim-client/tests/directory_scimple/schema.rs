use super::scim_client;
use rust_scim::error::Result;
use rust_scim::resource::schema::Schema;

#[tokio::test(flavor = "current_thread")]
async fn schema_works() -> Result<()> {
    let client = scim_client();

    let schemas = client.get_all::<Schema>().await?;
    eprintln!("{schemas:?}");

    Ok(())
}
