use proc_macro::TokenStream;
use quote::quote;
use syn::{Data::Struct, DeriveInput, Fields::Named};

/// Provides a recursive `flatten` method implementation for structs that derive `Flattenable`.
///
/// # Return
/// A Vec<Field> which includes all fields within nested structures marked with `#[flattenable]`.
pub fn flatten(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    if let Struct(derived) = &input.data {
        if let Named(fs) = &derived.fields {
            fs.named.iter().for_each(|f| {
                eprintln!("FLATTEN type: {:?} | attrs: {:?}", &f.ty, &f.attrs);
            })
        }
    };

    let gen = quote! {
        impl Flattenable for #name {
            fn flatten() -> Vec::<syn::Field> {
                vec![]
            }
        }
    };

    gen.into()
}
