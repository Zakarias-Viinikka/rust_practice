use crate::request::Request;
use protocol::serialization::Base64Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub message_id: usize,
    pub request: Request,
    pub content: Base64Bytes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub message_id: usize,
    pub request: Request,
    pub data: Base64Bytes,
}
