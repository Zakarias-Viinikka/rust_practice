use anyhow::{Result, anyhow};
use js_sys::Uint8Array;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataIn {
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DataOut {
    pub name: String,
}
//Data1Out { name: String },

pub trait Convert: Serialize + DeserializeOwned {
    fn serialize_wrapper(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow!(e))
    }

    fn deserialize_wrapper(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data).map_err(|e| anyhow!(e))
    }

    fn cure_from_js_value(value: JsValue) -> Result<Self> {
        let bytes = Uint8Array::from(value).to_vec();
        <Self>::deserialize_wrapper(&bytes)
    }
}

impl<T: Serialize + DeserializeOwned> Convert for T {}

#[derive(Serialize, Deserialize, Debug)]
pub enum DbError {}
