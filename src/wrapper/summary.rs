use crate::{
    get,
    response::{Error, SummaryResponse, WanikaniError},
    client::Client,
};

impl Client {
    get!(get_summary, "summary", SummaryResponse);
}
