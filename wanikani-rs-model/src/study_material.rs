use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::subject_type::SubjectType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StudyMaterial {
    pub created_at: DateTime<Utc>,
    pub hidden: bool,
    pub meaning_note: Option<String>,
    pub meaning_synonyms: Vec<String>,
    pub reading_note: Option<String>,
    pub subject_id: i64,
    pub subject_type: SubjectType,
}
