use clap::{command, Parser, Subcommand, ValueEnum};

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
    Configure { provider: Provider },
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Provider {
    OpenWeather,
    WeatherApi,
}
