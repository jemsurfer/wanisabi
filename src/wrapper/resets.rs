use crate::{
    get,
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wanikani_rs_model::reset::Reset;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResetFilter {
    Ids(Vec<i64>),
    UpdatedAfter(DateTime<Utc>),
}

impl WanikaniClient {
    get!(
        get_resets_filtered,
        "resets",
        ResetFilter,
        CollectionResponse<ResourceResponse<Reset>>
    );
    get!(
        get_resets,
        "resets",
        CollectionResponse<ResourceResponse<Reset>>
    );
    get!(get_reset, "resets/{id}", ResourceResponse<Reset>, id: i64);
}
