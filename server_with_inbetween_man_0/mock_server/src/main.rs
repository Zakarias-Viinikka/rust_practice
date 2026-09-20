use axum::{
    Router,
    extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::any,
};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let WsMessage::Text(text) = msg {
            println!("[mock_server] got: {}", text);

            let reply = match text.split_once('|') {
                Some((id, payload)) => format!("{}|mock reply to: {}", id, payload),
                None => format!("mock reply to: {}", text),
            };

            println!("[mock_server] replying: {}", reply);
            if let Err(e) = socket.send(WsMessage::Text(reply.into())).await {
                println!("send failed: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", any(ws_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[mock_server] listening on :3000");
    axum::serve(listener, app).await.unwrap();
}
