pub trait JapaneseCharacter {
    fn is_kanji(&self) -> bool;
}

impl JapaneseCharacter for char {
    fn is_kanji(&self) -> bool {
        ('\u{4e00}'..='\u{9faf}').contains(self)
    }
}

pub trait JapaneseString {
    fn has_kanji(&self) -> bool;
}

impl JapaneseString for &str {
    fn has_kanji(&self) -> bool {
        self.chars().any(|c| c.is_kanji())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case('日', true)]
    #[test_case('ひ', false)]
    #[test_case('ミ', false)]
    #[test_case('S', false)]
    fn test_kanji_caracter(character: char, expected: bool) {
        assert_eq!(character.is_kanji(), expected);
    }

    #[test_case("日本語", true)]
    #[test_case("This 文字 contains kanji", true)]
    #[test_case("This ストリング does not contain kanji", false)]
    fn test_kanji_string(text: &str, expected: bool) {
        assert_eq!(text.has_kanji(), expected);
    }
}
