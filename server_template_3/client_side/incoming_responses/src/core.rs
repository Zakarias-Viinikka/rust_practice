use error_stuff::{ErrorDetails, ServerRelatedError};
use futures::StreamExt;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::channel::oneshot;
use server_protocol::{Message, Response};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct ResponseRouter {
    outgoing_message_tx: UnboundedSender<Message>,
    next_request_id_ctr: Arc<AtomicUsize>,
    pending_responses: Arc<Mutex<HashMap<usize, oneshot::Sender<Response>>>>,
}

impl ResponseRouter {
    pub fn new(
        outgoing_message_tx: UnboundedSender<Message>,
        incoming_responses_rx: UnboundedReceiver<Response>,
    ) -> Self {
        let pending_responses: Arc<Mutex<HashMap<usize, oneshot::Sender<Response>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_responses_clone = Arc::clone(&pending_responses);

        spawn_local(async move {
            let mut incoming = incoming_responses_rx;
            while let Some(response) = incoming.next().await {
                let waiter = pending_responses_clone
                    .lock()
                    .unwrap()
                    .remove(&response.message_id);
                if let Some(waiter) = waiter {
                    let _ = waiter.send(response);
                }
            }
        });

        Self {
            outgoing_message_tx,
            next_request_id_ctr: Arc::new(AtomicUsize::new(0)),
            pending_responses,
        }
    }

    pub fn send_request<F>(&self, build_message: F) -> oneshot::Receiver<Response>
    where
        F: FnOnce(usize) -> Message,
    {
        let id = self.next_request_id_ctr.fetch_add(1, Ordering::Relaxed);
        let message = build_message(id);

        let (reply_tx, reply_rx) = oneshot::channel();
        self.pending_responses.lock().unwrap().insert(id, reply_tx);

        if self.outgoing_message_tx.unbounded_send(message).is_err() {
            let err = ServerRelatedError::ResponseRouter(ErrorDetails {
                err_msg: "outgoing channel to server_talker is closed".to_string(),
                file: file!().to_string(),
                method: "send_request".to_string(),
            });
            web_sys::console::error_1(&err.to_string().into());
            self.pending_responses.lock().unwrap().remove(&id);
        }

        reply_rx
    }
}
