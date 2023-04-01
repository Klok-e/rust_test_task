use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Prints weather
    Get {
        address: String,
        #[arg(default_value = "now")]
        date: String,
    },
    /// Configure provider
    Configure { provider: String },
}
