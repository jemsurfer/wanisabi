use crate::{
    client::{Client, QueryProcessor},
    get, parse_error, put,
    response::{CollectionResponse, ResourceResponse},
};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use wanisabi_model::assignment::Assignment;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AssignmentsFilter {
    AvailableAfter(DateTime<Utc>),
    AvailableBefore(DateTime<Utc>),
    Burned(bool),
    Hidden(bool),
    Ids(Vec<i64>),
    ImmediatelyAvailableForLessons,
    ImmediatelyAvailableForReview,
    InReview,
    Levels(Vec<i64>),
    SrsStages(Vec<i64>),
    Started(bool),
    SubjetIds(Vec<i64>),
    SubjetTypes(Vec<String>),
    Unlocked(bool),
    UpdatedAfter(DateTime<Utc>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StartAssignment {
    pub started_at: Option<DateTime<Utc>>,
}

impl Client {
    get!(
        get_assignments_filtered,
        "assignments",
        AssignmentsFilter,
        CollectionResponse<ResourceResponse<Assignment>>
    );
    get!(
        get_assignments,
        "assignments",
        CollectionResponse<ResourceResponse<Assignment>>
    );
    get!(
        get_assignment,
        "assignments/{id}",
        ResourceResponse<Assignment>,
        id: i64
    );
    put!(
        start_assignment,
        "assignments/{id}/start",
        StartAssignment,
        ResourceResponse<Assignment>,
        id: i64
    );
}
