use crate::ServerTalker;
use error_stuff::{ErrorDetails, ServerRelatedError};
use futures::StreamExt;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};
use server_protocol::{Message, Response};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{MessageEvent, WebSocket};

pub struct AllRtx {
    pub server_request_to_make_tx: UnboundedSender<Message>,
    pub server_request_to_make_rx: UnboundedReceiver<Message>,
    pub receive_server_response_tx: UnboundedSender<Response>,
    pub receive_server_response_rx: UnboundedReceiver<Response>,
}

impl AllRtx {
    pub fn new() -> Self {
        let (server_request_to_make_tx, server_request_to_make_rx) = unbounded::<Message>();
        let (receive_server_response_tx, receive_server_response_rx) = unbounded::<Response>();

        Self {
            server_request_to_make_tx,
            server_request_to_make_rx,
            receive_server_response_tx,
            receive_server_response_rx,
        }
    }
}

pub fn forward_response(tx: &UnboundedSender<Response>, response: Response) -> Result<(), String> {
    tx.unbounded_send(response)
        .map_err(|e| format!("could not forward response: {:?}", e))
}

pub fn connect_to_socket(url: &str) -> Result<WebSocket, String> {
    WebSocket::new(url).map_err(|e| {
        ServerRelatedError::ServerTalker(ErrorDetails {
            err_msg: e.as_string().unwrap_or_default(),
            file: file!().to_string(),
            method: "connect_to_socket".to_string(),
        })
        .to_string()
    })
}

pub fn decode_response(event: MessageEvent) -> Result<Response, ServerRelatedError> {
    let text = event.data().as_string().ok_or_else(|| {
        ServerRelatedError::ServerTalker(ErrorDetails {
            err_msg: "message event had no string data".to_string(),
            file: file!().to_string(),
            method: "decode_response".to_string(),
        })
    })?;
    serde_json::from_str(&text).map_err(|e| {
        ServerRelatedError::ServerTalker(ErrorDetails {
            err_msg: format!("could not parse response JSON: {}", e),
            file: file!().to_string(),
            method: "decode_response".to_string(),
        })
    })
}

pub fn something_went_wrong(msg: &str) {
    let err = ServerRelatedError::ServerTalker(ErrorDetails {
        err_msg: msg.to_string(),
        file: file!().to_string(),
        method: "onmessage".to_string(),
    });
    web_sys::console::error_1(&err.to_string().into());
}

pub fn attach_events(
    ws: &WebSocket,
    onmessage: Closure<dyn FnMut(MessageEvent)>,
    on_open: Closure<dyn FnMut(JsValue)>,
    on_error: Closure<dyn FnMut(JsValue)>,
    on_close: Closure<dyn FnMut(JsValue)>,
) {
    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();

    ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    on_error.forget();

    ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
    on_close.forget();
}

pub fn spawn_server_talker(mut man: ServerTalker) {
    spawn_local(async move {
        while let Some(msg) = man.server_request_to_make_rx.next().await {
            let json = match serde_json::to_string(&msg) {
                Ok(s) => s,
                Err(e) => {
                    something_went_wrong(&format!("could not serialize message: {}", e));
                    continue;
                }
            };
            if let Err(e) = man.ws.send_with_str(&json) {
                something_went_wrong(&format!("send failed: {:?}", e));
            }
        }
    });
}
