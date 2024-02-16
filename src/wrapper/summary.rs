use wanisabi_model::summary::Summary;

use crate::{client::Client, get, process_response, response::UniqueResponse};

impl Client {
    get!(get_summary, "summary", UniqueResponse<Summary>);
}
