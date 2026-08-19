use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message_id: usize,
    pub request: Request,
    pub content: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum Request {
    GetData,
    Insert,
}

pub fn transport_mode(msg: Message) -> Result<String, String> {
    serde_json::to_string(&msg).map_err(|_| "failed to serialize".to_string())
}

pub fn back_to_message(json: String) -> Result<Message, String> {
    serde_json::from_str(&json).map_err(|_| "failed to deserialize".to_string())
}
