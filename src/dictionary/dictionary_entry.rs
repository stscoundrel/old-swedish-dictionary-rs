use serde::{Deserialize, Serialize};

/// Individual dictionary entry.
/// Each entry contains word, and at least one definition for the word.
/// Many words have multiple definitions, or additional info.
///
#[derive(Serialize, Deserialize)]
pub struct DictionaryEntry {
    #[serde(alias = "a")]
    pub headword: String,
    #[serde(alias = "b")]
    pub part_of_speech: Vec<String>,
    #[serde(alias = "c")]
    pub grammatical_aspect: String,
    #[serde(alias = "d")]
    pub information: String,
    #[serde(alias = "e")]
    pub definitions: Vec<String>,
    #[serde(alias = "f")]
    pub alternative_forms: Vec<String>
}