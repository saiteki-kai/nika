use std::fs;
use std::path::Path;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::core::errors::Result;
use crate::core::models::link::Link;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub dictionaries: Vec<Link>,
    #[serde(default)]
    pub current_list: Option<String>,
}

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig {
            current_list: None,
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

impl UserConfig {
    pub fn load<P: AsRef<Path>>(filepath: &P) -> Result<UserConfig> {
        if !filepath.as_ref().exists() {
            let default_config = UserConfig::default();
            UserConfig::save(filepath, default_config.clone())?;

            return Ok(default_config);
        }

        let file = fs::read_to_string(filepath.as_ref())?;
        let config = toml::from_str::<UserConfig>(&file)?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(filepath: &P, config: UserConfig) -> Result<()> {
        let content = toml::to_string::<UserConfig>(&config)?;
        fs::write(filepath.as_ref(), content)?;
        Ok(())
    }
}
