// use color_eyre::Result;
// use crossterm::event::{self, Event};
// use ratatui::{DefaultTerminal, Frame};
use std::env;
use cdp::DomainClients;
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

#[tokio::main]
async fn main() {
    let mut args = env::args();
    // discard process name
    args.next().unwrap();
    let url = args.next().unwrap();

    // let mut browser = Process::browser(0);
    // loop {
    // thread::sleep(time::Duration::from_millis(2000));
    // browser.get_ws();
    // }
    // "ws://127.0.0.1:54348/devtools/browser/cb3a7675-8abb-4e5f-956c-28e47fdcc650"

    let port = args.next().unwrap();
    let websocket_url = cdp::websocket_url_from(format!("http://localhost:{}/json/new", port)).await.unwrap();
    println!("{:?}", websocket_url);

    let (write, read) = cdp::connect_to_websocket(websocket_url).await;
    let mut client = cdp::TungsteniteClient::new(write, read).await;
    let response = client.target().create_target(url, None, None, None, None, None).await;
    let target_id = response.unwrap().target_id;
    println!("{:?}", client.target().attach_to_target(
        target_id, Some(true)
    ).await);

    // println!("{:?}", client.page().enable().await);
    println!("{:?}", client.network().enable(Some(65535)).await);
    println!("{:?}", client.network().set_cache_disabled(true).await);


    client.target().receive_event().await;
    // client.print_buffer();
    loop {
        println!("{:?}", client.target().receive_event().await);
    }

//     color_eyre::install().unwrap();
//     let terminal = ratatui::init();
//     let result = run(terminal);
//     ratatui::restore();
}

// fn run(mut terminal: DefaultTerminal) -> Result<()> {
//     loop {
//         terminal.draw(render)?;
//         if matches!(event::read()?, Event::Key(_)) {
//             break Ok(());
//         }
//     }
// }
// 
// fn render(frame: &mut Frame) {
//     frame.render_widget("hello world", frame.area());
// }
