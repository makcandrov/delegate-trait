use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, PatType, PathArguments, TraitItem, TraitItemFn};

use crate::input::DelegateInput;
use crate::prefixer::Prefixer;
use crate::trait_path::ItemTraitPath;

pub fn generate_traits_match(input: &DelegateInput) -> TokenStream {
    let mut res = TokenStream::default();
    for trait_input in &input.traits {
        let trait_ident_string = &trait_input.path.segments.last().unwrap().ident.to_string();
        let trait_impl = generate_trait_impl(input, trait_input);
        res.extend(quote! { #trait_ident_string => { #trait_impl }, });
    }
    res
}

fn generate_trait_impl(input: &DelegateInput, trait_input: &ItemTraitPath) -> TokenStream {
    let root = input.root();

    let prefixer = Prefixer {
        prefix: root.clone(),
        crates: input.deps.iter().map(|ident| ident.to_string()).collect(),
    };

    let mut trait_path = trait_input.path.clone();
    // trait_path.segments.last_mut().unwrap().ident = config.ident.clone();
    prefixer.prefix_path(&mut trait_path);

    let mut trait_path_without_ident = trait_path.clone();
    trait_path_without_ident.segments.pop();

    let mut trait_path_without_generic = trait_path.clone();
    trait_path_without_generic.segments.last_mut().unwrap().arguments = PathArguments::None;

    let mut methods = TokenStream::default();

    for method in trait_input.items.iter().filter_map(|item| trait_item_as_fn(item)) {
        let mut method = method.clone();
        for arg in method.sig.inputs.iter_mut().filter_map(fn_arg_as_typed_argument_mut) {
            prefixer.prefix_type(&mut arg.ty);
        }

        method.default = None;
        method.semi_token = Some(Default::default());

        methods.extend(quote! {
            #[through(#trait_path_without_generic)]
            #method
        })
    }

    let hashtag = quote! { # };

    quote! {
        let to = &config.to;
        let wi = config.wi.clone().unwrap_or_default();
        let trait_ident = &config.ident;
        let trait_path = ::quote::quote! { #trait_path_without_ident #hashtag trait_ident };

        let methods = ::quote::quote! {
            #hashtag wi

            #root::delegate! {
                to #hashtag to {
                    #methods
                }
            }
        };

        config.wrap_methods(&context, &::quote::ToTokens::to_token_stream(&trait_path), &config.generics, &methods)
    }
}

fn trait_item_as_fn(trait_item: &TraitItem) -> Option<&TraitItemFn> {
    match trait_item {
        TraitItem::Fn(method) => Some(method),
        _ => None,
    }
}

pub fn fn_arg_as_typed_argument_mut(arg: &mut FnArg) -> Option<&mut PatType> {
    match arg {
        FnArg::Typed(t) => Some(t),
        _ => None,
    }
}
