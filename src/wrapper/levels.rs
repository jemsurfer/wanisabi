use crate::{
    get,
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wanikani_rs_model::level_progression::LevelProgression;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LevelProgressionFilter {
    Ids(Vec<i32>),
    UpdatedAfter(DateTime<Utc>),
}

impl WanikaniClient {
    get!(
        get_level_progressions_filtered,
        "level_progressions",
        LevelProgressionFilter,
        CollectionResponse<ResourceResponse<LevelProgression>>
    );
    get!(
        get_level_progressions,
        "level_progressions",
        CollectionResponse<ResourceResponse<LevelProgression>>
    );
    get!(
        get_level_progression,
        "level_progressions/{id}",
        ResourceResponse<LevelProgression>,
        id: i32
    );
}
