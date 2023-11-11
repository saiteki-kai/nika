use std::fs;
use std::sync::OnceLock;

use crate::cli::config::UserConfig;
use crate::config::{TAGS_BIN_PATH, WORDS_BIN_PATH};
use crate::core::repository::dictionary_repository::{DictionaryRepository, TagMap, WordMap};

static CONFIG: OnceLock<UserConfig> = OnceLock::new();

pub fn set_global_config(config: UserConfig) {
    CONFIG.set(config).expect("could not set config")
}

pub fn get_global_config() -> &'static UserConfig {
    CONFIG.get().expect("config is not initialized")
}

static WORD_REPOSITORY: OnceLock<DictionaryRepository> = OnceLock::new();

pub fn init_word_repository() {
    let dict = bincode::deserialize::<WordMap>(
        &fs::read(WORDS_BIN_PATH.as_path()).expect("cannot read words"),
    )
    .expect("cannot load words");

    let tags = bincode::deserialize::<TagMap>(
        &fs::read(TAGS_BIN_PATH.as_path()).expect("cannot read tags"),
    )
    .expect("cannot load tags");

    WORD_REPOSITORY
        .set(DictionaryRepository::from(dict, tags))
        .expect("could not initialize dictionary");
}

pub fn word_repository() -> &'static DictionaryRepository {
    WORD_REPOSITORY
        .get()
        .expect("dictionary repository is not initialized")
}
