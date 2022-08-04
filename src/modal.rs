use crate::render;
use dioxus::prelude::*;

#[derive(Props)]
pub struct Modal<'a> {
    children: Element<'a>,
}

pub fn Modal<'a>(cx: Scope<'a, Modal<'a>>) -> Element {
    cx.render(rsx!(div {}))
}
