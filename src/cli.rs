use chrono::{DateTime, NaiveDateTime, Utc};
use clap::{command, Parser, Subcommand, ValueEnum};

use crate::error;

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
        /// City name
        address: String,

        /// Date of the weather data. Must have the value of either "now" or a datetime string.
        /// Format: "%Y-%m-%d %H:%M:%S"
        #[arg(default_value = "now")]
        #[arg(value_parser = parse_date)]
        date: DateVariant,
    },
    /// Configure provider
    Configure { provider: Provider },
}

fn parse_date(arg: &str) -> error::Result<DateVariant> {
    let variant = match arg {
        "now" => DateVariant::Now,
        s => DateVariant::HistoryDate(
            NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")?
                .and_local_timezone(Utc)
                .latest()
                .ok_or(error::Error::InvalidTimezoneTime)?,
        ),
    };
    Ok(variant)
}

/// Enum representing weather service providers.
#[derive(Clone, Copy, ValueEnum)]
pub enum Provider {
    /// OpenWeather provider.
    OpenWeather,
    /// WeatherAPI provider.
    WeatherApi,
}

/// Enum representing either the current date or a historical date.
#[derive(Clone, Copy)]
pub enum DateVariant {
    /// The current date.
    Now,
    /// A specific historical date.
    HistoryDate(DateTime<Utc>),
}
