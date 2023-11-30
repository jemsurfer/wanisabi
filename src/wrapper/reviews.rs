use crate::{
    get, post,
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wanikani_rs_model::review::Review;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ReviewFilter {
    AssignmentIds(Vec<i64>),
    Ids(Vec<i64>),
    SubjectIds(Vec<i64>),
    UpdatedAfter(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReviewCreate {
    pub assignment_id: i64,
    pub subject_id: i64,
    pub incorrect_meaning_answers: i64,
    pub incorrect_reading_answers: i64,
    pub created_at: Option<DateTime<Utc>>,
}

impl WanikaniClient {
    get!(
        get_reviews_filtered,
        "reviews",
        ReviewFilter,
        CollectionResponse<ResourceResponse<Review>>
    );
    get!(
        get_reviews,
        "reviews",
        CollectionResponse<ResourceResponse<Review>>
    );
    get!(
        get_review,
        "reviews/{id}",
        ResourceResponse<Review>,
        id: i64
    );
    post!(
        create_review,
        "reviews",
        ReviewCreate,
        ResourceResponse<Review>
    );
}
