use crate::{
    client::{Client, QueryProcessor},
    get, parse_error,
    response::{CollectionResponse, IdResponse},
};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use wanisabi_model::reset::Reset;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResetFilter {
    Ids(Vec<i64>),
    UpdatedAfter(DateTime<Utc>),
}

impl Client {
    get!(
        get_resets_filtered,
        "resets",
        ResetFilter,
        CollectionResponse<Reset>
    );
    get!(get_resets, "resets", CollectionResponse<Reset>);
    get!(get_reset, "resets/{id}", IdResponse<Reset>, id: i64);
}
