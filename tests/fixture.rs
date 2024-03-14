use std::{collections::HashMap, fs, path::PathBuf};

use million_swc::{
    config::{self, Config, ProgramStateContext, ServerMode},
    transform::{auto_transform::HOCTrasnformVisitor, block_transform::million_block_transform},
};
use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{EsConfig, Syntax},
        transforms::{
            base::resolver,
            testing::{test, test_fixture, test_transform, FixtureTestConfig},
        },
        visit::as_folder,
    },
};
fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let ref mut context: config::ProgramStateContext = ProgramStateContext {
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
    };
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();
            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                million_block_transform(context)
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    )
}
