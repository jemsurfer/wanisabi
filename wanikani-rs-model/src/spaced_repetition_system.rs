use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SrsStage {
    pub interval: Option<i64>,
    pub interval_unit: Option<String>,
    pub position: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SpacedRepetitionSystem {
    pub burning_stage_position: i64,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub name: String,
    pub passing_stage_position: i64,
    pub stages: Vec<SrsStage>,
    pub starting_stage_position: i64,
    pub unlocking_stage_position: i64,
}
