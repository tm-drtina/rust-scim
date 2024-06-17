use super::scim_client;

#[tokio::test(flavor = "current_thread")]
async fn schema_works() {
    let client = scim_client();

    let schemas = client.get_schemas().await.unwrap();
    eprintln!("{schemas:?}");
}
