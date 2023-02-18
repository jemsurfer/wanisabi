use crate::response::UserResponse;
use reqwest::Error;

use crate::{get, wanikani_client::WanikaniClient};

impl WanikaniClient {
    get!(get_user_info, "user", UserResponse);
}
