extern crate proc_macro;

use proc_macro::TokenStream;

mod all_args_constructor;
mod builder;
mod getter;
mod no_args_constructor;
mod setter;
mod to_string;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    builder::builder(input)
}

#[proc_macro_derive(AllArgsConstructor)]
pub fn derive_all_args_constructor(input: TokenStream) -> TokenStream {
    all_args_constructor::all_args_constructor(input)
}

#[proc_macro_derive(NoArgsConstructor)]
pub fn derive_no_args_constructor(input: TokenStream) -> TokenStream {
    no_args_constructor::no_args_constructor(input)
}

#[proc_macro_derive(ToString)]
pub fn derive_to_string(input: TokenStream) -> TokenStream {
    to_string::to_string(input)
}

#[proc_macro_derive(Getter)]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    getter::getter(input)
}

#[proc_macro_derive(Setter)]
pub fn derive_setter(input: TokenStream) -> TokenStream {
    setter::setter(input)
}

#[proc_macro_derive(Data)]
pub fn data(input: TokenStream) -> TokenStream {
    TokenStream::from_iter(
        vec![
            to_string::to_string(input.clone()),
            getter::getter(input.clone()),
            setter::setter(input.clone()),
            all_args_constructor::all_args_constructor(input.clone()),
            no_args_constructor::no_args_constructor(input),
        ]
        .into_iter(),
    )
}
