use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, File};

use crate::input::DelegateInput;
use crate::{generate_traits_match, parse_input};

pub fn generate_delegate_impl_build_string<P: AsRef<Path>>(path: P) -> String {
    match parse_input(path.as_ref()) {
        Ok(input) => prettyplease::unparse(&parse2::<File>(generate_crate_impl_build(&input)).unwrap()),
        // Ok(input) => generate_crate_impl_build(&input).to_string(),
        Err(err) => err.to_compile_error().to_string(),
    }
}

fn generate_crate_impl_build(input: &DelegateInput) -> TokenStream {
    let macro_ident = &input.macro_ident;
    let macro_helper_ident = &input.macro_helper_ident;
    let macro_helper_ident_literal = &input.macro_helper_ident.to_string();

    let traits_match = generate_traits_match(input);

    quote! {
        #[proc_macro_derive(#macro_ident, attributes(#macro_helper_ident))]
        pub fn derive_delegate(input: proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            let input = ::syn::parse_macro_input!(input as syn::DeriveInput);
            derive(&input).into()
        }

        fn derive(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
            match try_expand(input) {
                Ok(expanded) => expanded,
                Err(err) => {
                    err.to_compile_error()
                }
            }
        }

        fn try_expand(input: &syn::DeriveInput) -> ::syn::Result<::proc_macro2::TokenStream> {
            let context = ::delegate_trait::Context::new(input);
            let mut res = ::proc_macro2::TokenStream::default();

            for attr in input.attrs.iter().filter(|attr| attr.path().is_ident(#macro_helper_ident_literal)) {
                let list = attr.meta.require_list()?;
                let config = list.parse_args::<::delegate_trait::TraitConfig>()?;

                let trait_implem: ::proc_macro2::TokenStream = match config.ident.to_string().as_str() {
                    #traits_match
                    _ => return Err(syn::Error::new_spanned(&config.ident, &format!("Unknown trait {}.", config.ident.to_string()))),
                };

                res.extend(trait_implem);
            }

            Ok(res)
        }
    }
}
