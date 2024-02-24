use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{GenericParam, Ident, Path, TraitItem, TraitItemFn};

use crate::generics::generic_param_name;
use crate::input::DelegateInput;
use crate::modifier::{GenericsRenamer, PathRootRenamer, TokenModifier};
use crate::trait_path::ItemTraitPath;
use crate::{Context, TraitConfig};

pub fn generate_traits_match(input: &DelegateInput) -> TokenStream {
    let mut res = TokenStream::default();
    let root = input.root();
    for trait_input in &input.traits {
        let trait_ident_string = &trait_input
            .path
            .segments
            .last()
            .expect("generate_traits_match: expected ident")
            .ident
            .to_string();
        let trait_impl = quote! {
            let trait_input = ::syn::parse2::<::delegate_trait::ItemTraitPath>(::quote::quote! { #trait_input }).unwrap();
            let root = ::syn::parse2::<::syn::Path>(::quote::quote! { #root }).unwrap();
            ::delegate_trait::generate_trait_impl(&context, config, root, trait_input)?
        };
        res.extend(quote! { #trait_ident_string => { #trait_impl }, });
    }
    res
}

pub fn generate_trait_impl(
    context: &Context<'_>,
    config: &TraitConfig,
    root: Path,
    mut trait_input: ItemTraitPath,
) -> syn::Result<TokenStream> {
    if let Ok(package_name) = std::env::var("CARGO_PKG_NAME") {
        let mut renamer = PathRootRenamer {
            original: package_name,
            rename: Ident::new("crate", Span::call_site()),
            remove_leading_colon: true,
        };
        renamer.modify_item_trait_path(&mut trait_input);
    }

    let mut generics_renamer = GenericsRenamer::default();

    for couple in trait_input.generics.params.iter().zip(config.generics.params.iter()) {
        match couple {
            (GenericParam::Lifetime(original), GenericParam::Lifetime(renamed)) => {
                generics_renamer.insert_lifetime(original.lifetime.ident.to_string(), renamed.lifetime.ident.clone())
            },
            (GenericParam::Type(original), GenericParam::Type(renamed)) => {
                generics_renamer.insert_type(original.ident.to_string(), renamed.ident.clone())
            },
            (GenericParam::Const(original), GenericParam::Const(renamed)) => {
                generics_renamer.insert_type(original.ident.to_string(), renamed.ident.clone())
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    &couple.1,
                    &format!(
                        "Expected {}, got {}.",
                        generic_param_name(&couple.0),
                        generic_param_name(&couple.1)
                    ),
                ))
            },
        }
    }

    let trait_path = trait_input.path.clone();

    let mut trait_path_without_ident = trait_path.clone();
    trait_path_without_ident.segments.pop();

    let mut methods = TokenStream::default();

    let trait_input_items = &trait_input.items;

    for method in trait_input_items.iter().filter_map(|item| trait_item_as_fn(item)) {
        let mut method = method.clone();

        method.default = None;
        method.semi_token = Some(Default::default());

        generics_renamer.modify_trait_item_fn(&mut method);

        methods.extend(quote! {
            #[through(#trait_path)]
            #method
        })
    }

    let to = &config.to;
    let wi = config.wi.clone().unwrap_or_default();
    let trait_path = &config.path;

    let methods = ::quote::quote! {
        #wi

        #root::delegate! {
            to #to {
                #methods
            }
        }
    };

    Ok(config.wrap_methods(&context, &trait_path.to_token_stream(), &config.generics, &methods))
}

fn trait_item_as_fn(trait_item: &TraitItem) -> Option<&TraitItemFn> {
    match trait_item {
        TraitItem::Fn(method) => Some(method),
        _ => None,
    }
}
