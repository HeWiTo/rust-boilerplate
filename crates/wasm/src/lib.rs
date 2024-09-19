use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    dioxus_web::launch(app);
    Ok(())
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold mb-4", "Rust SaaS Boilerplate" }
            p { class: "mb-4", "Welcome to your Rust-powered SaaS application!" }
            button { 
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| {
                    println!("Button clicked!");
                },
                "Click me!"
            }
        }
    })
}