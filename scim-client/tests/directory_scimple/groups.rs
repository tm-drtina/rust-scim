use scim_protocol::endpoint::GroupsEndpoint;

use super::scim_client;

#[tokio::test(flavor = "current_thread")]
async fn fetchall_works() {
    let client = scim_client();

    let groups = client.get_all::<GroupsEndpoint>().await.unwrap();
    eprintln!("{groups:?}");
}
