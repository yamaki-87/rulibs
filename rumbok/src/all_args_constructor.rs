use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::Access;

pub fn all_args_constructor(input: TokenStream, access: Access) -> TokenStream {
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
        let field_type = &f.ty;
        let field_name = f.ident.as_ref().unwrap();

        quote! {
            #field_name:#field_type
        }
    });

    let content = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();

        quote! {
            #field_name:#field_name
        }
    });

    let expand = match access {
        Access::Public => {
            quote! {
                impl #struct_name {
                    pub fn new_all(#(#args),*)-> Self{
                        Self{
                            #(#content),*
                        }
                    }
                }
            }
        }
        Access::Private => {
            quote! {
                impl #struct_name {
                    fn new_all(#(#args),*)-> Self{
                        Self{
                            #(#content),*
                        }
                    }
                }
            }
        }
    };

    TokenStream::from(expand)
}
