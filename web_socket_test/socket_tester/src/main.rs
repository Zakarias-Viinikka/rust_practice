use leptos::{prelude::*, task::spawn_local};
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

            <button on:click=move |_| {
                if let Some(socket_conn) = socket_conn.get() {
                    let result = socket_conn.send_with_str("hi");
                    if let Err(e) = result {
                        error_msg_set.set(e.as_string().unwrap_or_default());
                    }
                }
            } >
                "Say Hi"
            </button>
        </div>
    }
}

/*if {move || socket_conn.get().is_some()} {
    view! {
        if {move || failed_to_establish_conn.get()} {
            view! {
                <h4>"Failed to get establish conn:" <br/></h4>
                <p>{move || error_msg.get()}</p>
            }.into_any()
        } else {
            view! {
                //todo
                // show all the messages received
            }.into_any()
        }
    }.into_any()
} else {
    view! {
        "Getting connection"
    }.into_any()
}*/
