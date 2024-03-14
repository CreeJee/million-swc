use swc_core::ecma::{
    ast::Program,
    visit::{noop_fold_type, Fold, FoldWith},
};

use crate::config::ProgramStateContext;

use self::{auto_transform::million_auto_program, block_transform::block_transform};

pub mod auto_transform;
pub mod block_transform;

pub fn million_program(program: Program, context: &mut ProgramStateContext) -> Program {
    if context.options.rsc {
        // @TODO: how to owned by context?
        context.top_level_rsc = true
    }
    let auto_program = match context.options.auto {
        Some(true) => program.fold_with(&mut million_auto_program(context)),
        _ => program,
    };
    return auto_program.fold_with(&mut block_transform(context));
}
