mod generics_rename;
pub use generics_rename::GenericsRenamer;

mod lookup;
pub use lookup::LookupTokenModifier;

mod rename_path_root;
pub use rename_path_root::PathRootRenamer;

pub trait TokenModifier: Sized {
    fn modify_trait_item_fn(&mut self, item: &mut syn::TraitItemFn) {
        LookupTokenModifier(self).modify_trait_item_fn(item);
    }

    fn modify_signature(&mut self, item: &mut syn::Signature) {
        LookupTokenModifier(self).modify_signature(item)
    }

    fn modify_fn_arg(&mut self, item: &mut syn::FnArg) {
        LookupTokenModifier(self).modify_fn_arg(item)
    }

    fn modify_receiver(&mut self, item: &mut syn::Receiver) {
        LookupTokenModifier(self).modify_receiver(item)
    }

    fn modify_type(&mut self, item: &mut syn::Type) {
        LookupTokenModifier(self).modify_type(item)
    }

    fn modify_type_array(&mut self, item: &mut syn::TypeArray) {
        LookupTokenModifier(self).modify_type_array(item)
    }

    fn modify_type_bare_fn(&mut self, item: &mut syn::TypeBareFn) {
        LookupTokenModifier(self).modify_type_bare_fn(item)
    }

    fn modify_bound_lifetimes(&mut self, item: &mut syn::BoundLifetimes) {
        LookupTokenModifier(self).modify_bound_lifetimes(item)
    }

    fn modify_bare_fn_arg(&mut self, item: &mut syn::BareFnArg) {
        LookupTokenModifier(self).modify_bare_fn_arg(item)
    }

    fn modify_type_path(&mut self, item: &mut syn::TypePath) {
        LookupTokenModifier(self).modify_type_path(item)
    }

    fn modify_path(&mut self, item: &mut syn::Path) {
        LookupTokenModifier(self).modify_path(item)
    }

    fn modify_path_segment(&mut self, item: &mut syn::PathSegment) {
        LookupTokenModifier(self).modify_path_segment(item)
    }

    fn modify_path_arguments(&mut self, item: &mut syn::PathArguments) {
        LookupTokenModifier(self).modify_path_arguments(item)
    }

    fn modify_type_tuple(&mut self, item: &mut syn::TypeTuple) {
        LookupTokenModifier(self).modify_type_tuple(item)
    }

    fn modify_type_reference(&mut self, item: &mut syn::TypeReference) {
        LookupTokenModifier(self).modify_type_reference(item)
    }

    fn modify_lifetime(&mut self, item: &mut syn::Lifetime) {
        LookupTokenModifier(self).modify_lifetime(item)
    }

    fn modify_generic_param(&mut self, item: &mut syn::GenericParam) {
        LookupTokenModifier(self).modify_generic_param(item)
    }

    fn modify_lifetime_param(&mut self, item: &mut syn::LifetimeParam) {
        LookupTokenModifier(self).modify_lifetime_param(item)
    }

    fn modify_type_param(&mut self, item: &mut syn::TypeParam) {
        LookupTokenModifier(self).modify_type_param(item)
    }

    fn modify_const_param(&mut self, item: &mut syn::ConstParam) {
        LookupTokenModifier(self).modify_const_param(item)
    }

    fn modify_pat_type(&mut self, item: &mut syn::PatType) {
        LookupTokenModifier(self).modify_pat_type(item)
    }

    fn modify_angle_bracketed_generic_argument(&mut self, item: &mut syn::AngleBracketedGenericArguments) {
        LookupTokenModifier(self).modify_angle_bracketed_generic_argument(item)
    }

    fn modify_angle_parenthesized_generic_argument(&mut self, item: &mut syn::ParenthesizedGenericArguments) {
        LookupTokenModifier(self).modify_angle_parenthesized_generic_argument(item)
    }

    fn modify_generic_argument(&mut self, item: &mut syn::GenericArgument) {
        LookupTokenModifier(self).modify_generic_argument(item)
    }

    fn modify_return_type(&mut self, item: &mut syn::ReturnType) {
        LookupTokenModifier(self).modify_return_type(item)
    }

    fn modify_type_group(&mut self, item: &mut syn::TypeGroup) {
        LookupTokenModifier(self).modify_type_group(item)
    }

    fn modify_qself(&mut self, item: &mut syn::QSelf) {
        LookupTokenModifier(self).modify_qself(item)
    }

    fn modify_generics(&mut self, item: &mut syn::Generics) {
        LookupTokenModifier(self).modify_generics(item)
    }

    fn modify_ident(&mut self, item: &mut syn::Ident) {
        LookupTokenModifier(self).modify_ident(item)
    }

    fn modify_type_param_bound(&mut self, item: &mut syn::TypeParamBound) {
        LookupTokenModifier(self).modify_type_param_bound(item)
    }

    fn modify_trait_bound(&mut self, item: &mut syn::TraitBound) {
        LookupTokenModifier(self).modify_trait_bound(item)
    }

    fn modify_item_trait_path(&mut self, item: &mut crate::ItemTraitPath) {
        LookupTokenModifier(self).modify_item_trait_path(item)
    }

    fn modify_trait_item(&mut self, item: &mut syn::TraitItem) {
        LookupTokenModifier(self).modify_trait_item(item)
    }

    fn modify_trait_item_type(&mut self, item: &mut syn::TraitItemType) {
        LookupTokenModifier(self).modify_trait_item_type(item)
    }

    fn modify_trait_item_const(&mut self, item: &mut syn::TraitItemConst) {
        LookupTokenModifier(self).modify_trait_item_const(item)
    }

    fn modify_assoc_type(&mut self, item: &mut syn::AssocType) {
        LookupTokenModifier(self).modify_assoc_type(item)
    }

    fn modify_type_impl_trait(&mut self, item: &mut syn::TypeImplTrait) {
        LookupTokenModifier(self).modify_type_impl_trait(item)
    }

    fn modify_type_ptr(&mut self, item: &mut syn::TypePtr) {
        LookupTokenModifier(self).modify_type_ptr(item)
    }

    fn modify_type_slice(&mut self, item: &mut syn::TypeSlice) {
        LookupTokenModifier(self).modify_type_slice(item)
    }
}
