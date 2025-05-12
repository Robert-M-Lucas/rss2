mod args;
mod config;
mod edit;
mod extract;
// mod md_reader;
mod cat;
mod pack;
mod recompile;
mod run;
mod stats;
mod strip;
mod tree;
pub mod util;

use crate::args::{RssArgs, RssSubcommand};
use crate::config::{edit_config, get_config, get_config_path, reset_config};
use crate::edit::edit;
use crate::extract::extract;
use crate::pack::pack;
use crate::recompile::recompile;
use crate::run::{RunParam, run};
use crate::strip::strip;
#[macro_use]
extern crate static_assertions;
#[macro_use]
extern crate const_it;
use crate::cat::cat;
use crate::stats::stats;
use crate::tree::tree;
use clap::Parser;
use color_print::cprintln;
use std::path::PathBuf;
use std::process::exit;
use std::sync::OnceLock;

const TARGET_TRIPLE: &str = env!("TARGET");
const RS_SCRIPT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub static VERBOSE: OnceLock<bool> = OnceLock::new();

fn main() {
    #[cfg(all(not(windows), not(unix)))]
    compile_error!("Only Windows and Unix-derivatives are supported");

    if let Err(e) = wrapped_main() {
        cprintln!("\n<red, bold>{e}</>");
    }
}

fn wrapped_main() -> Result<(), String> {
    if let Some(arg1) = std::env::args().next() {
        if let Some(file_stem) = PathBuf::from(arg1).file_stem() {
            if file_stem.to_string_lossy() == "rs-script" {
                cprintln!(
                    "<yellow, bold>You are using the `rs-script` command. Use `rss` as a shorthand.</>"
                )
            }
        }
    }

    let args = RssArgs::parse();
    VERBOSE.set(args.verbose()).unwrap();

    match args.subcommand() {
        RssSubcommand::Readme => {
            // md_reader(include_str!("../README.md"))?;
            println!("{}", include_str!("../README.md"));
        }
        RssSubcommand::Run { file, args } => {
            let config = get_config()?;
            let binary_exists = run(&config, RunParam::Path(&file), args)?;

            // Build and re-run if binary doesn't exist
            let code = match binary_exists {
                Ok(code) => code,
                Err(no_binary_reason) => {
                    cprintln!("<yellow, bold>[!] {no_binary_reason} - recompiling...</>");
                    let compiled_binary = recompile(&config, file)?;
                    if let Some(compiled_binary) = compiled_binary {
                        if !VERBOSE.get().unwrap() {
                            println!("Running binary...");
                        }
                        run(&config, RunParam::<String>::Binary(compiled_binary), args)?
                            .unwrap_or(-1)
                    } else {
                        -1
                    }
                }
            };

            exit(code);
        }
        RssSubcommand::Edit { file } | RssSubcommand::New { file } => {
            let new = matches!(args.subcommand(), RssSubcommand::New { .. });

            let config = get_config()?;
            edit(&config, PathBuf::from(file), new)?;
        }
        RssSubcommand::Strip { file } => {
            let config = get_config()?;
            strip(&config, PathBuf::from(file))?;
        }
        RssSubcommand::Recompile { file } => {
            let config = get_config()?;
            recompile(&config, PathBuf::from(file))?;
        }
        RssSubcommand::Config { reset, r#where } => {
            if !reset && !r#where {
                // Normal config
                let config = get_config()?;
                edit_config(&config)?;
            } else if *r#where && !reset {
                // Where
                println!(
                    "Config at '{}'",
                    get_config_path()?.as_os_str().to_string_lossy()
                );
            } else {
                // Reset
                let (p, json) = reset_config()?;
                println!("Reset config to:\n{json}");
                println!("Reset config at '{p}'");
            }
        }
        RssSubcommand::Extract { file } => {
            let config = get_config()?;
            extract(&config, file)?;
        }
        RssSubcommand::Pack { directory } => {
            let config = get_config()?;
            pack(&config, directory)?;
        }
        RssSubcommand::Stats { file } => {
            let config = get_config()?;
            stats(&config, file)?;
        }
        RssSubcommand::Tree { file } => {
            let config = get_config()?;
            tree(&config, file)?;
        }
        RssSubcommand::Cat {
            file,
            name,
            extension,
            all,
        } => {
            let config = get_config()?;
            cat(
                &config,
                file,
                name.as_ref().map(|x| x.as_str()),
                extension.as_ref().map(|x| x.as_str()),
                *all,
            )?;
        }
    }

    Ok(())
}
