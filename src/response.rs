use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use wanikani_rs_model::summary::Summary;

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceActor {
    pub description: String,
    pub gender: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSubscription {
    pub active: bool,
    pub max_level_granted: i64,
    pub period_ends_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPreferences {
    pub default_voice_actor_id: i64,
    pub lessons_autoplay_audio: bool,
    pub lessons_batch_size: i64,
    pub lessons_presentation_order: String,
    pub reviews_autoplay_audio: bool,
    pub reviews_display_srs_indicator: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub current_vacation_started_at: Option<DateTime<Utc>>,
    pub level: i64,
    pub preferences: UserPreferences,
    pub profile_url: String,
    pub started_at: DateTime<Utc>,
    pub subscription: UserSubscription,
    pub username: String,
}

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
