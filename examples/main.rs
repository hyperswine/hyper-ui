use dioxus::{events::onclick, prelude::*};
use hyper_ui::{Col, Flex, Row};

fn main() {
    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_window(|w| w.with_title("HyperUI Demo").with_decorations(false))
    });
}

fn app(cx: Scope) -> Element {
    let window = dioxus::desktop::use_window(&cx);
    let fullscreen = use_state(&cx, || false);

    window.set_resizable(true);
    // window.set_title("Hyper UI");

    cx.render(rsx!(
        div {
            onmousedown: move |_| window.drag(),
            height: "100%",
            width: "100%",
            "Header",
        }
        div {
            "This is Hyper UI"
        }
        Flex {
            flex: Row,
            div {
                "This is flex container 2",
            }
            div {
                "This should be in a col"
            }
        }
    ))
}
