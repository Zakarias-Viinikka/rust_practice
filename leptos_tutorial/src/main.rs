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
    let (user_input, do_something_with_user_input) = signal(0);

    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element.get().expect("").value();
        let value = value.parse::<i32>().unwrap();
        do_something_with_user_input.set(value);
    };
    view! {

        <br /><br /><br /><br /><br />
        <div id="container">
            <form on:submit=on_submit/>
            <label for="fname">Input here:</label>
            <input type="text" id="input" name="user_input" value=user_input node_ref=input_element/><br /><br />
            //button here
            <input type="submit" value="Click me"/>
            //button here
            <h4>"Below is where the output gets printed:"</h4>
            <div id="outputDiv">
                {move || user_input.get()}
            </div>
        </div>

        /*
        #[component]
        fn UncontrolledInput() -> impl IntoView {
            use leptos::html::Input;

            let (uncontrolled, set_uncontrolled) = create_signal("".to_string());
            let input_element: NodeRef<Input> = create_node_ref();
            let on_submit = |ev: SubmitEvent| {
                ev.prevent_default();

                let value = input_element()
                    .expect("<input> doesn't exist!")
                    .value();
                println!("value of input is: {}", value);
                set_uncontrolled(value);
            };
            view! {
                <form on:submit=on_submit>
                    <input type="text"
                        value=uncontrolled
                        node_ref=input_element/>
                    <input type="submit" value="Submit"/>
                </form>
                <p>"Name is: "{uncontrolled}</p>
            }
        }
         */
    }
}
