use swc_core::ecma::ast::{Ident, ImportDecl, ImportSpecifier, JSXMemberExpr, MemberExpr};

use crate::config::ProgramStateContext;

pub trait SumType<T> {
    fn value(self, context: ProgramStateContext) -> T;
}

impl SumType<Option<ImportDecl>> for Ident {
    fn value(self, context: ProgramStateContext) -> Option<ImportDecl> {
        return context.identifiers.get(&self.to_id()).cloned();
    }
}
impl SumType<Option<ImportDecl>> for MemberExpr {
    fn value(self, context: ProgramStateContext) -> Option<ImportDecl> {
        let ident = self.prop.ident().unwrap();
        let ident_id = ident.to_id();
        let defs = context.namespaces.get(&ident_id);
        if defs.is_some() {
            defs.unwrap().into_iter().find(|it| {
                return it
                    .specifiers
                    .clone()
                    .into_iter()
                    .any(|specifier| match specifier {
                        ImportSpecifier::Named(item) => item.local.to_id() == ident_id,
                        ImportSpecifier::Default(item) => item.local.to_id() == ident_id,
                        ImportSpecifier::Namespace(item) => item.local.to_id() == ident_id,
                    });
            });
        }
        return None;
    }
}
impl SumType<Option<ImportDecl>> for JSXMemberExpr {
    fn value(self, context: ProgramStateContext) -> Option<ImportDecl> {
        let ident_id = self.prop.to_id();
        let defs = context.namespaces.get(&ident_id).cloned();
        return match defs {
            None => None,
            Some(vec) => {
                return vec
                    .into_iter()
                    .find(|it| {
                        return it.specifiers.clone().into_iter().any(
                            |specifier| match specifier {
                                ImportSpecifier::Named(item) => item.local.to_id() == ident_id,
                                ImportSpecifier::Default(item) => item.local.to_id() == ident_id,
                                ImportSpecifier::Namespace(item) => item.local.to_id() == ident_id,
                            },
                        );
                    })
                    .clone()
            }
        };
    }
}
pub fn get_vaild_import_definition<T>(expr: T, context: ProgramStateContext) -> Option<ImportDecl>
where
    T: SumType<Option<ImportDecl>>,
{
    return expr.value(context);
}
