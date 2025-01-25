use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::SinkExt;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    // /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9210
    let resp: serde_json::value::Value = client.put("http://localhost:9210/json/new")
        .send().await.unwrap()
        .json().await.unwrap();
    let url = resp.get("webSocketDebuggerUrl").unwrap();
    if let serde_json::Value::String(s) = url {
        let (ws_stream, _) = connect_async(s).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (mut write, read) = ws_stream.split();
        write.send(
            Message::text(
                "{\"id\": 1, \"method\": \"Target.setDiscoverTargets\", \"params\": { \"discover\": true } }"
            )
        ).await.unwrap();
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };
        pin_mut!(ws_to_stdout);
        ws_to_stdout.await;
    }
}

