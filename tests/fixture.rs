use std::{collections::HashMap, fs, path::PathBuf};

use million_swc::{
    config::{self, Config, ProgramStateContext, ServerMode},
    transform::block_transform::million_block_transform,
};
use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{EsConfig, Syntax},
        transforms::{
            base::resolver,
            testing::{test, test_fixture, FixtureTestConfig},
        },
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
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();
            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                million_block_transform(ProgramStateContext::from(Config {
                    hmr: Some(false),
                    server: Some(false),
                    auto: Some(false),
                    log: true,
                    rsc: false,
                }))
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    )
}
