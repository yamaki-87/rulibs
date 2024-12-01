use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn to_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => panic!("GetterSetter only supports structs with named fields"),
        },
        _ => panic!("struct only"),
    };

    let field_strings = fields.iter().map(|f| {
        let f_name = f.ident.as_ref().unwrap();

        quote! {
            format!("{}: {}",stringify!(#f_name),self.#f_name)
        }
    });

    let name = input.ident;

    let expanded = quote! {
        impl std::fmt::Display for #name{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{} {{ {} }}",
                    stringify!(#name),
                    // カンマを入れて展開
                    vec![#(#field_strings),*].join(" ")
                )
            }
        }
    };

    TokenStream::from(expanded)
}
