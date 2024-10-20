#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![doc = include_str!("../README.md")]

pub use delegate_trait_impl::{delegate_trait, delegate_trait_impl};

#[doc(hidden)]
pub mod __private {
    pub use delegate::delegate;
}
