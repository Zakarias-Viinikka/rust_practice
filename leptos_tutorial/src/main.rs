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
            "this is component 1."
            <br /><br />

        </div>
    }
}
#[component]
fn Component2() -> impl IntoView {
    view! {
        <div>
            "this is component 2."
            <br /><br />
        </div>
    }
}

#[component]
fn Page1() -> impl IntoView {
    view! {
        <Component1/>
        <Component2/>
    }
}

#[component]
fn Page2() -> impl IntoView {
    view! {
        <Component3
            comp3_variable ="hello".to_string()
        />
        <Component4
            variable1 = "good".to_string()
            variable2 = "bye".to_string()
        />
    }
}

#[component]
fn Component3(#[prop(optional)] comp3_variable: Option<String>) -> impl IntoView {
    view! {
        <div>
            "this is component 1 for page 2."
            <br />
            //is there a way to avoid clone?
            // ---
            // it's because im passing in a value and not a signal. signals let me reuse without cloning I think?
            "this is the optional value that belongs: {" {comp3_variable.clone()} "}"
            <br />
            {
                if comp3_variable.is_some() {
                    Some(
                        view! {
                            <div>"Text that only shows if the optional value was included."</div>
                        }
                    )
                } else {None}
            }
            <br /><br />

        </div>
    }
}
#[component]
fn Component4(#[prop(optional)] variable1: String, variable2: String) -> impl IntoView {
    view! {
        <div>
            "this is component 2 for page 2."
            <br />
            "this is the optional value that belongs: {" {variable1} "}"
            <br />
            "this is the second value that belongs: {" {variable2} "}"
            <br /><br />
        </div>
    }
}
