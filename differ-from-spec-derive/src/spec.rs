use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use syn::{parse_quote, DeriveInput, GenericParam, Generics, Path, Result};

pub fn expand_derive_differ_from_spec(input: &mut DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let generics = add_trait_bounds(input.generics.clone());

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let path: Cow<Path> = Cow::Owned(parse_quote!(crate));

    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics #path::DifferFromSpec for #ident #type_generics #where_clause {
            fn differ_from_spec(&self, spec: &Self) -> bool {
                if spec.team.differ_from_spec(&self.team) {
                    return true
                }
                if spec.sub.differ_from_spec(&self.sub) {
                    return true
                }
                false
            }
        }
    };

    // let description = match data {
    //     syn::Data::Struct(s) => {
    //         match s.fields {
    //             Fields::Named(FieldsNamed { named, .. }) => {
    //                 named.iter().map(|f| {
    //                     let name = &f.ident;
    //                     let ty = &f.ty;
    //                     format!("name: {:?}, ty: {:?}", name, ty)
    //                 })
    //                     .collect::<Vec<String>>()
    //                     .join("\n")
    //             }
    //             Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
    //                 let num_fields = unnamed.iter().count();
    //                 format!("a struct with {} unnamed fields", num_fields)
    //             }
    //             Fields::Unit => {
    //                 format!("a unit struct")
    //             }
    //         }
    //     }
    //     _ => panic!("{}::differ_from_spec() is not implemented for this type", ident),
    // };

    // Ok(impl_block.into())
    Ok(wrap_in_const(path, impl_block))
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    println!("add_trait_bounds: {:#?}", generics);
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::HeapSize));
        }
    }
    generics
}

fn wrap_in_const(path: Cow<Path>, code: TokenStream) -> TokenStream {
    let use_block = quote! {
        use #path::DifferFromSpec;
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
