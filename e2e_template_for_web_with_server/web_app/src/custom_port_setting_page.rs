use leptos::prelude::*;

/// Marker route. The tester lands here, fires the `set_port` and
/// `navigate` events, and the app moves on. Doesn't redirect on its own.
#[component]
pub fn CustomPortSettingPage() -> impl IntoView {
    view! {
        <div data-testid="custom-port-setting-page">"custom port setting page"</div>
    }
}
