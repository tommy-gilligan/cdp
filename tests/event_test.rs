use cdp::DomainClients;
use std::{thread, time};

mod support;

#[tokio::test]
async fn event_test() {
    let _server = support::Process::server();
    let _browser = support::Process::browser(9210);

    thread::sleep(time::Duration::from_millis(2000));

    // can read stdout for ws://127.0.0.1:9210/devtools/browser/baab0c19-568b-4d95-aa8a-9fc07fb86ff2
    let websocket_url = cdp::websocket_url_from("http://localhost:9210/json/new")
        .await
        .unwrap();
    let (write, read) = cdp::connect_to_websocket(websocket_url).await;
    let mut client = cdp::TungsteniteClient::new(write, read).await;
    let mut target = client.target();
    let _response = target.set_discover_targets(true, None).await;

    let mut target_created_a = false;
    let mut target_created_c = false;

    for _i in 0..3 {
        if let cdp::target::Event::TargetCreated(cdp::target::TargetCreated { mut target_info }) =
            target.receive_event().await
        {
            target_info.target_id = "".to_owned();
            target_info.browser_context_id = None;

            if target_info
                == (cdp::target::TargetInfo {
                    target_id: "".to_owned(),
                    r#type: "page".to_owned(),
                    title: "about:blank".to_owned(),
                    url: "about:blank".to_owned(),
                    attached: true,
                    opener_id: None,
                    can_access_opener: false,
                    opener_frame_id: None,
                    browser_context_id: None,
                    subtype: None,
                })
            {
                target_created_a = true;
            }
            // if target_info == (cdp::target::TargetInfo {
            //         target_id: "".to_owned(),
            //         r#type: "page".to_owned(),
            //         title: "New Tab".to_owned(),
            //         url: "chrome://newtab/".to_owned(),
            //         attached: false,
            //         opener_id: None,
            //         can_access_opener: false,
            //         opener_frame_id: None,
            //         browser_context_id: None,
            //         subtype: None
            //     }) {
            //     target_created_b = true;
            // }
            if target_info
                == (cdp::target::TargetInfo {
                    target_id: "".to_owned(),
                    r#type: "iframe".to_owned(),
                    title: "chrome-untrusted://new-tab-page/one-google-bar?paramsencoded="
                        .to_owned(),
                    url: "chrome-untrusted://new-tab-page/one-google-bar?paramsencoded=".to_owned(),
                    attached: false,
                    opener_id: None,
                    can_access_opener: false,
                    opener_frame_id: None,
                    browser_context_id: None,
                    subtype: None,
                })
            {
                target_created_c = true;
            }
        } else {
            panic!("");
        }
    }

    assert!(target_created_a);
    // assert!(target_created_b);
    assert!(target_created_c);
}
