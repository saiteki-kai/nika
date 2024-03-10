use std::{fs, path::PathBuf};

use crate::core::models::study_list::StudyConfig;

use super::study_list_stats::{Result, StudyListStats};

pub struct StudyListManager {
    dirpath: PathBuf,
    stats: StudyListStats,
}

impl StudyListManager {
    pub fn new(dirpath: PathBuf, stats_file: PathBuf) -> Result<Self> {
        let stats = StudyListStats::new(stats_file)?;

        Ok(StudyListManager { dirpath, stats })
    }

    pub fn add(&mut self, name: &str, filepath: &PathBuf) -> Result<()> {
        let new_filepath = self.dirpath.join(name);
        fs::copy(filepath, new_filepath)?;

        self.stats.update_stats(name, StudyConfig::default())
    }

    pub fn remove(&mut self, name: &str) -> Result<()> {
        let filepath = self.dirpath.join(name);
        fs::remove_file(filepath)?;

        self.stats.remove_stats(name)
    }

    pub fn select(&mut self, name: &str) -> Result<()> {
        self.stats.select_list(name)
    }

    pub fn get(&self, name: &str) -> Option<&StudyConfig> {
        self.stats.get_list(name)
    }

    pub fn list(&self) -> Vec<String> {
        self.stats.get_lists()
    }
}
