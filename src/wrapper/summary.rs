use wanisabi_model::summary::Summary;

use crate::{client::Client, get, response::UniqueResponse};

impl Client {
    get!(get_summary, "summary", UniqueResponse<Summary>);
}
