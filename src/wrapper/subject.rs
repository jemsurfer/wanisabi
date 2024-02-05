use crate::{
    client::{Client, QueryProcessor},
    get, parse_error,
    response::{CollectionResponse, Error, ResourceResponse, WanikaniError},
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
        CollectionResponse<ResourceResponse<Subject>>
    );
    get!(
        get_subjects,
        "subjects",
        CollectionResponse<ResourceResponse<Subject>>
    );
    get!(
        get_subject,
        "subjects/{id}",
        ResourceResponse<Subject>,
        id: i64
    );
}
