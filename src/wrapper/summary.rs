use crate::{
    client::Client,
    get, parse_error,
    response::{SummaryResponse},
};

impl Client {
    get!(get_summary, "summary", SummaryResponse);
}
