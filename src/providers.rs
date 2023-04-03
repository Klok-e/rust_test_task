mod openweather;
mod weatherapi;

use self::{
    openweather::{CurrentWeather, OpenWeather},
    weatherapi::WeatherApi,
};
use crate::error::{Error, Result};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// An enum representing user information required by weather providers.
#[derive(Debug, Serialize, Deserialize)]
pub enum ProviderUserInfo {
    /// User information required by the OpenWeather provider.
    OpenWeather {
        /// The API key required by the OpenWeather provider.
        api_key: String,
    },

    /// User information required by the WeatherApi provider.
    WeatherApi {
        /// The API key required by the WeatherApi provider.
        api_key: String,
    },
}

impl ProviderUserInfo {
    /// Builds a weather provider from the given provider user info.
    pub fn build_provider(self) -> Box<dyn WeatherProvider> {
        match self {
            ProviderUserInfo::OpenWeather { api_key } => Box::new(OpenWeather::new(api_key)),
            ProviderUserInfo::WeatherApi { api_key } => Box::new(WeatherApi::new(api_key)),
        }
    }

    /// Creates a `ProviderUserInfo` from a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or deserialized.
    pub fn from_file(file: &Path) -> Result<Self> {
        Ok(serde_json::from_reader(std::fs::File::open(file)?)?)
    }
}

/// A trait for weather providers that can provide current and historical weather data.
#[async_trait]
pub trait WeatherProvider {
    /// Retrieves the current weather data for the specified city.
    ///
    /// # Arguments
    ///
    /// * `city` - A string representing the name of the city for which to retrieve weather data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Weather` data for the specified city, or an error if the data could not be retrieved.
    async fn get_weather_city(&self, city: &str) -> Result<Weather>;

    /// Retrieves the historical weather data for the specified city and date.
    ///
    /// # Arguments
    ///
    /// * `city` - A string representing the name of the city for which to retrieve weather data.
    /// * `date` - A `DateTime<Utc>` representing the date for which to retrieve weather data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Weather` data for the specified city and date, or an error if the data could not be retrieved.
    async fn get_history_weather_city(&self, city: &str, date: DateTime<Utc>) -> Result<Weather>;
}

#[async_trait]
impl WeatherProvider for OpenWeather {
    async fn get_weather_city(&self, city: &str) -> Result<Weather> {
        let w = self.current_weather_city(city).await?;
        Ok(openweather_extract_weather_data(w))
    }

    async fn get_history_weather_city(&self, city: &str, date: DateTime<Utc>) -> Result<Weather> {
        let weather = self.history_weather(city, date).await?;
        let ok_or = weather
            .list
            .into_iter()
            .next()
            .ok_or(Error::WeatherNoHistory)?;
        Ok(openweather_extract_weather_data(ok_or))
    }
}

#[async_trait]
impl WeatherProvider for WeatherApi {
    async fn get_weather_city(&self, city: &str) -> Result<Weather> {
        let w = self.current_weather_city(city).await?;
        Ok({
            let w = w;
            Weather {
                cloudiness: w.current.cloud,
                description: w.current.condition.text,
                temperature: w.current.temp_c,
                wind: Wind {
                    speed: w.current.wind_kph / 3.6,
                    deg: w.current.wind_degree,
                },
                rain_volume: w.current.precip_mm,
                visibility: (w.current.vis_km * 1000.) as i64,
                location: format!(
                    "{}, {}, {}",
                    w.location.name, w.location.region, w.location.country
                ),
            }
        })
    }

    async fn get_history_weather_city(&self, city: &str, date: DateTime<Utc>) -> Result<Weather> {
        let w = self.history_weather(city, date).await?;
        let hour = w
            .forecast
            .forecastday
            .first()
            .ok_or(Error::WeatherNoHistory)?
            .hour
            .iter()
            .min_by_key(|x| (x.time_epoch - date.timestamp()).abs())
            .ok_or(Error::WeatherNoHistory)?;
        Ok(Weather {
            cloudiness: hour.cloud,
            description: hour.condition.text.clone(),
            temperature: hour.temp_c,
            wind: Wind {
                speed: hour.wind_kph / 3.6,
                deg: hour.wind_degree,
            },
            rain_volume: hour.precip_mm,
            visibility: (hour.vis_km * 1000.) as i64,
            location: format!(
                "{}, {}, {}",
                w.location.name, w.location.region, w.location.country
            ),
        })
    }
}

fn openweather_extract_weather_data(w: CurrentWeather) -> Weather {
    Weather {
        cloudiness: w.clouds.all,
        rain_volume: w.rain.and_then(|x| x.n1h).unwrap_or(0.),
        temperature: w.main.temp,
        visibility: w.visibility,
        wind: Wind {
            speed: w.wind.speed,
            deg: w.wind.deg,
        },
        description: w
            .weather
            .first()
            .map(|x| x.description.clone())
            .unwrap_or_else(|| "".into()),
        location: format!("{}, {}", w.name, w.sys.country),
    }
}

/// A struct representing weather information.
#[derive(Debug)]
pub struct Weather {
    /// The percentage of sky covered by clouds.
    pub cloudiness: i64,

    /// A short description of the weather condition, e.g. "Sunny", "Partly Cloudy".
    pub description: String,

    /// The temperature in degrees Celsius.
    pub temperature: f64,

    /// The wind speed and direction.
    pub wind: Wind,

    /// The volume of rain in millimeters.
    pub rain_volume: f64,

    /// The visibility in meters.
    pub visibility: i64,

    /// The name of the location for which the weather information pertains.
    pub location: String,
}

/// A struct representing wind information.
#[derive(Debug)]
pub struct Wind {
    /// The wind speed in meters per second.
    pub speed: f64,

    /// The direction that the wind is coming from, in degrees.
    ///
    /// A value of 0 degrees indicates a northerly wind, while a value of 180 degrees indicates a southerly wind.
    pub deg: i64,
}
