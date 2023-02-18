use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VoiceActor {
    pub description: String,
    pub gender: String,
    pub name: String,
}
