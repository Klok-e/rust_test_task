//! A CLI weather application written in Rust that provides current weather and historical weather
//! data for a given location. This library module contains the top level modules for the
//! application, including:
//!
//! - `cli`: Contains the CLI command definitions and parsing logic.
//! - `configure`: Contains the logic for configuring the application with API keys for weather
//!   service providers.
//! - `error`: Contains the custom error types used throughout the application.
//! - `get`: Contains the logic for fetching weather data from the chosen provider.
//! - `providers`: Contains the provider interface and provider implementations for the
//!   OpenWeather and WeatherAPI services.

pub mod cli;
pub mod configure;
pub mod error;
pub mod get;
pub mod providers;
