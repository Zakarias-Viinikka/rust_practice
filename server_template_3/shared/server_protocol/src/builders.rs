use crate::message::{Message, Response};
use crate::request::{Request, UnbuiltRequest, UnbuiltResponse};
use error_stuff::{ErrorDetails, ServerRelatedError};
use protocol::serialization::Base64Bytes;

pub fn build_msg(
    message_id: usize,
    request: Request,
    payload: Vec<u8>,
) -> Result<String, ServerRelatedError> {
    let content = Base64Bytes(payload);
    let msg = Message {
        message_id,
        request,
        content,
    };
    serde_json::to_string(&msg).map_err(|e| protocol_err("build_msg", e.to_string()))
}

pub fn unbuild_msg(json: &str) -> Result<(UnbuiltRequest, usize, Vec<u8>), ServerRelatedError> {
    let msg: Message =
        serde_json::from_str(json).map_err(|e| protocol_err("unbuild_msg", e.to_string()))?;

    let unbuilt = match msg.request {
        Request::Ping => UnbuiltRequest::Ping,
        Request::GetEntireTable { table_name } => {
            UnbuiltRequest::GetEntireTable { table_name }
        }
    };

    Ok((unbuilt, msg.message_id, msg.content.0))
}

pub fn build_response(
    message_id: usize,
    request: Request,
    payload: Vec<u8>,
) -> Result<String, ServerRelatedError> {
    let data = Base64Bytes(payload);
    let response = Response {
        message_id,
        request,
        data,
    };
    serde_json::to_string(&response).map_err(|e| protocol_err("build_response", e.to_string()))
}

pub fn unbuild_response(
    json: &str,
) -> Result<(UnbuiltResponse, usize, Vec<u8>), ServerRelatedError> {
    let response: Response =
        serde_json::from_str(json).map_err(|e| protocol_err("unbuild_response", e.to_string()))?;

    let unbuilt = match response.request {
        Request::Ping => UnbuiltResponse::Ping,
        Request::GetEntireTable { table_name } => {
            UnbuiltResponse::GetEntireTable { table_name }
        }
    };

    Ok((unbuilt, response.message_id, response.data.0))
}

fn protocol_err(method: &str, err_msg: String) -> ServerRelatedError {
    ServerRelatedError::Protocol(ErrorDetails {
        err_msg,
        file: file!().to_string(),
        method: method.to_string(),
    })
}
