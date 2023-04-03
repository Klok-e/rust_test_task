use anyhow::bail;
use clap::Parser;
use directories::ProjectDirs;
use weather_lib::{
    cli::{Cli, Commands},
    configure::configure,
    get::get_weather,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let Some(proj_dirs) = ProjectDirs::from("com", "MyOrg",  "Weather") else {
        bail!("couldn't access user config directory.");
    };

    let config = proj_dirs.config_dir();
    std::fs::create_dir_all(config)?;
    let config_file = config.join("config.json");

    match &cli.command {
        Commands::Configure { provider } => {
            configure(provider, &config_file).await?;
        }
        Commands::Get { address, date } => {
            get_weather(address, date, &config_file).await?;
        }
    }

    Ok(())
}
