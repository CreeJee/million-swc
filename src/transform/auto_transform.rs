use std::borrow::BorrowMut;

use crate::{
    config::ProgramStateContext, constants::internal::INTERNAL_IDENT_SYMBOL_NAME,
    utils::register::register_import_definition,
};
use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{
            CallExpr, Function, Ident, ImportDecl, ImportSpecifier, ModuleExportName, Str, VarDecl,
        },
        visit::Fold,
    },
};
pub struct AutoTransformVisitor {
    context: ProgramStateContext,
}

fn transform_function_declartion(visitor: &mut AutoTransformVisitor, decl: &mut ImportDecl) {
    let export_name_defauit_ident =
        ModuleExportName::Ident(Ident::from(INTERNAL_IDENT_SYMBOL_NAME));

    let from_react = decl.src.eq(&Box::from(Str::from("react")));
    if !from_react {
        return;
    }
    decl.to_owned()
        .specifiers
        .into_iter()
        .for_each(|item| match item {
            ImportSpecifier::Named(found_name) => {
                let imported = match found_name.imported {
                    Some(ref item) => item,
                    None => &export_name_defauit_ident,
                };
                if let ModuleExportName::Str(ref s) = imported {
                    if s.value.eq("memo") {
                        register_import_definition(
                            visitor.context.borrow_mut(),
                            decl,
                            ImportSpecifier::Named(found_name),
                        )
                    }
                }
            }
            _ => {}
        });
}
impl Fold for AutoTransformVisitor {
    fn fold_import_decl(&mut self, mut n: ImportDecl) -> ImportDecl {
        transform_function_declartion(self, &mut n);
        return n;
    }
    fn fold_function(&mut self, n: Function) -> Function {
        return n;
    }
    fn fold_var_decl(&mut self, n: VarDecl) -> VarDecl {
        return n;
    }
    fn fold_call_expr(&mut self, n: CallExpr) -> CallExpr {
        return n;
    }
}

pub fn million_auto_program<C>(context: ProgramStateContext, comments: C) -> impl Fold
where
    C: Comments,
{
    AutoTransformVisitor { context }
}
