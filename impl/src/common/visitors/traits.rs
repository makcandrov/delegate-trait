use std::collections::HashMap;

use syn::{
    punctuated::Punctuated,
    visit_mut::{self, VisitMut},
    ItemTrait, Path, PathArguments, PathSegment,
};

pub struct TraitsVisitor {
    imports: HashMap<String, Path>,
    current_mod_path: Path,
    current_use_path: Path,
    pub traits: HashMap<String, ItemTrait>,
    inside_trait: bool,
}

impl TraitsVisitor {
    pub fn new() -> Self {
        Self {
            imports: HashMap::new(),
            current_mod_path: empty_path(),
            current_use_path: empty_path(),
            traits: HashMap::new(),
            inside_trait: false,
        }
    }

    pub fn insert_mod_import(&mut self, ident: syn::Ident) {
        let mut path = self.current_mod_path.clone();
        path.segments.push(ident_to_path_segment(ident.clone()));
        self.imports.insert(ident.to_string(), path);
    }

    pub fn insert_use_import(&mut self, ident: syn::Ident) {
        self.insert_renamed_use_import(ident.clone(), ident);
    }

    pub fn insert_renamed_use_import(&mut self, ident: syn::Ident, rename: syn::Ident) {
        let mut path = self.current_use_path.clone();
        path.segments.push(ident_to_path_segment(ident));
        self.imports.insert(rename.to_string(), path);
    }
}

impl VisitMut for TraitsVisitor {
    // Methods

    fn visit_item_trait_mut(&mut self, i: &mut ItemTrait) {
        self.inside_trait = true;
        visit_mut::visit_item_trait_mut(self, i);
        self.inside_trait = false;
        self.traits.insert(i.ident.to_string(), i.clone());
    }

    fn visit_trait_item_fn_mut(&mut self, i: &mut syn::TraitItemFn) {
        if !self.inside_trait {
            return;
        }
        i.default = None;
        visit_mut::visit_trait_item_fn_mut(self, i);
    }

    fn visit_path_mut(&mut self, i: &mut Path) {
        if !self.inside_trait {
            return;
        }
        let first_ident = &i.segments.first().as_ref().unwrap().ident;
        if let Some(path) = self.imports.get(&first_ident.to_string()) {
            let mut new_segments = path.segments.clone();
            new_segments.pop().unwrap();
            new_segments.extend(i.segments.clone());
            i.segments = new_segments;
        }
        visit_mut::visit_path_mut(self, i);
    }

    // Imports

    fn visit_item_struct_mut(&mut self, i: &mut syn::ItemStruct) {
        self.insert_mod_import(i.ident.clone());
        visit_mut::visit_item_struct_mut(self, i);
    }

    fn visit_item_use_mut(&mut self, i: &mut syn::ItemUse) {
        self.current_use_path = empty_path();
        visit_mut::visit_item_use_mut(self, i);
        self.current_use_path = empty_path();
    }

    fn visit_use_path_mut(&mut self, i: &mut syn::UsePath) {
        self.current_use_path
            .segments
            .push(ident_to_path_segment(i.ident.clone()));
        visit_mut::visit_use_path_mut(self, i);
        self.current_use_path.segments.pop().unwrap();
    }

    fn visit_use_name_mut(&mut self, i: &mut syn::UseName) {
        self.insert_use_import(i.ident.clone());
        visit_mut::visit_use_name_mut(self, i);
    }

    fn visit_use_rename_mut(&mut self, i: &mut syn::UseRename) {
        self.insert_renamed_use_import(i.ident.clone(), i.rename.clone());
        visit_mut::visit_use_rename_mut(self, i);
    }

    fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
        self.current_mod_path
            .segments
            .push(ident_to_path_segment(i.ident.clone()));
        visit_mut::visit_item_mod_mut(self, i);
        self.current_mod_path.segments.pop().unwrap();
    }
}

pub fn ident_to_path_segment(ident: syn::Ident) -> PathSegment {
    PathSegment {
        ident,
        arguments: PathArguments::None,
    }
}

pub fn empty_path() -> Path {
    let segments = Punctuated::new();
    Path {
        leading_colon: None,
        segments,
    }
}
