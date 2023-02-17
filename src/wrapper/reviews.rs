use crate::{
    get,
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wanikani_rs_model::review::Review;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewFilter {
    AssignmentIds(Vec<i32>),
    Ids(Vec<i32>),
    SubjectIds(Vec<i32>),
    UpdatedAfter(DateTime<Utc>),
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
        id: i32
    );
}
