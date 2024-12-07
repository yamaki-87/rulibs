use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn value(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let fields = match derive_input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Value only supported structs with named fields"),
        },
        _ => panic!("Value only supported type struct"),
    };

    let struct_name = derive_input.ident;

    let value = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;

        let getter_name = quote::format_ident!("get_{}", field_name);
        quote! {
            pub fn #getter_name(&self) -> &#field_type{
                &self.#field_name
            }
        }
    });

    let expanded = quote! {
        impl #struct_name{
            #(#value)*
        }
    };

    TokenStream::from(expanded)
}
