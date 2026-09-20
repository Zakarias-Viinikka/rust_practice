use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};

pub fn connect_to_socket(
    on_message: impl Fn(String) + 'static,
    on_fail: impl Fn(String) + 'static,
    on_success: impl Fn(WebSocket) + 'static,
    on_close: impl Fn() + 'static,
) -> Result<(), String> {
    let ip = include_str!("ip.env").trim();

    let ws = WebSocket::new(&format!("ws://{}:3000/ws", ip))
        .map_err(|e| e.as_string().unwrap_or_else(|| format!("{:?}", e)))?;

    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
        if let Some(json_response) = e.data().as_string() {
            on_message(json_response);
        }
    });

    let on_fail_closure = Closure::<dyn FnMut(JsValue)>::new(move |_| {
        on_fail("WebSocket connection failed".to_string());
    });
    ws.set_onerror(Some(on_fail_closure.as_ref().unchecked_ref()));
    on_fail_closure.forget();

    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    let on_close_closure = Closure::<dyn FnMut(JsValue)>::new(move |_| {
        on_close();
    });
    ws.set_onclose(Some(on_close_closure.as_ref().unchecked_ref()));
    on_close_closure.forget();

    let ws_for_onopen = ws.clone();

    let onopen = Closure::<dyn FnMut(JsValue)>::new(move |_| {
        on_success(ws_for_onopen.clone());
    });

    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();

    Ok(())
}
