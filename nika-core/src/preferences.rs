use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::errors::Result;
use crate::models::link::Link;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    #[serde(skip)]
    filepath: PathBuf,
    pub external_dictionaries: Vec<Link>,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            filepath: PathBuf::default(),
            external_dictionaries: vec![
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

impl UserPreferences {
    /// Save user preferences to file.
    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty::<Self>(self)?;
        fs::write(&self.filepath, content)?;

        Ok(())
    }

    /// Load user preferences from file or return default preferences if file
    /// doesn't exist.
    pub fn load<P: AsRef<Path>>(filepath: &P) -> Result<Self> {
        if !filepath.as_ref().exists() {
            let default_config = Self {
                filepath: filepath.as_ref().to_path_buf(),
                ..UserPreferences::default()
            };
            default_config.save()?;
            return Ok(default_config);
        }

        let file = fs::read_to_string(filepath)?;

        let mut preferences = serde_json::from_str::<Self>(&file)?;
        preferences.filepath = filepath.as_ref().to_path_buf();

        Ok(preferences)
    }
}
