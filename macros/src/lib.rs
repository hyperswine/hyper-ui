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
