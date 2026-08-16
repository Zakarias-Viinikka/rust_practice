use leptos::logging::log;
use leptos::prelude::*;
use not_leptos_parts_of_code::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn send_data_to_js(data_in_transport_mode: &[u8]);
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let data = Data::Data1In {
        name: "John".to_string(),
    };

    javascript_take_the_wheel!("data_back", |js_value| {
        log!("received data from js");
        let data_in_transport_mode = js_sys::Uint8Array::from(js_value).to_vec();
        match Data::deserialize(&data_in_transport_mode) {
            Ok(data) => log!("data came back: {:?}", data),
            Err(e) => log!("failed to deserialize: {:?}", e),
        }
    });

    Effect::new(move || match data.serialize() {
        Ok(data_in_transport_mode) => {
            log!("sending data to js");
            send_data_to_js(&data_in_transport_mode);
        }
        Err(e) => log!("serialize failed: {:?}", e),
    });

    view! { "" }
}

#[macro_export]
macro_rules! javascript_take_the_wheel {
    ($name:expr, |$payload:ident| $callback:expr) => {
        let handle = window_event_listener_untyped($name, move |ev| {
            if let Ok(custom_ev) = ev.dyn_into::<web_sys::CustomEvent>() {
                let $payload: JsValue = custom_ev.detail();
                $callback
            }
        });

        on_cleanup(move || {
            handle.remove();
        });
    };
}
