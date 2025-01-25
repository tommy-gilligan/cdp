#[tokio::main]
async fn main() {
    let mut client = cdp::Client::new().await;
    let mut target = client.target();
    let r = target
        .create_target(
            "https://phoronix.com".to_owned(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;
    println!("{:?}", target.get_target_info(Some(r.target_id)).await);
    // client.command(cdp::target::SetDiscoverTargets { discover: true, filter: None }).await;
}
