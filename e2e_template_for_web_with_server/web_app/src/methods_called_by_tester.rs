//! Macros that listen for `e2e_*` and `set_port` / `navigate` window
//! events and act on them. Same pattern as `javascript_take_the_wheel!`
//! in the leptos_starter examples, just with fully-qualified paths so
//! several macros can be invoked in the same scope without their `use`
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

/// Listens for `set_port`. The event's `detail` is the port number. Writes
/// it to sessionStorage and flips the connect gate to allowed.
#[macro_export]
macro_rules! tester_can_set_port {
    () => {
        let handle = leptos::prelude::window_event_listener_untyped(
            "set_port",
            move |ev| {
                if let Ok(custom_ev) = wasm_bindgen::JsCast::dyn_into::<web_sys::CustomEvent>(ev) {
                    let detail: wasm_bindgen::JsValue = custom_ev.detail();
                    if let Some(n) = detail.as_f64() {
                        crate::server_port::set_port(n as u16);
                    }
                }
            },
        );

        leptos::prelude::on_cleanup(move || {
            handle.remove();
        });
    };
}

/// Listens for `navigate`. The event's `detail` is a path. Routes there as
/// an SPA change, so no page reload. Must be invoked inside the Router
/// scope; that's why the call site lives in a component below `<Router>`.
#[macro_export]
macro_rules! tester_can_navigate {
    () => {
        let navigate = leptos_router::hooks::use_navigate();
        let handle = leptos::prelude::window_event_listener_untyped(
            "navigate",
            move |ev| {
                if let Ok(custom_ev) = wasm_bindgen::JsCast::dyn_into::<web_sys::CustomEvent>(ev) {
                    let detail: wasm_bindgen::JsValue = custom_ev.detail();
                    if let Some(path) = detail.as_string() {
                        navigate(&path, Default::default());
                    }
                }
            },
        );

        leptos::prelude::on_cleanup(move || {
            handle.remove();
        });
    };
}