use dioxus::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum FlexDir {
    Row,
    Column,
}

#[derive(Props)]
pub struct FlexProps<'a> {
    flex: FlexDir,
    children: Element<'a>
}

pub fn Flex<'a>(cx: Scope<'a, FlexProps<'a>>) -> Element {
    cx.render(rsx!(div {
        flex_direction: "column",
        padding: "2",
        &cx.props.children
    }))
}
