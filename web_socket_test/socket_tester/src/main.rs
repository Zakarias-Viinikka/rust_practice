use leptos::prelude::*;
use shared_types::{data_structures, message_struct};
use socket_tester::{connect_to_socket, receive_message, send_msg::send_message};
use web_sys::WebSocket;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Debug, Clone)]
struct MessagesReceived {
    messages: Vec<String>,
    id: usize,
}
#[component]
fn App() -> impl IntoView {
    let (socket_conn, socket_conn_set) = signal(None::<WebSocket>);
    let failed_to_establish_conn = RwSignal::new(false);
    let (error_msg, error_msg_set) = signal(String::new());

    let (all_messages_received, all_messages_received_set) = signal(Vec::<MessagesReceived>::new());

    let on_message = move |json_response: String| {
        leptos::logging::log!("received message");
        'block: {
            let (response, message_id) = match receive_message::receive_response(&json_response) {
                Ok(data) => data,
                Err(e) => {
                    let error_message = format!("{:?}", e);
                    error_msg_set.set(error_message);
                    break 'block;
                }
            };
            leptos::logging::log!("response: {:?}", response);

            match response {
                receive_message::ExpectedResponse::GetData(get_data_out) => {
                    let rows = get_data_out.rows;
                    let row_strings = rows
                        .iter()
                        .map(|row| format!("{:?}", row.to_string_vec()))
                        .collect();
                    all_messages_received_set.update(|messages| {
                        messages.push(MessagesReceived {
                            messages: row_strings,
                            id: message_id,
                        });
                    });
                }
                _ => (),
            };
        };
    };

    let on_establish_conn_fail = move |err: String| {
        failed_to_establish_conn.set(true);
        error_msg_set.set(err);
    };

    let on_success_socket_connect = move |ws: WebSocket| {
        socket_conn_set.set(Some(ws));
    };

    let result = connect_to_socket(
        on_message,
        on_establish_conn_fail,
        on_success_socket_connect,
    );

    match result {
        Ok(_) => (),
        Err(e) => {
            failed_to_establish_conn.set(true);
            error_msg_set.set(e.to_string());
        }
    }

    let send_message_get_data = move |_| {
        let request = message_struct::Request::GetData;
        let table_name = "socket_testing_table".to_string();
        let arguments = data_structures::SelectArgument::All;
        let columns_to_read = vec![];

        let content = shared_types::data_structures::GetDataIn {
            table_name,
            arguments: vec![arguments],
            columns_to_read,
        };

        if let Err(e) = send_message(socket_conn.get(), request, content, error_msg_set) {
            error_msg_set.set(e);
        }
    };

    view! {
        <div id="container">
            <Show
                when=move || failed_to_establish_conn.get()
                fallback=move|| view! {
                    <Show
                        when=move || socket_conn.get().is_some()
                        fallback=|| view! { "Getting connection" }
                    >
                        "Connected to socket" <br/>
                        <For
                            each=move || all_messages_received.get()
                            key=|msg| msg.id
                            children=move |msg: MessagesReceived| {
                                view! {
                                    "msg id: " {msg.id}
                                    {msg.messages.into_iter().map(|m| view! { <p>{m}</p> }).collect_view()}
                                }
                            }
                        />
                    </Show>
                }
            >
                "Failed to establish connection"
                <br/>
            </Show>
            {move || if !error_msg.get().is_empty() {
                view! {
                    "Error: "
                    <p>{move || error_msg.get()}</p>
                    <br/>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
            <button on:click=send_message_get_data>
                "Say Hi"
            </button>
        </div>
    }
}
