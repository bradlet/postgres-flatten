//! Note: Leaving this to show the issue with my plan of making a recursive proc macro.
use itertools::{Either, Itertools};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data::Struct, DeriveInput, Fields::Named};

/// Provides a recursive `flatten` method implementation for structs that derive `Flattenable`.
///
/// # Return
/// A Vec<Field> which includes all fields within nested structures marked with `#[flattenable]`.
pub fn flatten(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let (flattenable_fields, other_fields) = if let Struct(derived) = &input.data {
        if let Named(fs) = &derived.fields {
            fs.named.iter().partition_map(|f| {
                // Check for fields with the `#[flattenable]` attribute
                if f.attrs.len() == 1
                    && Into::<&Ident>::into(f.attrs.first().unwrap().path().get_ident().unwrap())
                        == &Ident::new("flattenable", Span::call_site())
                {
                    Either::Left(f.clone())
                } else {
                    Either::Right(f.clone())
                }
            })
        } else {
            (vec![], vec![])
        }
    } else {
        (vec![], vec![])
    };

    // Any fields that need to be flattened will be appended to the fields that
    // don't need to be flattened. This is not a very efficient macro!
    // e.g. vec![field1, field2].append(field3.flatten()).append(field4.flatten())
    let gen = quote! {
        impl Flattenable for #name {
            fn flatten() -> Vec::<syn::Field> {
                vec![#(#other_fields),*]#(.append(#flattenable_fields.flatten()))*
            }
        }
    };

    gen.into()
}
