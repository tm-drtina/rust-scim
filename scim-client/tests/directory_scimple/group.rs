use rust_scim::error::Result;
use rust_scim::resource::group::GroupResource;

use super::scim_client;

#[tokio::test(flavor = "current_thread")]
async fn group_works() -> Result<()> {
    let client = scim_client();

    let group = client.get_all::<GroupResource>().await?;
    eprintln!("{group:?}");

    Ok(())
}
