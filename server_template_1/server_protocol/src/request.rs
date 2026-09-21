use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Ping,
}

#[derive(Debug, Clone)]
pub enum UnbuiltRequest {
    Ping,
}

#[derive(Debug, Clone)]
pub enum UnbuiltResponse {
    Ping,
}
