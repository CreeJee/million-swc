use crate::config::ProgramStateContext;
use swc_core::ecma::{
    ast::{CallExpr, Function, ImportDecl, Program, VarDecl},
    visit::{as_folder, Fold, FoldWith, VisitMut},
};
pub struct AutoTransformVisitor {
    context: ProgramStateContext,
}

impl Fold for AutoTransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

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

pub struct HOCTrasnformVisitor {
    context: ProgramStateContext,
}
impl Fold for HOCTrasnformVisitor {
    // fn visit_mut_import_decl(&mut self, n: &mut swc_core::ecma::ast::ImportDecl) {}
    fn fold_import_decl(&mut self, n: ImportDecl) -> ImportDecl {
        return n;
    }
}

pub fn million_auto_program(context: ProgramStateContext) -> impl Fold {
    return HOCTrasnformVisitor { context: context };
}
