use quote::quote;

pub fn merge_methods() -> proc_macro2::TokenStream {
    quote! {
        fn merge_where_clauses(
            w1: &mut Option<::syn::WhereClause>,
            w2: Option<&::syn::WhereClause>,
            keep_where_token: bool,
        ) {
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

        fn merge_generics(g1: &mut ::syn::Generics, g2: &::syn::Generics) {
            let mut i = 0;

            // 1. g1's lifetimes.
            let mut g1_iter = g1.params.iter();
            while let Some(param) = g1_iter.next() {
                if matches!(param, syn::GenericParam::Type(_)) {
                    break;
                }
                i += 1;
            }

            // 2. g2's lifetimes.
            let mut g2_iter = g2.params.iter().peekable();
            while let Some(param) = g2_iter.peek() {
                if matches!(param, syn::GenericParam::Type(_)) {
                    break;
                }
                let param = g2_iter.next().unwrap();
                g1.params.insert(i, param.clone());
                i += 1;
            }

            // 3. g1's types.
            // 4. g2's types.
            for param in g2_iter {
                g1.params.push(param.clone());
            }
        }
    }
}
