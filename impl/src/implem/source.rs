use syn::parse::{Parse, ParseStream};

#[derive(Clone)]
pub enum DelegatedTraitSource {
    Import(syn::LitStr),
    Code(syn::File),
}

impl Parse for DelegatedTraitSource {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(lit) = input.parse::<syn::LitStr>() {
            return Ok(Self::Import(lit));
        }

        let content;
        syn::braced!(content in input);
        let file = content.parse::<syn::File>()?;
        Ok(Self::Code(file))
    }
}
