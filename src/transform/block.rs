use swc_core::ecma::{
    ast::{Ident, ImportSpecifier, ModuleExportName, Str},
    visit::{as_folder, Fold, VisitMut},
};

use crate::{
    config::ProgramStateContext, constants::internal::INTERNAL_IDENT_SYMBOL_NAME,
    utils::register::register_import_definition,
};
pub struct BlockTransformVisitor<'a> {
    pub context: &'a mut ProgramStateContext,
}

fn transform_function_declartion(
    visitor: &mut BlockTransformVisitor,
    decl: &mut swc_core::ecma::ast::ImportDecl,
) {
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
                            visitor.context,
                            decl,
                            ImportSpecifier::Named(found_name),
                        )
                    }
                }
            }
            _ => {}
        });
}

impl VisitMut for BlockTransformVisitor<'_> {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

    fn visit_mut_import_decl(&mut self, n: &mut swc_core::ecma::ast::ImportDecl) {
        transform_function_declartion(self, n)
    }
    fn visit_mut_call_expr(&mut self, n: &mut swc_core::ecma::ast::CallExpr) {
        // transform-block
    }
}

// pub fn million_block_transform(context: ProgramStateContext) -> impl Fold {
//     let folder = BlockTransformVisitor { context };
//     as_folder(folder)
// }
