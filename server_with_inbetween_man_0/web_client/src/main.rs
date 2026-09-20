use futures::StreamExt;
use inbetween_man::InbetweenMan;
use leptos::prelude::*;
use leptos::task::spawn_local;
use popup::popup::{PopupContainer, create_popup};
use std::rc::Rc;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ip = include_str!("ip.env").trim();
    let url = format!("ws://{}:3000/ws", ip);

    let man = Rc::new(InbetweenMan::connect(&url).expect("failed to connect"));

    let send = {
        let man = man.clone();
        move |_| {
            let man = man.clone();
            spawn_local(async move {
                match man.send("hello") {
                    Ok(mut rx) => {
                        while let Some(reply) = rx.next().await {
                            create_popup(format!("got: {}", reply));
                        }
                    }
                    Err(e) => {
                        create_popup(format!("send failed: {}", e));
                    }
                }
            });
        }
    };

    view! {
        <div style="position: fixed; top: 0; left: 50%; transform: translateX(-50%); z-index: 1000;">
            <PopupContainer/>
        </div>
        <button on:click=send>"Send hello"</button>
    }
}
