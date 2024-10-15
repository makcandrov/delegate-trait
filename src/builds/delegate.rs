use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, File};

use crate::input::DelegateInput;
use crate::parse_input;

pub fn generate_delegate_build_string<P: AsRef<Path>>(path: P) -> String {
    match parse_input(path.as_ref()) {
        Ok(input) => prettyplease::unparse(
            &parse2::<File>(generate_crate_build(&input)).expect("prettyplease: unparse failed"),
        ),
        Err(err) => err.to_compile_error().to_string(),
    }
}

fn generate_crate_build(input: &DelegateInput) -> TokenStream {
    let delegate_impl_ident = &input.crate_impl_ident;
    let macro_ident = &input.macro_ident;

    quote! {
        pub use #delegate_impl_ident:: #macro_ident;

        #[doc(hidden)]
        pub mod __private {
            pub use delegate_trait::delegate;
        }
    }
}
