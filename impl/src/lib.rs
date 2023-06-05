//! into_flattened.rs
//! Author: bradlet

// Set max recursion limit (https://github.com/sfackler/rust-postgres/blob/master/postgres-derive/src/lib.rs)
#![recursion_limit = "256"]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data::Struct, DeriveInput, Fields::Named};

fn impl_to_flattened_sql(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let field_names = if let Struct(derived) = &input.data {
        if let Named(fs) = &derived.fields {
            fs
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect()
        } else {
           vec![] 
        }
    } else {
        vec![]
    };

    // quote! macro builds the Rust output code with templating support.
    let gen = quote! {
        impl ToFlattenedSql for #name {
            fn into_flattened_row() {
                println!("Congratulations on calling into_flattened_row() on {}!", stringify!(#name));
                println!("Are these your field names? stringify! #(#field_names)*");
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(ToFlattenedSql)]
pub fn to_flattened_sql_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_to_flattened_sql(&ast)
}
