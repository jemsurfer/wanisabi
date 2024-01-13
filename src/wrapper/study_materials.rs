use crate::{
    get, post, put,
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wanikani_rs_model::{study_material::StudyMaterial, subject_type::SubjectType};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StudyMaterialFilter {
    Hidden(bool),
    Ids(Vec<i64>),
    SubjectIds(Vec<i64>),
    SubjectTypes(Vec<SubjectType>),
    UpdatedAfter(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudyMaterialCreate {
    pub subjet_id: i64,
    pub meaning_note: Option<String>,
    pub reading_note: Option<String>,
    pub meaning_synonyms: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StudyMaterialCreateWrapper {
    study_material: StudyMaterialCreate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudyMaterialUpdate {
    pub meaning_note: Option<String>,
    pub reading_note: Option<String>,
    pub meaning_synonyms: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StudyMaterialUpdateWrapper {
    study_material: StudyMaterialUpdate,
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
    post!(
        create_study_material,
        "study_materials/",
        StudyMaterialCreate,
        ResourceResponse<StudyMaterial>,
        StudyMaterialCreateWrapper,
        study_material
    );
    put!(
        update_study_material,
        "study_materials/{id}",
        StudyMaterialUpdate,
        ResourceResponse<StudyMaterial>,
        StudyMaterialUpdateWrapper,
        study_material,
        id: i64
    );
}
