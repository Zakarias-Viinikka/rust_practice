use axum::{
    Router,
    extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    routing::any,
};
use db_wrapper::mascot::LiveForever;
use protocol::payload::{
    ColumnValue, CreateTableIn, GetDataIn, InsertDataIn, SelectArgument, SelectArguments,
};
use protocol::row_col::Col;
use protocol::serialization::Convert;
use server_db_schema::all_tables;
use server_protocol::{
    DbError, GetEntireTableOut, Request, UnbuiltRequest, build_response, ok_serialized,
    unbuild_msg,
};
use std::sync::{Arc, Mutex};

const DB_FILENAME: &str = "test_db.sqlite";

#[tokio::main]
async fn main() {
    let liver = Arc::new(Mutex::new(open_db()));
    {
        let liver_guard = liver.lock().unwrap();
        create_all_tables(&liver_guard);
        seed_tables(&liver_guard);
    }

    let liver_for_router = Arc::clone(&liver);
    let app = Router::new().route(
        "/ws",
        any(move |ws: WebSocketUpgrade| {
            let liver = Arc::clone(&liver_for_router);
            async move { ws.on_upgrade(move |socket| handle_socket(socket, liver)) }
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[server] listening on :3000");
    axum::serve(listener, app).await.unwrap();
}

fn open_db() -> LiveForever {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), DB_FILENAME);
    println!("[server] opening db at {}", path);
    match LiveForever::new(&path) {
        Ok(liver) => liver,
        Err(e) => {
            eprintln!("[server] could not open db: {}", e);
            std::process::exit(1);
        }
    }
}

fn create_all_tables(liver: &LiveForever) {
    for table in all_tables() {
        let name = table.table_name.clone();
        match liver.create_table(CreateTableIn {
            table_name: name.clone(),
            columns: table.columns,
        }) {
            Ok(()) => println!("[server] created table {}", name),
            Err(e) => println!("[server] table {} not created: {}", name, e),
        }
    }
}

fn table_is_empty(liver: &LiveForever, table_name: &str) -> Result<bool, String> {
    liver
        .get_data(GetDataIn {
            table_name: table_name.to_string(),
            arguments: SelectArguments::Single(SelectArgument::All),
            columns_to_read: Vec::new(),
        })
        .map(|out| out.rows.is_empty())
        .map_err(|e| e.to_string())
}

fn seed_tables(liver: &LiveForever) {
    seed_if_empty(
        liver,
        "simple_table",
        vec![
            vec![
                ("animal".to_string(), Col::Text("cat".to_string())),
                ("color".to_string(), Col::Text("black".to_string())),
            ],
            vec![
                ("animal".to_string(), Col::Text("dog".to_string())),
                ("color".to_string(), Col::Text("brown".to_string())),
            ],
        ],
    );

    seed_if_empty(
        liver,
        "cars",
        vec![
            vec![
                ("make".to_string(), Col::Text("Toyota".to_string())),
                ("year".to_string(), Col::Integer(2020)),
            ],
            vec![
                ("make".to_string(), Col::Text("Honda".to_string())),
                ("year".to_string(), Col::Integer(2018)),
            ],
        ],
    );
}

fn seed_if_empty(liver: &LiveForever, table_name: &str, rows: Vec<Vec<(String, Col)>>) {
    match table_is_empty(liver, table_name) {
        Ok(true) => {}
        Ok(false) => {
            println!("[server] {} already has rows, not seeding", table_name);
            return;
        }
        Err(e) => {
            println!("[server] could not check {} for rows: {}", table_name, e);
            return;
        }
    }

    for row in rows {
        let values = row
            .into_iter()
            .map(|(column_name, value)| ColumnValue { column_name, value })
            .collect();
        match liver.insert_data(InsertDataIn {
            table_name: table_name.to_string(),
            values,
        }) {
            Ok(()) => println!("[server] seeded a row into {}", table_name),
            Err(e) => println!("[server] could not seed {}: {}", table_name, e),
        }
    }
}

async fn handle_socket(mut socket: WebSocket, liver: Arc<Mutex<LiveForever>>) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let WsMessage::Text(text) = msg {
            let reply = match unbuild_msg(&text) {
                Ok((unbuilt, id, _payload)) => match handle_request(unbuilt, id, &liver) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("[server] request handling failed: {}", e);
                        continue;
                    }
                },
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

fn handle_request(
    unbuilt: UnbuiltRequest,
    id: usize,
    liver: &Mutex<LiveForever>,
) -> Result<String, String> {
    match unbuilt {
        UnbuiltRequest::Ping => {
            build_response(id, Request::Ping, ok_serialized()).map_err(|e| e.to_string())
        }
        UnbuiltRequest::GetEntireTable { table_name } => {
            let liver = liver.lock().unwrap();
            let payload: Result<GetEntireTableOut, DbError> = liver
                .get_data(GetDataIn {
                    table_name: table_name.clone(),
                    arguments: SelectArguments::Single(SelectArgument::All),
                    columns_to_read: Vec::new(),
                })
                .map(|out| GetEntireTableOut { rows: out.rows });
            build_response(
                id,
                Request::GetEntireTable { table_name },
                payload.to_payload(),
            )
            .map_err(|e| e.to_string())
        }
    }
}
