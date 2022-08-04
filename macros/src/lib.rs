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

// macro for functional component
#[proc_macro_attribute]
pub fn hyper_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    attr
}

// function component, simply create your function name like Name
// and your pair struct Name, which becomes NameProps automatically
// in your body, use the render!() macro to return cx.render(rsx!($body))
// oh I think in the macro you need dioxus_elements::div

// I dont think you can export normal macros
