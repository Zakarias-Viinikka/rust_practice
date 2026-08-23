use crate::byte_serialization::Convert;
use crate::data_structures::{GetDataIn, GetDataOut};
use crate::db_error::DbError;
use crate::message_struct::*;

#[uniffi::export]
pub fn build_get_data_in_msg(message_id: u64, payload: GetDataIn) -> Result<String, DbError> {
    let bytes = payload.serialize_wrapper();
    let msg = Message {
        message_id: message_id as usize,
        request: Request::GetData,
        content: Base64Bytes(bytes),
    };
    message_to_json_str(&msg).map_err(|e| DbError::SerializeError(e.to_string()))
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct GetDataOutUnbuilt {
    pub message_id: u64,
    pub request: Request,
    pub get_data_out: GetDataOut,
}

#[uniffi::export]
pub fn unbuild_get_data_out_response(response: String) -> Result<GetDataOutUnbuilt, DbError> {
    let result =
        json_str_to_response(&response).map_err(|e| DbError::SerializeError(e.to_string()))?;
    let (message_id, request, data) = (result.message_id, result.request, result.data);

    let get_data_out = GetDataOut::deserialize_wrapper(&data.0)?;

    Ok(GetDataOutUnbuilt {
        message_id: message_id as u64,
        request,
        get_data_out,
    })
}
