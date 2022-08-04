use crate::{render, render_plain};
use dioxus::prelude::{dioxus_elements::div, *};

#[derive(Props)]
pub struct Modal<'a> {
    children: Element<'a>,
}

/// The modal
pub fn Modal<'a>(cx: Scope<'a, Modal<'a>>) -> Element {
    // cx.render(rsx!(div {}))
    // render_plain!(cx, div {"Hi"})
    render!(cx, { "Hi" })
}

// maybe for now, always render a div?
