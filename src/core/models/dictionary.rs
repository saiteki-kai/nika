use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

pub type Tag = String;
pub type Language3Letter = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JMdict {
    pub version: String,
    #[serde(rename = "dictDate")]
    pub date: String,
    pub tags: HashMap<Tag, String>,
    pub words: Vec<Word>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Word {
    pub id: String,
    pub kana: Vec<Kana>,
    pub kanji: Vec<Kanji>,
    pub sense: Vec<Sense>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kanji {
    pub common: bool,
    pub tags: Vec<Tag>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kana {
    pub applies_to_kanji: Vec<String>,
    pub common: bool,
    pub tags: Vec<Tag>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sense {
    pub antonym: Vec<Xref>,
    pub applies_to_kana: Vec<String>,
    pub applies_to_kanji: Vec<String>,
    pub dialect: Vec<Tag>,
    pub field: Vec<Tag>,
    pub gloss: Vec<Gloss>,
    pub info: Vec<String>,
    pub language_source: Vec<LanguageSource>,
    pub misc: Vec<Tag>,
    pub part_of_speech: Vec<Tag>,
    pub related: Vec<Xref>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Xref {
    /// (kanji, kana, sense_index)
    WordReadingIndex((String, String, u64)),
    /// (kanji, kana)
    WordReading((String, String)),
    /// (kanji_or_kana, sense_index)
    WordIndex((String, u64)),
    /// kanji_or_kana
    Word((String,)),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageSource {
    pub full: bool,
    pub lang: Language3Letter,
    pub text: Option<String>,
    pub wasei: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GlossType {
    Literal,
    Figurative,
    Explanation,
    Trademark,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Gloss {
    pub gender: Option<Gender>,
    pub lang: Language3Letter,
    pub text: String,
    #[serde(rename = "type")]
    pub gloss_type: Option<GlossType>,
}

#[cfg(test)]
mod tests {
    use serde_json::Error;

    use super::*;

    #[test]
    fn test_xref() {
        // xref - word reading index
        let json_str = r#"["丸", "まる・1", 1]"#;
        let xref: Xref = serde_json::from_str(json_str).unwrap();

        let expected = Xref::WordReadingIndex(("丸".to_string(), "まる・1".to_string(), 1));
        assert_eq!(xref, expected);

        // xref - word reading
        let json_str = r#"["丸", "まる"]"#;
        let xref: Xref = serde_json::from_str(json_str).unwrap();

        let expected = Xref::WordReading(("丸".to_string(), "まる".to_string()));
        assert_eq!(xref, expected);

        // xref - word index
        let json_str = r#"["丸", 1]"#;
        let xref: Xref = serde_json::from_str(json_str).unwrap();

        let expected = Xref::WordIndex(("丸".to_string(), 1));
        assert_eq!(xref, expected);

        // xref - word
        let json_str = r#"["丸"]"#;
        let xref: Xref = serde_json::from_str(json_str).unwrap();

        let expected = Xref::Word(("丸".to_string(),));
        assert_eq!(xref, expected);

        // xref - invalid value
        let json_str = r#"[1, 2, 3]"#;
        let result: Result<Xref, Error> = serde_json::from_str(json_str);

        assert!(result.is_err());
    }

    #[test]
    fn test_related() {
        let json_str = r#"{"related": [["どの"], ["その", 1]]}"#;
        let data: HashMap<String, Vec<Xref>> = serde_json::from_str(json_str).unwrap();

        assert_eq!(data["related"].len(), 2);

        let related = data.get("related").unwrap();

        assert_eq!(related[0], Xref::Word(("どの".to_string(),)));
        assert_eq!(related[1], Xref::WordIndex(("その".to_string(), 1)));
    }
}
