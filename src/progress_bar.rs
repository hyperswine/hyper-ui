use crate::Color;
use dioxus::prelude::*;

#[derive(Props)]
pub struct CircleProgress<'a> {
    color: Color<'a>,
}

// Use some css for this

pub fn CircleProgress<'a>(cx: Scope<'a, CircleProgress<'a>>) -> Element {
    cx.render(rsx!(div {
        color: "{cx.props.color}"
    }))
}
