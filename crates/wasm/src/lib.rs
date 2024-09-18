use dioxus::prelude::*;

#[derive(Props)]
struct ButtonProps<'a> {
    onclick: EventHandler<'a, ()>,
    children: Element<'a>,
}

fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    cx.render(rsx! {
        button {
            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
            onclick: move |_| cx.props.onclick.call(()),
            &cx.props.children
        }
    })
}

#[derive(Props)]
struct CounterProps {
    initial: i32,
}

fn Counter(cx: Scope<CounterProps>) -> Element {
    let count = use_state(&cx, || cx.props.initial);

    cx.render(rsx! {
        div {
            p { "Count: {count}" }
            Button {
                onclick: move |_| count.set(*count + 1),
                "Increment"
            }
        }
    })
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn render() -> String {
    let mut app = VirtualDom::new(Counter);
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}