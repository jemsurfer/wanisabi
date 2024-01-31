use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubjectType {
    KanaVocabulary,
    Kanji,
    Radical,
    Vocabulary,
}
