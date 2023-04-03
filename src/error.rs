use thiserror::Error;

/// Enum representing errors that may occur during the weather CLI's execution.
#[derive(Error, Debug)]
pub enum Error {
    #[error("request error")]
    Request(#[from] reqwest::Error),
    #[error("io error")]
    File(#[from] std::io::Error),
    #[error("serialization error")]
    Serialization(#[from] serde_json::Error),
    #[error("date time parse error")]
    DateTimeParse(#[from] chrono::ParseError),
    #[error("invalid timezone time error")]
    InvalidTimezoneTime,
    #[error("no weather history error")]
    WeatherNoHistory,
}

pub type Result<T> = std::result::Result<T, Error>;
