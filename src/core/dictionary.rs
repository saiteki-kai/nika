#![allow(unused)]

use std::collections::HashMap;
use std::fs;

use anyhow::Error;
use rand::seq::SliceRandom;
use rayon::prelude::*;

use super::models::jmdict::Kanji;
use crate::config::TAGS_BIN_PATH;
use crate::config::WORDS_BIN_PATH;
use crate::core::models::jmdict::Tag;
use crate::core::models::jmdict::Word;

pub type WordMap = HashMap<String, Word>;
pub type TagMap = HashMap<Tag, String>;

#[derive(Debug)]
pub struct Dictionary {
    dictionary: WordMap,
    tags: TagMap,
}

impl Dictionary {
    pub fn new() -> Result<Self, Error> {
        let dict = bincode::deserialize::<WordMap>(&fs::read(WORDS_BIN_PATH.as_path())?)?;
        let tags = bincode::deserialize::<TagMap>(&fs::read(TAGS_BIN_PATH.as_path())?)?;

        Ok(Dictionary {
            dictionary: dict,
            tags,
        })
    }

    pub fn from(dictionary: WordMap, tags: TagMap) -> Self {
        Dictionary { dictionary, tags }
    }

    pub fn word(&self, id: &str) -> Option<&Word> {
        self.dictionary.get(id)
    }

    pub fn words(&self, ids: &[&str]) -> Vec<&Word> {
        ids.iter()
            .filter_map(|&id| self.dictionary.get(id))
            .collect()
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
        println!("common words: {}", common_words.len());

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
