use crate::{error::Result, providers::ProviderUserInfo};

pub async fn get_weather(_address: &str, _date: &str) -> Result<()> {
    let weather_api = ProviderUserInfo::OpenWeather {
        api_key: "5c37dd4ce498db4bc0b7f7c04b39c0f7".into(),
    }
    .build_provider();
    let weather = weather_api.get_weather(49.987_503, 36.285_374).await?;

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
    let arrows = ['→', '↗', '↑', '↖', '←', '↙', '↓', '↘'];
    let index = ((degrees) as f64 / 45.0).round() as usize % 8;
    arrows[index]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, '→')]
    #[case(45, '↗')]
    #[case(90, '↑')]
    #[case(135, '↖')]
    #[case(180, '←')]
    #[case(225, '↙')]
    #[case(280, '↓')]
    #[case(325, '↘')]
    #[case(355, '→')]
    fn test_add(#[case] deg: i64, #[case] expected_sym: char) {
        assert_eq!(wind_direction_symbol(deg), expected_sym);
    }
}
