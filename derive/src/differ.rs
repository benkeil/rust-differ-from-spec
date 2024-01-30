use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, Data, DeriveInput, Fields, Path, Result};

pub fn expand_derive_differ_from_spec(input: &mut DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;

    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();

    let path: Cow<Path> = Cow::Owned(parse_quote!(differ_from_spec));

    let body = statements(&path, &input.data);

    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics _differ_from_spec::DifferFromSpec for #ident #type_generics #where_clause {
            fn differ_from_spec(&self, spec: &Self) -> bool {
                #body
                false
            }
        }
    };

    Ok(wrap_in_const(path, impl_block))
}

fn statements(path: &Cow<Path>, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! { f.span() =>
                        if _differ_from_spec::DifferFromSpec::differ_from_spec(&self.#name, &spec.#name) {
                            return true;
                        }
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(_) => {
                unimplemented!()
            }
            Fields::Unit => {
                unimplemented!()
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn wrap_in_const(path: Cow<Path>, code: TokenStream) -> TokenStream {
    let use_block = quote! {
        // use #path::DifferFromSpec;
        extern crate #path as _differ_from_spec;
    };

    quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #use_block
            #code
        };
    }
}
