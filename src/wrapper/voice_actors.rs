use crate::{
    client::Client,
    get,
    response::{CollectionResponse, IdResponse},
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
        CollectionResponse<VoiceActor>
    );
    get!(
        get_voice_actors,
        "voice_actors",
        CollectionResponse<VoiceActor>
    );
    get!(
        get_voice_actor,
        "voice_actors/{id}",
        IdResponse<VoiceActor>,
        id: i64
    );
}
