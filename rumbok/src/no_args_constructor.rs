use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn no_args_constructor(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let struct_name = derive_input.ident;
    let fields = match derive_input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => panic!("NoArgsConstructor only supported structs with named fields"),
        },
        _ => panic!("NoArgsConstructor only supported type struct"),
    };

    let no_args_constructor = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();

        quote! {
            #field_name: ::core::default::Default::default()
        }
    });

    let expanded = quote! {
        impl ::core::default::Default for #struct_name{
            fn default() -> Self {
                Self{
                    #(#no_args_constructor),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
