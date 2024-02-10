use std::collections::HashMap;

use syn::{GenericParam, Generics, Ident, WhereClause};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum GenericIdent {
    Lifetime(Ident),
    Other(Ident),
}

impl From<&GenericParam> for GenericIdent {
    fn from(generic_param: &GenericParam) -> Self {
        match generic_param {
            GenericParam::Lifetime(l) => Self::Lifetime(l.lifetime.ident.clone()),
            GenericParam::Type(t) => Self::Other(t.ident.clone()),
            GenericParam::Const(c) => Self::Other(c.ident.clone().clone()),
        }
    }
}

pub fn merge_bounds(g1: &mut GenericParam, g2: &GenericParam) {
    match (g1, g2) {
        (GenericParam::Const(_), GenericParam::Const(_)) => {
            panic!("cannot merge const");
        },
        (GenericParam::Type(t1), GenericParam::Type(t2)) => {
            t1.bounds.extend(t2.bounds.clone());
        },
        (GenericParam::Lifetime(l1), GenericParam::Lifetime(l2)) => {
            l1.bounds.extend(l2.bounds.clone());
        },
        _ => panic!("generic param type mismatch"),
    }
}

pub fn merge_generics(g1: &mut Generics, g2: &Generics) {
    let ident_indices = g1
        .params
        .iter()
        .enumerate()
        .map(|(index, param)| (GenericIdent::from(param), index))
        .collect::<HashMap<GenericIdent, usize>>();

    for generic in g2.params.iter() {
        if let Some(index) = ident_indices.get(&GenericIdent::from(generic)).copied() {
            merge_bounds(&mut g1.params[index], generic);
        } else {
            g1.params.push(generic.clone());
        }
    }
}

pub fn merge_where_clauses(w1: &mut Option<WhereClause>, w2: &Option<WhereClause>, keep_where_token: bool) {
    let Some(w2) = w2 else {
        return;
    };
    let Some(w1) = w1 else {
        w1.replace(w2.clone());
        return;
    };

    for predicate in w2.predicates.iter() {
        w1.predicates.push(predicate.clone());
        if !keep_where_token {
            w1.where_token = w2.where_token.clone();
        }
    }
}
