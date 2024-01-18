use crate::{
    get,
    response::{ErrorResponse, SummaryResponse, WanikaniError},
    wanikani_client::WanikaniClient,
};

impl WanikaniClient {
    get!(get_summary, "summary", SummaryResponse);
}
