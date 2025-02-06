#![allow(clippy::too_many_arguments)]
#![allow(clippy::large_enum_variant)]
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

type CDPError = (isize, String);

#[cfg(feature = "reqwest")]
pub async fn websocket_url_from<U>(url: U) -> anyhow::Result<String> where U: reqwest::IntoUrl {
    let client = reqwest::Client::new();
    let resp: serde_json::value::Value = client
        .put(url)
        .send()
        .await?
        .json()
        .await?;

    println!("{:?}", resp);
    let ws_url = resp.get("webSocketDebuggerUrl").unwrap().as_str().unwrap();
    Ok(ws_url.to_owned())
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

pub async fn connect_to_websocket<R>(request: R) -> (
    SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>,
    SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>
) where R: tungstenite::client::IntoClientRequest + Unpin {
    let (ws_stream, _) = connect_async(request)
        .await
        .expect("Failed to connect");
    ws_stream.split()
}

pub struct Client {
    message_id: usize,
    write: SplitSink<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message,
    >,
    read: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    pub buffer: Arc<Mutex<std::collections::VecDeque<String>>>,
}

impl Client {
    pub async fn new(
        write: SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>,
        read: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>
    ) -> Self {
        Self {
            write,
            read,
            message_id: 0,
            buffer: Arc::new(Mutex::new(std::collections::VecDeque::new())),
        }
    }

    async fn send_command<T>(&mut self, method: &str, params: T) -> Result<T::Result, CDPError>
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
        self.write.send(Message::text(&message)).await.unwrap();
        println!("SENDING MESSAGE {}", message);
        let a = Arc::clone(&self.buffer);
        loop {
            let message: Message = self.read.by_ref().next().await.unwrap().unwrap();
            let text = message.to_text().unwrap();
            let v: Value = serde_json::from_str(text).unwrap();

            println!("RECEIVED MESSAGE {:?}", text);
            if v["id"] == message_id {

                if v["error"].is_object() {
                    return Err((v["error"]["code"].as_i64().unwrap() as isize, v["error"]["message"].as_str().unwrap().to_owned()));
                }

                let t = serde_json::to_string(&v["result"]).unwrap();
                let a = serde_json::from_str(&t).unwrap();
                return Ok(a);
            }

            let mut b = a.lock().unwrap();
            b.push_back(text.to_owned());
        }
    }

    pub async fn receive_event<T>(&mut self) -> T where T: DeserializeOwned {
        {
            let a = Arc::clone(&self.buffer);
            let mut b = a.lock().unwrap();
            if let Some((i, _f)) = b.iter().enumerate().find(|(_i, f)| serde_json::from_str::<T>(f).is_ok()) {
                return serde_json::from_str::<T>(&b.remove(i).unwrap()).unwrap()
            }
        }

        loop {
            let message: Message = self.read.by_ref().next().await.unwrap().unwrap();
            let t = message.to_text().unwrap();
            println!("RECEIVED MESSAGE {:?}", &t);
            if let Ok(d) = serde_json::from_str::<T>(t) {
                return d;
            } else {
                let a = Arc::clone(&self.buffer);
                let mut b = a.lock().unwrap();
                b.push_back(t.to_owned());
            }
        }
    }

    pub fn print_buffer(&mut self) {
            let a = Arc::clone(&self.buffer);
            let b = a.lock().unwrap();
            for v in &*b {
                println!("{:?}", v);
            }
    }
}

mod generated;
pub use generated::*;

#[test]
fn test_execution_context_created_event() {
    let json = "{\"method\":\"Runtime.executionContextCreated\",\"params\":{\"context\":{\"id\":1,\"origin\":\"://\",\"name\":\"\",\"uniqueId\":\"4722846047508269505.6700994648490791134\",\"auxData\":{\"isDefault\":true,\"type\":\"default\",\"frameId\":\"7A59BCC0C9D4887A16394E736EFF437D\"}}}}";
    serde_json::from_str::<crate::runtime::Event>(json).unwrap();
}
