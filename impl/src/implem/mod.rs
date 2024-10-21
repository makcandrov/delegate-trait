use quote::quote;
use syn::parse_macro_input;

mod attributes;
use attributes::generate_attributes;

mod delegated_trait;
use delegated_trait::DelegatedTrait;

mod input;
use input::DelegateTraitImplInput;

mod merges;
pub use merges::merge_methods;

mod source;
use source::DelegatedTraitSource;

mod visitor;

pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DelegateTraitImplInput);
    match try_expand(input) {
        Ok(res) => res.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn try_expand(input: DelegateTraitImplInput) -> syn::Result<proc_macro2::TokenStream> {
    let crate_ident = input.crate_ident.clone();
    let traits = input.into_delegated_traits()?;
    let mut traits_match = quote! {};

    for tr in traits {
        traits_match.extend(tr.generate_match_branch(crate_ident.clone()));
    }

    // Methods used for merging generics, where clauses, etc.
    let attributes = generate_attributes();

    // Module containing the proc macro attribute structure.
    let merge_methods = merge_methods();

    let hashtag = quote! { # };

    let result = quote! {
        use :: delegate_trait ::__private::proc_macro2;
        use :: delegate_trait ::__private::syn;
        use :: delegate_trait ::__private::quote;

        #merge_methods

        #attributes

        use attribute::DelegateAttribute;

        #[doc(hidden)]
        #[proc_macro_attribute]
        pub fn delegate_trait (args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let args =syn::parse_macro_input!(args as DelegateAttribute);
            let input =syn::parse_macro_input!(input as syn::DeriveInput);
            let derived = derive(args, &input);
           quote::quote! {#hashtag input #hashtag derived}.into()
        }

        fn derive(args: DelegateAttribute, input: &syn::DeriveInput) -> proc_macro2::TokenStream {
            match try_expand(args, input) {
                Ok(expanded) => expanded,
                Err(err) => {
                    err.to_compile_error()
                }
            }
        }

        fn try_expand(attribute: DelegateAttribute, derive_input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
            let Some(last_segment) = attribute.path.segments.last() else {
                return Err(syn::Error::new_spanned(&attribute.path, "ident expected"));
            };
            let trait_ident = &last_segment.ident;
            let trait_ident_string = trait_ident.to_string();

            let trait_implem: proc_macro2::TokenStream = match trait_ident_string.as_str() {
                #traits_match
                _ => return syn::Result::Err(syn::Error::new_spanned(trait_ident, &format!("Unknown trait {}.", trait_ident_string))),
            };

            #[allow(unreachable_code)]
            Ok(trait_implem)
        }
    };

    Ok(result)
}
