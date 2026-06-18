use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Page1() -> impl IntoView {
    view! {
        <br /><br /><br /><br /><br />
        <div id="container">
            "Page 1"
        </div>
        <br/>

        <A href="/page2">"Go to Page 2"</A>
    }
}
