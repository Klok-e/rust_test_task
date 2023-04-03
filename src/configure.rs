use std::path::Path;

use crate::{cli::Provider, error::Result, providers::ProviderUserInfo};

pub async fn configure(provider: &Provider, config_file: &Path) -> Result<()> {
    match provider {
        Provider::OpenWeather => {
            openweather(config_file)?;
        }
        Provider::WeatherApi => todo!(),
    }
    println!("Key saved successfully.");

    Ok(())
}

fn openweather(config_file: &Path) -> Result<()> {
    let key = inquire::Password::new("OpenWeather api key:")
        .without_confirmation()
        .prompt()?;
    let serialized = serde_json::to_string(&ProviderUserInfo::OpenWeather { api_key: key })?;
    std::fs::write(config_file, serialized)?;
    Ok(())
}
