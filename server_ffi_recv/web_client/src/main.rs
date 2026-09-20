use leptos::prelude::*;
use popup::popup::{PopupContainer, create_popup};
use server_protocol::{Request, UnbuiltResponse};
use web_client::{connect_to_socket, receive_message, send_msg::send_message};
use web_sys::WebSocket;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (socket_conn, socket_conn_set) = signal(None::<WebSocket>);
    let failed_to_establish_conn = RwSignal::new(false);
    let (error_msg, error_msg_set) = signal(String::new());

    let (pings_received, pings_received_set) = signal(Vec::<usize>::new());

    let on_message = move |json_response: String| {
        leptos::logging::log!("received message");
        match receive_message::receive_response(&json_response) {
            Ok((UnbuiltResponse::Ping, message_id)) => {
                pings_received_set.update(|v| v.push(message_id));
            }
            Err(e) => {
                error_msg_set.set(format!("{:?}", e));
            }
        }
    };

    let on_establish_conn_fail = move |err: String| {
        failed_to_establish_conn.set(true);
        error_msg_set.set(err);
        create_popup("connection failed".to_string());
    };

    let on_success_socket_connect = move |ws: WebSocket| {
        socket_conn_set.set(Some(ws));
        create_popup("connected".to_string());
    };

    let on_close = move || {
        create_popup("disconnected".to_string());
    };

    if let Err(e) = connect_to_socket(
        on_message,
        on_establish_conn_fail,
        on_success_socket_connect,
        on_close,
    ) {
        failed_to_establish_conn.set(true);
        error_msg_set.set(e.to_string());
    }

    let send_ping = move |_| {
        if let Err(e) = send_message(socket_conn.get(), Request::Ping, (), error_msg_set) {
            error_msg_set.set(e);
        }
        create_popup("ping sent".to_string());
    };

    view! {
        <div style="position: fixed; top: 0; left: 50%; transform: translateX(-50%); z-index: 1000;">
            <PopupContainer/>
        </div>
        <div id="container">
            <Show
                when=move || failed_to_establish_conn.get()
                fallback=move || view! {
                    <Show
                        when=move || socket_conn.get().is_some()
                        fallback=|| view! { "Getting connection" }
                    >
                        "Connected to socket" <br/>
                        <For
                            each=move || pings_received.get()
                            key=|id| *id
                            children=move |id: usize| {
                                view! { <p>"pong for msg id: " {id}</p> }
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
            <button on:click=send_ping>
                "Ping"
            </button>
        </div>
    }
}
