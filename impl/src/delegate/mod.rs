use input::DelegateTraitInput;
use quote::quote;
use syn::parse_macro_input;

mod input;

pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DelegateTraitInput);
    match try_expand(input) {
        Ok(res) => res.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn try_expand(input: DelegateTraitInput) -> syn::Result<proc_macro2::TokenStream> {
    let DelegateTraitInput {
        macro_ident,
        crate_impl_ident,
    } = input;

    #[cfg(feature = "local-imports")]
    let reexports = quote! {
        // todo
    };

    #[cfg(not(feature = "local-imports"))]
    let reexports = quote! {};

    let res = quote::quote! {
        pub use #crate_impl_ident:: delegate_trait as #macro_ident;

        #[doc(hidden)]
        pub mod __private {
            pub use ::delegate_trait::__private::delegate;
        }
        #reexports
    };

    Ok(res)
}
