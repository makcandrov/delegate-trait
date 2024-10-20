mod ident_map;
pub use ident_map::IdentMap;

pub fn try_braced<'a, 'b>(
    input: &'a syn::parse::ParseBuffer<'b>,
) -> syn::Result<syn::parse::ParseBuffer<'b>> {
    let content;
    syn::braced!(content in input);
    Ok(content)
}
