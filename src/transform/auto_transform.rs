use std::borrow::{Borrow, BorrowMut};

use crate::utils::register::register_import_definition;
use crate::{config::ProgramStateContext, constants::internal::INTERNAL_IDENT_SYMBOL_NAME};
use swc_core::ecma::ast::Ident;
use swc_core::ecma::{
    ast::{ImportSpecifier, ModuleExportName, Program, Str},
    visit::{as_folder, FoldWith, VisitMut},
};

fn transform_function_declartion(
    visitor: &mut HOCTrasnformVisitor,
    decl: &mut swc_core::ecma::ast::ImportDecl,
) {
    let export_name_defauit_ident =
        ModuleExportName::Ident(Ident::from(INTERNAL_IDENT_SYMBOL_NAME));

    let from_react = decl.src.eq(&Box::from(Str::from("react")));
    if !from_react {
        return;
    }

    let f = decl
        .to_owned()
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
                        // @TODO clone 동작이 너무 어색함
                        register_import_definition(
                            &mut visitor.context,
                            decl,
                            ImportSpecifier::Named(found_name),
                        )
                    }
                }
            }
            _ => {}
        });
}

pub struct AutoTransformVisitor {
    context: ProgramStateContext,
}

impl VisitMut for AutoTransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_function(&mut self, n: &mut swc_core::ecma::ast::Function) {}
    fn visit_mut_var_decl(&mut self, n: &mut swc_core::ecma::ast::VarDecl) {}
    fn visit_mut_call_expr(&mut self, n: &mut swc_core::ecma::ast::CallExpr) {}
}

pub struct HOCTrasnformVisitor {
    context: ProgramStateContext,
}
impl VisitMut for HOCTrasnformVisitor {
    fn visit_mut_import_decl(&mut self, n: &mut swc_core::ecma::ast::ImportDecl) {
        transform_function_declartion(self, n);
    }
}
pub fn million_auto_program(program: Program, context: ProgramStateContext) -> Program {
    program
        .fold_with(&mut as_folder(HOCTrasnformVisitor {
            context: context.clone(),
        }))
        .fold_with(&mut as_folder(AutoTransformVisitor {
            context: context.clone(),
        }))
}
