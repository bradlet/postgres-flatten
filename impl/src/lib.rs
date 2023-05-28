//! into_flattened.rs
//! Author: bradlet

use proc_macro::TokenStream;
use quote::quote;

fn impl_to_flattened_sql(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    // quote! macro builds the Rust output code with templating support.
    let gen = quote! {
        impl ToFlattenedSql for #name {
            fn into_flattened_row() {
                println!("Congratulations on calling into_flattened_row() on {}!", stringify!(#name));
                for attr in &input.attrs {
                    println!("Attr: {:?}", stringify!(attr))
                }
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(ToFlattenedSql)]
pub fn to_flattened_sql_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    impl_to_flattened_sql(&ast)
}
