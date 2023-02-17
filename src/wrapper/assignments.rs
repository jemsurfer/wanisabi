use crate::{
    get,
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wanikani_rs_model::assignment::Assignment;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssignmentsFilter {
    AvailableAfter(DateTime<Utc>),
    AvailableBefore(DateTime<Utc>),
    Burned(bool),
    Hidden(bool),
    Ids(Vec<i32>),
    ImmediatelyAvailableForLessons(()),
    ImmediatelyAvailableForReview(()),
    InReview(()),
    Levels(Vec<i32>),
    SrsStages(Vec<i32>),
    Started(bool),
    SubjetIds(Vec<i32>),
    SubjetTypes(Vec<String>),
    Unlocked(bool),
    UpdatedAfter(DateTime<Utc>),
}

impl WanikaniClient {
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
        id: i32
    );
}
