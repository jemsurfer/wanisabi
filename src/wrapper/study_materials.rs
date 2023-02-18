use crate::{
    get,
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wanikani_rs_model::{study_material::StudyMaterial, subject_type::SubjectType};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyMaterialFilter {
    Hidden(bool),
    Ids(Vec<i64>),
    SubjectIds(Vec<i64>),
    SubjectTypes(Vec<SubjectType>),
    UpdatedAfter(DateTime<Utc>),
}

impl WanikaniClient {
    get!(
        get_study_materials_filtered,
        "study_materials",
        StudyMaterialFilter,
        CollectionResponse<ResourceResponse<StudyMaterial>>
    );
    get!(
        get_study_materials,
        "study_materials",
        CollectionResponse<ResourceResponse<StudyMaterial>>
    );
    get!(
        get_study_material,
        "study_materials/{id}",
        ResourceResponse<StudyMaterial>,
        id: i64
    );
}
