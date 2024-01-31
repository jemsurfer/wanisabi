use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Review {
    pub assignment_id: i64,
    pub created_at: DateTime<Utc>,
    pub ending_srs_stage: i64,
    pub incorrect_meaning_answers: i64,
    pub incorrect_reading_answers: i64,
    pub spaced_repetition_system_id: i64,
    pub starting_srs_stage: i64,
    pub subject_id: i64,
}
