use std::collections::HashMap;

use syn::{punctuated::Punctuated, token::Comma};

use crate::common::try_braced;

use super::{DelegatedTrait, DelegatedTraitSource};

#[derive(Clone)]
pub struct DelegateTraitImplInput {
    pub traits: Vec<DelegatedTraitInput>,
}

#[derive(Clone)]
pub struct DelegatedTraitInput {
    pub traits: Vec<syn::Path>,
    pub source: DelegatedTraitSource,
}

impl syn::parse::Parse for DelegateTraitImplInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            traits: Punctuated::<DelegatedTraitInput, Comma>::parse_terminated(input)?
                .into_pairs()
                .map(|pair| pair.into_value())
                .collect::<Vec<_>>(),
        })
    }
}

impl syn::parse::Parse for DelegatedTraitInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut traits = Vec::<syn::Path>::new();

        syn::custom_keyword!(from);

        if let Ok(path) = input.parse::<syn::Path>() {
            traits.push(path);
        } else if let Ok(content) = try_braced(input) {
            let paths =
                syn::punctuated::Punctuated::<syn::Path, syn::token::Comma>::parse_terminated(
                    &content,
                )?;
            traits.extend(paths);
        }

        let _ = input.parse::<from>();

        let source = input.parse::<DelegatedTraitSource>()?;

        Ok(Self { traits, source })
    }
}

impl DelegateTraitImplInput {
    pub fn into_delegated_traits(self) -> syn::Result<Vec<DelegatedTrait>> {
        self.traits
            .into_iter()
            .map(|tr| tr.into_delegated_traits())
            .collect::<Result<Vec<_>, syn::Error>>()
            .map(|traits| traits.into_iter().flatten().collect::<Vec<_>>())
    }
}

impl DelegatedTraitInput {
    pub fn into_delegated_traits(self) -> syn::Result<Vec<DelegatedTrait>> {
        let mut traits = Vec::<DelegatedTrait>::new();

        let file = match self.source {
            DelegatedTraitSource::Import(lit_str) => {
                let file_str = match std::fs::read_to_string(lit_str.value()) {
                    Ok(file_str) => file_str,
                    Err(err) => {
                        return Err(syn::Error::new_spanned(
                            &lit_str,
                            &format!("could not open file: {err}"),
                        ))
                    }
                };

                let file_tokens = file_str.parse::<proc_macro2::TokenStream>()?;
                syn::parse2::<syn::File>(file_tokens)?
            }
            DelegatedTraitSource::Code(file) => file,
        };

        let mut ident_to_path = HashMap::new();
        for path in self.traits {
            let Some(last_segment) = path.segments.last() else {
                return Err(syn::Error::new_spanned(&path, "expected path to trait"));
            };
            if !last_segment.arguments.is_none() {
                return Err(syn::Error::new_spanned(
                    &last_segment.arguments,
                    "unexpected arguments",
                ));
            }
            let last_segment_ident = last_segment.ident.clone();
            if ident_to_path
                .insert(last_segment_ident.clone(), path)
                .is_some()
            {
                return Err(syn::Error::new_spanned(
                    &last_segment_ident,
                    "duplicate trait ident",
                ));
            };
        }

        for item in file.items {
            let syn::Item::Trait(tr) = item else {
                continue;
            };
            let Some(path) = ident_to_path.remove(&tr.ident) else {
                continue;
            };
            traits.push(DelegatedTrait { path, tr })
        }

        if let Some((trait_not_found, _)) = ident_to_path.into_iter().next() {
            return Err(syn::Error::new_spanned(&trait_not_found, "trait not found"));
        }

        Ok(traits)
    }
}
