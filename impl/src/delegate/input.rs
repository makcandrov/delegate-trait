use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

use crate::common::IdentMap;

#[derive(Clone)]
pub struct DelegateTraitInput {
    pub macro_ident: syn::Ident,
    pub crate_impl_ident: syn::Ident,
}

impl Parse for DelegateTraitInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let maps = Punctuated::<IdentMap<syn::Ident>, Comma>::parse_terminated(input)?;

        let mut macro_ident = None::<syn::Ident>;
        let mut crate_impl_ident = None::<syn::Ident>;

        let replace = |opt: &mut Option<syn::Ident>, map: IdentMap<syn::Ident>| {
            if opt.replace(map.to).is_some() {
                Err(syn::Error::new_spanned(map.from, "duplicate key"))
            } else {
                Ok(())
            }
        };

        for map in maps {
            match map.from.to_string().as_str() {
                "macro" => replace(&mut macro_ident, map)?,
                "impl" => replace(&mut crate_impl_ident, map)?,
                _ => return Err(syn::Error::new_spanned(map.from, "unknown key")),
            }
        }

        let result = Self {
            macro_ident: macro_ident.ok_or(syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing key `macro`",
            ))?,
            crate_impl_ident: crate_impl_ident.ok_or(syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing key `impl`",
            ))?,
        };

        Ok(result)
    }
}
