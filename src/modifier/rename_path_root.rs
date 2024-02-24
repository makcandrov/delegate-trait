use super::{LookupTokenModifier, TokenModifier};

pub struct PathRootRenamer {
    pub original: String,
    pub rename: syn::Ident,
    pub remove_leading_colon: bool,
}

impl TokenModifier for PathRootRenamer {
    fn modify_path(&self, item: &mut syn::Path) {
        if item.segments.len() > 1 {
            let first = item.segments.first_mut().unwrap();
            if first.ident.to_string() == self.original {
                first.ident = self.rename.clone();
                if self.remove_leading_colon {
                    item.leading_colon = None;
                }
            }
        }
        LookupTokenModifier(self).modify_path(item);
    }
}
