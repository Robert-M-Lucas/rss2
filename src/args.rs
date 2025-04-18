use clap::Parser;
use derive_getters::Getters;

#[derive(Parser, Debug, Getters)]
#[command(version, about, long_about = None)]
pub struct RssArgs {
    #[command(subcommand)]
    subcommand: RssSubcommand
}

#[derive(Parser, Debug)]
pub enum RssSubcommand {
    #[command(about = "Runs an rss file")]
    Run {
        #[arg(help = "File to run")]
        file: String,
    },

    #[command(about = "Edit/create an rss file")]
    Edit {
        #[arg(help = "File to edit")]
        file: String,
    },

    #[command(about = "Strips the compiled binary from an rss file")]
    Strip {
        #[arg(help = "File to strip")]
        file: String,
    },

    #[command(about = "Recompile the compiled binary for an rss file")]
    Recompile {
        #[arg(help = "File to recompile")]
        file: String,
    },

    #[command(about = "Change config options")]
    Config {
        #[arg(short, long, action, help = "Reset config to default")]
        reset: bool,
        #[arg(short, long, action, help = "Outputs the config file location")]
        r#where: bool,
    },
}