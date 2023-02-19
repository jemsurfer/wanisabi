use crate::{put, response::UserResponse};
use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::{get, wanikani_client::WanikaniClient};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPreferencesUpdate {
    pub default_voice_actor_id: Option<i64>,
    pub lessons_autoplay_audio: Option<bool>,
    pub lessons_batch_size: Option<i64>,
    pub lessons_presentation_order: Option<String>,
    pub reviews_autoplay_audio: Option<bool>,
    pub reviews_display_srs_indicator: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInnerUpdate {
    pub preferences: UserPreferencesUpdate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdate {
    pub user: UserInnerUpdate,
}

impl WanikaniClient {
    get!(get_user_info, "user", UserResponse);
    put!(update_user_info, "user", UserUpdate, UserResponse);
}
