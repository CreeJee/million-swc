use std::collections::HashMap;

use serde::Deserialize;
use swc_core::ecma::ast::{Id, ImportDecl};

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

#[derive(Clone, Copy, Default, Debug, Deserialize)]
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

impl Default for ServerMode {
    fn default() -> Self {
        Self::Client
    }
}

#[derive(Clone, Default, Debug)]
pub struct ProgramStateContext {
    pub options: Config,
    pub identifiers: HashMap<Id, ImportDecl>,
    pub namespaces: HashMap<Id, Vec<ImportDecl>>,
    pub imports: HashMap<Id, ImportDecl>,
    pub server_mode: ServerMode,
}
impl ProgramStateContext {
    pub fn from(options: Config) -> Self {
        Self {
            options,
            identifiers: HashMap::new(),
            namespaces: HashMap::new(),
            imports: HashMap::new(),
            server_mode: ServerMode::Client,
        }
    }
}
