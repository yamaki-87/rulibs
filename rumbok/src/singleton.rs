use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn singleton(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let struct_name = derive_input.ident;

    let fields = match derive_input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => panic!("AllArgsConstructor only supports structs with named fields"),
        },
        _ => panic!("struct only"),
    };

    let args = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;

        quote! {
            #field_name:#field_type
        }
    });

    let func_args = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();

        quote! {
            #field_name
        }
    });

    let expanded = quote! {
        static SINGLETON:std::sync::OnceLock<#struct_name> = std::sync::OnceLock::new();

        impl #struct_name{
            pub fn initialize_instance(#(#args),*) -> &'static #struct_name{
                SINGLETON.get_or_init(|| {#struct_name::new_all(#(#func_args),*)})
            }

            pub fn get_instance() -> Option<&'static #struct_name>{
                SINGLETON.get()
            }
        }
    };

    TokenStream::from(expanded)
}
