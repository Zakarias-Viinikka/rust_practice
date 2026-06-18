use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Page2() -> impl IntoView {
    view! {
        <br /><br /><br /><br /><br />
        <div id="container">
            "Page 2"
        </div>
        <br/>

        <A href="/page1">"Go to Page 1"</A>
    }
}
