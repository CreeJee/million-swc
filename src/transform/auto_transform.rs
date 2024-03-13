use crate::config::ProgramStateContext;
use swc_core::ecma::{
    ast::Program,
    visit::{as_folder, FoldWith, VisitMut},
};
pub struct AutoTransformVisitor<'a> {
    context: &'a mut ProgramStateContext,
}

impl VisitMut for AutoTransformVisitor<'_> {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_function(&mut self, n: &mut swc_core::ecma::ast::Function) {}
    fn visit_mut_var_decl(&mut self, n: &mut swc_core::ecma::ast::VarDecl) {}
    fn visit_mut_call_expr(&mut self, n: &mut swc_core::ecma::ast::CallExpr) {}
}

pub struct HOCTrasnformVisitor<'a> {
    context: &'a mut ProgramStateContext,
}
impl VisitMut for HOCTrasnformVisitor<'_> {
    fn visit_mut_import_decl(&mut self, n: &mut swc_core::ecma::ast::ImportDecl) {}
}
pub fn million_auto_program(program: Program, context: &mut ProgramStateContext) -> Program {
    program
        .fold_with(&mut as_folder(HOCTrasnformVisitor { context }))
        .fold_with(&mut as_folder(AutoTransformVisitor { context }))
}
