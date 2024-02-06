use crate::{
    client::{Client, QueryProcessor},
    get, parse_error,
    response::{CollectionResponse, ResourceResponse},
};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use wanisabi_model::voice_actor::VoiceActor;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VoiceActorFilter {
    Ids(Vec<i64>),
    UpdatedAfter(DateTime<Utc>),
}

impl Client {
    get!(
        get_voice_actors_filtered,
        "voice_actors",
        VoiceActorFilter,
        CollectionResponse<ResourceResponse<VoiceActor>>
    );
    get!(
        get_voice_actors,
        "voice_actors",
        CollectionResponse<ResourceResponse<VoiceActor>>
    );
    get!(
        get_voice_actor,
        "voice_actors/{id}",
        ResourceResponse<VoiceActor>,
        id: i64
    );
}
