use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSubscription {
    pub active: bool,
    pub max_level_granted: i64,
    pub period_ends_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPreferences {
    pub default_voice_actor_id: i64,
    pub lessons_autoplay_audio: bool,
    pub lessons_batch_size: i64,
    pub lessons_presentation_order: String,
    pub reviews_autoplay_audio: bool,
    pub reviews_display_srs_indicator: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
