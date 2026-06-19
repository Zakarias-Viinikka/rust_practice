#![allow(warnings)]
//use leptos::ev::SubmitEvent;
//use leptos::html;
use leptos::logging::*;
use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::*; // <- my code didn't work without this. the path! macro specifically
use leptos_use::*;
use std::time::Duration;
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
                <Route path=path!("/") view=SyncAnimation/>
            </Routes>
        </main>
      </Router>
    }
}

#[component]
fn SyncAnimation() -> impl IntoView {
    let letters: [&str; 7] = ["S", "y", "n", "c", "i", "n", "g"];
    let (syncing_text, syncing_text_set) = signal(letters);
    let (letter1, letter1_set) = signal(0u8);
    let (letter2, letter2_set) = signal(0u8);
    /*
    let animation_letters = AL::new(letter1.get(), letter2.get());
     *
     */
    let (animation_state, animation_state_set) =
        signal(AnimationState::FadingOutLetter(AL::new(0u8, 0u8)));

    let loopetyWoopety = set_interval(
        || {
            update_animation(UpdateAnimationsParams::new());
        },
        Duration::from_millis(100),
    );
    view! {
        <div id="container">
        <ForEnumerate
            each=move || syncing_text.get()
            key= |letter| *letter
            children=move |index, letter| {
                let mut n_counter = 0u8;
                //here
                view! {
                    <div id=get_letter_id(letter, &mut  n_counter) class="letterInDiv">
                        {letter}
                    </div>
                }
            }
        />
        <br/>
        </div>
    }
}

fn get_letter_id(letter: &str, n_counter: &mut u8) -> String {
    log!("letter: {}", letter);
    log!("n_counter: {}", n_counter);
    if letter == "n" {
        log!("we're inside");
        if n_counter != &0u8 {
            *n_counter = 1u8;
            return "letter-n1".to_string();
        } else {
            *n_counter = 0u8;
            return "letter-n0".to_string();
        }
    }
    "letter-".to_string() + letter
}

enum AnimationState {
    FadingOutLetter(AnimationLetters),
    ShowingRegret(AnimationLetters),
    Resetting(AnimationLetters),
}

struct AnimationLetters {
    letters: [u8; 2],
}

impl AnimationLetters {
    fn new(letter1: u8, letter2: u8) -> Self {
        Self {
            letters: [letter1, letter2],
        }
    }
}

use AnimationLetters as AL;

fn update_animation(ctx: UpdateAnimationsParams) {
    log!("Hello");
}

struct UpdateAnimationsParams {}

impl UpdateAnimationsParams {
    fn new() -> Self {
        Self {}
    }
}
