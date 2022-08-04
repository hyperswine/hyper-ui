use dioxus::{events::onclick, prelude::*};
use hyper_ui::{Flex, FlexDir};

fn main() {
    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_window(|w| w.with_title("HyperUI Demo").with_decorations(false))
    });
}

fn app(cx: Scope) -> Element {
    let window = dioxus::desktop::use_window(&cx);
    let fullscreen = use_state(&cx, || false);
    let always_on_top = use_state(&cx, || false);
    let decorations = use_state(&cx, || true);

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
            flex: FlexDir::Row,
            "This is a flex container"
        }
    ))
}
