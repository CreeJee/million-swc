use std::borrow::BorrowMut;

use swc_core::{
    atoms::Atom,
    ecma::{
        ast::{Ident, ImportDecl, ImportSpecifier, ModuleExportName},
        visit::{noop_fold_type, Fold},
    },
};

use crate::{
    config::{ProgramStateContext, ServerMode},
    constants::internal::INTERNAL_IDENT_SYMBOL_NAME,
    utils::register::register_import_definition,
};
fn transform_function_declartion(visitor: &mut BlockTransformVisitor, decl: &mut ImportDecl) {
    let export_name_defauit_ident =
        ModuleExportName::Ident(Ident::from(INTERNAL_IDENT_SYMBOL_NAME));

    // reverse-by-server-mode
    let source_value = match visitor.context.server_mode {
        ServerMode::Client => match decl.src.value.eq("million/react") {
            true => Atom::new("million/react-server"),
            false => Atom::from(decl.src.value.clone()),
        },
        ServerMode::Server => match decl.src.value.eq("million/react-server") {
            true => Atom::new("million/react"),
            false => Atom::from(decl.src.value.clone()),
        },
    };
    decl.src.value = source_value;

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
                    let is_import_from_million_alies = s.value.eq("block") || s.value.eq("for");
                    let is_import_form_million_package = decl.src.value.starts_with("million");

                    if is_import_from_million_alies && is_import_form_million_package {
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
pub struct BlockTransformVisitor {
    pub context: ProgramStateContext,
}
impl Fold for BlockTransformVisitor {
    noop_fold_type!();

    fn fold_import_decl(&mut self, mut n: ImportDecl) -> ImportDecl {
        transform_function_declartion(self, &mut n);
        return n;
    }
    fn fold_call_expr(
        &mut self,
        n: swc_core::ecma::ast::CallExpr,
    ) -> swc_core::ecma::ast::CallExpr {
        // transform-block
        n
    }
}

pub fn million_block_transform(context: ProgramStateContext) -> impl Fold {
    return BlockTransformVisitor { context };
}
