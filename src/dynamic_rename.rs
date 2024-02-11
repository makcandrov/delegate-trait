use std::collections::HashMap;

use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    AngleBracketedGenericArguments,
    BoundLifetimes,
    FnArg,
    GenericArgument,
    GenericParam,
    Ident,
    Lifetime,
    LifetimeParam,
    ParenthesizedGenericArguments,
    PatType,
    Path,
    PathArguments,
    PathSegment,
    Receiver,
    ReturnType,
    Signature,
    Token,
    TraitBound,
    TraitItemFn,
    Type,
    TypeParamBound,
    TypeTraitObject,
};

use crate::GenericIdent;

pub struct DynamicGenericRenamer {
    renames: HashMap<GenericIdent, Ident>,
}

impl DynamicGenericRenamer {
    pub fn new(renames: HashMap<GenericIdent, Ident>) -> Self {
        Self { renames }
    }

    pub fn renamed_type(&self, tokens: &mut TokenStream, ty: &Type) {
        match ty {
            Type::Array(arr) => {
                arr.bracket_token.surround(tokens, |tokens| {
                    self.renamed_type(tokens, &arr.elem);
                    arr.elem.to_tokens(tokens);
                    arr.semi_token.to_tokens(tokens);
                    arr.len.to_tokens(tokens);
                });
            },
            Type::BareFn(_) => todo!(),
            Type::Group(_) => todo!(),
            Type::ImplTrait(_) => todo!(),
            Type::Infer(infer) => infer.to_tokens(tokens),
            Type::Macro(_) => todo!(),
            Type::Never(never) => never.to_tokens(tokens),
            Type::Paren(paren) => {
                paren.paren_token.surround(tokens, |tokens| {
                    self.renamed_type(tokens, &paren.elem);
                });
            },
            Type::Path(type_path) => {
                let path = &type_path.path;
                let qself = match &type_path.qself {
                    Some(qself) => qself,
                    None => {
                        self.renamed_path(tokens, path);
                        return;
                    },
                };
                qself.lt_token.to_tokens(tokens);
                self.renamed_type(tokens, &qself.ty);

                let pos = std::cmp::min(qself.position, path.segments.len());
                let mut segments = path.segments.pairs();
                if pos > 0 {
                    qself.as_token.unwrap_or_default().to_tokens(tokens);
                    path.leading_colon.to_tokens(tokens);
                    for (i, segment) in segments.by_ref().take(pos).enumerate() {
                        if i + 1 == pos {
                            self.renamed_path_segment(tokens, segment.value());
                            qself.gt_token.to_tokens(tokens);
                            segment.punct().to_tokens(tokens);
                        } else {
                            self.renamed_path_segment(tokens, segment.value());
                            segment.punct().to_tokens(tokens);
                        }
                    }
                } else {
                    qself.gt_token.to_tokens(tokens);
                    path.leading_colon.to_tokens(tokens);
                }
                for segment in segments {
                    self.renamed_path_segment(tokens, segment.value());
                    segment.punct().to_tokens(tokens);
                }
            },
            Type::Ptr(_) => todo!(),
            Type::Reference(reference) => {
                reference.and_token.to_tokens(tokens);
                reference
                    .lifetime
                    .as_ref()
                    .map(|lifetime| self.renamed_lifetime(tokens, lifetime));
                reference.mutability.to_tokens(tokens);
                self.renamed_type(tokens, &reference.elem);
            },
            Type::Slice(_) => todo!(),
            Type::TraitObject(trait_object) => self.renamed_type_trait_object(tokens, trait_object),
            Type::Tuple(tuple) => {
                tuple.paren_token.surround(tokens, |tokens| {
                    tuple.elems.iter().for_each(|ty| self.renamed_type(tokens, ty));
                    // If we only have one argument, we need a trailing comma to
                    // distinguish TypeTuple from TypeParen.
                    if tuple.elems.len() == 1 && !tuple.elems.trailing_punct() {
                        <Token![,]>::default().to_tokens(tokens);
                    }
                });
            },
            Type::Verbatim(_) => todo!(),
            _ => todo!(),
        }
    }

