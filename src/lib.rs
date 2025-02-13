#![feature(cfg_version)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::large_enum_variant)]
#![feature(anonymous_pipe)]
use futures_util::{
    StreamExt,
    stream::{SplitSink, SplitStream},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned, ser::Serialize as Ser};
use tokio_tungstenite::{WebSocketStream, connect_async, tungstenite::protocol::Message};

mod pipe_client;
pub use pipe_client::*;
mod tungstenite_client;
pub use tungstenite_client::*;

mod generated;
pub use generated::*;

type CDPError = (isize, String);

#[cfg(feature = "reqwest")]
pub async fn websocket_url_from<U>(url: U) -> anyhow::Result<String>
where
    U: reqwest::IntoUrl,
{
    let client = reqwest::Client::new();
    let resp: serde_json::value::Value = client.put(url).send().await?.json().await?;

    // println!("{:?}", resp);
    let ws_url = resp.get("webSocketDebuggerUrl").unwrap().as_str().unwrap();
    Ok(ws_url.to_owned())
}

pub trait CommandTrait: Serialize {
    type Result: DeserializeOwned;
}

#[derive(Debug, PartialEq, Serialize)]
struct Command<T>
where
    T: Ser,
{
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub id: usize,
    pub params: T,
    pub method: String,
}

pub async fn connect_to_websocket<R>(
    request: R,
) -> (
    SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>,
    SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
)
where
    R: tungstenite::client::IntoClientRequest + Unpin,
{
    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
    ws_stream.split()
}

pub trait Client {
    fn send_command<T>(
        &mut self,
        method: &str,
        params: T,
    ) -> impl std::future::Future<Output = Result<T::Result, CDPError>> + Send
    where
        T: CommandTrait + Send;
    fn receive_event<T>(&mut self) -> impl std::future::Future<Output = T> + Send
    where
        T: DeserializeOwned;
}

#[test]
fn test_execution_context_created_event() {
    let json = "{\"method\":\"Runtime.executionContextCreated\",\"params\":{\"context\":{\"id\":1,\"origin\":\"://\",\"name\":\"\",\"uniqueId\":\"4722846047508269505.6700994648490791134\",\"auxData\":{\"isDefault\":true,\"type\":\"default\",\"frameId\":\"7A59BCC0C9D4887A16394E736EFF437D\"}}}}";
    serde_json::from_str::<crate::runtime::Event>(json).unwrap();
}
