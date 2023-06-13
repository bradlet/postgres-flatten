//! into_flattened.rs
//! Author: bradlet

// Set max recursion limit (https://github.com/sfackler/rust-postgres/blob/master/postgres-derive/src/lib.rs)
#![recursion_limit = "256"]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data::Struct, DeriveInput, Fields::Named};

mod impl_flattenable;
mod impl_from_flattened_sql;
mod impl_to_flattened_sql;

/// Declare the recursive `Flattenable` derive macro, as well as the `flattenable`
/// helper attribute which is used to mark member structs that should be flattened.
#[proc_macro_derive(Flattenable, attributes(flattenable))]
pub fn flatten_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_flattenable::flatten(&ast)
}

/// Declare `ToFlattenedSql` derive macro.
/// Note: To simplify the MVP, not implementing this. Instead, it has some methods
///  I used to figure out procedural macro implementation.
#[proc_macro_derive(ToFlattenedSql)]
pub fn to_flattened_sql_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_to_flattened_sql::to_flattened_sql(&ast)
}

/// Declare the `FromFlattenedSql` derive macro, which provides a `from_flattened_row`
/// implementation which lets clients build a strongly-typed struct from a postgres `Row`
#[proc_macro_derive(FromFlattenedSql)]
pub fn from_flattened_sql_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let stream = impl_from_flattened_sql::from_flattened_sql(&ast);
    println!("{}", stream); // TODO: Remove (or comment out) when main impl is done.
    stream
}
