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
                // eprintln!("FLATTEN type: {:?} | attrs: {:?}", &f.ty, &f.attrs);
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
    let gen = quote! {
        impl Flattenable for #name {
            fn flatten() -> Vec::<syn::Field> {
                vec![#(#other_fields),*]#(.append(#flattenable_fields.flatten()))*
            }
        }
    };

    gen.into()
}
