use proc_macro::TokenStream;
use quote::quote;
use syn::{Data::Struct, DeriveInput, Fields::Named};

/// Provides `from_flattened_row` which is used alongside `flatten`
/// from the derivable `Flattenable` trait to parse a `Row` into a
/// nested, strongly-typed object.
pub fn from_flattened_sql(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let field_names = if let Struct(derived) = &input.data {
        if let Named(fs) = &derived.fields {
            fs.named
                .iter()
                .map(|f| {
                    eprintln!("Type: {:?}", f.ty);
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
