mod args;
mod config;
mod edit;
mod md_reader;
mod recompile;
mod run;
mod strip;
pub mod util;

use crate::args::{RssArgs, RssSubcommand};
use crate::config::{edit_config, get_config, get_config_path, reset_config};
use crate::edit::edit;
use crate::md_reader::md_reader;
use crate::recompile::recompile;
use crate::run::run;
use crate::strip::strip;
use clap::Parser;
use color_print::cprintln;
use std::path::PathBuf;

const fn target_triple() -> &'static str {
    env!("TARGET")
}

fn main() {
    #[cfg(all(not(windows), not(unix)))]
    compile_error!("Only Windows and Unix-derivatives are supported");

    if let Err(e) = wrapped_main() {
        cprintln!("\n<red, bold>{e}</>");
    }
}

fn wrapped_main() -> Result<(), String> {
    let args = RssArgs::parse();

    match args.subcommand() {
        RssSubcommand::Readme => {
            md_reader(include_str!("../README.md"))?;
        }
        RssSubcommand::Run { file } => {
            let config = get_config()?;
            let binary_exists = run(&config, file)?;
            if !binary_exists {
                cprintln!("rss file has no binary - recompiling");
                let compile_succeeded = recompile(&config, file)?;
                if compile_succeeded {
                    run(&config, file)?;
                }
            }
        }
        RssSubcommand::Edit { file } => {
            let config = get_config()?;
            edit(&config, PathBuf::from(file))?;
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
                let config = get_config()?;
                edit_config(&config)?;
            } else if *r#where && !reset {
                println!(
                    "Config at '{}'",
                    get_config_path()?.as_os_str().to_string_lossy()
                );
            } else {
                let (p, json) = reset_config()?;
                println!("Reset config to:\n{json}");
                println!("Reset config at '{p}'");
            }
        }
    }

    Ok(())
}
