pub mod builds;

mod config;
pub use config::TraitConfig;

mod context;
pub use context::Context;

mod modifier;

mod generics;
pub use generics::GenericIdent;

mod input;
pub use input::parse_input;

mod trait_path;
pub use trait_path::ItemTraitPath;

mod trait_impl;
pub use delegate::delegate;
pub use trait_impl::{generate_trait_impl, generate_traits_match};
