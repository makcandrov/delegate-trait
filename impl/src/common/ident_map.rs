use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::FatArrow,
};

#[derive(Clone)]
pub struct IdentMap<T> {
    pub from: syn::Ident,
    pub to: T,
}

impl<T: Parse> Parse for IdentMap<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let from = syn::Ident::parse_any(input)?;
        let _ = input.parse::<FatArrow>()?;
        let to = input.parse::<T>()?;
        Ok(Self { from, to })
    }
}
