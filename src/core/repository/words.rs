#[derive(Debug, Clone)]
pub struct Word {
    pub text: String,
    pub reading: String,
}

pub fn daily_words() -> Vec<Word> {
    Vec::from([
        Word {
            text: String::from("放す"),
            reading: String::from("はなす"),
        },
        Word {
            text: "体力".into(),
            reading: "たいりょく".into(),
        },
        Word {
            text: "柔らかい".into(),
            reading: "やわらかい".into(),
        },
        Word {
            text: "怪しい".into(),
            reading: "あやしい".into(),
        },
        Word {
            text: "影響".into(),
            reading: "えいきょう".into(),
        },
    ])
}
