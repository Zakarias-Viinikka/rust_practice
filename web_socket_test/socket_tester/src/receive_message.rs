use shared_types::{byte_serialization::Convert, data_structures, db_error, message_struct};

pub fn receive_response(json: &str) -> Result<(ExpectedResponse, usize), ReceiveError> {
    let response = message_struct::deserialize_response(json)
        .map_err(|e| ReceiveError::OtherErr(e.to_string()))?;
    match response.request {
        message_struct::Request::GetData => {
            let data = response.result.map_err(ReceiveError::DbErr)?;
            let something = get_data(data)?;
            return Ok((ExpectedResponse::GetData(something), response.message_id));
        }
        _ => return Err(ReceiveError::UnexpectedResponse),
    }
}

pub fn get_data(
    base64: message_struct::Base64Bytes,
) -> Result<data_structures::GetDataOut, ReceiveError> {
    let vecu8 = base64.0;
    let get_data_out = match data_structures::GetDataOut::deserialize_wrapper(&vecu8) {
        Ok(deserialize_vecu8) => deserialize_vecu8,
        Err(err) => Err(ReceiveError::DbErr(err))?,
    };

    Ok(get_data_out)
}

#[derive(Debug)]
pub enum ReceiveError {
    DbErr(db_error::DbError),
    UnexpectedResponse,
    OtherErr(String),
}

pub enum ExpectedResponse {
    GetData(data_structures::GetDataOut),
}
