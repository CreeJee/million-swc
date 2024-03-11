mod config;
mod constants;
mod transform;
mod utils;

use std::collections::HashMap;

use crate::config::Config;

use config::{ProgramStateContext, ServerMode};
use swc_core::ecma::visit::as_folder;
use swc_core::ecma::{ast::Program, transforms::testing::test_inline, visit::FoldWith};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use transform::auto_transform::{million_auto_program, AutoTransformVisitor, HOCTrasnformVisitor};
use transform::block::BlockTransformVisitor;
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
            .expect("failed to get plugin config for transform-imports"),
    )
    .expect("invalid packages");
    let ref mut context: config::ProgramStateContext = ProgramStateContext {
        options: options,
        identifiers: HashMap::new(),
        namespaces: HashMap::new(),
        imports: HashMap::new(),
        server_mode: ServerMode::Client,
        top_level_rsc: false,
    };
    return million_program(program, context);
}
// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test_inline!(
    Default::default(),
    |_| as_folder(BlockTransformVisitor {
        context: {
            config::ProgramStateContext {
                options: Config {
                    hmr: Some(false),
                    server: Some(false),
                    auto: Some(false),
                    log: true,
                    rsc: false,
                },
                identifiers: HashMap::new(),
                namespaces: HashMap::new(),
                imports: HashMap::new(),
                server_mode: ServerMode::Client,
                top_level_rsc: false,
            }
        }
    }),
    boo,
    r#"let isDev = __DEV__;"#,
    r#"let isDev = false;"#
);
