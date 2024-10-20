use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_attributes() -> TokenStream {
    quote! {
        mod attribute {
            use ::syn::parse::Parse;
            use ::quote::ToTokens;

            mod kw {
                ::syn::custom_keyword!(to);
                ::syn::custom_keyword!(with);
            }

            #[derive(Clone)]
            pub struct DelegateAttribute {
                pub fo: ::syn::Generics,
                pub path: ::syn::Path,
                pub generics: ::syn::Generics,
                pub to: syn::Expr,
                pub wh: Option<::syn::WhereClause>,
                pub wi: Option<::proc_macro2::TokenStream>,
            }

            impl Parse for DelegateAttribute {
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    let fo = if input.parse::<syn::token::For>().is_ok() {
                        input.parse::<syn::Generics>()?
                    } else {
                        syn::Generics::default()
                    };

                    let mut path = input.parse::<::syn::Path>()?;

                    let arguments = core::mem::replace(
                        &mut path
                            .segments
                            .last_mut()
                            .expect("TraitConfig::parse: Ident expected")
                            .arguments,
                        ::syn::PathArguments::None,
                    );

                    let generics = ::syn::parse2::<::syn::Generics>(arguments.to_token_stream())?;

                    input.parse::<kw::to>()?;

                    let to = syn::Expr::parse_without_eager_brace(input)?;

                    let wh = if input.peek(::syn::Token![where]) {
                        Some(input.parse::<::syn::WhereClause>().unwrap())
                    } else {
                        None
                    };

                    let wi = if input.peek(kw::with) {
                        input.parse::<kw::with>().unwrap();
                        let content;
                        ::syn::braced!(content in input);
                        Some(content.parse::<::proc_macro2::TokenStream>()?)
                    } else {
                        None
                    };

                    Ok(Self {
                        fo,
                        path,
                        generics,
                        to,
                        wh,
                        wi,
                    })
                }
            }
        }
    }
}
