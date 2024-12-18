use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

pub fn builder(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let struct_name = derive_input.ident;
    let mut new_generics = derive_input.generics.clone();
    for param in new_generics.params.iter_mut() {
        if let syn::GenericParam::Type(type_param) = param {
            type_param
                .bounds
                .push(syn::parse_quote!(::core::default::Default));
        }
    }
    let (impl_generics, ty_generics, where_clause) = new_generics.split_for_impl();
    let fields = match derive_input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Builder only supported structs with named fields"),
        },
        _ => panic!("Builder only supported type struct"),
    };

    let builder = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;

        if is_option(field_type) {
            quote! {
                #field_name:#field_type
            }
        } else {
            quote! {
                #field_name: Option<#field_type>
            }
        }
    });

    let default_field = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();

        quote! {
            #field_name: None
        }
    });

    let builder_func = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;

        if is_option(field_type) {
            quote! {
                pub fn #field_name(mut self,value:#field_type) -> Self{
                    self.#field_name = value;
                    self
                }
            }
        } else {
            quote! {
                pub fn #field_name(mut self,value:#field_type) -> Self{
                    self.#field_name = Some(value);
                    self
                }
            }
        }
    });

    let builder_field = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;

        if is_option(field_type) {
            quote! {
                #field_name: self.#field_name
            }
        } else {
            quote! {
                #field_name: self.#field_name.unwrap_or_default()
            }
        }
    });
    // unwrap_or_default()
    let builder_struct_name = quote::format_ident!("{}Builder", struct_name);

    let expanded = quote! {
        struct #builder_struct_name #ty_generics #where_clause  {
            #(#builder),*
        }

        impl #impl_generics #builder_struct_name #ty_generics #where_clause{
            #(#builder_func)*

            pub fn build(self) -> #struct_name #ty_generics{
                #struct_name{
                #(#builder_field),*
                }
            }
        }

        impl #impl_generics #struct_name #ty_generics #where_clause{
            pub fn builder() -> #builder_struct_name #ty_generics {
                #builder_struct_name{
                    #(#default_field),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn is_option(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(seg) = type_path.path.segments.first() {
            return seg.ident == "Option";
        }
    }
    false
}
