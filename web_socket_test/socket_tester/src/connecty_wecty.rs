use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};

pub fn connect(on_message: impl Fn(String) + 'static) -> Result<WebSocket, String> {
    let ws = WebSocket::new("ws://127.0.0.1:3000/ws")
        .map_err(|e| e.as_string().unwrap_or_else(|| format!("{:?}", e)))?;

    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
        if let Some(text) = e.data().as_string() {
            on_message(text);
        }
    });
    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    Ok(ws) // <- hand back the live connection
}
