use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryLesson {
    pub available_at: DateTime<Utc>,
    pub subject_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryReview {
    pub available_at: DateTime<Utc>,
    pub subject_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Summary {
    pub lessons: Vec<SummaryLesson>,
    pub next_reviews_at: Option<DateTime<Utc>>,
    pub reviews: Vec<SummaryReview>,
}
