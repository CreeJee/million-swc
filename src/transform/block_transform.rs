use swc_core::ecma::{
    ast::{Ident, ImportDecl, ImportSpecifier, ModuleExportName, Str},
    visit::{noop_fold_type, Fold},
};

use crate::{
    config::ProgramStateContext, constants::internal::INTERNAL_IDENT_SYMBOL_NAME,
    utils::register::register_import_definition,
};

fn transform_function_declartion(visitor: &mut BlockTransformVisitor, decl: &mut ImportDecl) {
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
pub struct BlockTransformVisitor<'a> {
    pub context: &'a mut ProgramStateContext,
}
impl Fold for BlockTransformVisitor<'_> {
    noop_fold_type!();

    fn fold_import_decl(&mut self, mut n: ImportDecl) -> ImportDecl {
        transform_function_declartion(self, &mut n);
        return n;
    }
}

pub fn million_block_transform(context: &mut ProgramStateContext) -> impl Fold {
    return BlockTransformVisitor { context };
}
