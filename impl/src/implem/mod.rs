use quote::quote;
use syn::parse_macro_input;

mod attributes;
use attributes::generate_attributes;

mod delegated_trait;
use delegated_trait::DelegatedTrait;

mod input;
use input::{DelegateTraitImplInput, DelegatedTraitInput};

mod source;
use source::DelegatedTraitSource;

pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DelegateTraitImplInput);
    match try_expand(input) {
        Ok(res) => res.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn try_expand(input: DelegateTraitImplInput) -> syn::Result<proc_macro2::TokenStream> {
    let traits = input.into_delegated_traits()?;
    let traits_match = quote! {};

    let attributes = generate_attributes();
    let hashtag = quote! { # };

    let result = quote! {
        #attributes

        use attribute::DelegateAttribute;

        #[doc(hidden)]
        #[proc_macro_attribute]
        pub fn delegate_trait (args: ::proc_macro::TokenStream, input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            let args = ::syn::parse_macro_input!(args as DelegateAttribute);
            let input = ::syn::parse_macro_input!(input as ::syn::DeriveInput);
            let derived = derive(args, &input);
            ::quote::quote! {#hashtag input #hashtag derived}.into()
        }

        fn derive(args: DelegateAttribute, input: &::syn::DeriveInput) -> ::proc_macro2::TokenStream {
            match try_expand(args, input) {
                Ok(expanded) => expanded,
                Err(err) => {
                    err.to_compile_error()
                }
            }
        }

        fn try_expand(attribute: DelegateAttribute, input: &::syn::DeriveInput) -> ::syn::Result<::proc_macro2::TokenStream> {
            let Some(last_segment) = attribute.path.segments.last() else {
                return Err(syn::Error::new_spanned(&attribute.path, "ident expected"));
            };
            let trait_ident = &last_segment.ident;
            let trait_ident_string = trait_ident.to_string();

            let trait_implem: ::proc_macro2::TokenStream = match trait_ident_string.as_str() {
                #traits_match
                _ => return ::syn::Result::Err(::syn::Error::new_spanned(trait_ident, &format!("Unknown trait {}.", trait_ident_string))),
            };

            Ok(quote::quote!{})
        }
    };

    Ok(result)
}
