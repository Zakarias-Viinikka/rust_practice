use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Data {
    Data1In { name: String },
    Data1Out { name: String },
}

impl Data {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow!(e))
    }
    pub fn deserialize(data_to_unpack: &[u8]) -> Result<Self> {
        bincode::deserialize(data_to_unpack).map_err(|e| anyhow!(e))
    }
}
