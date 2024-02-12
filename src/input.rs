use std::fs::read_to_string;
use std::path::Path;

use proc_macro2::{Ident, Span, TokenStream};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::PathSep;
use syn::{braced, parse2, PathArguments, PathSegment, Token};

use crate::trait_path::ItemTraitPath;

pub fn parse_input<P: AsRef<Path>>(path: P) -> syn::Result<DelegateInput> {
    let file = read_to_string(path.as_ref()).expect(&format!("Could not open {:?}", path.as_ref()));

    let stream = file.parse::<TokenStream>()?;

    parse2::<DelegateInput>(stream)
}

pub struct DelegateInput {
    pub crate_ident: Ident,
    pub crate_impl_ident: Ident,
    pub macro_ident: Ident,
    pub macro_helper_ident: Ident,
    pub traits: Vec<ItemTraitPath>,
}

impl DelegateInput {
    pub fn root(&self) -> syn::Path {
        let mut segments = Punctuated::<PathSegment, PathSep>::new();

        segments.push(PathSegment {
            ident: self.crate_ident.clone(),
            arguments: PathArguments::None,
        });
        segments.push(PathSegment {
            ident: Ident::new("__private", Span::call_site()),
            arguments: PathArguments::None,
        });

        syn::Path {
            leading_colon: Some(PathSep::default()),
            segments,
        }
    }
}

impl Parse for DelegateInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut crate_ident = Option::<Ident>::None;
        let mut crate_impl_ident = Option::<Ident>::None;
        let mut macro_ident = Option::<Ident>::None;
        let mut macro_helper_ident = Option::<Ident>::None;
        let mut traits = Option::<Vec<ItemTraitPath>>::None;

        while !input.is_empty() {
            let ident = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            match ident.to_string().as_str() {
                "crate_ident" => {
                    if crate_ident.is_some() {
                        return Err(syn::Error::new_spanned(&ident, "Already specified."));
                    }
                    crate_ident.replace(input.parse::<Ident>()?);
                },
                "crate_impl_ident" => {
                    if crate_impl_ident.is_some() {
                        return Err(syn::Error::new_spanned(&ident, "Already specified."));
                    }
                    crate_impl_ident.replace(input.parse::<Ident>()?);
                },
                "macro_ident" => {
                    if macro_ident.is_some() {
                        return Err(syn::Error::new_spanned(&ident, "Already specified."));
                    }
                    macro_ident.replace(input.parse::<Ident>()?);
                },
                "macro_helper_ident" => {
                    if macro_helper_ident.is_some() {
                        return Err(syn::Error::new_spanned(&ident, "Already specified."));
                    }
                    macro_helper_ident.replace(input.parse::<Ident>()?);
                },
                "traits" => {
                    if traits.is_some() {
                        return Err(syn::Error::new_spanned(&ident, "Already specified."));
                    }
                    let content;
                    braced!(content in input);
                    let mut result = Vec::<ItemTraitPath>::new();
                    while !content.is_empty() {
                        result.push(content.parse::<ItemTraitPath>()?);
                    }
                    traits.replace(result);
                },
                _ => {
                    return Err(syn::Error::new_spanned(
                        &ident,
                        &format!("Unknown identifier {}.", ident.to_string()),
                    ))
                },
            }
            input.parse::<Token![;]>()?;
        }

        Ok(Self {
            crate_ident: crate_ident.ok_or(syn::Error::new(
                Span::call_site(),
                "No item `crate_ident_ident` specified.",
            ))?,
            crate_impl_ident: crate_impl_ident.ok_or(syn::Error::new(
                Span::call_site(),
                "No item `cate_impl_ident` specified.",
            ))?,
            macro_ident: macro_ident.ok_or(syn::Error::new(Span::call_site(), "No item `macro_ident` specified."))?,
            macro_helper_ident: macro_helper_ident.ok_or(syn::Error::new(
                Span::call_site(),
                "No item `macro_helper_ident` specified.",
            ))?,
            traits: traits.ok_or(syn::Error::new(Span::call_site(), "No item `trait` specified."))?,
        })
    }
}
