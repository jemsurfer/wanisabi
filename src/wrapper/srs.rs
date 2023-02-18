use crate::{
    get,
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wanikani_rs_model::spaced_repetition_system::SpacedRepetitionSystem;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpacedRepetitionSystemFilter {
    Ids(Vec<i64>),
    UpdatedAfter(DateTime<Utc>),
}

impl WanikaniClient {
    get!(
        get_spaced_repetition_systems_filtered,
        "spaced_repetition_systems",
        SpacedRepetitionSystemFilter,
        CollectionResponse<ResourceResponse<SpacedRepetitionSystem>>
    );
    get!(
        get_spaced_repetition_systems,
        "spaced_repetition_systems",
        CollectionResponse<ResourceResponse<SpacedRepetitionSystem>>
    );
    get!(
        get_spaced_repetition_system,
        "spaced_repetition_systems/{id}",
        ResourceResponse<SpacedRepetitionSystem>,
        id: i64
    );
}
