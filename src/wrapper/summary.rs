use crate::{
    client::Client,
    get, parse_error,
    response::{Error, SummaryResponse, WanikaniError},
};

impl Client {
    get!(get_summary, "summary", SummaryResponse);
}
