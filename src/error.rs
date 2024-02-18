use core::{
    fmt::{self, Display, Formatter},
    num::TryFromIntError,
};
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    ///Something's gone wrong with the request before/while being sent
    Reqwest(String),
    ///The wrapper couldn't parse the result properly
    Deserialization(String),
    ///The ranges wanikani gave us don't fit
    Range(String),
    ///There was a problem constructing the rate limiter
    RateLimit(String),
    ///Sql DB error
    Sqlx(String),
    ///Migrations error
    Migrations(String),
    NoApiKey,
    ///Wanikani has a problem with the data we've uploaded
    #[serde(untagged)]
    Wanikani {
        error: String,
        code: i64,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Wanikani { error, code } => {
                write!(f, "Wanikani error: {} (Code {})", error, code)
            }
            Error::Reqwest(e) => {
                write!(f, "Reqwest error: {e}")
            }
            Error::Deserialization(e) => {
                write!(f, "Error deserializing response: {e}")
            }
            Error::Range(e) => {
                write!(f, "Range error: {e}")
            }
            Error::RateLimit(e) => {
                write!(f, "Rate Limit error: {e}")
            }
            Error::Sqlx(e) => {
                write!(f, "Sqlx error: {e}")
            }
            Error::NoApiKey => {
                write!(f, "Failed to create a client using default(). 
                       Ensure you have set the WANIKANI_API_KEY environment variable, or use Client::new(KEY).")
            }
            Error::Migrations(e) => {
                write!(f, "Failed to execute SQL migrations: {e}")
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialization(value.to_string())
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Self::Range(value.to_string())
    }
}

impl From<ratelimit::Error> for Error {
    fn from(value: ratelimit::Error) -> Self {
        Self::RateLimit(value.to_string())
    }
}

impl From<sqlx::error::Error> for Error {
    fn from(value: sqlx::error::Error) -> Self {
        Self::Sqlx(value.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Migrations(value.to_string())
    }
}
