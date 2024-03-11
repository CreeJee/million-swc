use swc_core::ecma::visit::{as_folder, Fold, VisitMut};

use crate::config::ProgramStateContext;

fn block_import_definition(n: &mut swc_core::ecma::ast::ImportDecl) {
    // n.
}

pub struct BlockTransformVisitor {
    pub context: ProgramStateContext,
}

impl VisitMut for BlockTransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

    fn visit_mut_import_decl(&mut self, n: &mut swc_core::ecma::ast::ImportDecl) {
        block_import_definition(n);
    }
    fn visit_mut_call_expr(&mut self, n: &mut swc_core::ecma::ast::CallExpr) {
        // self.visit_mut_call_expr(n);
    }
}

// pub fn million_block_transform(context: ProgramStateContext) -> impl Fold {
//     let folder = BlockTransformVisitor { context };
//     as_folder(folder)
// }
