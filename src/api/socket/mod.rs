use std::{collections::HashMap};
use websocket::{r#async::{client::{ClientNew, TlsStream}, TcpStream}, futures::{future::FutureResult, Future, Stream}, header::Headers, ws::dataframe::DataFrame, ClientBuilder, WebSocketError};

pub struct BlacketWebsocket {
    pub listeners: HashMap<String, Vec<fn(data: serde_json::Value)>>,
}

impl BlacketWebsocket {
    pub async fn new() -> Self {
        BlacketWebsocket {
            listeners: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event: String, listener: fn(data: serde_json::Value)) {
        if self.listeners.contains_key(&event) {
            self.listeners.get_mut(&event).unwrap().push(listener);
        } else {
            self.listeners.insert(event, vec![listener]);
        }
    }

    pub fn pop(&mut self, event: String) {
        self.listeners.remove(&event);
    }

    pub async fn run(&mut self, token: String) {
        let mut headers = Headers::new();
        headers.append_raw("Cookie", ("token=".to_string() + &token).as_bytes().to_vec());
        let _ = ClientBuilder::new("wss://blacket.org/worker/socket")
            .unwrap()
            .custom_headers(&headers)
            .async_connect_secure(None)
            .and_then(|(c, _)| -> FutureResult<u8, WebSocketError> {
                return Result::Ok(0).into();
            });
    }
}