use crate::server_talker::ServerTalker;
use crate::{tester_can_clear_message, tester_can_set_message};
use leptos::prelude::*;

fn server_websocket_url() -> String {
    let port = read_server_port_from_url().unwrap_or(3000);
    format!("ws://127.0.0.1:{}/ws", port)
}

fn read_server_port_from_url() -> Option<u16> {
    let window = web_sys::window()?;
    let search = window.location().search().ok()?;
    let trimmed = search.strip_prefix('?')?;
    for pair in trimmed.split('&') {
        let (key, value) = pair.split_once('=')?;
        if key == "server_port" {
            return value.parse().ok();
        }
    }
    None
}

#[component]
pub fn AppMain() -> impl IntoView {
    let talker = ServerTalker::connect(&server_websocket_url());
    let connected = talker.connected;
    let last_reply = talker.last_reply;

    let (tester_message, tester_message_set) = signal(String::new());
    tester_can_set_message!(tester_message_set);
    tester_can_clear_message!(tester_message_set);

    let send_hello = move |_| talker.send("hello");

    view! {
        <div class="container">
            <h1>"Step 1"</h1>
            <p data-testid="connected">
                {move || if connected.get() { "connected" } else { "connecting..." }}
            </p>
            <button
                data-testid="send-hello"
                disabled=move || !connected.get()
                on:click=send_hello
            >
                "Send hello"
            </button>
            <p data-testid="reply">
                {move || format!("(servers response) {}", last_reply.get())}
            </p>
            <p data-testid="tester-message">{move || tester_message.get()}</p>
        </div>
    }
}