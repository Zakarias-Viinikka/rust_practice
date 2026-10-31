//! Macros that listen for `e2e_*` window events and write to a Leptos
//! signal the view is watching. Same pattern as `javascript_take_the_wheel!`
//! in the leptos_starter examples, just with fully-qualified paths so
//! two macros can be invoked in the same scope without their `use`
//! statements colliding.

#[macro_export]
macro_rules! tester_can_set_message {
    ($setter:ident) => {
        let handle = leptos::prelude::window_event_listener_untyped(
            "e2e_set_message",
            move |ev| {
                if let Ok(custom_ev) = wasm_bindgen::JsCast::dyn_into::<web_sys::CustomEvent>(ev) {
                    let detail: wasm_bindgen::JsValue = custom_ev.detail();
                    if let Some(text) = detail.as_string() {
                        $setter.set(text);
                    }
                }
            },
        );

        leptos::prelude::on_cleanup(move || {
            handle.remove();
        });
    };
}

#[macro_export]
macro_rules! tester_can_clear_message {
    ($setter:ident) => {
        let handle = leptos::prelude::window_event_listener_untyped(
            "e2e_clear_message",
            move |_| {
                $setter.set(String::new());
            },
        );

        leptos::prelude::on_cleanup(move || {
            handle.remove();
        });
    };
}