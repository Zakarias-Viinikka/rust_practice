#![allow(warnings)]
use leptos::{leptos_dom::*, prelude::*, reactive::effect};
use leptos_use::{
    UseUserMediaOptions, UseUserMediaReturn, use_user_media, use_user_media_with_options,
};
use wasm_bindgen::JsCast;
use web_sys::{PermissionState, PermissionStatus, window};

fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let options = UseUserMediaOptions::default().audio(true).video(false);
    let audio_ref = NodeRef::<leptos::html::Audio>::new();
    let UseUserMediaReturn { stream, start, .. } = use_user_media_with_options(options);

    //start();

    Effect::new(move |_| {
        audio_ref.get().map(|v| match stream.get() {
            Some(Ok(s)) => v.set_src_object(Some(&s)),
            Some(Err(e)) => error!("Failed to get media stream: {:?}", e),
            None => log!("No stream yet"),
        })
    });

    /*
        Effect::new(move |_| {
            // immediately prints "Value: 0" and subscribes to `a`
            let _mic_status = mic_status.get();
            //dbg!(_mic_status);

            logging::console_debug_error(&format!("Microphone permission: {:?}", _mic_status));
            //mic_status_set(microphone_access.status().get().as_str());
        });
    */
    view! {
        <br/>        <br/>        <br/>
        <div>
            "Hello World"
        </div>
        <br/>        <br/>        <br/>

        <button on:click=move |_| start()>
            "Enable Microphone"
        </button>

        <br/>        <br/>        <br/>
        "This is where the audo element is supposed to be"
        <audio node_ref=audio_ref controls=false autoplay=true muted=true></audio>
    }
}
