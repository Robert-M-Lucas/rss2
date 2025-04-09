mod args;
mod config;
mod edit;
mod run;
mod file_contents;

use std::env;
use std::path::PathBuf;
use crate::args::{RssArgs, RssSubcommand};
use crate::config::{edit_config, get_config, get_config_path, reset_config, Config};
use clap::Parser;
use tempfile::TempDir;
use crate::edit::edit;
use crate::run::run;

fn main() {

    if let Err(e) = wrapped_main() {
        println!("{e}");
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
