use protocol::row_col::Row;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetEntireTableOut {
    pub rows: Vec<Row>,
}
