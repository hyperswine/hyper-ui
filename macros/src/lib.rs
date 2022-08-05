use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HyperProps)]
pub fn hyper_props(body: TokenStream) -> TokenStream {
    body
}

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
