use leptos::prelude::*;
use shared_types::{data_structures, message_struct};
use socket_tester::send_msg::send_message;
use socket_tester::*;
use web_sys::WebSocket;

fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open

    mount_to_body(App);
}

#[derive(Clone)]
struct MessagesReceived {
    message: String,
    id: usize,
}

impl MessagesReceived {
    fn new(id: usize, message: String) -> Self {
        Self { message, id }
    }
}

#[component]
fn App() -> impl IntoView {
    let (socket_conn, socket_conn_set) = signal(None::<WebSocket>);
    let failed_to_establish_conn = RwSignal::new(false);
    let (error_msg, error_msg_set) = signal(String::new());

    let (all_messages_received, all_messages_received_set) = signal(Vec::<MessagesReceived>::new());

    let result = connect(move |text| {
        all_messages_received_set.update(|messages| {
            let id = messages.len();
            messages.push(MessagesReceived::new(id, text));
        });
    });
    match result {
        Ok(ws) => socket_conn_set.set(Some(ws)),
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
                        <For
                            each=move || all_messages_received.get()
                            key=|msg| msg.id
                            children=move |msg: MessagesReceived| {
                                view! { <p>{msg.message}</p> }
                            }
                        />
                    </Show>
                }
            >
                "Failed to establish connection"
                <br/>
                <p>{move || error_msg.get()}</p>
            </Show>

            <button on:click=send_message_get_data>
                "Say Hi"
            </button>
        </div>
    }
}
