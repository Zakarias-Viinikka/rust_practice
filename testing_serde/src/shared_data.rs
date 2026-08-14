use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataIn {
    pub data1: String,
    pub data2: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataOut {
    pub data_combined: String,
}
