use crate::model::subject_type::SubjectType;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelProgression {
    pub created_at: DateTime<Utc>,
    pub abandoned_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub level: i32,
    pub passed_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub unlocked_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reset {
    pub confirmed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub original_level: i32,
    pub target_level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    pub assignment_id: i32,
    pub created_at: DateTime<Utc>,
    pub cending_srs_stage: i32,
    pub incorrect_meaning_answers: i32,
    pub incorrect_reading_answers: i32,
    pub spaced_repetition_system_id: i32,
    pub starting_srs_stage: i32,
    pub subject_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewStatistic {
    pub created_at: DateTime<Utc>,
    pub hidden: bool,
    pub meaning_correct: i32,
    pub meaning_current_streak: i32,
    pub meaning_incorrect: i32,
    pub meaning_max_streak: i32,
    pub percentage_correct: i32,
    pub reading_correct: i32,
    pub reading_current_streak: i32,
    pub reading_incorrect: i32,
    pub reading_max_streak: i32,
    pub subject_id: i32,
    pub subject_type: SubjectType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SrsStage {
    pub interval: Option<i32>,
    pub interval_unit: Option<String>,
    pub position: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpacedRepetionSystem {
    pub burning_stage_position: i32,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub name: String,
    pub passing_stage_position: i32,
    pub stages: Vec<SrsStage>,
    pub starting_stage_position: i32,
    pub unlocking_stage_position: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudyMaterial {
    pub created_at: DateTime<Utc>,
    pub hidden: bool,
    pub meaning_note: Option<String>,
    pub meaning_synonyms: Vec<String>,
    pub reading_note: Option<String>,
    pub subject_id: i32,
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
    pub lesson_position: i32,
    pub level: i32,
    pub meaning_mnemonic: String,
    pub meanings: Vec<Meaning>,
    pub slug: String,
    pub spaced_repetition_system_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryLesson {
    pub available_at: DateTime<Utc>,
    pub subject_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryReview {
    pub available_at: DateTime<Utc>,
    pub subject_ids: Vec<i32>,
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
    pub max_level_granted: i32,
    pub period_ends_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPreferences {
    pub default_voice_actor_id: i32,
    pub lessons_autoplay_audio: bool,
    pub lessons_batch_size: i32,
    pub lessons_presentation_order: String,
    pub reviews_autoplay_audio: bool,
    pub reviews_display_srs_indicator: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub current_vacation_started_at: Option<DateTime<Utc>>,
    pub level: i32,
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
pub struct ResourceResponse<T> {
    pub id: i32,
    pub object: String,
    pub url: String,
    pub data_updated_at: DateTime<Utc>,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PagesResponse {
    pub next_url: Option<String>,
    pub previous_url: Option<String>,
    pub per_page: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionResponse<T> {
    pub object: String,
    pub url: String,
    pub data_updated_at: Option<DateTime<Utc>>,
    pub pages: PagesResponse,
    pub total_count: i32,
    pub data: Vec<T>,
}
