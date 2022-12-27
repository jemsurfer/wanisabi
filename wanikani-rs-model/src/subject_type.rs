#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum SubjectType {
    Kanji,
    Radical,
    Vocabulary
}
