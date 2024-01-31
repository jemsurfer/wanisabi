use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use wanisabi_model::{summary::Summary, user::User};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub object: String,
    pub url: String,
    pub data_updated_at: DateTime<Utc>,
    pub data: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryResponse {
    pub object: String,
    pub url: String,
    pub data_updated_at: DateTime<Utc>,
    pub data: Summary,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceResponse<T> {
    pub id: i64,
    pub object: String,
    pub url: String,
    pub data_updated_at: DateTime<Utc>,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagesResponse {
    pub next_url: Option<String>,
    pub previous_url: Option<String>,
    pub per_page: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionResponse<T> {
    pub object: String,
    pub url: String,
    pub data_updated_at: Option<DateTime<Utc>>,
    pub pages: PagesResponse,
    pub total_count: i64,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WanikaniError {
    error: String,
    code: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    Wanikani(WanikaniError),
    Reqwest(String),
    Deserialization(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::Wanikani(e) => {
                write!(f, "Wanikani error: {} (Code {})", e.error, e.code)
            }
            Error::Reqwest(e) => {
                write!(f, "Reqwest error: {e}")
            }
            Error::Deserialization(e) => {
                write!(f, "Error deserializing response: {e}")
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
