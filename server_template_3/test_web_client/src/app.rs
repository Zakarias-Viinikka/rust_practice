use futures::StreamExt;
use futures::channel::mpsc::{UnboundedSender, unbounded};
use leptos::prelude::*;
use leptos::task::spawn_local;
use popup::popup::{PopupContainer, create_popup};
use router_for_responses_coming_to_client::ResponseRouter;
use server_protocol::{
    Base64Bytes, Convert, DbError, GetEntireTableOut, Message, Request, ok_serialized,
};
use server_talker::ServerTalker;

#[derive(Clone, Copy, PartialEq)]
enum ConnectionState {
    NotConnected,
    Loading,
    Connected,
    Failed,
}

#[component]
pub fn App() -> impl IntoView {
    let (state, state_set) = signal(ConnectionState::NotConnected);
    let (response_text, response_text_set) = signal(String::new());
    let (input_text, input_text_set) = signal(String::new());
    let (request_tx, request_tx_set) = signal(None::<UnboundedSender<String>>);

    let connect = Callback::new(move |_: ()| {
        state_set.set(ConnectionState::Loading);

        let ip = include_str!("ip.env").trim();
        let url = format!("ws://{}:3000/ws", ip);

        let handles = match ServerTalker::new(&url) {
            Ok(h) => h,
            Err(e) => {
                something_went_wrong(&format!("could not connect: {}", e));
                state_set.set(ConnectionState::Failed);
                return;
            }
        };

        let router = ResponseRouter::new(handles.outgoing_messages, handles.incoming_responses);
        let mut connection_events_rx = handles.connection_events;

        let (table_tx, mut table_rx) = unbounded::<String>();
        request_tx_set.set(Some(table_tx));

        spawn_local(async move {
            while let Some(event) = connection_events_rx.next().await {
                match event {
                    Ok(()) => state_set.set(ConnectionState::Connected),
                    Err(_) => state_set.set(ConnectionState::Failed),
                }
            }
        });

        spawn_local(async move {
            while let Some(table_name) = table_rx.next().await {
                let reply_rx = router.send_request(|id| Message {
                    message_id: id,
                    request: Request::GetEntireTable {
                        table_name: table_name.clone(),
                    },
                    content: Base64Bytes(ok_serialized()),
                });

                let response = match reply_rx.await {
                    Ok(r) => r,
                    Err(_) => {
                        response_text_set.set("router could not deliver a response".to_string());
                        continue;
                    }
                };

                match Result::<GetEntireTableOut, DbError>::un_payloadify(&response.data.0) {
                    Ok(Ok(out)) => {
                        let text = out
                            .rows
                            .iter()
                            .map(|r| r.to_string_vec().join(" | "))
                            .collect::<Vec<_>>()
                            .join("\n");
                        response_text_set.set(text);
                    }
                    Ok(Err(e)) => response_text_set.set(format!("server error: {}", e)),
                    Err(e) => response_text_set.set(format!("could not decode payload: {}", e)),
                }
            }
        });
    });

    view! {
        <PopupContainerWrapper/>
        {move || match state.get() {
            ConnectionState::NotConnected => view! {
                <button on:click=move |_| connect.run(())>"Connect"</button>
            }.into_any(),
            ConnectionState::Loading => view! { <p>"Connecting..."</p> }.into_any(),
            ConnectionState::Failed => view! {
                <button on:click=move |_| connect.run(())>"Retry"</button>
            }.into_any(),
            ConnectionState::Connected => view! {
                <div style="display: flex; flex-direction: column; align-items: center; min-height: 100vh; padding: 40px;">
                    <div style="display: flex; gap: 8px;">
                        <input
                            type="text"
                            prop:value=move || input_text.get()
                            on:input=move |ev| input_text_set.set(event_target_value(&ev))
                        />
                        <button on:click=move |_| {
                            let val = input_text.get_untracked();
                            if let Some(tx) = request_tx.get_untracked() {
                                let _ = tx.unbounded_send(val);
                            }
                        }>"Send"</button>
                    </div>
                    <div style="margin-top: 20px; width: 600px; min-height: 200px; border: 1px solid gray; padding: 12px; white-space: pre-wrap; font-family: monospace;">
                        {move || response_text.get()}
                    </div>
                </div>
            }.into_any(),
        }}
    }.into_any()
}

#[component]
fn PopupContainerWrapper() -> impl IntoView {
    view! {
        <div style="position: fixed; top: 0; left: 50%; transform: translateX(-50%); z-index: 1000;">
            <PopupContainer/>
        </div>
    }
}

fn something_went_wrong(msg: &str) {
    create_popup(format!("error: {}", msg));
}
