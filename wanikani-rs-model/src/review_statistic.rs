use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::subject_type::SubjectType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ReviewStatistic {
    pub created_at: DateTime<Utc>,
    pub hidden: bool,
    pub meaning_correct: i64,
    pub meaning_current_streak: i64,
    pub meaning_incorrect: i64,
    pub meaning_max_streak: i64,
    pub percentage_correct: i64,
    pub reading_correct: i64,
    pub reading_current_streak: i64,
    pub reading_incorrect: i64,
    pub reading_max_streak: i64,
    pub subject_id: i64,
    pub subject_type: SubjectType,
}
