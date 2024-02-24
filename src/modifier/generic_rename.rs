use std::collections::HashMap;

use syn::Ident;

use super::TokenModifier;

pub struct GenericRenamer {
    type_renames: HashMap<String, Ident>,
    lifetime_renames: HashMap<String, Ident>,
}

impl TokenModifier for GenericRenamer {}
