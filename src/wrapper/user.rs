use crate::{put, response::UniqueResponse};

use serde::{Deserialize, Serialize};
use wanisabi_model::user::User;

use crate::{client::Client, get};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPreferencesUpdate {
    pub default_voice_actor_id: Option<i64>,
    pub lessons_autoplay_audio: Option<bool>,
    pub lessons_batch_size: Option<i64>,
    pub lessons_presentation_order: Option<LessonPresentationOrder>,
    pub reviews_autoplay_audio: Option<bool>,
    pub reviews_display_srs_indicator: Option<bool>,
    pub reviews_presentation_order: Option<ReviewsPresentationOrder>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum LessonPresentationOrder {
    #[default]
    AscendingLevelThenSubject,
    Shuffled,
    AscendingLevelThenShuffled,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReviewsPresentationOrder {
    #[default]
    Shuffled,
    LowerLevelsFirst,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserUpdate {
    pub preferences: UserPreferencesUpdate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserUpdateWrapper {
    pub user: UserUpdate,
}

impl Client {
    get!(get_user_info, "user", UniqueResponse<User>);
    put!(
        update_user_info,
        "user",
        UserUpdate,
        UniqueResponse<User>,
        UserUpdateWrapper,
        user
    );
}
