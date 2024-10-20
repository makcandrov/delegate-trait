use proc_macro2::TokenStream;
use quote::quote;
use syn::TraitItem;

#[derive(Clone)]
pub struct DelegatedTrait {
    pub path: syn::Path,
    pub tr: syn::ItemTrait,
}

impl DelegatedTrait {
    pub fn generate_match_branch(&self, crate_ident: syn::Ident) -> TokenStream {
        let trait_path = &self.path;
        let trait_ident = &self.tr.ident;
        let trait_ident_str = &self.tr.ident.to_string();
        let through_trait = &self.path;

        let hashtag = quote! { # };

        let mut methods = quote! {};

        for item in &self.tr.items {
            let TraitItem::Fn(trait_item_fn) = item else {
                continue;
            };
            methods.extend(quote! { #[through(#through_trait)] #trait_item_fn });
        }

        let trait_impl = quote! {
            let name = &derive_input.ident;

            // whith { ... }
            let wi = &attribute.wi;

            // for <...>
            let fo = &attribute.fo;

            // to ...
            let to = &attribute.to;

            // where { ... }
            let mut wh = attribute.wh.clone();

            // struct A<...> / enum A<...>
            let input_generics = &derive_input.generics;

            let (_, ty_generics, where_clause) = input_generics.split_for_impl();
            merge_where_clauses(&mut wh, where_clause, true);

            let mut impl_generics = input_generics.clone();
            merge_generics(&mut impl_generics, fo);
            let (impl_generics, _, _) = impl_generics.split_for_impl();

            let trait_generics = &attribute.generics;

            ::quote::quote! {
                impl #hashtag impl_generics #trait_path #hashtag trait_generics for #hashtag name #hashtag ty_generics #hashtag wh {
                    #hashtag wi

                    // todo: change
                    :: #crate_ident ::__private::delegate! {
                        to #hashtag to {
                            #methods
                        }
                    }
                }

            }
        };

        quote! {
            #trait_ident_str => { #trait_impl },
        }
    }
}
