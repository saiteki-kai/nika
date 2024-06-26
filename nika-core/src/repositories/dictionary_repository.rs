#![allow(unused)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use bincode::Error;
use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::models::jmdict::Kanji;
use crate::models::jmdict::Tag;
use crate::models::jmdict::Word;

pub type WordMap = HashMap<String, Word>;
pub type TagMap = HashMap<Tag, String>;

#[derive(Debug)]
pub struct DictionaryRepository {
    dictionary: WordMap,
    tags: TagMap,
}

impl DictionaryRepository {
    pub fn new<P: AsRef<Path>>(words_bin_path: &P, tags_bin_path: &P) -> Result<Self, Error> {
        let dict = bincode::deserialize::<WordMap>(&fs::read(words_bin_path)?)?;
        let tags = bincode::deserialize::<TagMap>(&fs::read(tags_bin_path)?)?;

        Ok(DictionaryRepository {
            dictionary: dict,
            tags,
        })
    }

    pub fn from(dictionary: WordMap, tags: TagMap) -> Self {
        DictionaryRepository { dictionary, tags }
    }

    pub fn word(&self, id: &str) -> Option<&Word> {
        self.dictionary.get(id)
    }

    pub fn words(&self, ids: &[&str]) -> Vec<&Word> {
        ids.iter()
            .filter_map(|&id| self.dictionary.get(id))
            .collect()
    }

    pub fn tag(&self, abv: &str) -> Option<&String> {
        self.tags.get(abv)
    }

    /// Search for words in the dictionary that match the given query.
    ///
    /// Returns the `common` kanjis.
    pub fn search(&self, query: &str, common: Option<bool>) -> Vec<Word> {
        self.dictionary
            .iter()
            .filter(|(id, word)| word.kanji.iter().any(|k| k.text.contains(query)))
            .map(|(id, word)| {
                if let Some(common) = common {
                    let mut common_word = word.clone();

                    common_word.kanji = common_word
                        .kanji
                        .iter()
                        .filter(|&k| k.common == common)
                        .cloned()
                        .collect::<Vec<Kanji>>();

                    common_word
                } else {
                    word.clone()
                }
            })
            .collect::<Vec<Word>>()
    }

    pub fn num_words(&self) -> usize {
        self.dictionary.len()
    }

    pub fn random_words(&self, amount: usize) -> Vec<&Word> {
        let mut rng = rand::thread_rng();

        let common_words = self
            .dictionary
            .values()
            .collect::<Vec<&Word>>()
            .iter()
            .filter(|w| w.kanji.iter().any(|x| x.common))
            .cloned()
            .collect::<Vec<&Word>>();

        let random_keys: Vec<&String> = self
            .dictionary
            .keys()
            .collect::<Vec<&String>>()
            .choose_multiple(&mut rng, amount)
            .cloned()
            .collect();

        random_keys
            .into_par_iter()
            .filter_map(|key| self.dictionary.get(key))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use rayon::prelude::IntoParallelIterator;
    use rayon::prelude::ParallelIterator;

    use super::*;
    use crate::models::jmdict::JMdict;

    fn setup_repo() -> DictionaryRepository {
        let fixtures_path = Path::new("tests").join("fixtures").join("words.json");

        let words = fs::read_to_string(fixtures_path).unwrap();
        let data: JMdict = serde_json::from_str(&words).unwrap();

        let words: WordMap = data
            .words
            .into_par_iter()
            .map(|word| (word.id.clone(), word))
            .collect();

        let tags: TagMap = data.tags;

        DictionaryRepository::from(words, tags)
    }

    mod get_word_by_id {
        use super::setup_repo;

        #[test]
        fn test_existing_id() {
            let repo = setup_repo();

            let res = repo.word("1358280");
            assert!(res.is_some());

            if let Some(word) = res {
                assert_eq!(word.id, "1358280");
            }
        }

        #[test]
        fn test_non_existing_id() {
            let repo = setup_repo();

            let res = repo.word("9999999");
            assert!(res.is_none());
        }
    }

    mod get_words {
        use super::setup_repo;

        #[test]
        fn test_all_existing_ids() {
            let repo = setup_repo();

            let ids: Vec<&str> = vec!["1008590", "1318720"];
            let n_elements = ids.len();

            let res = repo.words(&ids);
            assert_eq!(res.len(), n_elements);

            for word in res.clone() {
                assert!(ids.contains(&word.id.as_str()));
            }
        }

        #[test]
        fn test_one_existing_id() {
            let repo = setup_repo();

            let ids: Vec<&str> = vec!["0000000", "1318720", "0000001"];

            let res = repo.words(&ids);
            assert_eq!(res.len(), 1);

            assert!(ids.contains(&res.first().unwrap().id.as_str()));
        }

        #[test]
        fn test_all_non_existing_ids() {
            let repo = setup_repo();

            let ids: Vec<&str> = vec!["0000000", "0000001"];

            let res = repo.words(&ids);
            assert_eq!(res.len(), 0);
        }

        #[test]
        fn test_empty() {
            let repo = setup_repo();

            let ids: Vec<&str> = vec![];

            let res = repo.words(&ids);
            assert_eq!(res.len(), 0);
        }
    }

    mod random_words {
        use super::setup_repo;

        #[test]
        fn test_zero_random_words() {
            let repo = setup_repo();

            let res = repo.random_words(0);
            assert!(res.is_empty());
        }

        #[test]
        fn test_one_random_word() {
            let repo = setup_repo();

            let res = repo.random_words(1);
            assert_eq!(res.len(), 1);
        }

        #[test]
        fn test_duplicated_words() {
            let repo = setup_repo();

            let res = repo.random_words(4);
            assert_eq!(res.len(), 4);

            let mut ids: Vec<String> = res.iter().map(|word| word.id.clone()).collect();
            ids.sort();
            ids.dedup();
            assert_eq!(ids.len(), 4);
        }

        #[test]
        fn test_more_than_total_words() {
            let repo = setup_repo();
            let total = repo.num_words();

            let res = repo.random_words(total + 1);
            assert_eq!(res.len(), total);
        }
    }
}
