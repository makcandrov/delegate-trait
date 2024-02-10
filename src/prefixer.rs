use std::collections::HashSet;

use syn::{Path, Type, TypeParamBound};

pub struct Prefixer {
    pub prefix: Path,
    pub crates: HashSet<String>,
}

impl Prefixer {
    pub fn prefix_path(&self, path: &mut Path) {
        let Some(first) = path.segments.first() else {
            return;
        };
        if self.crates.contains(&first.ident.to_string()) {
            for prefix_segment in self.prefix.segments.iter().rev() {
                path.segments.insert(0, prefix_segment.clone());
            }
            path.leading_colon = self.prefix.leading_colon.clone();
        }
    }

    pub fn prefix_type(&self, ty: &mut Type) {
        match ty {
            Type::Array(arr) => self.prefix_type(&mut arr.elem),
            Type::BareFn(bare_fn) => bare_fn.inputs.iter_mut().for_each(|arg| self.prefix_type(&mut arg.ty)),
            Type::Group(group) => self.prefix_type(&mut group.elem),
            Type::ImplTrait(impl_trait) => impl_trait
                .bounds
                .iter_mut()
                .for_each(|bound| self.prefix_type_param_bound(bound)),
            Type::Infer(_) => (),
            Type::Macro(_) => (),
            Type::Never(_) => (),
            Type::Paren(paren) => self.prefix_type(&mut paren.elem),
            Type::Path(path) => self.prefix_path(&mut path.path),
            Type::Ptr(ptr) => self.prefix_type(&mut ptr.elem),
            Type::Reference(reference) => self.prefix_type(&mut reference.elem),
            Type::Slice(slice) => self.prefix_type(&mut slice.elem),
            Type::TraitObject(trait_object) => trait_object
                .bounds
                .iter_mut()
                .for_each(|bound| self.prefix_type_param_bound(bound)),
            Type::Tuple(tuple) => tuple.elems.iter_mut().for_each(|ty| self.prefix_type(ty)),
            Type::Verbatim(_) => (),
            _ => (),
        }
    }

    pub fn prefix_type_param_bound(&self, type_param_bound: &mut TypeParamBound) {
        match type_param_bound {
            TypeParamBound::Trait(trait_bound) => self.prefix_path(&mut trait_bound.path),
            _ => (),
        }
    }
}
