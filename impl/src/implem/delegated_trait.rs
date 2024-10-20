use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone)]
pub struct DelegatedTrait {
    pub path: syn::Path,
    pub tr: syn::ItemTrait,
}

impl DelegatedTrait {
    pub fn generate_match_branch(&self) -> TokenStream {
        quote! {}
    }
}
