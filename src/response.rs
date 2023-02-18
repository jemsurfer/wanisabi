use crate::model::subject_type::SubjectType;
use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SrsStage {
    pub interval: Option<i64>,
    pub interval_unit: Option<String>,
    pub position: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpacedRepetionSystem {
    pub burning_stage_position: i64,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub name: String,
    pub passing_stage_position: i64,
    pub stages: Vec<SrsStage>,
    pub starting_stage_position: i64,
    pub unlocking_stage_position: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudyMaterial {
    pub created_at: DateTime<Utc>,
    pub hidden: bool,
    pub meaning_note: Option<String>,
    pub meaning_synonyms: Vec<String>,
    pub reading_note: Option<String>,
    pub subject_id: i64,
    pub subject_type: SubjectType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuxiliaryMeaningType {
    Whitelist,
    Blacklist,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuxiliaryMeaning {
    pub meaning: String,
    #[serde(rename = "type")]
    pub kind: AuxiliaryMeaningType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meaning {
    pub meaning: String,
    pub primary: bool,
    pub accepted_answer: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub auxiliary_meanings: Vec<AuxiliaryMeaning>,
    pub characters: String,
    pub created_at: DateTime<Utc>,
    pub document_url: String,
    pub hidden_at: Option<DateTime<Utc>>,
    pub lesson_position: i64,
    pub level: i64,
    pub meaning_mnemonic: String,
    pub meanings: Vec<Meaning>,
    pub slug: String,
    pub spaced_repetition_system_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryLesson {
    pub available_at: DateTime<Utc>,
    pub subject_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryReview {
    pub available_at: DateTime<Utc>,
    pub subject_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub lessons: Vec<SummaryLesson>,
    pub next_reviews_at: Option<DateTime<Utc>>,
    pub reviews: Vec<SummaryReview>,
}

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
