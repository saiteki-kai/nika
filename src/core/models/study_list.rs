use std::fs;
use std::path::Path;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::core::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StudyConfig {
    pub current_index: usize,
    pub items_per_day: usize,
}

impl Default for StudyConfig {
    fn default() -> Self {
        Self {
            current_index: 0,
            items_per_day: 5,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StudyList {
    pub name: String,
    pub config: StudyConfig,
    pub items: Vec<String>,
}

impl StudyList {
    pub fn new(name: &str, items: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            config: StudyConfig::default(),
            items,
        }
    }

    pub fn load<P: AsRef<Path>>(filepath: &P) -> Result<StudyList> {
        let file = fs::read(filepath)?;
        let config = bincode::deserialize::<StudyList>(&file)?;

        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(filepath: &P, study_list: StudyList) -> Result<()> {
        let content = bincode::serialize::<StudyList>(&study_list)?;
        fs::write(filepath, content)?;

        Ok(())
    }
}
