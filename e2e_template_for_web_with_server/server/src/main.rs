mod main_for_replying_to_websocket_messages;

use axum::{
    Router,
    extract::ws::WebSocketUpgrade,
    response::Response,
    routing::any,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = read_port();

    let app = Router::new().route("/ws", any(ws_handler));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("[server] listening on 127.0.0.1:{}", port);
    axum::serve(listener, app).await?;
    Ok(())
}

fn read_port() -> u16 {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000)
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(
        main_for_replying_to_websocket_messages::main_for_replying_to_websocket_messages,
    )
}