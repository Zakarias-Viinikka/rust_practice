use error_stuff::InBetweenErr;
use futures::StreamExt;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{MessageEvent, WebSocket};

pub struct InbetweenMan {
    ws: WebSocket,
    server_request_to_make_rx: UnboundedReceiver<(u64, String)>,
}

impl InbetweenMan {
    pub fn new(
        url: &str,
    ) -> Result<
        (
            UnboundedReceiver<Result<(), String>>,
            UnboundedSender<(u64, String)>,
            UnboundedReceiver<(u64, String)>,
        ),
        String,
    > {
        let ws = connect_to_socket(url)?;

        let (connect_tx, connect_rx) = unbounded::<Result<(), String>>();
        let all_rtx = AllRtx::new();

        let response_forwarder = all_rtx.receive_server_response_tx.clone();
        let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| 'block: {
            let Some(decoded_response) = decode_response(e) else {
                something_went_wrong("could not decode response");
                break 'block;
            };

            if let Err(e) = forward_response(&response_forwarder, decoded_response) {
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

        spawn_man(man);

        Ok((
            connect_rx,
            all_rtx.server_request_to_make_tx,
            all_rtx.receive_server_response_rx,
        ))
    }
}

// ### ### ###
// ### ### ###
//
// HELPERS DOWN HERE
//
// ### ### ###
// ### ### ###
//
struct AllRtx {
    server_request_to_make_tx: UnboundedSender<(u64, String)>,
    server_request_to_make_rx: UnboundedReceiver<(u64, String)>,
    receive_server_response_tx: UnboundedSender<(u64, String)>,
    receive_server_response_rx: UnboundedReceiver<(u64, String)>,
}

impl AllRtx {
    fn new() -> Self {
        let (server_request_to_make_tx, server_request_to_make_rx) = unbounded::<(u64, String)>();
        let (receive_server_response_tx, receive_server_response_rx) = unbounded::<(u64, String)>();

        Self {
            server_request_to_make_tx,
            server_request_to_make_rx,
            receive_server_response_tx,
            receive_server_response_rx,
        }
    }
}

fn forward_response(
    tx: &UnboundedSender<(u64, String)>,
    decoded: ResponseDecoded,
) -> Result<(), String> {
    tx.unbounded_send((decoded.id, decoded.payload))
        .map_err(|e| format!("could not forward response: {:?}", e))
}

fn connect_to_socket(url: &str) -> Result<WebSocket, String> {
    WebSocket::new(url).map_err(|e| {
        InBetweenErr {
            err_msg: e.as_string().unwrap_or_default(),
            file: file!().to_string(),
            method: "connect_to_socket".to_string(),
        }
        .to_string()
    })
}

pub struct ResponseDecoded {
    pub id: u64,
    pub payload: String,
}

fn decode_response(event: MessageEvent) -> Option<ResponseDecoded> {
    let text = event.data().as_string()?;
    let (id_str, payload) = text.split_once('|')?;
    let id = id_str.parse::<u64>().ok()?;
    Some(ResponseDecoded {
        id,
        payload: payload.to_string(),
    })
}

fn something_went_wrong(msg: &str) {
    let err = InBetweenErr {
        err_msg: msg.to_string(),
        file: file!().to_string(),
        method: "onmessage".to_string(),
    };
    web_sys::console::error_1(&err.to_string().into());
}

fn attach_events(
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

fn spawn_man(mut man: InbetweenMan) {
    spawn_local(async move {
        while let Some((id, payload)) = man.server_request_to_make_rx.next().await {
            let msg = format!("{}|{}", id, payload);
            if let Err(e) = man.ws.send_with_str(&msg) {
                something_went_wrong(&format!("send failed: {:?}", e));
            }
        }
    });
}
