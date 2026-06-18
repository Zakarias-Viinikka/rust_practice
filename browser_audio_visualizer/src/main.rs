#![allow(warnings)]
use leptos::{leptos_dom::*, prelude::*, reactive::effect};
use leptos_use::use_permission;
use wasm_bindgen::JsCast;
use web_sys::{PermissionState, PermissionStatus, window};

fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (mic_status, mic_status_set) = signal("unknown".to_string());
    let microphone_access = use_permission("microphone");

    Effect::new(move |_| {
        // immediately prints "Value: 0" and subscribes to `a`
        let _mic_status = mic_status.get();
        //dbg!(_mic_status);
        console_debug_log("Microphone permission: {:?}", _mic_status);
        //mic_status_set(microphone_access.status().get().as_str());
    });

    view! {
        <br/>        <br/>        <br/>
        <div>
            "Hello World"
        </div>
    }
}
