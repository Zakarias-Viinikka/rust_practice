//use leptos::ev::SubmitEvent;
//use leptos::html;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*; // <- my code didn't work without this. the path! macro specifically

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
            //I don't understand what <A> is about? my code didn't work when I did <A href... instead of <a href...
            //
            // From the book:
            // <A> Correctly resolves relative nested routes. Relative routing with ordinary <a> tags can be tricky. For example, if you have a route like /post/:id, <A href="1"> will generate the correct relative route, but <a href="1"> likely will not (depending on where it appears in your view.) <A/> resolves routes relative to the path of the nested route within which it appears.
            <a href="page1">"Page 1"</a>
            <br/>
            <a href="page2">"Page 2"</a>
        </div>
    }
}

#[component]
fn Page1() -> impl IntoView {
    view! {
        <br /><br /><br /><br /><br />
        <div id="container">
            "Page 1"
        </div>
    }
}

#[component]
fn Page2() -> impl IntoView {
    view! {
        <br /><br /><br /><br /><br />
        <div id="container">
            "Page 2"
        </div>
    }
}
