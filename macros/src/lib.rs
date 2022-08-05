extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;

// derive macro for component prop

#[proc_macro_derive(HyperProps)]
pub fn hyper_props(body: TokenStream) -> TokenStream {
    body
}

// the first one is always going to be ident = fn I think?
// the second one might be the params. But there should be no params I think
// well actually.. yea the params are all contained in cx and props

// macro for functional component
#[proc_macro_attribute]
pub fn hyper_component_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    let x = &item;

    // for tt in &item.into_iter() {
    // maybe try to match the ident = "render"?
    // which will be a used keyword

    // if let proc_macro::TokenTree::Ident(ident) = tt {
    // if ident.to_string() == "render" {
    // create structs

    // }
    // }
    // }

    let x = item.into_iter().last().unwrap();


    quote!(
        fn #ident<'component>(cx: Scope<'component, (#ident)Props>) -> Element {

            cx.render(rsx!{
                div {
                    #body
                }
            })
        }
    )
    .into()
}

// function component, simply create your function name like Name
// and your pair struct Name, which becomes NameProps automatically
// in your body, use the render!() macro to return cx.render(rsx!($body))
// oh I think in the macro you need dioxus_elements::div

// I dont think you can export normal macros

/*
#[HyperComponent]
or
#[hyper_component]
fn Name() -> {
    state_and_other_functions();

    render!(
        {

        }
    )
}

component!(
    Name,
    {
        state_and_other_setup();
    },
    {
        h1{

        }
        cx.props.children
    }
)

=> {
    fn Name<'a>(cx: Scope<'a>) -> Element {
        state_and_other_setup();

        cx.render(rsx!({
            div {
                h1{

            }
            cx.props.children
                }
        }))
    }

}

*/

/*
proc macro attr

match "render" (
    stuff,
    stuff2,
    "HI",
    &cx.props.children
)

take everything inside "render" block
and call that "block"
then move it to div {}

#[component]
fn Name() {
    do_stuff()

    (
        div {
            stuff*
        }
    )
}

Also need companion Prop "NameProps"
*/

#[proc_macro]
pub fn hyper_component(body: TokenStream) -> TokenStream {
    for tt in body.into_iter() {
        match tt {
            proc_macro::TokenTree::Group(g) => todo!(),
            proc_macro::TokenTree::Ident(ident) => todo!(),
            proc_macro::TokenTree::Punct(p) => todo!(),
            proc_macro::TokenTree::Literal(l) => todo!(),
        }
    }

    TokenStream::new()
}

#[test]
fn test_proc_attr() {}
