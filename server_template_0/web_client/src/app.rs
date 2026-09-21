use futures::StreamExt;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};
use leptos::prelude::*;
use leptos::task::spawn_local;
use popup::popup::{PopupContainer, create_popup};
use server_talker::InbetweenMan;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, PartialEq)]
enum ConnectionState {
    Loading,
    Connected,
    Failed,
}

#[component]
pub fn App() -> impl IntoView {
    let state = RwSignal::new(ConnectionState::Loading);

    let Ok((man_tx, man_rx)) = setup_connection(state) else {
        return connection_failed_view().into_any();
    };

    spawn_response_matcher(man_rx);

    view! {
        <PopupContainerWrapper/>
        {move || match state.get() {
            ConnectionState::Loading => view! { <p>"Connecting..."</p> }.into_any(),
            ConnectionState::Failed => view! { <p>"Failed to connect"</p> }.into_any(),
            ConnectionState::Connected => connected_view(man_tx.clone()),
        }}
    }
    .into_any()
}

fn setup_connection(
    state: RwSignal<ConnectionState>,
) -> Result<
    (
        UnboundedSender<(u64, String)>,
        UnboundedReceiver<(u64, String)>,
    ),
    String,
> {
    let ip = include_str!("ip.env").trim();
    let url = format!("ws://{}:3000/ws", ip);

    let Ok((mut connect_rx, man_tx, man_rx)) = InbetweenMan::new(&url) else {
        something_went_wrong("could not connect to inbetween man");
        state.set(ConnectionState::Failed);
        return Err("could not connect".to_string());
    };

    spawn_local(async move {
        while let Some(event) = connect_rx.next().await {
            match event {
                Ok(()) => {
                    create_popup("connected".to_string());
                    state.set(ConnectionState::Connected);
                }
                Err(reason) => {
                    create_popup(reason);
                    state.set(ConnectionState::Failed);
                }
            }
        }
    });

    Ok((man_tx, man_rx))
}

fn spawn_response_matcher(mut man_rx: UnboundedReceiver<(u64, String)>) {
    let pending: Arc<Mutex<HashMap<u64, UnboundedSender<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let pending_for_button = pending.clone();

    // save the map somewhere the button can reach it
    provide_context(pending_for_button);

    spawn_local(async move {
        while let Some((id, payload)) = man_rx.next().await {
            let sender = pending.lock().unwrap().get(&id).cloned();
            if let Some(sender) = sender {
                let _ = sender.unbounded_send(payload);
            }
        }
    });
}

fn connected_view(man_tx: UnboundedSender<(u64, String)>) -> AnyView {
    view! {
        <button on:click=move |_| {
            let man_tx = man_tx.clone();
            spawn_local(async move {
                send_hello_and_show_replies(man_tx).await;
            });
        }>"Send hello"</button>
    }
    .into_any()
}

async fn send_hello_and_show_replies(man_tx: UnboundedSender<(u64, String)>) {
    let (reply_tx, mut reply_rx) = unbounded::<String>();

    let id = next_request_id();
    let pending = use_context::<Arc<Mutex<HashMap<u64, UnboundedSender<String>>>>>()
        .expect("pending map not provided");
    pending.lock().unwrap().insert(id, reply_tx);

    if man_tx.unbounded_send((id, "hello".to_string())).is_err() {
        something_went_wrong("could not send request to inbetween man");
        return;
    }

    while let Some(reply) = reply_rx.next().await {
        create_popup(format!("got: {}", reply));
    }
}

fn next_request_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

#[component]
fn PopupContainerWrapper() -> impl IntoView {
    view! {
        <div style="position: fixed; top: 0; left: 50%; transform: translateX(-50%); z-index: 1000;">
            <PopupContainer/>
        </div>
    }
}

fn connection_failed_view() -> impl IntoView {
    view! {
        <PopupContainerWrapper/>
        <p>"Failed to connect to server"</p>
    }
}

fn something_went_wrong(msg: &str) {
    create_popup(format!("error: {}", msg));
}
