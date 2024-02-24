use super::TokenModifier;

#[derive(Debug)]
pub struct LookupTokenModifier<'a, T>(pub &'a mut T);

impl<'a, T: TokenModifier> TokenModifier for LookupTokenModifier<'a, T> {
    fn modify_trait_item_fn(&mut self, item: &mut syn::TraitItemFn) {
        self.0.modify_signature(&mut item.sig);
    }

    fn modify_signature(&mut self, item: &mut syn::Signature) {
        self.0.modify_ident(&mut item.ident);
        self.0.modify_generics(&mut item.generics);
        item.inputs.iter_mut().for_each(|input| self.0.modify_fn_arg(input));
        self.0.modify_return_type(&mut item.output)
    }

    fn modify_fn_arg(&mut self, item: &mut syn::FnArg) {
        match item {
            syn::FnArg::Receiver(receiver) => self.0.modify_receiver(receiver),
            syn::FnArg::Typed(pat_type) => self.0.modify_pat_type(pat_type),
        }
    }

    fn modify_receiver(&mut self, item: &mut syn::Receiver) {
        self.0.modify_type(&mut item.ty)
    }

    fn modify_type(&mut self, item: &mut syn::Type) {
        match item {
            syn::Type::Array(type_array) => self.0.modify_type_array(type_array),
            syn::Type::BareFn(type_bare_fn) => self.0.modify_type_bare_fn(type_bare_fn),
            syn::Type::Group(type_group) => self.0.modify_type_group(type_group),
            syn::Type::ImplTrait(_) => todo!(),
            syn::Type::Infer(_) => todo!(),
            syn::Type::Macro(_) => todo!(),
            syn::Type::Never(_) => todo!(),
            syn::Type::Paren(_) => todo!(),
            syn::Type::Path(type_path) => self.0.modify_type_path(type_path),
            syn::Type::Ptr(_) => todo!(),
            syn::Type::Reference(type_reference) => self.0.modify_type_reference(type_reference),
            syn::Type::Slice(_) => todo!(),
            syn::Type::TraitObject(_) => todo!(),
            syn::Type::Tuple(type_tuple) => self.0.modify_type_tuple(type_tuple),
            syn::Type::Verbatim(_) => (),
            _ => (),
        }
    }

    fn modify_type_array(&mut self, item: &mut syn::TypeArray) {
        self.0.modify_type(&mut item.elem);
    }

    fn modify_type_bare_fn(&mut self, item: &mut syn::TypeBareFn) {
        item.lifetimes
            .as_mut()
            .map(|lifetimes| self.0.modify_bound_lifetimes(lifetimes));
        item.inputs
            .iter_mut()
            .for_each(|input| self.0.modify_bare_fn_arg(input));
        self.0.modify_return_type(&mut item.output);
    }

    fn modify_bound_lifetimes(&mut self, item: &mut syn::BoundLifetimes) {
        item.lifetimes
            .iter_mut()
            .for_each(|generic_param| self.0.modify_generic_param(generic_param));
    }

    fn modify_bare_fn_arg(&mut self, item: &mut syn::BareFnArg) {
        item.name.as_mut().map(|(ident, _)| self.0.modify_ident(ident));
        self.0.modify_type(&mut item.ty)
    }

    fn modify_type_path(&mut self, item: &mut syn::TypePath) {
        item.qself.as_mut().map(|qself| self.0.modify_qself(qself));
        self.0.modify_path(&mut item.path)
    }

    fn modify_path(&mut self, item: &mut syn::Path) {
        item.segments
            .iter_mut()
            .for_each(|segment| self.0.modify_path_segment(segment));
    }

    fn modify_path_segment(&mut self, item: &mut syn::PathSegment) {
        self.0.modify_ident(&mut item.ident);
        self.0.modify_path_arguments(&mut item.arguments);
    }

    fn modify_path_arguments(&mut self, item: &mut syn::PathArguments) {
        match item {
            syn::PathArguments::AngleBracketed(arg) => self.0.modify_angle_bracketed_generic_argument(arg),
            syn::PathArguments::Parenthesized(arg) => self.0.modify_angle_parenthesized_generic_argument(arg),
            _ => (),
        }
    }

    fn modify_type_tuple(&mut self, item: &mut syn::TypeTuple) {
        item.elems.iter_mut().for_each(|elem| self.0.modify_type(elem))
    }

    fn modify_type_reference(&mut self, item: &mut syn::TypeReference) {
        item.lifetime.as_mut().map(|lifetime| self.0.modify_lifetime(lifetime));
        self.0.modify_type(&mut item.elem)
    }

    fn modify_lifetime(&mut self, _item: &mut syn::Lifetime) {}

    fn modify_generic_param(&mut self, item: &mut syn::GenericParam) {
        match item {
            syn::GenericParam::Lifetime(lifetime_param) => self.0.modify_lifetime_param(lifetime_param),
            syn::GenericParam::Type(type_param) => self.0.modify_type_param(type_param),
            syn::GenericParam::Const(const_param) => self.0.modify_const_param(const_param),
        }
    }

