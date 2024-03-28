use std::borrow::BorrowMut;

use swc_core::{
    atoms::Atom,
    common::comments::Comments,
    ecma::{
        ast::{CallExpr, Ident, ImportDecl, ImportSpecifier, ModuleExportName, SpanExt},
        visit::{noop_fold_type, Fold},
    },
    plugin::proxies::PluginCommentsProxy,
};

use crate::{
    config::{ProgramStateContext, ServerMode},
    constants::{constant_string::SKIP_ANNOTATION, internal::INTERNAL_IDENT_SYMBOL_NAME},
    utils::{ast::is_comment_exist, register::register_import_definition},
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
fn transform_block<C>(node: &mut CallExpr, comments: C)
where
    C: Comments,
{
    let range = node.comment_range();
    let comment = comments.get_leading(range.lo);
    if is_comment_exist(&comment.unwrap_or_default(), SKIP_ANNOTATION) {
        return;
    }
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
    fn fold_call_expr(&mut self, mut n: CallExpr) -> CallExpr {
        transform_block(&mut n, PluginCommentsProxy);
        return n;
    }
}

pub fn million_block_transform<C>(context: ProgramStateContext, comments: C) -> impl Fold
where
    C: Comments,
{
    return BlockTransformVisitor { context };
}
