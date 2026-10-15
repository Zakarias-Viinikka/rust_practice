mod queue_executor;
mod requests;
mod setup;
mod socket;

use axum::{Router, extract::ws::WebSocketUpgrade, routing::any};
use queue::QueueManager;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // db table setup
    let liver = setup::open_db();
    setup::create_all_tables(&liver);
    setup::seed_tables(&liver);

    // rx tx stuff for the two queues.
    let (important_tx, important_rx) = mpsc::unbounded_channel();
    let (normal_tx, normal_rx) = mpsc::unbounded_channel();

    // you talk to queue manager when you want to make a request to the server
    let queue_manager = QueueManager::new(important_tx, normal_tx);

    queue_executor::start(important_rx, normal_rx, liver);

    // the entrance for websocket connections: when a browser first connects to /ws,
    // upgrade it to a websocket and give it a way to add requests to the queues
    let app = Router::new().route(
        "/ws",
        any(move |ws: WebSocketUpgrade| {
            let queue_manager = queue_manager.clone();
            async move { ws.on_upgrade(move |socket| socket::handle_socket(socket, queue_manager)) }
        }),
    );

    // listen on port 3000 for incoming connections
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[server] listening on :3000");
    axum::serve(listener, app).await.unwrap();
}
