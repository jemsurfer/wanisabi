use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LevelProgression {
    pub created_at: DateTime<Utc>,
    pub abandoned_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub level: i64,
    pub passed_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub unlocked_at: Option<DateTime<Utc>>,
}
