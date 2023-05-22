//! into_flattened.rs
//! Author: bradlet

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_to_flattened_sql(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    // quote! macro builds the Rust output code with templating support.
    let gen = quote! {
        impl ToFlattenedRow for #name {
            fn into_flattened_row() {
                println!("Congratulations on calling into_flattened_row() on {}!", stringify!(#name));
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
