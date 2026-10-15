use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Ping,
    GetEntireTable { table_name: String },
}

#[derive(Debug, Clone)]
pub enum UnbuiltRequest {
    Ping,
    GetEntireTable { table_name: String },
}

#[derive(Debug, Clone)]
pub enum UnbuiltResponse {
    Ping,
    GetEntireTable { table_name: String },
}
