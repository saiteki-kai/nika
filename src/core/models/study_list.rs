use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct StudyListConfig {
    pub current: Option<String>,
    pub lists: HashMap<String, StudyConfig>,
}
