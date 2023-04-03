use std::path::Path;

use crate::{cli::DateVariant, error::Result, providers::ProviderUserInfo};

/// Retrieves weather data for a specified address and date, and prints it to the console.
///
/// # Arguments
///
/// * `address` - A string representing the address for which to retrieve weather data.
/// * `date` - A `DateVariant` representing the date for which to retrieve weather data.
/// * `config_file` - A `Path` representing the path to the configuration file containing user information for the weather provider.
///
/// # Returns
///
/// A `Result` indicating whether the operation was successful.
pub async fn get_weather(address: &str, date: &DateVariant, config_file: &Path) -> Result<()> {
    let weather_api = ProviderUserInfo::from_file(config_file)?.build_provider();
    let weather = match date {
        DateVariant::HistoryDate(d) => weather_api.get_history_weather_city(address, *d).await?,
        DateVariant::Now => weather_api.get_weather_city(address).await?,
    };

    println!("{}", weather.location);
    println!("{}", weather.description);
    println!("{:+} °C", weather.temperature);

    let speed_kmh = weather.wind.speed * 3.6;
    println!(
        "{} {:.1} km/h",
        wind_direction_symbol(weather.wind.deg),
        speed_kmh
    );
    println!("{} m", weather.visibility);
    println!("{:.1} mm", weather.rain_volume);
    Ok(())
}

fn wind_direction_symbol(degrees: i64) -> char {
    let arrows = ['↑', '↖', '←', '↙', '↓', '↘', '→', '↗'];
    let index = ((degrees) as f64 / 45.0).round() as usize % 8;
    arrows[index]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(270, '→')]
    #[case(315, '↗')]
    #[case(0, '↑')]
    #[case(45, '↖')]
    #[case(90, '←')]
    #[case(135, '↙')]
    #[case(180, '↓')]
    #[case(225, '↘')]
    #[case(292, '→')]
    #[case(293, '↗')]
    fn test_add(#[case] deg: i64, #[case] expected_sym: char) {
        assert_eq!(wind_direction_symbol(deg), expected_sym);
    }
}
