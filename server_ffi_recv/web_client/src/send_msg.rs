use leptos::reactive::{signal::WriteSignal, traits::Set};
use protocol::serialization::Convert;
use server_protocol::{build_msg, Request};
use web_sys::WebSocket;

use std::sync::atomic::{AtomicU64, Ordering};

static MSG_ID: AtomicU64 = AtomicU64::new(0);

pub fn send_message<T: Convert>(
    socket_conn: Option<WebSocket>,
    request: Request,
    payload_before_serialization: T,
    error_msg_set: WriteSignal<String>,
) -> Result<(), String> {
    let message_id = MSG_ID.fetch_add(1, Ordering::Relaxed) as usize;
    let msg_json = build_msg(message_id, request, payload_before_serialization)
        .map_err(|e| e.to_string())?;

    if let Some(socket_conn) = socket_conn {
        if let Err(e) = socket_conn.send_with_str(&msg_json) {
            error_msg_set.set(e.as_string().unwrap_or_default());
        }
    }

    Ok(())
}
