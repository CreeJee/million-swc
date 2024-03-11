use std::collections::HashMap;

use serde::Deserialize;
use swc_core::ecma::ast::{Ident, ImportDecl};

#[derive(Deserialize)]
#[serde(untagged)]
enum LogOptionValue {
    Boolean(bool),
    Info,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MillionTelemetry {}

fn default_boolean() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Config {
    #[serde(default)]
    pub hmr: Option<bool>,

    #[serde(default)]
    pub server: Option<bool>,

    #[serde(default)]
    pub auto: Option<bool>,

    #[serde(default = "default_boolean")]
    pub log: bool,

    #[serde(default = "default_boolean")]
    pub rsc: bool,
    // @TODO: support options
    // pub telemetry: MillionTelemetry,
}
#[derive(Clone, Debug)]
pub enum ServerMode {
    Server,
    Client,
}

#[derive(Clone, Debug)]
pub struct ProgramStateContext {
    pub options: Config,
    pub identifiers: HashMap<Ident, ImportDecl>,
    pub namespaces: HashMap<Ident, Vec<ImportDecl>>,
    pub imports: HashMap<Ident, ImportDecl>,
    pub top_level_rsc: bool,
    pub server_mode: ServerMode,
}
