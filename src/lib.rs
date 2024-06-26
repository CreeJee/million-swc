pub mod config;
pub mod constants;
pub mod transform;
pub mod utils;

use crate::config::Config;

use config::ProgramStateContext;
use swc_core::ecma::ast::Program;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use transform::million_program;

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let options = serde_json::from_str::<Config>(
        &metadata
            .get_transform_plugin_config()
            .expect("failed to get plugin config for millionjs"),
    )
    .expect("invalid packages");
    let context = ProgramStateContext::from(options);
    return match metadata.comments {
        Some(comments) => million_program(program, context, comments),
        None => panic!("million-swc should required comments option"),
    };
}
