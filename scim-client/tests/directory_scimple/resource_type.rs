use super::scim_client;

#[tokio::test(flavor = "current_thread")]
async fn resource_type_works() {
    let client = scim_client();

    let resource_types = client.get_resource_types().await.unwrap();
    eprintln!("{resource_types:?}");
}
