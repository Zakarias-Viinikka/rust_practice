use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::any,
};
use db_weebee::mascot::LiveForever;
use shared_types::message_struct;
use web_socket_test::{allowed_requests, parse_request, personal_db_wrapper};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            let json_msg = text.as_str();
            let msg = message_struct::json_str_to_message(json_msg);
            let msg = match msg {
                Ok(msg) => msg,
                Err(e) => {
                    //make an error message and return it serialized to json as str
                    todo!()
                }
            };
            let do_request_result = allowed_requests::do_request(&msg, liver);
        }
    }
}

#[tokio::main]
async fn main() {
    let liver = personal_db_wrapper::new_liver();
    let app = Router::new().route("/ws", any(ws_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
