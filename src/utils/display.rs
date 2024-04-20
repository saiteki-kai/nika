use nika_core::models::dictionary::jmdict::Word;

pub enum DisplayMode {
    Short,
    Long,
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::Long
    }
}

pub fn print_word(word: &Word, mode: DisplayMode) {
    let kanji = word.kanji.first().map(|k| k.text.as_str()).unwrap_or("");
    let kana = word.kana.first().map(|k| k.text.as_str()).unwrap_or("");

    match mode {
        DisplayMode::Short => println!("{} {}", kanji, kana),
        DisplayMode::Long => todo!(),
    }
}
