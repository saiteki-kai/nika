use std::fmt::Display;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Query {
    pub kanji: Option<String>,
    pub kana: Option<String>,
    pub meaning: Option<String>,
}

impl Query {
    pub fn new(kanji: Option<String>, kana: Option<String>, meaning: Option<String>) -> Self {
        Self {
            kanji,
            kana,
            meaning,
        }
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kanji: {:?}, kana: {:?}", self.kanji, self.kana)
    }
}
