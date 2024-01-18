use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};
use wanikani_rs_model::{summary::Summary, user::User};

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
pub enum ErrorResponse {
    WanikaniError(WanikaniError),
    ReqwestError(String),
    DeserializationError(String),
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorResponse::WanikaniError(e) => {
                write!(f, "Error from wanikani: {} (Code {})", e.error, e.code)
            }
            ErrorResponse::ReqwestError(e) => {
                write!(f, "Error from reqwest: {e}")
            }
            ErrorResponse::DeserializationError(e) => {
                write!(f, "Error deserializing response: {e}")
            }
        }
    }
}

impl Error for ErrorResponse {}

impl From<reqwest::Error> for ErrorResponse {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value.to_string())
    }
}

impl From<serde_json::Error> for ErrorResponse {
    fn from(value: serde_json::Error) -> Self {
        Self::DeserializationError(value.to_string())
    }
}
