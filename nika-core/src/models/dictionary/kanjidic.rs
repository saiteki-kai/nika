use serde_derive::{Deserialize, Serialize};

pub type Language2Letter = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kanjidic {
    #[serde(rename = "dictDate")]
    pub date: String,
    pub version: String,
    pub characters: Vec<Kanji>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kanji {
    pub misc: Misc,
    pub literal: String,
    pub reading_meaning: Option<ReadingMeaning>,
    // queryCodes
    // radicals
    // codepoints
    // dictionaryReferences
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadingMeaning {
    pub groups: Vec<ReadingMeaningGroup>,
    pub nanori: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadingMeaningGroup {
    pub meanings: Vec<Meaning>,
    pub readings: Vec<Reading>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meaning {
    pub lang: Language2Letter,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Reading {
    #[serde(rename = "ja_on")]
    JapaneseOn(String),
    #[serde(rename = "ja_kun")]
    JapaneseKun(String),
    #[serde(rename = "korean_r")]
    KoreanRomanji(String),
    #[serde(rename = "korean_h")]
    KoreanHangul(String),
    #[serde(rename = "pinyin")]
    Pinyin(String),
    #[serde(rename = "vietnam")]
    Vietnam(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Misc {
    pub grade: Option<u16>,
    pub frequency: Option<u16>,
    pub jlpt_level: Option<u8>,
    pub stroke_counts: Vec<u8>,
    // variants
    // radical_names
}
