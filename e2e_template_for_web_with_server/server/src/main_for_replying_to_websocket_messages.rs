//! One websocket connection from a browser. Receives a text message,
//! replies with a fixed string.

use axum::extract::ws::{Message as WsMessage, WebSocket};

const THE_REPLY: &str = "hello from server";

pub async fn main_for_replying_to_websocket_messages(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let WsMessage::Text(text) = &msg {
            println!("[server] got: {}", text);
            if socket
                .send(WsMessage::Text(THE_REPLY.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    }
}