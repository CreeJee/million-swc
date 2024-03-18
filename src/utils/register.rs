use swc_core::{common::sync::HashMapExt, ecma::ast::ImportSpecifier};

use crate::config::ProgramStateContext;

pub fn register_import_definition(
    context: &mut ProgramStateContext,
    decl: &mut swc_core::ecma::ast::ImportDecl,
    specifier: swc_core::ecma::ast::ImportSpecifier,
) {
    let value = decl.clone();
    match specifier {
        ImportSpecifier::Default(item) => context.identifiers.insert_same(item.local, value),
        ImportSpecifier::Named(item) => {
            let current = context.namespaces.get_mut(&item.local);
            if current.is_none() {
                return;
            }
            match current {
                Some(found_current) => found_current.push(value),
                None => context.namespaces.insert_same(item.local, [value].to_vec()),
            }
        }
        ImportSpecifier::Namespace(item) => context.identifiers.insert_same(item.local, value),
    }
}
