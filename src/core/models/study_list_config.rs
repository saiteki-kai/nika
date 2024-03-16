use std::collections::BTreeMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct StudyConfig {
    pub current_index: usize,
    pub items_per_day: usize,
}

impl Default for StudyConfig {
    fn default() -> Self {
        StudyConfig {
            current_index: 0,
            items_per_day: 5,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
pub struct StudyListConfig {
    pub current: Option<String>,
    pub lists: BTreeMap<String, StudyConfig>,
}
