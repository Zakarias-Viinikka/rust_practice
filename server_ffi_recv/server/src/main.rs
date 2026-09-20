use axum::{
    Router,
    extract::{
        ConnectInfo,
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    },
    response::Response,
    routing::any,
};
use server_protocol::{Request, UnbuiltRequest, build_response, unbuild_msg};
use std::net::SocketAddr;

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response {
    println!("--- New WebSocket connection attempt detected! | ip: {} ---", addr);
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let WsMessage::Text(text) = msg {
            let json_msg = text.as_str();
            println!("got message: {:?}", json_msg);

            let (unbuilt, message_id) = match unbuild_msg(json_msg) {
                Ok(v) => v,
                Err(_) => {
                    let _ = socket
                        .send(WsMessage::Text(
                            "Message doesn't use correct protocol".into(),
                        ))
                        .await;
                    continue;
                }
            };

            let response_json = match unbuilt {
                UnbuiltRequest::Ping => build_response(message_id, Request::Ping, ()),
            };

            let response_json = match response_json {
                Ok(s) => s,
                Err(e) => {
                    println!("failed to build response: {}", e);
                    continue;
                }
            };

            println!("about to send message: {:?}", response_json);
            if let Err(e) = socket.send(WsMessage::Text(response_json.into())).await {
                println!("failed to send message: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", any(ws_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
