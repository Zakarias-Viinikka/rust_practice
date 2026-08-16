use anyhow::{Result, anyhow};
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

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

    pub fn cure_from_js_value(value: JsValue) -> Result<Self> {
        let bytes = Uint8Array::from(value).to_vec();
        Self::deserialize(&bytes)
    }
}
