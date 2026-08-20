use leptos::reactive::{signal::WriteSignal, traits::Set};
use shared_types::{byte_serialization::Convert, message_struct};
use web_sys::WebSocket;

use std::sync::atomic::{AtomicU64, Ordering};

static MSG_ID: AtomicU64 = AtomicU64::new(0);

pub fn prepare_message<T: Convert>(
    request: message_struct::Request,
    content: T,
) -> Result<String, String> {
    let content = content.serialize_wrapper();
    let content = message_struct::Base64Bytes(content);

    let message_id = MSG_ID.fetch_add(1, Ordering::Relaxed) as usize;

    let msg = message_struct::Message {
        message_id,
        request,
        content,
    };

    let msg_json = message_struct::serialize_message(&msg).map_err(|e| e.to_string());
    msg_json
}

pub fn send_message<T: Convert>(
    socket_conn: Option<WebSocket>,
    request: message_struct::Request,
    content: T,
    error_msg_set: WriteSignal<String>,
) -> Result<(), String> {
    let msg_json = prepare_message(request, content)?;

    if let Some(socket_conn) = socket_conn {
        let result = socket_conn.send_with_str(&msg_json);
        if let Err(e) = result {
            error_msg_set.set(e.as_string().unwrap_or_default());
        }
    }

    Ok(())
}
