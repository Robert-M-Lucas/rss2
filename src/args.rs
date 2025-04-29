use clap::Parser;
use derive_getters::Getters;

#[derive(Parser, Debug, Getters)]
#[command(version, about, long_about = None)]
pub struct RssArgs {
    #[command(subcommand)]
    subcommand: RssSubcommand,
    #[arg(short, long, action, help = "Print additional information")]
    verbose: bool,
}

#[derive(Parser, Debug)]
pub enum RssSubcommand {
    #[command(about = "Runs an rss file")]
    Run {
        #[arg(help = "File to run")]
        file: String,
        #[arg(trailing_var_arg = true, help = "Arguments to pass to the program")]
        args: Vec<String>,
    },

    #[command(about = "Edit/create an rss file")]
    Edit {
        #[arg(help = "File to edit")]
        file: String,
    },

    #[command(about = "Change config options")]
    Config {
        #[arg(short, long, action, help = "Reset config to default")]
        reset: bool,
        #[arg(short, long, action, help = "Outputs the config file location")]
        r#where: bool,
    },

    #[command(about = "Strips the compiled binary from an rss file")]
    Strip {
        #[arg(help = "File to strip")]
        file: String,
    },

    #[command(about = "Extracts the Rust project from the rss file")]
    Extract {
        #[arg(help = "File to extract")]
        file: String,
    },

    #[command(about = "Creates an rss file from a Rust project")]
    Pack {
        #[arg(help = "Rust project folder")]
        directory: String,
    },

    #[command(about = "Recompile the compiled binary for an rss file")]
    Recompile {
        #[arg(help = "File to recompile")]
        file: String,
    },

    #[command(about = "Read the README")]
    Readme,
}
