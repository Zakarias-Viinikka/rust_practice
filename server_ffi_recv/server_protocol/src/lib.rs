use error_stuff::{ErrorDetails, ServErr};
use protocol::serialization::{Base64Bytes, Convert};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub message_id: usize,
    pub request: Request,
    pub content: Base64Bytes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub message_id: usize,
    pub request: Request,
    pub data: Base64Bytes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Ping,
}

#[derive(Debug, Clone)]
pub enum UnbuiltRequest {
    Ping,
}

pub fn build_msg<T: Convert>(
    message_id: usize,
    request: Request,
    payload_before_serialization: T,
) -> Result<String, ServErr> {
    let content = Base64Bytes(payload_before_serialization.to_payload());

    let msg = Message {
        message_id,
        request,
        content,
    };

    serde_json::to_string(&msg).map_err(|e| {
        ServErr::ServerError(ErrorDetails {
            err_msg: e.to_string(),
            file: file!().to_string(),
            method: "build_msg".to_string(),
        })
    })
}

pub fn unbuild_msg(json: &str) -> Result<(UnbuiltRequest, usize), ServErr> {
    let msg: Message = serde_json::from_str(json).map_err(|e| {
        ServErr::ServerError(ErrorDetails {
            err_msg: e.to_string(),
            file: file!().to_string(),
            method: "unbuild_msg".to_string(),
        })
    })?;

    let unbuilt = match msg.request {
        Request::Ping => UnbuiltRequest::Ping,
    };

    Ok((unbuilt, msg.message_id))
}

pub fn build_response<T: Convert>(
    message_id: usize,
    request: Request,
    payload_before_serialization: T,
) -> Result<String, ServErr> {
    let data = Base64Bytes(payload_before_serialization.to_payload());

    let response = Response {
        message_id,
        request,
        data,
    };

    serde_json::to_string(&response).map_err(|e| {
        ServErr::ServerError(ErrorDetails {
            err_msg: e.to_string(),
            file: file!().to_string(),
            method: "build_response".to_string(),
        })
    })
}

#[derive(Debug, Clone)]
pub enum UnbuiltResponse {
    Ping,
}

pub fn unbuild_response(json: &str) -> Result<(UnbuiltResponse, usize), ServErr> {
    let response: Response = serde_json::from_str(json).map_err(|e| {
        ServErr::ServerError(ErrorDetails {
            err_msg: e.to_string(),
            file: file!().to_string(),
            method: "unbuild_response".to_string(),
        })
    })?;

    let unbuilt = match response.request {
        Request::Ping => UnbuiltResponse::Ping,
    };

    Ok((unbuilt, response.message_id))
}
