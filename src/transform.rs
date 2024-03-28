use swc_core::ecma::{ast::Program, visit::FoldWith};
use swc_core::plugin::proxies::PluginCommentsProxy;

use self::auto_transform::million_auto_program;
use self::block_transform::million_block_transform;
use crate::config::ProgramStateContext;

pub mod auto_transform;
pub mod block_transform;

pub fn million_program(
    program: Program,
    ctx: ProgramStateContext,
    comments: PluginCommentsProxy,
) -> Program {
    let auto_program = match ctx.options.auto {
        Some(true) => program.fold_with(&mut million_auto_program(ctx.clone(), comments)),
        _ => program,
    };
    return auto_program.fold_with(&mut million_block_transform(ctx.clone(), comments));
}
