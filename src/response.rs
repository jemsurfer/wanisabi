use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSubscription {
    active: bool,
    max_level_granted: i32,
    period_ends_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    kind: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPreferences {
    default_voice_actor_id: i32,
    lessons_autoplay_audio: bool,
    lessons_batch_size: i32,
    lessons_presentation_order: String,
    reviews_autoplay_audio: bool,
    reviews_display_srs_indicator: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    current_vacation_started_at: Option<DateTime<Utc>>,
    level: i32,
    preferences: UserPreferences,
    profile_url: String,
    started_at: DateTime<Utc>,
    subscription: UserSubscription,
    username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    object: String,
    url: String,
    data_updated_at: DateTime<Utc>,
    data: User,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceResponse<T> {
    id: i32,
    object: String,
    url: String,
    data_updated_at: DateTime<Utc>,
    data: T,
}

#[derive(Serialize, Deserialize, Debug)]
struct PagesResponse {
    next_url: String,
    previous_url: String,
    per_page: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct CollectionResponse<T> {
    object: String,
    url: String,
    data_updated_at: DateTime<Utc>,
    pages: PagesResponse,
    total_count: i32,
    data: Vec<T>,
}
