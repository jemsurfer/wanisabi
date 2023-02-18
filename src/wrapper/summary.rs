use crate::{get, response::SummaryResponse, wanikani_client::WanikaniClient};
use reqwest::Error;

impl WanikaniClient {
    get!(get_summary, "summary", SummaryResponse);
}
