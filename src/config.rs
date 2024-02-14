use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::{parse2, GenericParam, Generics, PathArguments, Token, WhereClause};

use crate::generics::{merge_generics, merge_where_clauses};
use crate::Context;

pub struct TraitConfig {
    pub path: syn::Path,
    pub generics: syn::Generics,
    pub to: syn::Expr,
    pub wh: Option<WhereClause>,
    pub wi: Option<TokenStream>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum GenericIdent<'a> {
    Lifetime(&'a Ident),
    Other(&'a Ident),
}

impl<'a> From<&'a GenericParam> for GenericIdent<'a> {
    fn from(generic_param: &'a GenericParam) -> Self {
        match generic_param {
            GenericParam::Lifetime(l) => Self::Lifetime(&l.lifetime.ident),
            GenericParam::Type(t) => Self::Other(&t.ident),
            GenericParam::Const(c) => Self::Other(&c.ident),
        }
    }
}

impl TraitConfig {
    pub fn wrap_methods(
        &self,
        context: &Context<'_>,
        trait_ident: &TokenStream,
        trait_generics: &Generics,
        methods: &TokenStream,
    ) -> TokenStream {
        let (_, ty_generics, _) = context.generics.split_for_impl();

        let mut impl_generics = context.generics.clone();
        merge_generics(&mut impl_generics, &self.generics);

        let mut where_clause = context.generics.where_clause.clone();
        merge_where_clauses(&mut where_clause, &self.wh, false);

        let item_ident = &context.ident;

        quote! {
            impl #impl_generics #trait_ident #trait_generics for #item_ident #ty_generics #where_clause {
                #methods
            }
        }
    }
}

mod keyword {
    syn::custom_keyword!(to);
    syn::custom_keyword!(with);
}

impl Parse for TraitConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut path = input.parse::<syn::Path>()?;

        let arguments = core::mem::replace(
            &mut path
                .segments
                .last_mut()
                .expect("TraitConfig::parse: Ident expected")
                .arguments,
            PathArguments::None,
        );
        let generics: Generics = parse2(arguments.to_token_stream())?;

        input.parse::<keyword::to>()?;

        let to = syn::Expr::parse_without_eager_brace(input)?;

        let wh = if input.peek(Token![where]) {
            Some(input.parse::<WhereClause>().unwrap())
        } else {
            None
        };

        let wi = if input.peek(keyword::with) {
            input.parse::<keyword::with>().unwrap();
            let content;
            syn::braced!(content in input);
            Some(content.parse::<TokenStream>()?)
        } else {
            None
        };

        Ok(Self {
            path,
            generics,
            to,
            wh,
            wi,
        })
    }
}
