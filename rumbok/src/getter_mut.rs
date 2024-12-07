use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn getter_mut(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let fields = match derive_input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Getter only supported structs with named fields"),
        },
        _ => panic!("Getter only supported type struct"),
    };

    let struct_name = derive_input.ident;

    let getters = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;

        let getter_name: syn::Ident = quote::format_ident!("get_mut_{}", field_name);
        quote! {
            pub fn #getter_name(&mut self) -> &mut #field_type{
                &mut self.#field_name
            }
        }
    });

    let expanded = quote! {
        impl #struct_name {
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}
