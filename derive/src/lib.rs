use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

use crate::differ::expand_derive_differ_from_spec;

mod differ;

#[proc_macro_derive(DifferFromSpec)]
pub fn differ_from_spec(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    expand_derive_differ_from_spec(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
