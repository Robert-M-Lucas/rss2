mod args;
mod config;
mod edit;
mod run;
mod strip;
mod recompile;
pub mod util;

use crate::args::{RssArgs, RssSubcommand};
use crate::config::{edit_config, get_config, get_config_path, reset_config};
use crate::edit::edit;
use crate::run::run;
use clap::Parser;
use color_print::cprintln;
use std::path::PathBuf;
use crate::recompile::recompile;
use crate::strip::strip;

fn main() {

    if let Err(e) = wrapped_main() {
        cprintln!("\n<red, bold>{e}</>");
    }
}

fn wrapped_main() -> Result<(), String> {
    let args = RssArgs::parse();

    let config = get_config()?;

    match args.subcommand() {
        RssSubcommand::Run { file } => {
            run(&config, file)?;
        }
        RssSubcommand::Edit { file } => {
            edit(&config, PathBuf::from(file))?;
        }
        RssSubcommand::Strip { file } => {
            strip(&config, PathBuf::from(file))?;
        }
        RssSubcommand::Recompile { file } => {
            recompile(&config, PathBuf::from(file))?;
        }
        RssSubcommand::Config { reset, r#where } => {
            if !reset && !r#where {
                edit_config(&config)?;
            }
            else if *r#where && !reset {
                println!("Config at {:?}", get_config_path()?);
            }
            else {
                let (p, json) = reset_config()?;
                println!("Config:\n{json}");
                println!("Reset config at {p}");
            }
        }
    }

    Ok(())
}
