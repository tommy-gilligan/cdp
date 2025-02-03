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

pub async fn websocket_url_from<U>(url: U) -> anyhow::Result<String> where U: reqwest::IntoUrl {
    let client = reqwest::Client::new();
    let resp: serde_json::value::Value = client
        .put(url)
        .send()
        .await?
        .json()
        .await?;

    let ws_url = resp.get("webSocketDebuggerUrl").unwrap().as_str().unwrap();
    return Ok(ws_url.to_owned());
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
    pub async fn new<R>(request: R) -> Self where R: tungstenite::client::IntoClientRequest + Unpin {
        let (ws_stream, _) = connect_async(request)
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
