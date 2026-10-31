//! The only file in the app that touches web_sys::WebSocket directly.
//!
//! Callers do:
//!
//!     let talker = ServerTalker::connect(url);
//!     talker.send("1|add hello");
//!     // read talker.connected / talker.last_reply inside a view
//!
//! Nothing else imports web_sys.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};

pub struct ServerTalker {
    pub connected: RwSignal<bool>,
    pub last_reply: RwSignal<String>,
    ws: RwSignal<Option<WebSocket>>,
}

impl ServerTalker {
    pub fn connect(url: &str) -> Self {
        let connected = RwSignal::new(false);
        let last_reply = RwSignal::new(String::new());
        let ws_holder: RwSignal<Option<WebSocket>> = RwSignal::new(None);

        match WebSocket::new(url) {
            Ok(ws) => {
                let on_open = {
                    let connected = connected;
                    Closure::<dyn FnMut(JsValue)>::new(move |_| connected.set(true))
                };
                ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
                on_open.forget();

                let on_message = {
                    let last_reply = last_reply;
                    Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
                        if let Some(text) = e.data().as_string() {
                            let body = text
                                .split_once('|')
                                .map(|(_, rest)| rest.to_string())
                                .unwrap_or(text);
                            last_reply.set(body);
                        }
                    })
                };
                ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
                on_message.forget();

                let on_close = {
                    let connected = connected;
                    Closure::<dyn FnMut(JsValue)>::new(move |_| connected.set(false))
                };
                ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
                on_close.forget();

                ws_holder.set(Some(ws));
            }
            Err(_) => {
                last_reply.set("could not create ws".to_string());
            }
        }

        Self {
            connected,
            last_reply,
            ws: ws_holder,
        }
    }

    pub fn send(&self, text: &str) {
        if let Some(ws) = self.ws.get_untracked() {
            let _ = ws.send_with_str(text);
        }
    }
}
