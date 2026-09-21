use futures::StreamExt;
use leptos::prelude::*;
use leptos::task::spawn_local;
use popup::popup::{PopupContainer, create_popup};
use router_for_responses_coming_to_client::ResponseRouter;
use server_protocol::{Base64Bytes, Convert, DbError, Message, Request, ok_serialized};
use server_talker::ServerTalker;

#[derive(Clone, Copy, PartialEq)]
enum ConnectionState {
    Loading,
    Connected,
    Failed,
}

#[component]
pub fn App() -> impl IntoView {
    let state = RwSignal::new(ConnectionState::Loading);

    let Ok(router) = setup_connection(state) else {
        return connection_failed_view().into_any();
    };

    view! {
        <PopupContainerWrapper/>
        {move || match state.get() {
            ConnectionState::Loading => view! { <p>"Connecting..."</p> }.into_any(),
            ConnectionState::Failed => view! { <p>"Failed to connect"</p> }.into_any(),
            ConnectionState::Connected => connected_view(router.clone()),
        }}
    }
    .into_any()
}

fn setup_connection(state: RwSignal<ConnectionState>) -> Result<ResponseRouter, String> {
    let ip = include_str!("ip.env").trim();
    let url = format!("ws://{}:3000/ws", ip);

    let handles = match ServerTalker::new(&url) {
        Ok(h) => h,
        Err(e) => {
            something_went_wrong(&format!("could not connect to Server Talker: {}", e));
            state.set(ConnectionState::Failed);
            return Err(e);
        }
    };

    let router = ResponseRouter::new(handles.outgoing_messages, handles.incoming_responses);
    let mut connection_events_rx = handles.connection_events;

    spawn_local(async move {
        while let Some(event) = connection_events_rx.next().await {
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

    Ok(router)
}

fn connected_view(router: ResponseRouter) -> AnyView {
    view! {
        <button on:click=move |_| {
            let router = router.clone();
            spawn_local(async move {
                send_ping_and_show_reply(router).await;
            });
        }>"Send ping"</button>
    }
    .into_any()
}

async fn send_ping_and_show_reply(router: ResponseRouter) {
    let reply_rx = router.send_request(|id| Message {
        message_id: id,
        request: Request::Ping,
        content: Base64Bytes(ok_serialized()),
    });

    let Ok(response) = reply_rx.await else {
        something_went_wrong("the router could not deliver a response");
        return;
    };

    match Result::<(), DbError>::un_payloadify(&response.data.0) {
        Ok(Ok(())) => create_popup(format!("ping ok (id {})", response.message_id)),
        Ok(Err(server_err)) => create_popup(format!("server error: {}", server_err)),
        Err(decode_err) => create_popup(format!("could not decode payload: {}", decode_err)),
    }
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
