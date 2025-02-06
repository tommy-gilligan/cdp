#[tokio::main]
async fn main() {
    let websocket_url = cdp::websocket_url_from("http://localhost:9210/json/new").await.unwrap();
    let (write, read) = cdp::connect_to_websocket(websocket_url).await;
    let mut client = cdp::Client::new(write, read).await;
    let mut target = client.target();
    target
        .create_target(
            "https://phoronix.com".to_owned(),
            None,
            None,
            None,
            None,
            None,
        )
        .await;
    // println!("{:?}", target.get_target_info(Some(r.target_id)).await);
    // client.command(cdp::target::SetDiscoverTargets { discover: true, filter: None }).await;
}
