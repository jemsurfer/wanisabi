use crate::{
    client::{Client, QueryProcessor},
    get, parse_error,
    response::{CollectionResponse, ResourceResponse},
};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use wanisabi_model::review_statistic::ReviewStatistic;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatisticFilter {
    Hidden(bool),
    Ids(Vec<i64>),
    PercentagesGreaterThan(i64),
    PercentagesLessThan(i64),
    SubjectIds(Vec<i64>),
    SubjectTypes(Vec<String>),
    UpdatedAfter(DateTime<Utc>),
}

impl Client {
    get!(
        get_review_statistics_filtered,
        "review_statistics",
        ReviewStatisticFilter,
        CollectionResponse<ResourceResponse<ReviewStatistic>>
    );
    get!(
        get_review_statistics,
        "review_statistics",
        CollectionResponse<ResourceResponse<ReviewStatistic>>
    );
    get!(
        get_review_statistic,
        "review_statistics/{id}",
        ResourceResponse<ReviewStatistic>,
        id: i64
    );
}
