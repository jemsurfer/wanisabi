use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
