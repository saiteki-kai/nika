use anyhow::{Error, Result};
use std::fs;
use std::sync::OnceLock;

use crate::config::{
    app_cache_dir, app_config_dir, app_data_dir, APP_NAME, CONFIG_NAME, TAGS_BIN_PATH,
    WORDS_BIN_PATH,
};
use crate::core::config::ConfigService;
use crate::core::dictionary::{Dictionary, TagMap, WordMap};

static CONFIG: OnceLock<ConfigService> = OnceLock::new();

fn set_global_config(config: ConfigService) {
    CONFIG.set(config).expect("could not set config")
}

pub fn config_service() -> &'static ConfigService {
    CONFIG.get().expect("config is not initialized")
}

static DICTIONARY: OnceLock<Dictionary> = OnceLock::new();

pub fn init_dictionary() {
    // TODO: when not initialized, run the updater

    let dict = bincode::deserialize::<WordMap>(
        &fs::read(WORDS_BIN_PATH.as_path()).expect("cannot read words"),
    )
    .expect("cannot load words");

    let tags = bincode::deserialize::<TagMap>(
        &fs::read(TAGS_BIN_PATH.as_path()).expect("cannot read tags"),
    )
    .expect("cannot load tags");

    DICTIONARY
        .set(Dictionary::from(dict, tags))
        .expect("could not initialize dictionary");
}

pub fn dictionary() -> &'static Dictionary {
    DICTIONARY
        .get()
        .expect("dictionary repository is not initialized")
}

pub fn init_folders() -> Result<(), Error> {
    fs::create_dir_all(app_cache_dir().join("data"))?;
    fs::create_dir_all(app_config_dir())?;
    fs::create_dir_all(app_data_dir())?;

    Ok(())
}

pub fn init_config() -> Result<(), Error> {
    let config_service = ConfigService::new(APP_NAME.into(), Some(CONFIG_NAME.into()));
    config_service.load_config()?;
    set_global_config(config_service);

    Ok(())
}
