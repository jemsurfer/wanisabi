use crate::{
    client::{Client, QueryProcessor},
    get, process_response,
    response::{CollectionResponse, IdResponse},
};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use wanisabi_model::subject::Subject;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubjectFilter {
    Ids(Vec<i64>),
    Types(Vec<String>),
    Slugs(Vec<String>),
    Levels(Vec<i64>),
    Hidden(bool),
    UpdatedAfter(DateTime<Utc>),
}

impl Client {
    get!(
        get_subjects_filtered,
        "subjects",
        SubjectFilter,
        CollectionResponse<Subject>
    );
    get!(get_subjects, "subjects", CollectionResponse<Subject>);
    get!(
        get_subject,
        "subjects/{id}",
        IdResponse<Subject>,
        id: i64
    );
}
