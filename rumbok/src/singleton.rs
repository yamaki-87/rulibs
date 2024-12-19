use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

// rustのシングルトンでジェネリックスを持つ構造体を定義するのは難しい

pub fn singleton(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let struct_name = derive_input.ident;
    // let (impl_generics, type_generics, where_clause) = derive_input.generics.split_for_impl();
    // fieldに名前付き構造体のみ対応
    // Ok
    // struct Test<T> {
    //     a:T,
    //     b:Option<i32>,
    //     c:String,
    // }
    //
    // NG
    // struct Tw(i32)
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

    let upper_sturct_name = struct_name.clone().to_string().to_uppercase();
    // 名前の重複を避けるため
    let singleton_name = quote::format_ident!("SINGLETON_{}", upper_sturct_name);
    let expanded = quote! {

        static #singleton_name:std::sync::OnceLock<#struct_name> = std::sync::OnceLock::new();

        impl #struct_name {

            pub fn initialize_instance(#(#args),*) -> &'static #struct_name {
                #singleton_name.get_or_init(|| {#struct_name::new_all(#(#func_args),*)})
            }

            pub fn get_instance() -> Option<&'static #struct_name>{
                #singleton_name.get()
            }
        }
    };

    TokenStream::from(expanded)
}
