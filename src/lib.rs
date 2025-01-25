#![allow(clippy::too_many_arguments)]
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use serde::de::DeserializeOwned;
use serde::ser::Serialize as Ser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{WebSocketStream, connect_async, tungstenite::protocol::Message};

async fn websocket_url() -> String {
    let client = reqwest::Client::new();
    // /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9210

    let resp: serde_json::value::Value = client
        .put("http://localhost:9210/json/new")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    if let serde_json::Value::String(s) = resp.get("webSocketDebuggerUrl").unwrap() {
        return s.to_owned();
    }
    panic!();
}

trait CommandTrait: Serialize {
    type Result: DeserializeOwned;
}

#[derive(Debug, PartialEq, Serialize)]
struct Command<T>
where
    T: Ser,
{
    pub id: usize,
    pub params: T,
    pub method: String,
}

pub struct Client {
    message_id: usize,
    write: SplitSink<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message,
    >,
    read: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    buffer: Arc<Mutex<Vec<String>>>,
}

impl Client {
    pub async fn new() -> Self {
        let (ws_stream, _) = connect_async(websocket_url().await)
            .await
            .expect("Failed to connect");
        let (write, read) = ws_stream.split();

        Self {
            write,
            read,
            message_id: 0,
            buffer: Arc::new(Mutex::new(vec![])),
        }
    }

    async fn send_command<T>(&mut self, method: &str, params: T) -> T::Result
    where
        T: CommandTrait,
    {
        let message_id = self.message_id;
        self.message_id += 1;
        let command = Command {
            id: message_id,
            params,
            method: method.to_owned(),
        };
        let message = serde_json::to_string(&command).unwrap();

        self.write.send(Message::text(message)).await.unwrap();
        let a = Arc::clone(&self.buffer);
        loop {
            let message: Message = self.read.by_ref().next().await.unwrap().unwrap();
            let text = message.to_text().unwrap();
            let v: Value = serde_json::from_str(text).unwrap();

            if v["id"] == message_id {
                let t = serde_json::to_string(&v["result"]).unwrap();
                return serde_json::from_str(&t).unwrap();
            }

            let mut b = a.lock().unwrap();
            b.push(text.to_owned());
            println!("{}", text);
        }
    }
}

mod generated;
pub use generated::*;
