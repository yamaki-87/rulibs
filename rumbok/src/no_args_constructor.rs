use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn no_args_constructor(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let struct_name = derive_input.ident;
    let mut new_generics = derive_input.generics;
    for param in new_generics.params.iter_mut() {
        if let syn::GenericParam::Type(type_param) = param {
            type_param
                .bounds
                .push(syn::parse_quote!(::core::default::Default));
        }
    }
    let (impl_generics, type_generics, where_clause) = new_generics.split_for_impl();

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
        impl #impl_generics ::core::default::Default for #struct_name #type_generics #where_clause {
            fn default() -> Self {
                Self{
                    #(#no_args_constructor),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
