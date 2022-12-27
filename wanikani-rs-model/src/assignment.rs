#[derive(Serialize, Deserialize, Debug)]
pub struct Assignment {
    available_at: Option<DateTime<Utc>>,
    burned_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    hidden: bool,
    passed_at: Option<DateTime<Utc>>,
    resurrected_at: Option<DateTime<Utc>>,
    srs_stage: i32,
    started_at: Option<DateTime<Utc>>,
    subject_id: i32,
    subject_type: SubjectType,
    unlocked_at: Option<DateTime<Utc>>,
}
