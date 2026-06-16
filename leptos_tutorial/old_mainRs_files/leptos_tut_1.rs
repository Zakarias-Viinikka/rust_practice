use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (user_input_get, user_input_set) = signal("0".to_string());
    let (signal2_get, signal2_set) = signal("".to_string());

    view! {

        <br /><br /><br /><br /><br />
        <div id="container">
                <label for="fname">Input here:</label>
                <input type="text"
                on:input:target= move |ev| {
                    user_input_set.set(ev.target().value());
                }
                    value=user_input_get
                    /><br /><br />
                //button here
                <input type="submit" value="Click me"
                    on:click=move |_| {
                        signal2_set.set(user_input_get.get());
                    }
                />
                //button here
                <h4>"Below is where the output gets printed:"</h4>
                <div id="outputDiv">
                    {signal2_get}
                </div>
        </div>
    }
}
