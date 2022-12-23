use chrono::prelude::*;
use serde::{Deserialize, Serialize};





#[derive(Serialize, Deserialize, Debug)]
pub struct LevelProgression {
    created_at: DateTime<Utc>,
    abandoned_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    level: i32,
    passed_at: Option<DateTime<Utc>>,
    started_at: Option<DateTime<Utc>>,
    unlocked_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reset {
    confirmed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    original_level: i32,
    target_level: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    assignment_id: i32,
    created_at: DateTime<Utc>,
    cending_srs_stage: i32,
    incorrect_meaning_answers: i32,
    incorrect_reading_answers: i32,
    spaced_repetition_system_id: i32,
    starting_srs_stage: i32,
    subject_id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewStatistic {
    created_at: DateTime<Utc>,
    hidden: bool,
    meaning_correct: i32,
    meaning_current_streak: i32,
    meaning_incorrect: i32,
    meaning_max_streak: i32,
    percentage_correct: i32,
    reading_correct: i32,
    reading_current_streak: i32,
    reading_incorrect: i32,
    reading_max_streak: i32,
    subject_id: i32,
    subject_type: SubjectType
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SrsStage {
    interval: Option<i32>,
    interval_unit: Option<String>,
    position: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpacedRepetionSystem {
    burning_stage_position: i32,
    created_at: DateTime<Utc>,
    description: String,
    name: String,
    passing_stage_position: i32,
    stages: Vec<SrsStage>,
    starting_stage_position: i32,
    unlocking_stage_position: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudyMaterial {
    created_at: DateTime<Utc>,
    hidden: bool,
    meaning_note: Option<String>,
    meaning_synonyms: Vec<String>,
    reading_note: Option<String>,
    subject_id: i32,
    subject_type: SubjectType
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum AuxiliaryMeaningType {
    Whitelist,
    Blacklist
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuxiliaryMeaning {
    meaning: String,
    #[serde(rename = "type")]
    kind: AuxiliaryMeaningType
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meaning {
    meaning: String,
    primary: bool,
    accepted_answer: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    auxiliary_meanings: Vec<AuxiliaryMeaning>,
    characters: String,
    created_at: DateTime<Utc>,
    document_url: String,
    hidden_at: Option<DateTime<Utc>>,
    lesson_position: i32,
    level: i32,
    meaning_mnemonic: String,
    meanings: Vec<Meaning>,
    slug: String,
    spaced_repetition_system_id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryLesson {
    available_at: DateTime<Utc>,
    subject_ids: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryReview {
    available_at: DateTime<Utc>,
    subject_ids: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    lessons: Vec<SummaryLesson>,
    next_reviews_at: Option<DateTime<Utc>>,
    reviews: Vec<SummaryReview>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceActor {
    description: String,
    gender: String,
    name: String
}

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
