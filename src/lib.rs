pub mod builds;

mod config;
pub use config::TraitConfig;

mod context;
pub use context::Context;

mod dynamic_rename;

mod generics;
pub use generics::GenericIdent;

mod input;
pub use input::parse_input;

pub mod prefixer;

mod trait_path;

mod trait_impl;
pub use delegate::delegate;
pub use trait_impl::generate_traits_match;
