use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::subject_type::SubjectType;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Assignment {
    pub available_at: Option<DateTime<Utc>>,
    pub burned_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub hidden: bool,
    pub passed_at: Option<DateTime<Utc>>,
    pub resurrected_at: Option<DateTime<Utc>>,
    pub srs_stage: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub subject_id: i32,
    pub subject_type: SubjectType,
    pub unlocked_at: Option<DateTime<Utc>>,
}
