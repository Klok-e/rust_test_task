use crate::error::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenWeather {
    api_key: String,
    client: Client,
}

impl OpenWeather {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    fn format_addr(&self, query: &str) -> String {
        const BASE_HTTP: &str = "https://api.openweathermap.org/data/2.5/";
        format!(
            "{}{}&appid={}&units=metric",
            &BASE_HTTP, &query, self.api_key
        )
    }

    pub async fn current_weather(&self, lat: f32, lon: f32) -> Result<CurrentWeather> {
        let addr = self.format_addr(&format!("weather?lat={lat}&lon={lon}"));
        let response = self.client.get(&addr).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn current_weather_city(&self, city: &str) -> Result<CurrentWeather> {
        let addr = self.format_addr(&format!("weather?q={city}"));
        let response = self.client.get(&addr).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "sea_level")]
    pub sea_level: Option<i64>,
    #[serde(rename = "grnd_level")]
    pub grnd_level: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rain {
    #[serde(rename = "1h")]
    pub n1h: Option<f64>,
    #[serde(rename = "3h")]
    pub n3h: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}
