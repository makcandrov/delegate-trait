#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod common;
mod delegate;
mod implem;

#[proc_macro]
pub fn delegate_trait(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    delegate::expand(tokens)
}

#[proc_macro]
pub fn delegate_trait_impl(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    implem::expand(tokens)
}
