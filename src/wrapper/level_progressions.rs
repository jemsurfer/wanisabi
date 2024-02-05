use crate::{
    client::{Client, QueryProcessor},
    get, parse_error,
    response::{CollectionResponse, Error, ResourceResponse, WanikaniError},
};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use wanisabi_model::level_progression::LevelProgression;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LevelProgressionFilter {
    Ids(Vec<i64>),
    UpdatedAfter(DateTime<Utc>),
}

impl Client {
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
        id: i64
    );
}
