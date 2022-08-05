use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// I dont think you need the actual derive trait in the proc lib itself
// hyper props have children only if you insert the 'children' keyword
// at the end of the struct as a psuedo field instruction
// hyper props cant have new, debug, etc cause its not a proper struct
// which uhh, is kind of a problem if you ask me
// why cant you derive debug again? maybe just impl it then

#[proc_macro_derive(HyperProps)]
pub fn hyper_props(body: TokenStream) -> TokenStream {
    let is_pub = true;
    // Identifier
    let name = "";
    // fields of the struct. Note if you
    let body = "";
    let has_children = true;

    quote!(
        #[derive(Props)]
        // pub?
        struct #name {
            #body
            children: Element<'a>,
        }

        impl Debug for #name {
            pub fn debug(&self) {

            }
        }
    )
    .into()
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
