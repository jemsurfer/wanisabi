use chrono::{prelude::DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuxiliaryMeaningType {
    Whitelist,
    Blacklist,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuxiliaryMeaning {
    pub meaning: String,
    #[serde(rename = "type")]
    pub kind: AuxiliaryMeaningType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ReadingType {
    Kunyomi,
    Nanori,
    Onyomi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meaning {
    pub meaning: String,
    pub primary: bool,
    pub accepted_answer: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterImageMetadata {
    pub inline_styles: Option<bool>,
    pub color: Option<String>,
    pub dimensions: Option<String>,
    pub style_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterImage {
    pub url: String,
    pub content_type: String,
    pub metadata: CharacterImageMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectRadical {
    pub auxiliary_meanings: Vec<AuxiliaryMeaning>,
    pub characters: Option<String>,
    pub created_at: DateTime<Utc>,
    pub document_url: String,
    pub hidden_at: Option<DateTime<Utc>>,
    pub lesson_position: i64,
    pub level: i64,
    pub meaning_mnemonic: String,
    pub meanings: Vec<Meaning>,
    pub slug: String,
    pub spaced_repetition_system_id: i64,
    pub amalgamation_subject_ids: Vec<i64>,
    pub character_images: Vec<CharacterImage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reading {
    pub reading: String,
    pub primary: bool,
    pub accepted_answer: bool,
    #[serde(rename = "type")]
    pub kind: ReadingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VocabularyReading {
    pub reading: String,
    pub primary: bool,
    pub accepted_answer: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectKanji {
    pub auxiliary_meanings: Vec<AuxiliaryMeaning>,
    pub characters: String,
    pub created_at: DateTime<Utc>,
    pub document_url: String,
    pub hidden_at: Option<DateTime<Utc>>,
    pub lesson_position: i64,
    pub level: i64,
    pub meaning_mnemonic: String,
    pub meanings: Vec<Meaning>,
    pub slug: String,
    pub spaced_repetition_system_id: i64,
    pub amalgamation_subject_ids: Vec<i64>,
    pub component_subject_ids: Vec<i64>,
    pub meaning_hint: Option<String>,
    pub reading_hint: Option<String>,
    pub reading_mnemonic: String,
    pub readings: Vec<Reading>,
    pub visually_similar_subject_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sentence {
    #[serde(rename = "en")]
    english: String,
    #[serde(rename = "ja")]
    japanese: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PronunciationAudioMetadata {
    pub gender: String,
    pub source_id: i64,
    pub pronunciation: String,
    pub voice_actor_id: i64,
    pub voice_actor_name: String,
    pub voice_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PronunciationAudio {
    pub url: String,
    pub content_type: String,
    pub metadata: PronunciationAudioMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectVocabulary {
    pub auxiliary_meanings: Vec<AuxiliaryMeaning>,
    pub characters: String,
    pub created_at: DateTime<Utc>,
    pub document_url: String,
    pub hidden_at: Option<DateTime<Utc>>,
    pub lesson_position: i64,
    pub level: i64,
    pub meaning_mnemonic: String,
    pub meanings: Vec<Meaning>,
    pub slug: String,
    pub spaced_repetition_system_id: i64,
    pub component_subject_ids: Vec<i64>,
    pub context_sentences: Vec<Sentence>,
    pub parts_of_speech: Vec<String>,
    pub pronunciation_audios: Vec<PronunciationAudio>,
    pub readings: Vec<VocabularyReading>,
    pub reading_mnemonic: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectKanaVocabulary {
    pub context_sentences: Vec<Sentence>,
    pub meaning_mnemonic: String,
    pub parts_of_speech: Vec<String>,
    pub pronunciation_audios: Vec<PronunciationAudio>,
    pub created_at: DateTime<Utc>,
    pub level: i64,
    pub slug: String,
    pub hidden_at: Option<DateTime<Utc>>,
    pub document_url: String,
    pub characters: String,
    pub meanings: Vec<Meaning>,
    pub auxiliary_meanings: Vec<AuxiliaryMeaning>,
    pub lesson_position: i64,
    pub spaced_repetition_system_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Subject {
    Radical(SubjectRadical),
    Kanji(SubjectKanji),
    Vocabulary(SubjectVocabulary),
    KanaVocabulary(SubjectKanaVocabulary),
}
