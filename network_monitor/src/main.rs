// use std::thread;
// use std::time::Duration;
// use std::time;
// use std::io::Read;

// #[cfg(target_os = "macos")]
// static CHROME_PATH: &str = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
// #[cfg(not(target_os = "macos"))]
// static CHROME_PATH: &str = "/usr/bin/chromium-browser";
//
// struct Process(std::io::Result<std::process::Child>);
// use std::process::{Command, Stdio};
//
// impl Process {
//     fn browser(port: u16) -> Self {
//         Self(
//             std::process::Command::new(CHROME_PATH)
//                 .arg(format!("--remote-debugging-port={}", port))
//                 .arg("--unsafely-treat-insecure-origin-as-secure")
//                 .stderr(Stdio::piped())
//                 .stdout(Stdio::piped())
//                 .spawn()
//         )
//     }
//
//     fn get_ws(&mut self) {
//         let mut buffer = Vec::new();
//         let br = self.0.as_mut().unwrap().stderr
//             .as_mut().unwrap().read(&mut buffer).unwrap();
//         let st = String::from_utf8(buffer).unwrap();
//         println!("THIS IS THE STRING {} {}", br, &st);
//
//         let mut buffer = Vec::new();
//         let br = self.0.as_mut().unwrap().stdout
//             .as_mut().unwrap().read(&mut buffer).unwrap();
//         let st = String::from_utf8(buffer).unwrap();
//         println!("THIS IS THE STRING {} {}", br, &st);
//         // st.lines().find(|l| l.starts_with("DevTools listening on ")).unwrap().strip_prefix("DevTools listening on ").unwrap().to_owned()
//     }
// }

mod action;
mod app;
mod cli;
mod components;
mod config;
mod errors;
mod logging;
mod tui;

use app::App;
use cdp::DomainClients;
use std::env;

#[tokio::main]
async fn main() {
    let mut args = env::args();
    // discard process name
    args.next().unwrap();
    let url = args.next().unwrap();

    let port = args.next().unwrap();
    let websocket_url = cdp::websocket_url_from(format!("http://localhost:{}/json/new", port))
        .await
        .unwrap();

    let (write, read) = cdp::connect_to_websocket(websocket_url).await;
    let mut client = cdp::TungsteniteClient::new(write, read).await;

    let response = client
        .target()
        .create_target(url, None, None, None, None, None)
        .await;
    let response = client
        .target()
        .attach_to_target(response.unwrap().target_id, Some(true))
        .await
        .unwrap();
    client.set_session_id(response.session_id);
    client.page().enable().await.unwrap();
    client.network().enable(Some(65535)).await.unwrap();
    client.network().set_cache_disabled(true).await.unwrap();

    crate::errors::init().unwrap();
    crate::logging::init().unwrap();

    // loop {
    //     println!("{:?}", client.network().receive_event().await);
    // }

    let mut app = App::new(client).unwrap();
    app.run().await.unwrap();
}
