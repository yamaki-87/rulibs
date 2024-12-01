use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn setter(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let fields = match derive_input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Setter only supports structs with named fields"),
        },
        _ => panic!("struct only"),
    };

    let struct_name = derive_input.ident;

    let setters = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_ty = &f.ty;

        let setter_name = quote::format_ident!("set_{}", field_name);

        quote! {
            pub fn #setter_name(&mut self,value:#field_ty){
                self.#field_name = value;
            }
        }
    });

    let expanded = quote! {
        impl #struct_name{
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}
