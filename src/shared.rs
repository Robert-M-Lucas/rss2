use std::sync::OnceLock;

pub mod args;
pub mod config;
pub mod edit;
pub mod extract;
// mod md_reader;
pub mod cat;
pub mod pack;
pub mod recompile;
pub mod run;
pub mod stats;
pub mod strip;
pub mod tree;
pub mod util;
pub mod wrapped_run;
pub mod install;

pub const TARGET_TRIPLE: &str = env!("TARGET");
pub const RS_SCRIPT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub static VERBOSE: OnceLock<bool> = OnceLock::new();
