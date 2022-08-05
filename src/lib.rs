#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use dioxus::prelude::*;

// MODULES

pub mod modal;
pub mod progress_bar;

// Useful Constants

pub type Color<'a> = &'a str;

pub const Row: &'static str = "row";
pub const Col: &'static str = "column";

// Colors

pub const Black: &'static str = "0x000";
pub const White: &'static str = "0xFFF";

// Components

#[derive(Props)]
pub struct FlexProps<'a> {
    flex: &'a str,
    children: Element<'a>,
}

pub fn Flex<'a>(cx: Scope<'a, FlexProps<'a>>) -> Element {
    // render! doesnt work here
    cx.render(rsx!(div {
        flex_direction: "{cx.props.flex}",
        display: "flex",
        &cx.props.children
    }))
    // cx should be passed in somehow..., attached somehow I meant
    // maybe another macro? to append a $cx.props.children somewhere in the scope

    // the key problem is {cx.props.flex}
    // let dir = "{cx.props.flex}";
    // that gets expanded by rsx!

    // render!(
    //     cx,
    //     {
    //         flex_direction: "{cx.props.flex}",
    //         display: "flex",
    //         &cx.props.children
    //     }
    // )
}

#[derive(Props)]
pub struct BoxProps<'a> {
    children: Element<'a>,
}

pub fn Box<'a>(cx: Scope<'a, BoxProps<'a>>) -> Element {
    render!(cx, { &cx.props.children })
}

// Animations

// Mostly wrappers around css

// MACROS

#[macro_export]
macro_rules! render {
    ($cx:expr, $input:tt) => {
        $cx.render(rsx!(div $input))
    };
}

// maybe pass the stuff in between {} in input. Match a literal "{" and "}"
// and pass that in
// macro_rules! reorder_body {
//     ($input:tt) => {

//     };
// }

#[macro_export]
macro_rules! render_hyper {
    ($cx:expr, $input:block) => {
        $cx.render(rsx!(div {$input, &$cx.props.children}))
    };
}

#[macro_export]
macro_rules! render_plain {
    ($cx:expr, $outer_component:ident $body:tt) => {
        $cx.render(rsx!($outer_component $body))
    };
}

// maybe make a derive macro that does this, in a macros lib

#[macro_export]
macro_rules! component {
    ($name:ident,$body:vis) => {
        #[derive(Props)]
        pub struct $ident<'a> {
            $body,
            children: Element<'a>,
        }
    };
}

// maybe a derive macro for the actual component itself

#[test]
fn test_macros() {}
