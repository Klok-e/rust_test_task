use clap::Parser;
use weather_lib::cli::Cli;
use weather_lib::error::Result;
use weather_lib::{cli::Commands, configure::configure, get::get_weather};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Configure { provider } => {
            configure(provider).await;
        }
        Commands::Get { address, date } => {
            get_weather(address, date).await?;
        }
    }

    Ok(())
}
