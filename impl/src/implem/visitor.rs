use std::collections::HashMap;

use syn::{
    visit_mut::{self, VisitMut},
    AngleBracketedGenericArguments, GenericArgument, GenericParam, Generics,
};

#[derive(Clone)]
pub struct RuntimeVisitor {
    lifetime_renames: HashMap<syn::Ident, syn::Ident>,
    generic_renames: HashMap<syn::Ident, syn::Type>,
    const_renames: HashMap<syn::Ident, syn::Expr>,
}

impl RuntimeVisitor {
    pub fn new(definition: &Generics, arguments: &AngleBracketedGenericArguments) -> Self {
        assert_eq!(definition.params.len(), arguments.args.len());

        let mut lifetime_renames = HashMap::new();
        let mut generic_renames = HashMap::new();
        let mut const_renames = HashMap::new();

        for (def, arg) in definition.params.iter().zip(arguments.args.iter()) {
            match (def, arg) {
                (GenericParam::Lifetime(def), GenericArgument::Lifetime(arg)) => {
                    lifetime_renames.insert(def.lifetime.ident.clone(), arg.ident.clone());
                }
                (GenericParam::Type(def), GenericArgument::Type(arg)) => {
                    generic_renames.insert(def.ident.clone(), arg.clone());
                }
                (GenericParam::Const(def), GenericArgument::Const(arg)) => {
                    const_renames.insert(def.ident.clone(), arg.clone());
                }
                _ => panic!("inconsistent generics"),
            }
        }

        Self {
            lifetime_renames,
            generic_renames,
            const_renames,
        }
    }
}

impl VisitMut for RuntimeVisitor {
    fn visit_path_mut(&mut self, i: &mut syn::Path) {
        if let Some(first) = i.segments.first_mut() {
            if let Some(e) = self.generic_renames.get(&first.ident) {}
        }
        visit_mut::visit_path_mut(self, i);
    }
}
