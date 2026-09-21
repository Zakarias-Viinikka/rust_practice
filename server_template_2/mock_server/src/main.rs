use axum::{
    Router,
    extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::any,
};
use server_protocol::{Request, build_response, ok_serialized, unbuild_msg};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", any(ws_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[mock_server] listening on :3000");
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let WsMessage::Text(text) = msg {
            let reply = match unbuild_msg(&text) {
                Ok((_unbuilt, id, _payload)) => {
                    match build_response(id, Request::Ping, ok_serialized()) {
                        Ok(s) => s,
                        Err(e) => {
                            println!("[mock_server] build_response failed: {}", e);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    println!("[mock_server] unbuild_msg failed: {}", e);
                    "Message doesn't use correct protocol".to_string()
                }
            };
            println!("[mock_server] got: {} | replying: {}", text, reply);
            if let Err(e) = socket.send(WsMessage::Text(reply.into())).await {
                println!("[mock_server] send failed: {}", e);
                break;
            }
        }
    }
}
