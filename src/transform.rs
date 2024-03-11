use swc_core::ecma::{
    ast::Program,
    visit::{as_folder, FoldWith},
};

use crate::config::ProgramStateContext;

use self::{auto_transform::million_auto_program, block::BlockTransformVisitor};

pub mod auto_transform;
pub mod block;

pub fn million_program(program: Program, context: &mut ProgramStateContext) -> Program {
    if context.options.rsc {
        // @TODO: how to owned by context?
        context.top_level_rsc = true
    }
    match context.options.auto {
        Some(true) => million_auto_program(program, context),
        _ => program,
    }
    .fold_with(&mut as_folder(BlockTransformVisitor {
        context: context.clone(),
    }))
}
