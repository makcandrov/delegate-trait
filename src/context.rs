use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub struct Context<'a> {
    pub ident: &'a syn::Ident,
    pub generics: &'a syn::Generics,
}

impl<'a> Context<'a> {
    pub fn new(input: &'a DeriveInput) -> Self {
        Self {
            ident: &input.ident,
            generics: &input.generics,
        }
    }

    pub fn in_impl(&self, trait_for: &TokenStream, tokens: &TokenStream) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let ident = self.ident;
        quote! {
            impl #impl_generics #trait_for #ident #ty_generics #where_clause {
                #tokens
            }
        }
    }
}
