//use leptos::ev::SubmitEvent;
//use leptos::html;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*; // <- my code didn't work without this. the path! macro specifically
use leptos_tutorial::{page1::*, page2::*};
/*
use leptos_tutorial::routing_test;
*/

fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
            // all our routes will appear inside <main>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=DefaultPage/>
                <Route path=path!("/page1") view=Page1/>
                <Route path=path!("/page2") view=Page2/>
            </Routes>
        </main>
      </Router>
    }
}

#[component]
fn DefaultPage() -> impl IntoView {
    view! {
        <br /><br /><br /><br /><br />
        <div id="container">
            "Default Page"
            <br/>
            <A href="page1">"Page 1"</A>
            <br/>
            <A href="page2">"Page 2"</A>
        </div>
    }
}
