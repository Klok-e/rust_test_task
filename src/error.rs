use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("request error")]
    Request(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
