use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};

pub struct InbetweenMan {
    ws: WebSocket,
    pending: Arc<Mutex<HashMap<u64, UnboundedSender<String>>>>,
    next_id: Arc<Mutex<u64>>,
}

impl InbetweenMan {
    pub fn connect(url: &str) -> Result<Self, String> {
        let ws = WebSocket::new(url)
            .map_err(|e| e.as_string().unwrap_or_else(|| format!("{:?}", e)))?;

        let pending: Arc<Mutex<HashMap<u64, UnboundedSender<String>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_for_handler = pending.clone();

        let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
            let Some(text) = e.data().as_string() else { return };
            let Some((id_str, payload)) = text.split_once('|') else { return };
            let Ok(id) = id_str.parse::<u64>() else { return };

            let sender = pending_for_handler.lock().unwrap().get(&id).cloned();
            if let Some(sender) = sender {
                let _ = sender.unbounded_send(payload.to_string());
            }
        });
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();

        Ok(Self {
            ws,
            pending,
            next_id: Arc::new(Mutex::new(0)),
        })
    }

    pub fn send(&self, payload: &str) -> Result<UnboundedReceiver<String>, String> {
        let id = {
            let mut n = self.next_id.lock().unwrap();
            let id = *n;
            *n += 1;
            id
        };

        let (tx, rx) = unbounded();
        self.pending.lock().unwrap().insert(id, tx);

        let msg = format!("{}|{}", id, payload);
        self.ws
            .send_with_str(&msg)
            .map_err(|e| e.as_string().unwrap_or_default())?;

        Ok(rx)
    }
}
