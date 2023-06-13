use proc_macro::TokenStream;
use quote::quote;
use syn::{Data::Struct, DeriveInput, Fields::Named};

/// Not really implemented. I used this procedural macro to figure out some of the API.
pub fn to_flattened_sql(input: &DeriveInput) -> TokenStream {
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
