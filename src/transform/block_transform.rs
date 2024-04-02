use std::{borrow::BorrowMut, env::Args};

use swc_core::{
    atoms::Atom,
    common::{comments::Comments, util::take::Take},
    ecma::{
        ast::{
            CallExpr, Callee, Expr, Ident, ImportDecl, ImportSpecifier, ModuleExportName, SpanExt,
        },
        utils::ExprFactory,
        visit::{noop_fold_type, Fold, FoldWith},
    },
    plugin::proxies::PluginCommentsProxy,
};

use crate::{
    config::{ProgramStateContext, ServerMode},
    constants::{constant_string::SKIP_ANNOTATION, internal::INTERNAL_IDENT_SYMBOL_NAME},
    utils::{
        assertion::is_react_component, ast::is_comment_exist, register::register_import_definition,
    },
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
fn transform_block<C>(context: &ProgramStateContext, node: &mut CallExpr, comments: C)
where
    C: Comments,
{
    let callee = node.callee.clone();
    let args = node.args.clone();

    let is_valid_block_call = callee
        .as_expr()
        .unwrap_or(&Box::from(Expr::dummy()))
        .as_ident()
        .unwrap_or(&Ident::from(INTERNAL_IDENT_SYMBOL_NAME))
        .eq(&Ident::from("block"));

    let range = node.comment_range();
    let comment = comments.get_leading(range.lo);
    if !is_valid_block_call || is_comment_exist(&comment.unwrap_or_default(), SKIP_ANNOTATION) {
        return;
    }

    // Make sure that we have at least one argument,
    // and that argument is a component.
    if args.len() <= 0 {
        return;
    }
    let first_args_expr = &args[0].expr;
    // Handle identifiers differently
    if first_args_expr.is_ident() {
        return;
    }

    let is_react_component = first_args_expr.is_arrow() || first_args_expr.is_fn_expr();
    if !is_react_component {
        return;
    }
    // TODO: traverse visitor
    // first_args_expr.fold_with()
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
        transform_block(&self.context, &mut n, PluginCommentsProxy);
        return n;
    }
}

pub fn million_block_transform<C>(context: ProgramStateContext, comments: C) -> impl Fold
where
    C: Comments,
{
    return BlockTransformVisitor { context };
}
