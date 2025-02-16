use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
    Up,
    Down,
    AddNetworkRequest {
        r#type: Option<cdp::network::ResourceType>,
        request_id: String,
        url: String,
        initiator: cdp::network::Initiator,
    },
    UpdateNetworkRequestA {
        request_id: String,
        encoded_data_length: u64,
        data_length: u64,
    },
    UpdateNetworkRequestB {
        request_id: String,
        status: u64,
        mime_type: String,
        protocol: Option<String>,
    },
}
