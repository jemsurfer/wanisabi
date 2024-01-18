use crate::{
    get,
    response::{CollectionResponse, ErrorResponse, ResourceResponse, WanikaniError},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use wanikani_rs_model::voice_actor::VoiceActor;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VoiceActorFilter {
    Ids(Vec<i64>),
    UpdatedAfter(DateTime<Utc>),
}

impl WanikaniClient {
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