    fn modify_lifetime_param(&mut self, item: &mut syn::LifetimeParam) {
        self.0.modify_lifetime(&mut item.lifetime);
        item.bounds.iter_mut().for_each(|bound| self.0.modify_lifetime(bound));
    }

    fn modify_type_param(&mut self, item: &mut syn::TypeParam) {
        self.0.modify_ident(&mut item.ident);
        item.bounds
            .iter_mut()
            .for_each(|bound| self.0.modify_type_param_bound(bound));
    }

    fn modify_const_param(&mut self, item: &mut syn::ConstParam) {
        self.0.modify_type(&mut item.ty);
    }

    fn modify_pat_type(&mut self, item: &mut syn::PatType) {
        self.0.modify_type(&mut item.ty)
    }

    fn modify_angle_bracketed_generic_argument(&mut self, item: &mut syn::AngleBracketedGenericArguments) {
        item.args.iter_mut().for_each(|arg| self.0.modify_generic_argument(arg))
    }

    fn modify_angle_parenthesized_generic_argument(&mut self, item: &mut syn::ParenthesizedGenericArguments) {
        item.inputs.iter_mut().for_each(|input| self.0.modify_type(input));
        self.0.modify_return_type(&mut item.output);
    }

    fn modify_generic_argument(&mut self, item: &mut syn::GenericArgument) {
        match item {
            syn::GenericArgument::Lifetime(lfietime) => self.0.modify_lifetime(lfietime),
            syn::GenericArgument::Type(ty) => self.0.modify_type(ty),
            syn::GenericArgument::Const(_) => todo!(),
            syn::GenericArgument::AssocType(assoc_type) => self.0.modify_assoc_type(assoc_type),
            syn::GenericArgument::AssocConst(_) => todo!(),
            syn::GenericArgument::Constraint(_) => todo!(),
            _ => (),
        }
    }

    fn modify_return_type(&mut self, item: &mut syn::ReturnType) {
        match item {
            syn::ReturnType::Type(_, ty) => self.0.modify_type(ty),
            _ => (),
        }
    }

    fn modify_type_group(&mut self, item: &mut syn::TypeGroup) {
        self.0.modify_type(&mut item.elem)
    }

    fn modify_qself(&mut self, item: &mut syn::QSelf) {
        self.0.modify_type(&mut item.ty)
    }

    fn modify_generics(&mut self, item: &mut syn::Generics) {
        item.params
            .iter_mut()
            .for_each(|param| self.0.modify_generic_param(param));
        // todo: where clause
    }

    fn modify_ident(&mut self, _item: &mut syn::Ident) {}

    fn modify_type_param_bound(&mut self, item: &mut syn::TypeParamBound) {
        match item {
            syn::TypeParamBound::Trait(trait_bound) => self.0.modify_trait_bound(trait_bound),
            syn::TypeParamBound::Lifetime(lifetime) => self.0.modify_lifetime(lifetime),
            _ => (),
        }
    }

    fn modify_trait_bound(&mut self, item: &mut syn::TraitBound) {
        // todo: modifier
        item.lifetimes
            .as_mut()
            .map(|lifetimes| self.0.modify_bound_lifetimes(lifetimes));
    }

    fn modify_item_trait_path(&mut self, item: &mut crate::ItemTraitPath) {
        self.0.modify_path(&mut item.path);
        self.0.modify_generics(&mut item.generics);
        item.items
            .iter_mut()
            .for_each(|trait_item| self.0.modify_trait_item(trait_item));
    }

    fn modify_trait_item(&mut self, item: &mut syn::TraitItem) {
        match item {
            syn::TraitItem::Const(trait_item_const) => self.0.modify_trait_item_const(trait_item_const),
            syn::TraitItem::Fn(trait_item_fn) => self.0.modify_trait_item_fn(trait_item_fn),
            syn::TraitItem::Type(trait_item_type) => self.0.modify_trait_item_type(trait_item_type),
            _ => (),
        }
    }

    fn modify_trait_item_type(&mut self, item: &mut syn::TraitItemType) {
        self.0.modify_ident(&mut item.ident);
        self.0.modify_generics(&mut item.generics);
        item.bounds
            .iter_mut()
            .for_each(|bound| self.0.modify_type_param_bound(bound));
        item.default.as_mut().map(|(_, default)| self.0.modify_type(default));
    }

    fn modify_trait_item_const(&mut self, item: &mut syn::TraitItemConst) {
        self.0.modify_ident(&mut item.ident);
        self.0.modify_generics(&mut item.generics);
        self.0.modify_type(&mut item.ty);
        // todo: default
    }

    fn modify_assoc_type(&mut self, item: &mut syn::AssocType) {
        self.0.modify_ident(&mut item.ident);
        item.generics
            .as_mut()
            .map(|generics| self.0.modify_angle_bracketed_generic_argument(generics));
        self.0.modify_type(&mut item.ty);
    }
}
