extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let sname = &ast.ident;
    use syn::Data::Struct;
    use syn::{DataStruct, Fields, FieldsNamed};
    let memb = match &ast.data {
        Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named
            .iter()
            .map(|m| {
                let name = m.ident.as_ref().expect("member has name");
                quote! { ret.push((stringify!(#name).to_owned(), Box::new(self.#name))); }
            })
            .collect::<Vec<_>>(),
        _ => todo!("not a named member struct"),
    };
    let gen = quote! {
        impl HelloMacro for #sname {
            fn members(self) -> Vec<(String, Box<dyn std::any::Any>)> {
                let mut ret : Vec<(String, Box<dyn std::any::Any>)> = Vec::new();
                #( #memb )*
                ret
            }
        }
    };
    gen.into()
}
