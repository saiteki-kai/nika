#![allow(unused)]

use std::collections::HashMap;
use std::fs;

use anyhow::Error;
use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::core::config::{TAGS_BIN_PATH, WORDS_BIN_PATH};
use crate::core::models::jmdict::{Tag, Word};

pub type WordMap = HashMap<String, Word>;
pub type TagMap = HashMap<Tag, String>;

pub struct DictionaryRepository {
    dictionary: WordMap,
    tags: TagMap,
}

impl DictionaryRepository {
    pub fn new() -> Result<Self, Error> {
        let dict = bincode::deserialize::<WordMap>(&fs::read(WORDS_BIN_PATH.as_path())?)?;
        let tags = bincode::deserialize::<TagMap>(&fs::read(TAGS_BIN_PATH.as_path())?)?;

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

    pub fn random_words(&self, amount: usize) -> Vec<&Word> {
        let mut rng = rand::thread_rng();

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
