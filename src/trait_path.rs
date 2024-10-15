//! Rewrite of `syn::ItemTrait` with a `Path` instead of an `Ident`.

use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    braced, bracketed, parse2, token, AttrStyle, Attribute, Generics, ImplRestriction, Path,
    PathArguments, Token, TraitItem, TypeParamBound, Visibility,
};

pub struct ItemTraitPath {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Option<Token![unsafe]>,
    pub auto_token: Option<Token![auto]>,
    pub restriction: Option<ImplRestriction>,
    pub trait_token: Token![trait],
    pub path: Path,
    pub generics: Generics,
    pub colon_token: Option<Token![:]>,
    pub supertraits: Punctuated<TypeParamBound, Token![+]>,
    pub brace_token: token::Brace,
    pub items: Vec<TraitItem>,
}

impl Parse for ItemTraitPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let outer_attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let unsafety: Option<Token![unsafe]> = input.parse()?;
        let auto_token: Option<Token![auto]> = input.parse()?;
        let trait_token: Token![trait] = input.parse()?;
        let mut path: Path = input.parse()?;
        let arguments = core::mem::replace(
            &mut path
                .segments
                .last_mut()
                .expect("ItemTraitPath::parse: expected ident")
                .arguments,
            PathArguments::None,
        );
        let generics: Generics = parse2(arguments.to_token_stream())?;
        parse_rest_of_trait(
            input,
            outer_attrs,
            vis,
            unsafety,
            auto_token,
            trait_token,
            path,
            generics,
        )
    }
}

fn parse_rest_of_trait(
    input: ParseStream,
    mut attrs: Vec<Attribute>,
    vis: Visibility,
    unsafety: Option<Token![unsafe]>,
    auto_token: Option<Token![auto]>,
    trait_token: Token![trait],
    path: Path,
    mut generics: Generics,
) -> syn::Result<ItemTraitPath> {
    let colon_token: Option<Token![:]> = input.parse()?;

    let mut supertraits = Punctuated::new();
    if colon_token.is_some() {
        loop {
            if input.peek(Token![where]) || input.peek(token::Brace) {
                break;
            }
            supertraits.push_value(input.parse()?);
            if input.peek(Token![where]) || input.peek(token::Brace) {
                break;
            }
            supertraits.push_punct(input.parse()?);
        }
    }

    generics.where_clause = input.parse()?;

    let content;
    let brace_token = braced!(content in input);
    parse_inner(&content, &mut attrs)?;
    let mut items = Vec::new();
    while !content.is_empty() {
        items.push(content.parse()?);
    }

    Ok(ItemTraitPath {
        attrs,
        vis,
        unsafety,
        auto_token,
        restriction: None,
        trait_token,
        path,
        generics,
        colon_token,
        supertraits,
        brace_token,
        items,
    })
}

fn parse_inner(input: ParseStream, attrs: &mut Vec<Attribute>) -> syn::Result<()> {
    while input.peek(Token![#]) && input.peek2(Token![!]) {
        attrs.push(input.call(single_parse_inner)?);
    }
    Ok(())
}

fn single_parse_inner(input: ParseStream) -> syn::Result<Attribute> {
    let content;
    Ok(Attribute {
        pound_token: input.parse()?,
        style: AttrStyle::Inner(input.parse()?),
        bracket_token: bracketed!(content in input),
        meta: content.parse()?,
    })
}

impl ToTokens for ItemTraitPath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.vis.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.auto_token.to_tokens(tokens);
        self.trait_token.to_tokens(tokens);
        self.path.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        if !self.supertraits.is_empty() {
            self.colon_token
                .clone()
                .unwrap_or_default()
                .to_tokens(tokens);
            self.supertraits.to_tokens(tokens);
        }
        self.generics.where_clause.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_all(&self.items);
        });
    }
}
