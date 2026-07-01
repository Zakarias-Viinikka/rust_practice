use leptos::{leptos_dom::*, prelude::*, reactive::effect};
use leptos_use::{
    UseUserMediaOptions, UseUserMediaReturn, use_user_media, use_user_media_with_options,
};
use limelight_primitives::{Circle, CircleLayer, Color};

mod draw_to_canvas;
use draw_to_canvas::artist::*;
fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open
    mount_to_body(App);
}

#[component]
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
        let artist = Artist::new(canvas); //::new(canvas_ref);
        /*
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        gl.clear_color(1.0, 0.0, 0.0, 1.0); // red
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let radius = Rc::new(RefCell::new(0.0));
        let radius_clone = radius.clone();
        set_interval(
            move || {
                if *radius_clone.borrow() >= 1.0 {
                    *radius_clone.borrow_mut() = 0.0;
                } else {
                    *radius_clone.borrow_mut() += 0.002;
                }

                let mut renderer = Renderer::new(gl.clone()); // Pass ownership of gl to renderer
                // After this, `gl` is gone. You use `renderer` now.
                //draw_circle(&mut renderer, radius_clone.borrow().clone());
            },
            Duration::from_millis(16),
            //with the std duration thingy i want to say set interval once every 100 ms
        );
        */
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

/*
fn draw_circle(renderer: &mut Renderer, radius: f32) {
    // Create circle data
    let mut circles = CircleLayer::new();
    circles.buffer().set_data(vec![Circle {
        position: [0.0, 0.0],
        radius: radius,
        color: Color::from(palette::named::LIGHTSKYBLUE).opacity(0.4),
        //Color::from<Alpha<Rgb<Srgb, u8>, u8>>(),
    }]);

    // Draw it - render() needs &mut self
    circles.draw(renderer);
}

//rgb::Srgb<u8> -> <limelight_primitives::Color>

fn draw_borderless_circle(renderer: &mut Renderer, radius: f32) {
    let detail = 64u8;
}
*/
