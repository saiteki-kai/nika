#![allow(unused)]
extern crate confy;
extern crate serde;

use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::utils::links::Link;

const APP_NAME: &str = "nika";
const CONFIG_NAME: &str = "config";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub dictionaries: Vec<Link>,
}

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig {
            dictionaries: vec![
                Link {
                    text: "Jisho.org".into(),
                    base_url: "https://jisho.org/search/".into(),
                },
                Link {
                    text: "Jpdb.io".into(),
                    base_url: "https://jpdb.io/search?q=".into(),
                },
                Link {
                    text: "Weblio.jp".into(),
                    base_url: "https://www.weblio.jp/content/".into(),
                },
                Link {
                    text: "Goo.ne.jp".into(),
                    base_url: "https://dictionary.goo.ne.jp/word/".into(),
                },
            ],
        }
    }
}

pub fn config_path() -> Result<PathBuf, ConfyError> {
    confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)
}

pub fn load_config() -> Result<UserConfig, ConfyError> {
    confy::load(APP_NAME, CONFIG_NAME)
}

pub fn save_config(config: UserConfig) -> Result<(), ConfyError> {
    confy::store(APP_NAME, CONFIG_NAME, config)
}
