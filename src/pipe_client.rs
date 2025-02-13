use serde::de::DeserializeOwned;
use serde_json::Value;
#[cfg(not(version("1.86")))]
use std::pipe::{PipeReader, PipeWriter};
use std::{
    io::{BufRead, BufReader, Write},
    sync::{Arc, Mutex},
};

use crate::{CDPError, Client, Command, CommandTrait, DomainClients};
#[cfg(version("1.86"))]
use std::io::{PipeReader, PipeWriter};

pub struct PipeClient {
    message_id: usize,
    write: PipeWriter,
    read: BufReader<PipeReader>,
    pub buffer: Arc<Mutex<std::collections::VecDeque<String>>>,
    session_id: Option<String>,
}

impl PipeClient {
    pub fn new(write: PipeWriter, read: PipeReader) -> Self {
        Self {
            write,
            read: BufReader::new(read),
            message_id: 0,
            buffer: Arc::new(Mutex::new(std::collections::VecDeque::new())),
            session_id: None,
        }
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = Some(session_id);
    }
}

impl DomainClients for PipeClient {}

impl Client for PipeClient {
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
        let message = format!("{}\0", serde_json::to_string(&command).unwrap());
        // should be async but isn't
        self.write.write_all(message.as_bytes()).unwrap();
        // println!("SENDING MESSAGE {}", message);
        let a = Arc::clone(&self.buffer);
        loop {
            let mut buffer = Vec::new();
            self.read.read_until(0, &mut buffer).unwrap();
            let t = String::from_utf8(buffer)
                .unwrap()
                .strip_suffix("\0")
                .unwrap()
                .to_owned();
            // println!("RECEIVED {:?}", &t);
            let v: Value = serde_json::from_str(&t).unwrap();

            // println!("RECEIVED MESSAGE {:?}", &t);
            if v["id"] == message_id {
                if v["error"].is_object() {
                    return Err((
                        v["error"]["code"].as_i64().unwrap() as isize,
                        v["error"]["message"].as_str().unwrap().to_owned(),
                    ));
                }

                let t = serde_json::to_string(&v["result"]).unwrap();
                // println!("{:?}", &t);
                let a = serde_json::from_str(&t).unwrap();
                return Ok(a);
            }

            let mut b = a.lock().unwrap();
            b.push_back(t.to_owned());
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
            let mut buffer = Vec::new();

            // let res = tokio::task::spawn_blocking(move || {
            self.read.read_until(0, &mut buffer).unwrap();
            // }).await.unwrap();

            let t = String::from_utf8(buffer).unwrap();
            // println!("RECEIVED MESSAGE {:?}", &t);
            if let Ok(d) = serde_json::from_str::<T>(&t) {
                return d;
            } else {
                let a = Arc::clone(&self.buffer);
                let mut b = a.lock().unwrap();
                b.push_back(t.to_owned());
            }
        }
    }
}
