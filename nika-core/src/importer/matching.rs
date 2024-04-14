use std::collections::HashMap;
use std::collections::HashSet;

use rayon::prelude::*;

use super::query::Query;
use crate::models::jmdict::Kana;
use crate::models::jmdict::Kanji;
use crate::models::jmdict::Word;
use crate::utils::japanese::JapaneseString;

pub struct Matcher {
    words: Vec<Word>,
    senses_map: HashMap<String, HashSet<String>>,
}

impl Matcher {
    pub fn new(words: Vec<Word>, senses_map: HashMap<String, HashSet<String>>) -> Self {
        Self { words, senses_map }
    }

    pub fn find(
        &self,
        query: &Query,
        common: Option<bool>,
        first_only: Option<bool>,
    ) -> Vec<&Word> {
        let results: Vec<&Word> = self
            .words
            .par_iter()
            .filter(|word| filter_word(word, query, common, first_only))
            .collect();

        if results.len() > 1 {
            let mut debug_info = String::new();

            if let Some(meaning) = &query.meaning {
                let context_words = meaning.split_whitespace().collect::<HashSet<&str>>();

                debug_info += &format!(
                    "\n\nLooking for {} ({})\n",
                    query.kanji.clone().unwrap(),
                    context_words
                        .iter()
                        .map(|w| w.to_owned())
                        .collect::<Vec<&str>>()
                        .join(", ")
                );

                let mut counts = HashMap::<&Word, usize>::new();

                for word in &results {
                    let senses = self
                        .senses_map
                        .get(&word.id)
                        .unwrap_or_else(|| {
                            panic!(
                                "Senses not found for word {}",
                                word.kanji
                                    .first()
                                    .map_or(word.kana.first().unwrap().text.as_str(), |k| k
                                        .text
                                        .as_str())
                            )
                        })
                        .iter()
                        .map(|s| s.as_str())
                        .collect();

                    let n_overlaps = context_words.intersection(&senses).count();

                    debug_info += &format!(
                        "[{}]\t({}, {})\n",
                        n_overlaps,
                        word.kanji
                            .iter()
                            .map(|k| k.text.clone())
                            .collect::<Vec<String>>()
                            .join(", "),
                        senses
                            .iter()
                            .map(|s| s.to_owned())
                            .collect::<Vec<&str>>()
                            .join(", "),
                    );

                    counts.insert(word, n_overlaps);
                }

                debug_info += "\n";

                let max_count = *counts.values().max().unwrap_or(&0);

                let r = results
                    .iter()
                    .filter(|w| *counts.get(*w).unwrap_or(&0) == max_count)
                    .copied()
                    .collect::<Vec<&Word>>();

                if r.len() > 1 {
                    println!("{}", debug_info);
                }

                return r;
            }
        }

        results
    }
}

trait KanjiOrKana {
    fn text(&self) -> &String;
    fn common(&self) -> &bool;

    fn matches(&self, text: &str, common: &Option<bool>) -> bool {
        if let Some(common) = common {
            self.text() == text && self.common() == common
        } else {
            self.text() == text
        }
    }
}

impl KanjiOrKana for Kanji {
    fn text(&self) -> &String {
        &self.text
    }

    fn common(&self) -> &bool {
        &self.common
    }
}

impl KanjiOrKana for Kana {
    fn text(&self) -> &String {
        &self.text
    }

    fn common(&self) -> &bool {
        &self.common
    }
}

fn filter_word(word: &Word, query: &Query, common: Option<bool>, first_only: Option<bool>) -> bool {
    match (&query.kanji, &query.kana) {
        (Some(kanji), Some(kana)) => {
            if kanji == kana {
                filter_kanji_or_kana(word, kanji, common, first_only)
            } else {
                filter_kanji_kana(word, kanji, kana, common, first_only)
            }
        }
        (Some(kanji_or_kana), None) | (None, Some(kanji_or_kana)) => {
            filter_kanji_or_kana(word, kanji_or_kana, common, first_only)
        }
        _ => false,
    }
}

fn filter<T: KanjiOrKana + Sync>(
    elements: &[T],
    text: &str,
    common: Option<bool>,
    first_only: Option<bool>,
) -> bool {
    if let Some((first, rest)) = elements.split_first() {
        return match first_only {
            Some(true) => first.matches(text, &common),
            Some(false) => rest.par_iter().any(|k| k.matches(text, &common)),
            None => elements.par_iter().any(|k| k.matches(text, &common)),
        };
    }

    false
}

fn filter_kanji_or_kana(
    word: &Word,
    text: &str,
    common: Option<bool>,
    first_only: Option<bool>,
) -> bool {
    if text.has_kanji() {
        filter(&word.kanji, text, common, first_only)
    } else {
        filter(&word.kana, text, common, first_only)
    }
}

fn filter_kanji_kana(
    word: &Word,
    kanji: &str,
    kana: &str,
    common: Option<bool>,
    first_only: Option<bool>,
) -> bool {
    filter(&word.kanji, kanji, common, first_only) && word.kana.par_iter().any(|k| k.text == *kana)
}

// TODO: test order of the entries after the import.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use test_case::test_case;

    use super::*;
    use crate::repositories::dictionary_repository::WordMap;

    fn setup() -> Matcher {
        let dict = bincode::deserialize::<WordMap>(
            &fs::read(PathBuf::from("/home/giuseppe/.local/share/nika/jmdict-words.bin").as_path())
                .expect("cannot read words"),
        )
        .expect("cannot load words");

        Matcher::new(dict.values().cloned().collect(), HashMap::new())
    }

    #[test]
    fn test_match_other_kanji_forms() {
        let matcher = setup();

        let query = Query::new(Some("空しい".to_owned()), Some("むなしい".to_owned()), None);

        let results = matcher.find(&query, None, None);
        assert_eq!(results.len(), 1);
    }

    #[test_case("空", 3)]
    #[test_case("そら", 2)]
    fn test_match_kana_multiple_results(kanji_or_kana: &str, count: usize) {
        let matcher = setup();

        let query = Query::new(Some(kanji_or_kana.to_owned()), None, None);
        let results = matcher.find(&query, None, None);

        assert_eq!(results.len(), count);
    }

    #[test_case("きそく", Some(true), 1)]
    #[test_case("きそく", None, 4)]
    fn test_match_common_kana(kana: &str, common: Option<bool>, count: usize) {
        let matcher = setup();

        let query = Query::new(None, Some(kana.to_owned()), None);
        let results = matcher.find(&query, common, None);

        assert_eq!(results.len(), count);
    }

    #[test]
    fn test_match_kanji_kana_one_result() {
        let matcher = setup();

        let query = Query::new(Some("髪".to_owned()), Some("かみ".to_owned()), None);
        let results = matcher.find(&query, None, None);

        assert_eq!(results.len(), 1);
    }

    #[test_case("髪髪髪", "かみ", 0)]
    #[test_case("髪", "っっっっっ", 0)]
    fn test_match_non_existing_kanji_kana(kanji: &str, kana: &str, count: usize) {
        let matcher = setup();

        let query = Query::new(Some(kanji.to_owned()), Some(kana.to_owned()), None);
        let results = matcher.find(&query, None, None);

        assert_eq!(results.len(), count);
    }

    #[test_case(None, 9)]
    #[test_case(Some("recycle for paper use".to_owned()), 1)]
    fn test_match_meaning(meaning: Option<String>, count: usize) {
        let matcher = setup();

        let query = Query::new(Some("こし".to_owned()), Some("こし".to_owned()), meaning);
        let results = matcher.find(&query, None, Some(true));

        assert_eq!(results.len(), count);
    }
}
