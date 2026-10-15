use axum::extract::ws::{Message as WsMessage, WebSocket};
use queue::{Priority, QueueManager};
use server_protocol::{UnbuiltRequest, unbuild_msg};

pub async fn handle_socket(mut socket: WebSocket, queue_manager: QueueManager) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let WsMessage::Text(text) = msg {
            let reply = match unbuild_msg(&text) {
                Ok((unbuilt, id, _payload)) => {
                    let priority = if matches!(&unbuilt, UnbuiltRequest::Ping) {
                        Priority::Important
                    } else {
                        Priority::Normal
                    };
                    let reply_rx = match queue_manager.submit(unbuilt, id, priority) {
                        Ok(rx) => rx,
                        Err(e) => {
                            println!("[server] could not queue request: {}", e);
                            continue;
                        }
                    };
                    match reply_rx.await {
                        Ok(Ok(r)) => r,
                        Ok(Err(e)) => {
                            println!("[server] request handling failed: {}", e);
                            continue;
                        }
                        Err(_) => {
                            println!("[server] executor dropped the request");
                            continue;
                        }
                    }
                }
                Err(e) => {
                    println!("[server] unbuild_msg failed: {}", e);
                    "Message doesn't use correct protocol".to_string()
                }
            };
            println!("[server] got: {} | replying: {}", text, reply);
            if let Err(e) = socket.send(WsMessage::Text(reply.into())).await {
                println!("[server] send failed: {}", e);
                break;
            }
        }
    }
}
