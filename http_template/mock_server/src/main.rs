use axum::{Router, body::Bytes, http::StatusCode, routing::post};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/call", post(handle_call));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[mock_server] listening on :3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_call(body: Bytes) -> (StatusCode, String) {
    let incoming = String::from_utf8_lossy(&body).to_string();
    let reply = make_mock_reply(&incoming);
    println!("[mock_server] got: {} | replying: {}", incoming, reply);
    (StatusCode::OK, reply)
}

fn make_mock_reply(incoming: &str) -> String {
    format!("mock reply to: {}", incoming)
}