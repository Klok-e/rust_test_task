use clap::Parser;
use cli::{Cli, Commands};

use crate::{configure::configure, get::get_weather};

mod cli;
mod configure;
mod get;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Configure { provider } => {
            configure(provider);
        }
        Commands::Get { address, date } => {
            get_weather(address, date);
        }
    }
}