    pub fn renamed_lifetime(&self, tokens: &mut TokenStream, lifetime: &Lifetime) {
        if let Some(ident) = self.renames.get(&GenericIdent::Lifetime(lifetime.ident.clone())) {
            let hashtag = quote! { # };
            let mut apostrophe = Punct::new('\'', Spacing::Joint);
            apostrophe.set_span(lifetime.apostrophe);
            tokens.extend(quote! { #apostrophe #hashtag #ident });
        } else {
            lifetime.to_tokens(tokens);
        }
    }

    pub fn renamed_fn_arg(&self, tokens: &mut TokenStream, fn_arg: &FnArg) {
        match fn_arg {
            FnArg::Receiver(receiver) => self.renamed_receiver(tokens, receiver),
            FnArg::Typed(pat_type) => self.renamed_pat_type(tokens, pat_type),
        }
    }

    pub fn renamed_inputs(&self, tokens: &mut TokenStream, inputs: &Punctuated<FnArg, Comma>) {
        inputs.pairs().for_each(|pair| {
            self.renamed_fn_arg(tokens, pair.value());
            pair.punct().to_tokens(tokens);
        });
    }

    pub fn renamed_signature(&self, tokens: &mut TokenStream, signature: &Signature) {
        signature.constness.to_tokens(tokens);
        signature.asyncness.to_tokens(tokens);
        signature.unsafety.to_tokens(tokens);
        signature.abi.to_tokens(tokens);
        signature.fn_token.to_tokens(tokens);
        signature.ident.to_tokens(tokens);
        signature.generics.to_tokens(tokens); // todo
        signature.paren_token.surround(tokens, |tokens| {
            self.renamed_inputs(tokens, &signature.inputs);
            if let Some(variadic) = &signature.variadic {
                if !signature.inputs.empty_or_trailing() {
                    <Token![,]>::default().to_tokens(tokens);
                }
                variadic.to_tokens(tokens);
            }
        });
        self.renamed_return_type(tokens, &signature.output);
        signature.generics.where_clause.to_tokens(tokens);
    }

    pub fn renamed_path_arguments(&self, tokens: &mut TokenStream, path_arguments: &PathArguments) {
        match path_arguments {
            PathArguments::None => PathArguments::None.to_tokens(tokens),
            PathArguments::AngleBracketed(args) => self.renamed_angle_bracketed_generic_arguments(tokens, args),
            PathArguments::Parenthesized(args) => self.renamed_parenthesized_generic_arguments(tokens, args),
        }
    }

    pub fn renamed_angle_bracketed_generic_arguments(
        &self,
        tokens: &mut TokenStream,
        args: &AngleBracketedGenericArguments,
    ) {
        args.colon2_token.to_tokens(tokens);
        args.lt_token.to_tokens(tokens);

        // Print lifetimes before types/consts/bindings, regardless of their
        // order in self.args.
        let mut trailing_or_empty = true;
        for param in args.args.pairs() {
            match param.value() {
                GenericArgument::Lifetime(lifetime) => {
                    self.renamed_lifetime(tokens, lifetime);
                    param.punct().to_tokens(tokens);
                    trailing_or_empty = param.punct().is_some();
                },
                _ => {},
            }
        }
        for param in args.args.pairs() {
            match param.value() {
                GenericArgument::Type(_)
                | GenericArgument::Const(_)
                | GenericArgument::AssocType(_)
                | GenericArgument::AssocConst(_)
                | GenericArgument::Constraint(_) => {
                    if !trailing_or_empty {
                        <Token![,]>::default().to_tokens(tokens);
                    }
                    self.renamed_generic_argument(tokens, param.value());
                    param.punct().to_tokens(tokens);
                    trailing_or_empty = param.punct().is_some();
                },
                _ => {},
            }
        }

        args.gt_token.to_tokens(tokens);
    }

    pub fn renamed_parenthesized_generic_arguments(
        &self,
        _tokens: &mut TokenStream,
        _args: &ParenthesizedGenericArguments,
    ) {
        todo!()
    }

    pub fn renamed_return_type(&self, tokens: &mut TokenStream, return_type: &ReturnType) {
        match return_type {
            ReturnType::Default => ReturnType::Default.to_tokens(tokens),
            ReturnType::Type(arrow, ty) => {
                arrow.to_tokens(tokens);
                self.renamed_type(tokens, ty);
            },
        }
    }

    pub fn renamed_trait_item_fn(&self, tokens: &mut TokenStream, trait_item_fn: &TraitItemFn) {
        // tokens.append_all(self.attrs.outer());
        self.renamed_signature(tokens, &trait_item_fn.sig);
        match &trait_item_fn.default {
            Some(block) => {
                block.brace_token.surround(tokens, |tokens| {
                    // tokens.append_all(trait_item_fn.attrs.inner());
                    tokens.append_all(&block.stmts);
                });
            },
            None => {
                trait_item_fn.semi_token.to_tokens(tokens);
            },
        }
    }

    pub fn renamed_generic_argument(&self, tokens: &mut TokenStream, generic_argument: &GenericArgument) {
        match generic_argument {
            GenericArgument::Lifetime(lt) => self.renamed_lifetime(tokens, lt),
            GenericArgument::Type(ty) => {
                self.renamed_type(tokens, ty);
            },
            _ => generic_argument.to_tokens(tokens),
        }
    }

    pub fn renamed_type_trait_object(&self, tokens: &mut TokenStream, type_trait_object: &TypeTraitObject) {
        type_trait_object.dyn_token.to_tokens(tokens);
        type_trait_object.bounds.pairs().for_each(|pair| {
            self.renamed_type_param_bound(tokens, pair.value());
            pair.punct().to_tokens(tokens);
        });
    }

    pub fn renamed_type_param_bound(&self, tokens: &mut TokenStream, type_param_bound: &TypeParamBound) {
        match type_param_bound {
            TypeParamBound::Trait(trait_bound) => self.renamed_trait_bound(tokens, trait_bound),
            TypeParamBound::Lifetime(lt) => self.renamed_lifetime(tokens, lt),
            _ => type_param_bound.to_tokens(tokens),
        }
    }

    pub fn renamed_trait_bound(&self, tokens: &mut TokenStream, trait_bound: &TraitBound) {
        let to_tokens = |tokens: &mut TokenStream| {
            trait_bound.modifier.to_tokens(tokens);
            trait_bound
                .lifetimes
                .as_ref()
                .map(|lt| self.renamed_bound_lifetimes(tokens, lt));
            self.renamed_path(tokens, &trait_bound.path);
        };
        match &trait_bound.paren_token {
            Some(paren) => paren.surround(tokens, to_tokens),
            None => to_tokens(tokens),
        }
    }

    pub fn renamed_bound_lifetimes(&self, tokens: &mut TokenStream, bound_lifetimes: &BoundLifetimes) {
        bound_lifetimes.for_token.to_tokens(tokens);
        bound_lifetimes.lt_token.to_tokens(tokens);
        bound_lifetimes.lifetimes.pairs().for_each(|pair| {
            self.renamed_generic_param(tokens, pair.value());
            pair.punct().to_tokens(tokens);
        });
        bound_lifetimes.gt_token.to_tokens(tokens);
    }

    pub fn renamed_generic_param(&self, tokens: &mut TokenStream, gemeric_param: &GenericParam) {
        match gemeric_param {
            GenericParam::Lifetime(lifetime_param) => self.renamed_lifetime_param(tokens, lifetime_param),
            GenericParam::Type(_) => todo!(),
            GenericParam::Const(_) => todo!(),
        }
    }

    pub fn renamed_lifetime_param(&self, tokens: &mut TokenStream, lifetime_param: &LifetimeParam) {
        // tokens.append_all(self.attrs.outer());
        self.renamed_lifetime(tokens, &lifetime_param.lifetime);
        if !lifetime_param.bounds.is_empty() {
            lifetime_param.colon_token.unwrap_or_default().to_tokens(tokens);
            lifetime_param.bounds.pairs().for_each(|pair| {
                self.renamed_lifetime(tokens, pair.value());
                pair.punct().to_tokens(tokens);
            });
        }
    }

    pub fn renamed_path(&self, tokens: &mut TokenStream, path: &Path) {
        if path.leading_colon.is_none() && path.segments.len() == 1 {
            let segment = path.segments.first().unwrap();
            if segment.arguments.is_none() {
                if let Some(ident) = self.renames.get(&GenericIdent::Other(segment.ident.clone())) {
                    let hashtag = quote! { # };
                    tokens.extend(quote! { #hashtag #ident });
                    return;
                }
            }
        }
        path.leading_colon.to_tokens(tokens);
        path.segments.pairs().for_each(|pair| {
            self.renamed_path_segment(tokens, pair.value());
            pair.punct().to_tokens(tokens);
        });
    }

    pub fn renamed_path_segment(&self, tokens: &mut TokenStream, path: &PathSegment) {
        path.ident.to_tokens(tokens);
        self.renamed_path_arguments(tokens, &path.arguments);
    }

    pub fn renamed_receiver(&self, tokens: &mut TokenStream, receiver: &Receiver) {
        // tokens.append_all(receiver.attrs.outer());
        if let Some((ampersand, lifetime)) = &receiver.reference {
            ampersand.to_tokens(tokens);
            lifetime
                .as_ref()
                .map(|lifetime| self.renamed_lifetime(tokens, lifetime));
        }
        receiver.mutability.to_tokens(tokens);
        receiver.self_token.to_tokens(tokens);
        if let Some(colon_token) = &receiver.colon_token {
            colon_token.to_tokens(tokens);
            self.renamed_type(tokens, &receiver.ty);
        } else {
            let consistent = match (&receiver.reference, &receiver.mutability, &*receiver.ty) {
                (Some(_), mutability, Type::Reference(ty)) => {
                    mutability.is_some() == ty.mutability.is_some()
                        && match &*ty.elem {
                            Type::Path(ty) => ty.qself.is_none() && ty.path.is_ident("Self"),
                            _ => false,
                        }
                },
                (None, _, Type::Path(ty)) => ty.qself.is_none() && ty.path.is_ident("Self"),
                _ => false,
            };
            if !consistent {
                <Token![:]>::default().to_tokens(tokens);
                self.renamed_type(tokens, &receiver.ty);
            }
        }
    }

    pub fn renamed_pat_type(&self, tokens: &mut TokenStream, pat_type: &PatType) {
        // tokens.append_all(self.attrs.outer());
        pat_type.pat.to_tokens(tokens);
        pat_type.colon_token.to_tokens(tokens);
        self.renamed_type(tokens, &pat_type.ty);
    }
}
