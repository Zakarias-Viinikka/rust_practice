use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

mod app_main;
mod connecting_gate;
mod custom_port_setting_page;
mod methods_called_by_tester;
mod server_port;
mod server_talker;

use connecting_gate::{ConnectingGate, CONNECTING_GATE};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    crate::tester_can_set_port!();

    view! {
        <Router>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=AppMainGate/>
                    <Route
                        path=path!("/custom_port_setting_page")
                        view=custom_port_setting_page::CustomPortSettingPage
                    />
                </Routes>
                <MainForNavigatingFromTheTester/>
            </main>
        </Router>
    }
}

/// Mounts `AppMain` only once the connect gate is allowed. `AppMain`'s
/// body reads the port and opens the websocket at mount, so gating here
/// is what keeps the port and the connect in the right order.
#[component]
fn AppMainGate() -> impl IntoView {
    view! {
        {move || {
            if CONNECTING_GATE.get() == ConnectingGate::AllowedToConnect {
                view! { <app_main::AppMain/> }.into_any()
            } else {
                view! { <div data-testid="waiting-for-port">"waiting for port"</div> }
                    .into_any()
            }
        }}
    }
}

#[component]
fn MainForNavigatingFromTheTester() -> impl IntoView {
    crate::tester_can_navigate!();
    ()
}