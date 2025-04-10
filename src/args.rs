use clap::Parser;
use derive_getters::Getters;

#[derive(Parser, Debug, Getters)]
#[command(version, about, long_about = None)]
pub struct RssArgs {
    #[command(subcommand)]
    subcommand: RssSubcommand,
}

#[derive(Parser, Debug)]
pub enum RssSubcommand {
    #[command(about = "Runs a .rss file")]
    Run {
        #[arg(help = "File to run")]
        file: String,
    },

    #[command(about = "Edit/create a .rss file")]
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
}