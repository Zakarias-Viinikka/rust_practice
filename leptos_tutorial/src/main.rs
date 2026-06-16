use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;
use leptos::tachys::view;

fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {

        <br /><br /><br /><br /><br />
        <div id="container">
            <Page1/>
            <Page2/>
        </div>
    }
}

#[component]
fn Component1() -> impl IntoView {
    view! {
        <div>
            "this is component 1."<br /><br />

        </div>
    }
}
#[component]
fn Component2() -> impl IntoView {
    view! {
        <div>
            "this is component 2."<br /><br />
        </div>
    }
}

fn Page1() -> impl IntoView {
    view! {
        <Component1/>
        <Component2/>
    }
}

fn Page2() -> impl IntoView {
    view! {
        <Component3/>
        <Component4/>
    }
}

#[component]
fn Component3() -> impl IntoView {
    view! {
        <div>
            "this is component 1 for page 2."<br /><br />

        </div>
    }
}
#[component]
fn Component4() -> impl IntoView {
    view! {
        <div>
            "this is component 2 for page 2."<br /><br />
        </div>
    }
}
