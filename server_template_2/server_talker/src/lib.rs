use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};
use server_protocol::{Message, Response};
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};

mod helper;

use helper::{
    AllRtx, attach_events, connect_to_socket, decode_response, forward_response,
    something_went_wrong, spawn_server_talker,
};

pub struct ServerTalker {
    ws: WebSocket,
    server_request_to_make_rx: UnboundedReceiver<Message>,
}

pub struct ServerTalkerHandles {
    pub connection_events: UnboundedReceiver<Result<(), String>>,
    pub outgoing_messages: UnboundedSender<Message>,
    pub incoming_responses: UnboundedReceiver<Response>,
}

impl ServerTalker {
    pub fn new(url: &str) -> Result<ServerTalkerHandles, String> {
        let ws = connect_to_socket(url)?;

        let (connect_tx, connect_rx) = unbounded::<Result<(), String>>();
        let all_rtx = AllRtx::new();

        let response_forwarder = all_rtx.receive_server_response_tx.clone();
        let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| 'block: {
            let response = match decode_response(e) {
                Ok(r) => r,
                Err(err) => {
                    something_went_wrong(&err.to_string());
                    break 'block;
                }
            };

            if let Err(e) = forward_response(&response_forwarder, response) {
                something_went_wrong(&e);
            }
        });

        let connect_tx_clone = connect_tx.clone();
        let on_open = Closure::<dyn FnMut(JsValue)>::new(move |_| {
            let _ = connect_tx_clone.unbounded_send(Ok(()));
        });

        let connect_tx_clone = connect_tx.clone();
        let on_error = Closure::<dyn FnMut(JsValue)>::new(move |_| {
            let _ = connect_tx_clone.unbounded_send(Err("could not connect".to_string()));
        });

        let connect_tx_clone = connect_tx.clone();
        let on_close = Closure::<dyn FnMut(JsValue)>::new(move |_| {
            let _ = connect_tx_clone.unbounded_send(Err("connection closed".to_string()));
        });

        attach_events(&ws, onmessage, on_open, on_error, on_close);

        let man = Self {
            ws,
            server_request_to_make_rx: all_rtx.server_request_to_make_rx,
        };

        spawn_server_talker(man);

        Ok(ServerTalkerHandles {
            connection_events: connect_rx,
            outgoing_messages: all_rtx.server_request_to_make_tx,
            incoming_responses: all_rtx.receive_server_response_rx,
        })
    }
}
