#![allow(unused)]
extern crate confy;
extern crate serde;

use std::path::PathBuf;

use anyhow::Context;
use anyhow::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::core::models::link::Link;

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
