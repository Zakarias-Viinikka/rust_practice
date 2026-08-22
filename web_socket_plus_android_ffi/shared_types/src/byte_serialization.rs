use serde::{Serialize, de::DeserializeOwned};

use crate::db_error::DbError;

pub trait Convert: Serialize + DeserializeOwned {
    fn serialize_wrapper(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_else(|e| {
            bincode::serialize(&DbError::SerializeError(
                "Failed to serialize: ".to_string() + &e.to_string(),
            ))
            .unwrap()
        })
    }

    fn deserialize_wrapper(data: &[u8]) -> Result<Self, DbError> {
        bincode::deserialize(data).map_err(|e| DbError::CureFail(e.to_string()))
    }
}

pub fn ok_serialized() -> Vec<u8> {
    Ok::<(), DbError>(()).serialize_wrapper()
}

impl<T: Serialize + DeserializeOwned> Convert for T {}
