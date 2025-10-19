#[macro_use]
extern crate const_it;
#[macro_use]
extern crate static_assertions;

mod shared;

use crate::shared::VERBOSE;
use crate::shared::args::{RssArgs, RssSubcommand};
use crate::shared::cat::cat;
use crate::shared::config::{edit_config, get_config, get_config_path, reset_config};
use crate::shared::ctrl_c_handler::init_ctrl_c_handler;
use crate::shared::edit::edit;
use crate::shared::extract::extract;
use crate::shared::install::install;
use crate::shared::pack::pack;
use crate::shared::recompile::recompile;
use crate::shared::stats::stats;
use crate::shared::strip::strip;
use crate::shared::tree::tree;
use crate::shared::wrapped_run::wrapped_run;
use clap::Parser;
use color_print::cprintln;
use std::path::PathBuf;

fn main() {
    #[cfg(all(not(windows), not(unix)))]
    compile_error!("Only Windows and Unix-derivatives are supported");

    init_ctrl_c_handler();

    if let Err(e) = wrapped_main() {
        cprintln!("\n<red, bold>{e}</>");
    }
}

fn wrapped_main() -> Result<(), String> {
    if let Some(arg1) = std::env::args().next() {
        if let Some(file_stem) = PathBuf::from(arg1).file_stem() {
            if file_stem.to_string_lossy() == "rs-script" {
                cprintln!(
                    "<yellow, bold>You are using the `rs-script` command. `rss` is preferred as a shorthand.</>"
                )
            }
        }
    }

    let args = RssArgs::parse();
    VERBOSE.set(args.verbose()).unwrap();

    match args.subcommand() {
        RssSubcommand::Readme => {
            println!("{}", include_str!("../README.md"));
        }
        RssSubcommand::Run { file, args } => {
            wrapped_run(file, args)?;
        }
        RssSubcommand::Edit { file } | RssSubcommand::New { file } => {
            let new = matches!(args.subcommand(), RssSubcommand::New { .. });

            let config = get_config()?;
            edit(&config, PathBuf::from(file), new)?;
        }
        RssSubcommand::Install { file } => {
            let config = get_config()?;
            install(&config, PathBuf::from(file))?;
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
        RssSubcommand::Tree { file, show_hidden } => {
            let config = get_config()?;
            tree(&config, file, *show_hidden)?;
        }
        RssSubcommand::Cat {
            file,
            name,
            extension,
            all,
            show_hidden,
        } => {
            let config = get_config()?;
            cat(
                &config,
                file,
                name.as_ref().map(|x| x.as_str()),
                extension.as_ref().map(|x| x.as_str()),
                *all,
                *show_hidden,
            )?;
        }
    }

    Ok(())
}
