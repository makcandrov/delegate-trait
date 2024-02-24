use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Path, TraitItem, TraitItemFn};

use crate::dynamic_rename::DynamicGenericRenamer;
use crate::input::DelegateInput;
use crate::modifier::{PathRootRenamer, TokenModifier};
use crate::trait_path::ItemTraitPath;
use crate::{Context, GenericIdent, TraitConfig};

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
            ::delegate_trait::generate_trait_impl(&context, config, root, trait_input)
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
) -> TokenStream {
    if let Ok(package_name) = std::env::var("CARGO_PKG_NAME") {
        let mut renamer = PathRootRenamer {
            original: package_name,
            rename: Ident::new("crate", Span::call_site()),
            remove_leading_colon: true,
        };
        renamer.modify_item_trait_path(&mut trait_input);
    }

    // let hashtag = quote! { # };

    // let mut generic_idents = HashMap::<GenericIdent, Ident>::new();
    // let mut generic_renames = TokenStream::default();

    // for (i, generic) in trait_input.generics.params.iter().enumerate() {
    //     let rename_ident = Ident::new(&format!("generic_{i}"), Span::call_site());
    //     generic_renames
    //         .extend(quote! { let #rename_ident = ::delegate_trait::GenericIdent::from(&config.generics.params[#i]); });
    //     generic_idents.insert(GenericIdent::from(generic), rename_ident);
    // }

    // let renamer = DynamicGenericRenamer::new(generic_idents);

    let trait_path = trait_input.path.clone();

    let mut trait_path_without_ident = trait_path.clone();
    trait_path_without_ident.segments.pop();

    let mut methods = TokenStream::default();

    let trait_input_items = &trait_input.items;

    for method in trait_input_items.iter().filter_map(|item| trait_item_as_fn(item)) {
        let mut method = method.clone();

        method.default = None;
        method.semi_token = Some(Default::default());

        // let mut renamed_method = TokenStream::default();
        // renamer.renamed_trait_item_fn(&mut renamed_method, &method);

        methods.extend(quote! {
            #[through(#trait_path)]
            #method
        })
    }

    // #generic_renames

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

    config.wrap_methods(
        &context,
        &::quote::ToTokens::to_token_stream(&trait_path),
        &config.generics,
        &methods,
    )
}

fn trait_item_as_fn(trait_item: &TraitItem) -> Option<&TraitItemFn> {
    match trait_item {
        TraitItem::Fn(method) => Some(method),
        _ => None,
    }
}
