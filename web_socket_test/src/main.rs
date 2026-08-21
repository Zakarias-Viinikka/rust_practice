use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
    routing::any,
};
use db_weebee::mascot::LiveForever;
use shared_types::message_struct;
use std::sync::Arc;
use web_socket_test::reply::{craft_response, send_message};
use web_socket_test::{allowed_requests, personal_db_wrapper};

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(liver): State<Arc<std::sync::Mutex<LiveForever>>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, liver))
}

async fn handle_socket(mut socket: WebSocket, liver: Arc<std::sync::Mutex<LiveForever>>) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            let json_msg = text.as_str();
            println!("got message: {:?}", json_msg);
            let msg = message_struct::json_str_to_message(json_msg);
            let (msg_request, msg_id, msg_content);
            let msg = match msg {
                Ok(msg) => {
                    msg_request = msg.request;
                    msg_id = msg.message_id;
                    msg_content = msg.content.0;
                }
                Err(e) => {
                    //make an error message and return it serialized to json as str
                    todo!()
                }
            };
            //println!("extracted data out of message into vecu8 and about to use it");
            let do_request_result = {
                //println!("about to lock");
                let liver_locked = liver.lock().unwrap();
                allowed_requests::do_request(&msg_request, msg_content, &liver_locked)
            };
            //println!("lock over and request was fulfilled");
            let response = craft_response(do_request_result, msg_id, msg_request);
            println!("about to send message: {:?}", response);
            let result = send_message(&mut socket, response).await;
            //println!("message was sent");
            if let Err(e) = result {
                //println!("{}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let liver_inner = personal_db_wrapper::new_liver();
    if let Err(e) = personal_db_wrapper::create_table_if_not_exist(&liver_inner) {
        println!("failed to create table: {:?}", e);
    }
    if let Err(e) = personal_db_wrapper::create_data_if_not_exist(&liver_inner) {
        println!("failed to create data: {:?}", e);
    }
    let liver = Arc::new(std::sync::Mutex::new(liver_inner));
    let app = Router::new()
        .route("/ws", any(ws_handler))
        .with_state(liver); //passes liver to ws_handler which passes it to handle_socket

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
