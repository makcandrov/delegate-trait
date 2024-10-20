use std::collections::HashMap;

use quote::ToTokens;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Comma,
    visit_mut::VisitMut,
};

use crate::common::{try_braced, IdentMap, TraitsVisitor};

use super::{DelegatedTrait, DelegatedTraitSource};

#[derive(Clone)]
pub struct DelegateTraitImplInput {
    pub crate_ident: syn::Ident,
    pub traits: Vec<DelegatedTraitInput>,
}

#[derive(Clone)]
pub struct DelegatedTraitInput {
    pub traits: Vec<syn::Path>,
    pub source: DelegatedTraitSource,
}

impl Parse for DelegateTraitImplInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        enum IdentOrTraits {
            Ident(syn::Ident),
            Traits(proc_macro2::TokenStream),
        }

        impl Parse for IdentOrTraits {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                if let Ok(ident) = input.parse::<syn::Ident>() {
                    return Ok(Self::Ident(ident));
                } else {
                    let content;
                    braced!(content in input);
                    Ok(Self::Traits(content.parse()?))
                }
            }
        }

        let maps = Punctuated::<IdentMap<IdentOrTraits>, Comma>::parse_terminated(input)?;

        let mut crate_ident_opt = None::<syn::Ident>;
        let mut traits_opt = None::<Vec<DelegatedTraitInput>>;

        for map in maps {
            match map.from.to_string().as_str() {
                "crate" => {
                    let IdentOrTraits::Ident(ident) = map.to else {
                        return Err(syn::Error::new_spanned(&map.from, "expected ident"));
                    };
                    if crate_ident_opt.replace(ident).is_some() {
                        return Err(syn::Error::new_spanned(map.from, "duplicate key"));
                    }
                }
                "traits" => {
                    let IdentOrTraits::Traits(traits) = map.to else {
                        return Err(syn::Error::new_spanned(&map.from, "expected ident"));
                    };

                    struct Temp(Vec<DelegatedTraitInput>);
                    impl Parse for Temp {
                        fn parse(input: ParseStream) -> syn::Result<Self> {
                            let traits =
                                Punctuated::<DelegatedTraitInput, Comma>::parse_terminated(input)?
                                    .into_pairs()
                                    .map(|pair| pair.into_value())
                                    .collect::<Vec<_>>();
                            Ok(Self(traits))
                        }
                    }

                    let traits = parse2::<Temp>(traits)?.0;

                    if traits_opt.replace(traits).is_some() {
                        return Err(syn::Error::new_spanned(map.from, "duplicate key"));
                    }
                }
                _ => return Err(syn::Error::new_spanned(map.from, "unknown key")),
            }
        }

        let result = Self {
            crate_ident: crate_ident_opt.ok_or(syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing key `crate`",
            ))?,
            traits: traits_opt.ok_or(syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing key `traits`",
            ))?,
        };

        Ok(result)
    }
}

impl Parse for DelegatedTraitInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

        let mut file = match self.source {
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
                .insert(last_segment_ident.to_string(), path)
                .is_some()
            {
                return Err(syn::Error::new_spanned(
                    &last_segment_ident,
                    "duplicate trait ident",
                ));
            };
        }

        let mut visitor = TraitsVisitor::new();
        visitor.visit_file_mut(&mut file);

        std::fs::write("debug", file.to_token_stream().to_string()).unwrap();

        for (ident_str, tr) in visitor.traits {
            let Some(path) = ident_to_path.remove(&ident_str) else {
                continue;
            };
            traits.push(DelegatedTrait { path, tr })
        }

        if let Some((trait_not_found, path)) = ident_to_path.into_iter().next() {
            return Err(syn::Error::new_spanned(
                &path,
                &format!("trait `{trait_not_found}` not found"),
            ));
        }

        Ok(traits)
    }
}
