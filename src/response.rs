use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use wanikani_rs_model::{summary::Summary, user::User};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub object: String,
    pub url: String,
    pub data_updated_at: DateTime<Utc>,
    pub data: User,
}

#[derive(Serialize, Deserialize, Debug)]
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
