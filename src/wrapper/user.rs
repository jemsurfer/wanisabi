use crate::response::UserResponse;
use reqwest::Error;
use serde::Serialize;

use crate::{get, wanikani_client::WanikaniClient};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserQueryParam {}

impl WanikaniClient {
    get!(get_user_info, "user", UserResponse);
}
