use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubjectType {
    Kanji,
    Radical,
    Vocabulary,
}
