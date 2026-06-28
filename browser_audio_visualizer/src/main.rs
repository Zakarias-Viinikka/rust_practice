#![allow(warnings)]
use leptos::{leptos_dom::*, prelude::*, reactive::effect};
use leptos_router::components::{Route, Router, Routes}; // routing
use leptos_use::{
    UseUserMediaOptions, UseUserMediaReturn, use_user_media, use_user_media_with_options,
};
use limelight::{DrawMode, Renderer}; //also for drawing the circle
use limelight_primitives::{Circle, CircleLayer};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, PermissionState, PermissionStatus, WebGl2RenderingContext,
    js_sys::Intl::{DurationFormatPartType::Milliseconds, RelativeTimeFormatUnit::Seconds},
    window,
}; //drawing a circle
fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open
    mount_to_body(App);
}

#[component]

/*
* Routing syntax
*
*
<Router>
  <nav>
    /* ... */
  </nav>
  <main>
      <Routes fallback=|| "Not found.">
          <Route path=path!("/") view=DefaultPage/>
          <Route path=path!("/page1") view=Page1/>        //<- both work
          <Route path=path!("/page2") view=page2::Page2/> //<- both work
      </Routes>
  </main>
</Router>


#[component]
fn DefaultPage() -> impl IntoView {
    view! {
        <br/>   <br/>   <br/>
        <h2> "Default Page" </h2>
        <A href="/page1">"Page 1"</A>
        <br/>
        <A href="/page2">"Page 2"</A>
    }
}

*/
fn App() -> impl IntoView {
    let options = UseUserMediaOptions::default().audio(true).video(false);
    let audio_ref = NodeRef::<leptos::html::Audio>::new();
    let UseUserMediaReturn { stream, start, .. } = use_user_media_with_options(options);

    //start();

    Effect::new(move |_| {
        audio_ref.get().map(|v| match stream.get() {
            Some(Ok(s)) => v.set_src_object(Some(&s)),
            Some(Err(e)) => error!("Failed to get media stream: {:?}", e),
            None => log!("No stream yet"),
        })
    });

    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    canvas_ref.on_load(|canvas| {
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        gl.clear_color(1.0, 0.0, 0.0, 1.0); // red
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let grader = Rc::new(RefCell::new(0));
        let grader_clone = grader.clone();
        set_interval(
            move || {
                *grader_clone.borrow_mut() += 1;

                let renderer = Renderer::new(gl); // Pass ownership of gl to renderer
                // After this, `gl` is gone. You use `renderer` now.
                draw_circle(&renderer);
            },
            Duration::from_millis(100),
            //with the std duration thingy i want to say set interval once every 100 ms
        );
    });

    view! {
        <br/>        <br/>        <br/>
        <div>
            <canvas
                node_ref=canvas_ref
                width="512"
                height="512"
                style="border: 1px solid black;"
            ></canvas>
        </div>
        <br/>        <br/>        <br/>

        <button on:click=move |_| start()>
            "Enable Microphone"
        </button>

        <br/>        <br/>        <br/>
        "This is where the audo element is supposed to be"
        <audio node_ref=audio_ref controls=false autoplay=true muted=true></audio>
    }
}

fn draw_circle(renderer: &mut Renderer) {
    // Create circle data
    let mut circles = CircleLayer::new();
    circles.buffer().set_data(vec![Circle {
        position: [0.0, 0.0],
        radius: 0.5,
        color: [1.0, 0.0, 0.0, 1.0].into(),
    }]);

    // Draw it - render() needs &mut self
    renderer.render(&mut circles, circles.buffer()).unwrap();
}
