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
            fs.named.iter().map(|f| f.ident.as_ref().unwrap()).collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    };
    // TODO: Figure out how to pull this in one operation with field_names or use a macro_rules! macro
    let field_types = if let Struct(derived) = &input.data {
        if let Named(fs) = &derived.fields {
            fs.named.iter().map(|f| &f.ty).collect()
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
                #(println!("Field: {} [type = {}]", stringify!(#field_names), stringify!(#field_types)));*
            }

            // fn default() -> Self {
            //     Self {
            //         #(#field_names : String::from("test")),*
            //     }
            // }
        }
    };

    gen.into()
}

#[proc_macro_derive(ToFlattenedSql)]
pub fn to_flattened_sql_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_to_flattened_sql(&ast)
}

/*
Plan:
1. Get field name and type.
2. If type is struct,
 */
fn impl_from_flattened_sql(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let field_names = if let Struct(derived) = &input.data {
        if let Named(fs) = &derived.fields {
            fs.named
                .iter()
                .map(|f| {
                    println!("Type: {:?}", f.ty);
                    f.ident.as_ref().unwrap()
                })
                .collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let gen = quote! {
        impl FromFlattenedSql for #name {
            fn from_flattened_row(row: postgres::Row) -> Self {
                Self {
                    #(
                        #field_names : row
                            .try_get(stringify!(#field_names))
                            .unwrap_or_else(|err| panic!("Type mismatch: {}", err.into_source().unwrap()))
                    ),*
                }
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(FromFlattenedSql)]
pub fn from_flattened_sql_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let stream = impl_from_flattened_sql(&ast);
    println!("{}", stream); // TODO: Remove (or comment out) when main impl is done.
    stream
}
