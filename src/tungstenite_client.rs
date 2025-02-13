use crate::{CDPError, Client, Command, CommandTrait, DomainClients};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{WebSocketStream, tungstenite::protocol::Message};

pub struct TungsteniteClient {
    message_id: usize,
    write: SplitSink<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message,
    >,
    read: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    pub buffer: Arc<Mutex<std::collections::VecDeque<String>>>,
    pub session_id: Option<String>,
}

impl TungsteniteClient {
    pub async fn new(
        write: SplitSink<
            WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
            Message,
        >,
        read: SplitStream<
            WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        >,
    ) -> Self {
        Self {
            write,
            read,
            message_id: 0,
            buffer: Arc::new(Mutex::new(std::collections::VecDeque::new())),
            session_id: None,
        }
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = Some(session_id);
    }
}

impl DomainClients for TungsteniteClient {}

impl Client for TungsteniteClient {
    async fn send_command<T>(&mut self, method: &str, params: T) -> Result<T::Result, CDPError>
    where
        T: CommandTrait + Send,
    {
        let message_id = self.message_id;
        self.message_id += 1;
        let command = Command {
            session_id: self.session_id.clone(),
            id: message_id,
            params,
            method: method.to_owned(),
        };

        let message = serde_json::to_string(&command).unwrap();
        self.write.send(Message::text(&message)).await.unwrap();

        // println!("SENDING MESSAGE {}", message);
        let a = Arc::clone(&self.buffer);
        loop {
            let message: Message = self.read.by_ref().next().await.unwrap().unwrap();
            let text = message.to_text().unwrap();
            let v: Value = serde_json::from_str(text).unwrap();

            // println!("RECEIVED MESSAGE {:?}", text);
            if v["id"] == message_id {
                if v["error"].is_object() {
                    return Err((
                        v["error"]["code"].as_i64().unwrap() as isize,
                        v["error"]["message"].as_str().unwrap().to_owned(),
                    ));
                }

                let t = serde_json::to_string(&v["result"]).unwrap();
                let a = serde_json::from_str(&t).unwrap();
                return Ok(a);
            }

            let mut b = a.lock().unwrap();
            b.push_back(text.to_owned());
        }
    }

    async fn receive_event<T>(&mut self) -> T
    where
        T: DeserializeOwned,
    {
        {
            let a = Arc::clone(&self.buffer);
            let mut b = a.lock().unwrap();
            if let Some((i, _f)) = b
                .iter()
                .enumerate()
                .find(|(_i, f)| serde_json::from_str::<T>(f).is_ok())
            {
                return serde_json::from_str::<T>(&b.remove(i).unwrap()).unwrap();
            }
        }

        loop {
            let message: Message = self.read.by_ref().next().await.unwrap().unwrap();
            let t = message.to_text().unwrap();
            if let Ok(d) = serde_json::from_str::<T>(t) {
                return d;
            } else {
                let a = Arc::clone(&self.buffer);
                let mut b = a.lock().unwrap();
                b.push_back(t.to_owned());
            }
        }
    }
}
