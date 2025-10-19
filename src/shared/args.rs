use clap::{Parser, Subcommand};
use derive_getters::Getters;

#[derive(Parser, Debug, Getters)]
#[command(version, about, long_about = None)]
pub struct RssArgs {
    #[command(subcommand)]
    subcommand: RssSubcommand,
    #[arg(short, long, action, help = "Print additional information")]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum RssSubcommand {
    #[command(visible_alias = "r", about = "Runs an rss file")]
    Run {
        #[arg(help = "File to run")]
        file: String,
        #[arg(trailing_var_arg = true, help = "Arguments to pass to the program")]
        args: Vec<String>,
    },

    #[command(visible_alias = "e", about = "Edit/create an rss file")]
    Edit {
        #[arg(help = "File to edit")]
        file: String,
    },

    #[command(visible_alias = "n", about = "Create an rss file")]
    New {
        #[arg(help = "File to create")]
        file: String,
    },

    #[command(
        visible_alias = "i",
        about = "Install an rss file globally with `cargo install`"
    )]
    Install {
        #[arg(help = "File to install")]
        file: String,
    },

    #[command(visible_alias = "conf", about = "Change config options")]
    Config {
        #[arg(short, long, action, help = "Reset config to default")]
        reset: bool,
        #[arg(short, long, action, help = "Outputs the config file location")]
        r#where: bool,
    },

    #[command(
        visible_alias = "s",
        about = "Strips the compiled binary from an rss file"
    )]
    Strip {
        #[arg(help = "File to strip")]
        file: String,
    },

    #[command(
        visible_alias = "ext",
        about = "Extracts the Rust project from the rss file"
    )]
    Extract {
        #[arg(help = "File to extract")]
        file: String,
    },

    #[command(visible_alias = "p", about = "Creates an rss file from a Rust project")]
    Pack {
        #[arg(help = "Rust project folder")]
        directory: String,
    },

    #[command(
        visible_alias = "rcmp",
        about = "Recompile the compiled binary for an rss file"
    )]
    Recompile {
        #[arg(help = "File to recompile")]
        file: String,
    },

    #[command(visible_alias = "stat", about = "Prints statistics about an rss file")]
    Stats {
        #[arg(help = "File to get statistics of")]
        file: String,
    },

    #[command(visible_alias = "t", about = "Prints the file tree within an rss file")]
    Tree {
        #[arg(help = "File to print tree of")]
        file: String,
        #[arg(help = "Show hidden files")]
        show_hidden: bool,
    },

    #[command(
        visible_alias = "c",
        about = "Prints the contents of files within an rss file"
    )]
    Cat {
        #[arg(help = "Rss file to print file contents of")]
        file: String,
        #[arg(short, long, action, help = "Filter files by name")]
        name: Option<String>,
        #[arg(short, long, action, help = "Filter files by extension")]
        extension: Option<String>,
        #[arg(
            short,
            long,
            action,
            help = "Prints all files (default is only .rs files)"
        )]
        all: bool,
        #[arg(help = "Print hidden files")]
        show_hidden: bool,
    },

    #[command(about = "Read the README")]
    Readme,
}
