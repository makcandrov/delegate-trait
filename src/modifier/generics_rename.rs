use std::collections::{HashMap, HashSet};

use syn::Ident;

use super::TokenModifier;

#[derive(Debug, Default, Clone)]
pub struct GenericsRenamer {
    types_renames: HashMap<String, Ident>,
    types_renamed: HashSet<String>,
    lifetimes_renames: HashMap<String, Ident>,
    lifetimes_renamed: HashSet<String>,
}

impl GenericsRenamer {
    pub fn insert_type(&mut self, original: String, rename: Ident) {
        self.types_renamed.insert(rename.to_string());
        self.types_renames.insert(original, rename);
    }

    pub fn insert_lifetime(&mut self, original: String, rename: Ident) {
        self.lifetimes_renamed.insert(rename.to_string());
        self.lifetimes_renames.insert(original, rename);
    }

    // pub fn remove_type(&mut self, original: String) {
    //     let Some(rename) = self.types_renames.remove(&original) else {
    //         return;
    //     };
    //     self.types_renamed.remove(&rename.to_string());
    // }

    // pub fn remove_lifetime(&mut self, original: String) {
    //     let Some(rename) = self.lifetimes_renames.remove(&original) else {
    //         return;
    //     };
    //     self.lifetimes_renamed.remove(&rename.to_string());
    // }
}

impl TokenModifier for GenericsRenamer {
    fn modify_lifetime(&mut self, item: &mut syn::Lifetime) {
        if let Some(rename) = self.lifetimes_renames.get(&item.ident.to_string()) {
            item.ident = rename.clone();
        }
    }

    fn modify_ident(&mut self, item: &mut syn::Ident) {
        if let Some(rename) = self.types_renames.get(&item.to_string()) {
            *item = rename.clone();
        }
    }
}
